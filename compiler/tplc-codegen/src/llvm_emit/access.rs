//! access — split from llvm_emit.rs.
use super::*;

impl<'ctx> Gen<'ctx> {
    pub(crate) fn i8p(&self) -> inkwell::types::PointerType<'ctx> {
        self.ctx.ptr_type(inkwell::AddressSpace::default())
    }

    /// gep on an i8* base by constant offset
    pub(crate) fn off_ptr(&self, base: PointerValue<'ctx>, off: u64) -> PointerValue<'ctx> {
        unsafe {
            self.b
                .build_in_bounds_gep(
                    self.ctx.i8_type(),
                    base,
                    &[self.ctx.i64_type().const_int(off, false)],
                    "p",
                )
                .unwrap()
        }
    }

    /// ctx field load: i8* ctx + off → typed load
    pub(crate) fn ctx_load(&self, off: u64, ty: BasicTypeEnum<'ctx>, n: &str) -> BasicValueEnum<'ctx> {
        let p = self.off_ptr(self.ctx_ptr, off);
        self.b.build_load(ty, p, n).unwrap()
    }

    pub(crate) fn blob_ptr(&self) -> PointerValue<'ctx> {
        self.this_ptr
    }

    /// array index expr → i64 offset arithmetic operand; the index
    /// keeps its own width in MIR, so widen/truncate here honoring
    /// signedness (unsigned indices zero-extend)
    pub(crate) fn arr_idx(&mut self, ctx: &Ctx, idx: &Expr) -> IntValue<'ctx> {
        let v = self.expr(ctx, idx).into_int_value();
        let bits = v.get_type().get_bit_width();
        let i64t = self.ctx.i64_type();
        let signed = matches!(&idx.ty, Type::Int { signed: true, .. });
        if bits < 64 {
            if signed {
                self.b.build_int_s_extend(v, i64t, "ix").unwrap()
            } else {
                self.b.build_int_z_extend(v, i64t, "ix").unwrap()
            }
        } else if bits > 64 {
            self.b.build_int_truncate(v, i64t, "ix").unwrap()
        } else {
            v
        }
    }

    // ---- ref resolution → (ptr, Type) ----
    pub(crate) fn member_of(&self, fbname: &str) -> tplc_mir::layout::FbLayout {
        self.layout
            .fb_types
            .get(fbname)
            .cloned()
            .or_else(|| builtin_fb_layout(fbname))
            .unwrap_or_else(|| panic!("no FB layout {fbname}"))
    }

    pub(crate) fn resolve(&mut self, ctx: &Ctx, r: &Ref) -> (PointerValue<'ctx>, Type) {
        match r {
            /* r^: the slot holds a blob offset; null → -17, out of
            range → -18. The pointee type comes from the slot's
            declared Ref type. */
            Ref::Deref(inner) => {
                let (slot, ty) = self.resolve(ctx, inner);
                let to = match &ty {
                    Type::Ref { to } => (**to).clone(),
                    _ => Type::Bool,
                };
                let i64t = self.ctx.i64_type();
                let off = self
                    .b
                    .build_load(i64t, slot, "ro")
                    .unwrap()
                    .into_int_value();
                let f = self.b.get_insert_block().unwrap().get_parent().unwrap();
                let ok_bb = self.ctx.append_basic_block(f, "ref.ok");
                let null = self
                    .b
                    .build_int_compare(IntPredicate::EQ, off, i64t.const_int(u64::MAX, false), "rn")
                    .unwrap();
                let cur = self.b.get_insert_block().unwrap();
                self.b
                    .build_conditional_branch(null, self.panic_bb.unwrap(), ok_bb)
                    .unwrap();
                self.b.position_at_end(ok_bb);
                let blob_sz = i64t.const_int(self.layout.total_size, false);
                let end = self
                    .b
                    .build_int_add(off, i64t.const_int(to.size(), false), "re")
                    .unwrap();
                let over = self
                    .b
                    .build_int_compare(IntPredicate::UGT, end, blob_sz, "rb")
                    .unwrap();
                let ok2 = self.ctx.append_basic_block(f, "ref.ok2");
                self.b
                    .build_conditional_branch(over, self.bounds_bb.unwrap(), ok2)
                    .unwrap();
                self.b.position_at_end(ok2);
                let _ = cur;
                let base8 = self
                    .b
                    .build_pointer_cast(self.blob_ptr(), self.i8p(), "b8")
                    .unwrap();
                let p = unsafe {
                    self.b
                        .build_gep(self.ctx.i8_type(), base8, &[off], "rd")
                        .unwrap()
                };
                (p, to)
            }
            Ref::Var(name) => {
                let (o, t) = ctx
                    .get(name.as_str())
                    .cloned()
                    .unwrap_or_else(|| panic!("unresolved var {name}"));
                let p = self.off_ptr(self.blob_ptr(), o);
                /* VAR_IN_OUT slot: an i64 holding the caller storage
                pointer — null → TPLC_TASK_E_PANIC like a null REF */
                if let Type::Ref { to } = &t {
                    if self.cur_inouts.contains(name.as_str()) {
                        let i64t = self.ctx.i64_type();
                        let iv = self.b.build_load(i64t, p, "iop").unwrap().into_int_value();
                        let f = self.b.get_insert_block().unwrap().get_parent().unwrap();
                        let ok_bb = self.ctx.append_basic_block(f, "io.ok");
                        let null = self
                            .b
                            .build_int_compare(IntPredicate::EQ, iv, i64t.const_int(0, false), "ion")
                            .unwrap();
                        self.b
                            .build_conditional_branch(null, self.panic_bb.unwrap(), ok_bb)
                            .unwrap();
                        self.b.position_at_end(ok_bb);
                        let pp = self.b.build_int_to_ptr(iv, self.i8p(), "iopd").unwrap();
                        return (pp, (**to).clone());
                    }
                }
                (p, t)
            }
            Ref::Field(inner, name) => {
                let (p, ty) = self.resolve(ctx, inner);
                match &ty {
                    Type::Fb { name: fb } => {
                        let fbl = self.member_of(fb);
                        let (moff, mty) = fbl.members[name.as_str()].clone();
                        (self.off_ptr(p, moff), mty)
                    }
                    Type::Struct { fields, .. } => {
                        let mut o = 0u64;
                        for (n, t) in fields {
                            o = (o + t.align() - 1) & !(t.align() - 1);
                            if n == name {
                                return (self.off_ptr(p, o), t.clone());
                            }
                            o += t.size();
                        }
                        panic!("no field {name}");
                    }
                    _ => panic!("field on non-aggregate"),
                }
            }
            Ref::Index(inner, idx) => {
                let (p, ty) = self.resolve(ctx, inner);
                if let Type::Array { elem, lo, .. } = ty {
                    let iv = self.arr_idx(ctx, idx);
                    let iv = self
                        .b
                        .build_int_sub(iv, self.ctx.i64_type().const_int(lo as u64, true), "idx")
                        .unwrap();
                    let esz = self.ctx.i64_type().const_int(elem.size(), false);
                    let bo = self.b.build_int_mul(iv, esz, "bo").unwrap();
                    unsafe {
                        let q = self
                            .b
                            .build_gep(self.ctx.i8_type(), p, &[bo], "ai")
                            .unwrap();
                        (q, *elem)
                    }
                } else {
                    panic!("index on non-array")
                }
            }
        }
    }

    pub(crate) fn read(&mut self, ctx: &Ctx, r: &Ref) -> BasicValueEnum<'ctx> {
        let (p, ty) = self.resolve(ctx, r);
        if is_bool(&ty) {
            let v = self.b.build_load(self.ctx.i8_type(), p, "lb").unwrap();
            self.b
                .build_int_compare(
                    IntPredicate::NE,
                    v.into_int_value(),
                    self.ctx.i8_type().const_zero(),
                    "lb1",
                )
                .unwrap()
                .into()
        } else {
            self.b.build_load(llty(self.ctx, &ty), p, "l").unwrap()
        }
    }

    pub(crate) fn write(&mut self, ctx: &Ctx, r: &Ref, v: BasicValueEnum<'ctx>, from: &Type) {
        let (p, ty) = self.resolve(ctx, r);
        /* narrow/widen the value to the slot width first — a literal
        or wider expr into a BYTE/WORD member must not clobber the
        neighbors (stores are raw-width, no implicit truncation) */
        let v = self.coerce(v, from, &ty);
        let v = if is_bool(&ty) {
            self.b
                .build_int_z_extend(v.into_int_value(), self.ctx.i8_type(), "z")
                .unwrap()
                .into()
        } else if let Type::Str { cap } = &ty {
            // source array may differ in width (literal cap vs target
            // cap): rebuild to the target width, zero-padding/truncating
            self.str_fit(v, 1 + *cap as u32)
        } else if let Type::WStr { cap } = &ty {
            self.str_fit(v, 1 + *cap as u32)
        } else {
            v
        };
        self.b.build_store(p, v).unwrap();
    }

    /// coerce an array value to `want` elements, zero-padding or
    /// truncating (STRING store semantics: len byte + data, rest 0)
    pub(crate) fn str_fit(&mut self, v: BasicValueEnum<'ctx>, want: u32) -> BasicValueEnum<'ctx> {
        let av = match v {
            BasicValueEnum::ArrayValue(a) => a,
            other => return other,
        };
        let have = av.get_type().len();
        if have == want {
            return av.into();
        }
        let elem = av.get_type().get_element_type().into_int_type();
        let zeros: Vec<inkwell::values::IntValue<'ctx>> = vec![elem.const_zero(); want as usize];
        let mut acc = elem.const_array(&zeros);
        for i in 0..want.min(have) {
            let el = self.b.build_extract_value(av, i, "se").unwrap();
            acc = self
                .b
                .build_insert_value(acc, el, i, "si")
                .unwrap()
                .into_array_value();
        }
        if have > want {
            /* truncating store: the len element must not claim data
            that no longer fits (a len > cap is a malformed string) */
            let len = self
                .b
                .build_extract_value(acc, 0, "tl")
                .unwrap()
                .into_int_value();
            let i32t = self.ctx.i32_type();
            let len32 = self.as_i32(len);
            let max = i32t.const_int((want - 1) as u64, false);
            let over = self
                .b
                .build_int_compare(IntPredicate::UGT, len32, max, "tov")
                .unwrap();
            let clamped = self
                .b
                .build_select(over, max, len32, "tc")
                .unwrap()
                .into_int_value();
            let clamped = if elem.get_bit_width() < 32 {
                self.b.build_int_truncate(clamped, elem, "tce").unwrap()
            } else {
                clamped
            };
            acc = self
                .b
                .build_insert_value(acc, clamped, 0, "tsi")
                .unwrap()
                .into_array_value();
        }
        acc.into()
    }

    /// blob offset of an lvalue as an i64 value (REF(x) / `&x`):
    /// constant for vars/fields, runtime for array indices and for
    /// re-binding through an existing reference
    pub(crate) fn ref_offset(&mut self, ctx: &Ctx, r: &Ref) -> inkwell::values::IntValue<'ctx> {
        let i64t = self.ctx.i64_type();
        match r {
            Ref::Var(name) => {
                let (o, _) = ctx
                    .get(name.as_str())
                    .cloned()
                    .unwrap_or_else(|| panic!("unresolved var {name}"));
                i64t.const_int(o, false)
            }
            Ref::Field(inner, name) => {
                let (_, ty) = self.resolve(ctx, inner);
                let moff = match &ty {
                    Type::Fb { name: fb } => {
                        let fbl = tplc_mir::layout::builtin_fb_layout(fb)
                            .or_else(|| self.layout.fb_types.get(fb).cloned())
                            .unwrap_or_else(|| panic!("no FB layout {fb}"));
                        fbl.members
                            .get(name.as_str())
                            .map(|(o, _)| *o)
                            .unwrap_or_else(|| panic!("{fb} has no member {name}"))
                    }
                    Type::Struct { fields, .. } => {
                        let mut o = 0u64;
                        let mut found = None;
                        for (n, t) in fields {
                            o = (o + t.align() - 1) & !(t.align() - 1);
                            if n == name {
                                found = Some(o);
                                break;
                            }
                            o += t.size();
                        }
                        found.unwrap_or_else(|| panic!("struct has no field {name}"))
                    }
                    _ => panic!("field on non-aggregate"),
                };
                let base = self.ref_offset(ctx, inner);
                self.b
                    .build_int_add(base, i64t.const_int(moff, false), "fo")
                    .unwrap()
            }
            Ref::Index(inner, idx) => {
                let (_, ty) = self.resolve(ctx, inner);
                let (lo, elem) = match &ty {
                    Type::Array { elem, lo, .. } => (*lo, (**elem).clone()),
                    _ => panic!("index on non-array"),
                };
                let iv = self.arr_idx(ctx, idx);
                let iv = self
                    .b
                    .build_int_sub(iv, i64t.const_int(lo as u64, true), "ri")
                    .unwrap();
                let esz = self
                    .b
                    .build_int_mul(iv, i64t.const_int(elem.size(), false), "re")
                    .unwrap();
                let base = self.ref_offset(ctx, inner);
                self.b.build_int_add(base, esz, "ra").unwrap()
            }
            Ref::Deref(inner) => {
                let (slot, _) = self.resolve(ctx, inner);
                self.b
                    .build_load(i64t, slot, "rl")
                    .unwrap()
                    .into_int_value()
            }
        }
    }

}
