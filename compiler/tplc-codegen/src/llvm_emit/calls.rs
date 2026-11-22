//! calls — split from llvm_emit.rs.
use super::*;

impl<'ctx> Gen<'ctx> {
    // ---- calls ----
    pub(crate) fn pou_fn(&mut self, name: &str) -> FunctionValue<'ctx> {
        let fname = format!("__tplc_pou_{name}");
        if let Some(f) = self.module.get_function(&fname) {
            return f;
        }
        let i8p = self.i8p();
        let ft = self
            .ctx
            .i32_type()
            .fn_type(&[i8p.into(), i8p.into()], false);
        self.module
            .add_function(&fname, ft, Some(Linkage::External))
    }

    pub(crate) fn rt_fn(&mut self, name: &str) -> FunctionValue<'ctx> {
        let fname = format!("__tplc_rt_{name}");
        if let Some(f) = self.module.get_function(&fname) {
            return f;
        }
        let i8p = self.i8p();
        let ft = self
            .ctx
            .i32_type()
            .fn_type(&[i8p.into(), i8p.into()], false);
        self.module
            .add_function(&fname, ft, Some(Linkage::External))
    }

    /// check rc after call → propagate nonzero to abort_bb
    pub(crate) fn rc_check(&mut self, cs: inkwell::values::CallSiteValue<'ctx>) {
        let f = self.b.get_insert_block().unwrap().get_parent().unwrap();
        let cont = self.ctx.append_basic_block(f, "rc.cont");
        let rc = cs.try_as_basic_value().expect_basic("rc").into_int_value();
        let bad = self
            .b
            .build_int_compare(IntPredicate::NE, rc, self.ctx.i32_type().const_zero(), "rc")
            .unwrap();
        self.b
            .build_conditional_branch(bad, self.abort_bb.unwrap(), cont)
            .unwrap();
        self.b.position_at_end(cont);
    }

    /// copy an aggregate (ARRAY) actual into a slot — llty can't
    /// carry array values, so element access is the only value path;
    /// args/assignments move the bytes raw. Returns false when the
    /// actual isn't a ref (arrays only exist behind refs).
    pub(crate) fn agg_copy(&mut self, ctx: &Ctx, pe: &Expr, dst: PointerValue<'ctx>, bytes: u64) -> bool {
        let ExprKind::Ref(r) = &pe.kind else {
            return false;
        };
        let (sp, sty) = self.resolve(ctx, r);
        self.b
            .build_memcpy(
                dst,
                1,
                sp,
                1,
                self.ctx.i64_type().const_int(bytes.min(sty.size()), false),
            )
            .unwrap();
        true
    }

    pub(crate) fn fb_call(&mut self, ctx: &Ctx, inst: &Ref, args: &[(String, Expr)]) {
        let (instp, ty) = self.resolve(ctx, inst);
        let fbname = match &ty {
            Type::Fb { name } => name.clone(),
            _ => panic!("call on non-FB"),
        };
        let fbl = self.member_of(&fbname);
        /* VAR_IN_OUT slots hold raw pointers to the caller's lvalue
        (distinct from REF vars, which hold blob offsets — both are
        Type::Ref members, so gate on the callee's declared inouts) */
        let inouts: std::collections::HashSet<&str> = self
            .mir
            .pous
            .iter()
            .find(|p| p.name == fbname && p.kind == PouKind::FunctionBlock)
            .map(|p| {
                p.iface
                    .iter()
                    .filter(|v| v.kind == VarKind::InOut)
                    .map(|v| v.name.as_str())
                    .collect()
            })
            .unwrap_or_default();
        // bind args into instance fields
        for (pname, pe) in args {
            if let Some((moff, mty)) = fbl.members.get(pname.as_str()).cloned() {
                let p = self.off_ptr(instp, moff);
                if inouts.iter().any(|n| n.eq_ignore_ascii_case(pname)) {
                    let ExprKind::Ref(r) = &pe.kind else {
                        panic!("inout {pname} bound to non-reference")
                    };
                    let (op_ptr, _) = self.resolve(ctx, r);
                    let iv = self
                        .b
                        .build_ptr_to_int(op_ptr, self.ctx.i64_type(), "iop")
                        .unwrap();
                    self.b.build_store(p, iv).unwrap();
                    continue;
                }
                if matches!(mty, Type::Array { .. }) && self.agg_copy(ctx, pe, p, mty.size()) {
                    continue;
                }
                let v = self.expr(ctx, pe);
                /* narrow/widen to the member width first — a literal
                i32 into a u16/u8 member must not clobber neighbors */
                let v = self.coerce(v, &pe.ty, &mty);
                let v = if is_bool(&mty) {
                    self.b
                        .build_int_z_extend(v.into_int_value(), self.ctx.i8_type(), "z")
                        .unwrap()
                        .into()
                } else if let Type::Str { cap } = &mty {
                    self.str_fit(v, 1 + *cap as u32)
                } else if let Type::WStr { cap } = &mty {
                    self.str_fit(v, 1 + *cap as u32)
                } else {
                    v
                };
                self.b.build_store(p, v).unwrap();
            }
        }
        // rt builtins take (this, ctx); generated pou fns take (ctx, this)
        let (callee, a0, a1) = if builtin_fb_layout(&fbname).is_some() {
            (self.rt_fn(&fbname), instp, self.ctx_ptr)
        } else {
            (self.pou_fn(&fbname), self.ctx_ptr, instp)
        };
        let cs = self
            .b
            .build_call(callee, &[a0.into(), a1.into()], "c")
            .unwrap();
        self.rc_check(cs);
    }

    /// FUNCTION call: caller-side scratch alloca, bind inputs, invoke,
    /// read the name-var back.
    /// byte-indexed element pointer over a typed array pointer
    pub(crate) fn elem_ptr(
        &mut self,
        base: PointerValue<'ctx>,
        idx: inkwell::values::IntValue<'ctx>,
        esz: u64,
    ) -> PointerValue<'ctx> {
        let i64t = self.ctx.i64_type();
        let idx64 = if idx.get_type().get_bit_width() < 64 {
            self.b.build_int_z_extend(idx, i64t, "i64").unwrap()
        } else {
            idx
        };
        let bo = self
            .b
            .build_int_mul(idx64, i64t.const_int(esz, false), "bo")
            .unwrap();
        let base8 = self.b.build_pointer_cast(base, self.i8p(), "b8").unwrap();
        unsafe {
            self.b
                .build_gep(self.ctx.i8_type(), base8, &[bo], "ep")
                .unwrap()
        }
    }

    /// extend/truncate an int value to i32 (position/length maths)
    pub(crate) fn as_i32(&mut self, v: inkwell::values::IntValue<'ctx>) -> inkwell::values::IntValue<'ctx> {
        let w = v.get_type().get_bit_width();
        let i32t = self.ctx.i32_type();
        if w < 32 {
            self.b.build_int_z_extend(v, i32t, "e32").unwrap()
        } else if w > 32 {
            self.b.build_int_truncate(v, i32t, "t32").unwrap()
        } else {
            v
        }
    }

    /// clamp `v` into [lo, hi] (all i32)
    pub(crate) fn clamp32(
        &mut self,
        v: inkwell::values::IntValue<'ctx>,
        lo: i64,
        hi: inkwell::values::IntValue<'ctx>,
    ) -> inkwell::values::IntValue<'ctx> {
        let i32t = self.ctx.i32_type();
        let lov = i32t.const_int(lo as u64, true);
        let below = self
            .b
            .build_int_compare(IntPredicate::SLT, v, lov, "cb")
            .unwrap();
        let v = self
            .b
            .build_select(below, lov, v, "cl")
            .unwrap()
            .into_int_value();
        let above = self
            .b
            .build_int_compare(IntPredicate::SGT, v, hi, "ca")
            .unwrap();
        self.b
            .build_select(above, hi, v, "ch")
            .unwrap()
            .into_int_value()
    }

    /// one runtime-bound unit copy: dst[dst_idx..] <- src[src_idx..]
    pub(crate) fn copy_units(
        &mut self,
        dst: PointerValue<'ctx>,
        dst_idx: inkwell::values::IntValue<'ctx>,
        src: PointerValue<'ctx>,
        src_idx: inkwell::values::IntValue<'ctx>,
        count: inkwell::values::IntValue<'ctx>,
        elem: inkwell::types::IntType<'ctx>,
        esz: u64,
        name: &str,
    ) {
        let i32t = self.ctx.i32_type();
        let f = self.b.get_insert_block().unwrap().get_parent().unwrap();
        let pre = self.b.get_insert_block().unwrap();
        let cond_bb = self.ctx.append_basic_block(f, "cp.cond");
        let body_bb = self.ctx.append_basic_block(f, "cp.body");
        let end_bb = self.ctx.append_basic_block(f, "cp.end");
        self.b.build_unconditional_branch(cond_bb).unwrap();
        self.b.position_at_end(cond_bb);
        let i = self.b.build_phi(i32t, "i").unwrap();
        i.add_incoming(&[(&i32t.const_zero(), pre)]);
        let iv = i.as_basic_value().into_int_value();
        let done = self
            .b
            .build_int_compare(IntPredicate::SGE, iv, count, "cd")
            .unwrap();
        self.b
            .build_conditional_branch(done, end_bb, body_bb)
            .unwrap();
        self.b.position_at_end(body_bb);
        let di = self.b.build_int_add(dst_idx, iv, "di").unwrap();
        let si = self.b.build_int_add(src_idx, iv, "si").unwrap();
        let dp = self.elem_ptr(dst, di, esz);
        let sp = self.elem_ptr(src, si, esz);
        let v = self.b.build_load(elem, sp, name).unwrap();
        self.b.build_store(dp, v).unwrap();
        let next = self
            .b
            .build_int_add(iv, i32t.const_int(1, false), "in")
            .unwrap();
        i.add_incoming(&[(&next, body_bb)]);
        self.b.build_unconditional_branch(cond_bb).unwrap();
        self.b.position_at_end(end_bb);
    }

    /// LEFT/RIGHT/MID/INSERT/DELETE/REPLACE/FIND on narrow and wide
    /// strings. Positions are 1-based (IEC), clamped; L<=0 is empty.
    pub(crate) fn str_slice_call(
        &mut self,
        ctx: &Ctx,
        b: tplc_mir::ty::BuiltinFn,
        args: &[(String, Expr)],
        ret_ty: &Type,
    ) -> BasicValueEnum<'ctx> {
        use tplc_mir::ty::BuiltinFn;
        let i32t = self.ctx.i32_type();
        let av = self.expr(ctx, &args[0].1).into_array_value();
        let aty = av.get_type();
        let elem = aty.get_element_type().into_int_type();
        let esz = (elem.get_bit_width() / 8) as u64;
        let pa = self.b.build_alloca(aty, "sa").unwrap();
        self.b.build_store(pa, av).unwrap();
        let la = self.b.build_load(elem, pa, "la").unwrap().into_int_value();
        let la32 = self.as_i32(la);
        let zero = i32t.const_zero();
        let one = i32t.const_int(1, false);

        let arg_i32 = |g: &mut Self, i: usize| {
            let v = g.expr(ctx, &args[i].1).into_int_value();
            g.as_i32(v)
        };

        if b == BuiltinFn::Find {
            let bv = self.expr(ctx, &args[1].1).into_array_value();
            let bty = bv.get_type();
            let pb = self.b.build_alloca(bty, "sb").unwrap();
            self.b.build_store(pb, bv).unwrap();
            let lb = self.b.build_load(elem, pb, "lb").unwrap().into_int_value();
            let lb32 = self.as_i32(lb);
            /* result starts at 0 (not found); outer scan i in 1..=la-lb+1,
            inner compare of lb units with an early exit             */
            let found = self.b.build_alloca(i32t, "fd").unwrap();
            self.b.build_store(found, zero).unwrap();
            let f = self.b.get_insert_block().unwrap().get_parent().unwrap();
            let ocond = self.ctx.append_basic_block(f, "fd.oc");
            let obody = self.ctx.append_basic_block(f, "fd.ob");
            let oend = self.ctx.append_basic_block(f, "fd.oe");
            let pre = self.b.get_insert_block().unwrap();
            self.b.build_unconditional_branch(ocond).unwrap();
            self.b.position_at_end(ocond);
            let i = self.b.build_phi(i32t, "i").unwrap();
            i.add_incoming(&[(&one, pre)]);
            let iv = i.as_basic_value().into_int_value();
            let last = self.b.build_int_sub(la32, lb32, "lm").unwrap();
            let last = self.b.build_int_add(last, one, "lmp").unwrap();
            let past = self
                .b
                .build_int_compare(IntPredicate::SGT, iv, last, "fp")
                .unwrap();
            let empty = self
                .b
                .build_int_compare(IntPredicate::SLE, lb32, zero, "fe")
                .unwrap();
            let stop = self.b.build_or(past, empty, "fs").unwrap();
            self.b.build_conditional_branch(stop, oend, obody).unwrap();
            self.b.position_at_end(obody);
            let jcond = self.ctx.append_basic_block(f, "fd.jc");
            let jbody = self.ctx.append_basic_block(f, "fd.jb");
            let jnext = self.ctx.append_basic_block(f, "fd.jn");
            let ok = self.b.build_alloca(self.ctx.bool_type(), "ok").unwrap();
            self.b
                .build_store(ok, self.ctx.bool_type().const_all_ones())
                .unwrap();
            self.b.build_unconditional_branch(jcond).unwrap();
            self.b.position_at_end(jcond);
            let j = self.b.build_phi(i32t, "j").unwrap();
            j.add_incoming(&[(&one, obody)]);
            let jv = j.as_basic_value().into_int_value();
            let jdone = self
                .b
                .build_int_compare(IntPredicate::SGT, jv, lb32, "jd")
                .unwrap();
            let cur_ok = self
                .b
                .build_load(self.ctx.bool_type(), ok, "ol")
                .unwrap()
                .into_int_value();
            let quit = self
                .b
                .build_or(jdone, self.b.build_not(cur_ok, "on").unwrap(), "q")
                .unwrap();
            self.b.build_conditional_branch(quit, jnext, jbody).unwrap();
            self.b.position_at_end(jbody);
            let si = self.b.build_int_add(iv, jv, "si").unwrap();
            let si = self.b.build_int_sub(si, one, "si1").unwrap();
            let sp = self.elem_ptr(pa, si, esz);
            let dp = self.elem_ptr(pb, jv, esz);
            let va = self.b.build_load(elem, sp, "va").unwrap().into_int_value();
            let vb = self.b.build_load(elem, dp, "vb").unwrap().into_int_value();
            let eq = self
                .b
                .build_int_compare(IntPredicate::EQ, va, vb, "eq")
                .unwrap();
            self.b.build_store(ok, eq).unwrap();
            let jn = self.b.build_int_add(jv, one, "jn").unwrap();
            j.add_incoming(&[(&jn, jbody)]);
            self.b.build_unconditional_branch(jcond).unwrap();
            self.b.position_at_end(jnext);
            let fin = self
                .b
                .build_load(self.ctx.bool_type(), ok, "fl")
                .unwrap()
                .into_int_value();
            let hit = self.ctx.append_basic_block(f, "fd.hit");
            let miss = self.ctx.append_basic_block(f, "fd.miss");
            self.b.build_conditional_branch(fin, hit, miss).unwrap();
            self.b.position_at_end(hit);
            self.b.build_store(found, iv).unwrap();
            self.b.build_unconditional_branch(oend).unwrap();
            self.b.position_at_end(miss);
            let inext = self.b.build_int_add(iv, one, "inx").unwrap();
            i.add_incoming(&[(&inext, miss)]);
            self.b.build_unconditional_branch(ocond).unwrap();
            self.b.position_at_end(oend);
            let res = self
                .b
                .build_load(i32t, found, "fr")
                .unwrap()
                .into_int_value();
            let want = llty(self.ctx, ret_ty).into_int_type();
            return if want.get_bit_width() < 32 {
                self.b.build_int_truncate(res, want, "frt").unwrap().into()
            } else {
                res.into()
            };
        }

        let cap = match ret_ty {
            Type::Str { cap } | Type::WStr { cap } => *cap,
            _ => 0,
        } as u32;
        let want = 1 + cap;
        let pr = self.b.build_alloca(elem.array_type(want), "sr").unwrap();
        let p0 = self.b.build_pointer_cast(pr, self.i8p(), "p0").unwrap();
        self.b.build_store(p0, elem.const_zero()).unwrap();
        let cap32 = i32t.const_int(cap as u64, false);

        let total: inkwell::values::IntValue<'ctx> = match b {
            BuiltinFn::Left => {
                let l = arg_i32(self, 1);
                let n = self.clamp32(l, 0, la32);
                self.copy_units(pr, one, pa, one, n, elem, esz, "u");
                n
            }
            BuiltinFn::Right => {
                let l = arg_i32(self, 1);
                let n = self.clamp32(l, 0, la32);
                let si = self.b.build_int_sub(la32, n, "rs").unwrap();
                let si = self.b.build_int_add(si, one, "rs1").unwrap();
                self.copy_units(pr, one, pa, si, n, elem, esz, "u");
                n
            }
            BuiltinFn::Mid => {
                let l = arg_i32(self, 1);
                let p = arg_i32(self, 2);
                let start0 = self.clamp32(self.b.build_int_sub(p, one, "m0").unwrap(), 0, la32);
                let avail = self.b.build_int_sub(la32, start0, "ma").unwrap();
                let n = self.clamp32(l, 0, avail);
                let si = self.b.build_int_add(start0, one, "ms").unwrap();
                self.copy_units(pr, one, pa, si, n, elem, esz, "u");
                n
            }
            BuiltinFn::Insert | BuiltinFn::Replace => {
                let bv = self.expr(ctx, &args[1].1).into_array_value();
                let bty = bv.get_type();
                let pb = self.b.build_alloca(bty, "sb").unwrap();
                self.b.build_store(pb, bv).unwrap();
                let lb = self.b.build_load(elem, pb, "lb").unwrap().into_int_value();
                let lb32 = self.as_i32(lb);
                let (d, p_arg) = if b == BuiltinFn::Insert {
                    (zero, 2usize)
                } else {
                    (arg_i32(self, 2), 3usize)
                };
                let p = arg_i32(self, p_arg);
                let last = self.b.build_int_add(la32, one, "lp").unwrap();
                let p32 = self.clamp32(p, 1, last);
                let start0 = self.b.build_int_sub(p32, one, "s0").unwrap();
                let avail = self.b.build_int_sub(la32, start0, "av").unwrap();
                let dcl = self.clamp32(d, 0, avail);
                let tail = self.b.build_int_sub(avail, dcl, "tl").unwrap();
                self.copy_units(pr, one, pa, one, start0, elem, esz, "p1");
                self.copy_units(pr, p32, pb, one, lb32, elem, esz, "p2");
                let si = self.b.build_int_add(p32, dcl, "ps").unwrap();
                let di = self.b.build_int_add(start0, lb32, "pd").unwrap();
                let di = self.b.build_int_add(di, one, "pd1").unwrap();
                self.copy_units(pr, di, pa, si, tail, elem, esz, "p3");
                let t = self.b.build_int_add(start0, lb32, "t1").unwrap();
                self.b.build_int_add(t, tail, "t2").unwrap()
            }
            BuiltinFn::Delete => {
                let l = arg_i32(self, 1);
                let p = arg_i32(self, 2);
                let last = self.b.build_int_add(la32, one, "lp").unwrap();
                let p32 = self.clamp32(p, 1, last);
                let start0 = self.b.build_int_sub(p32, one, "s0").unwrap();
                let avail = self.b.build_int_sub(la32, start0, "av").unwrap();
                let dcl = self.clamp32(l, 0, avail);
                let tail = self.b.build_int_sub(avail, dcl, "tl").unwrap();
                self.copy_units(pr, one, pa, one, start0, elem, esz, "p1");
                let si = self.b.build_int_add(p32, dcl, "ps").unwrap();
                let di = self.b.build_int_add(start0, one, "pd").unwrap();
                self.copy_units(pr, di, pa, si, tail, elem, esz, "p2");
                self.b.build_int_add(start0, tail, "t").unwrap()
            }
            _ => unreachable!(),
        };
        /* result length: clamped to the target cap */
        let over = self
            .b
            .build_int_compare(IntPredicate::SGT, total, cap32, "ov")
            .unwrap();
        let tcl = self
            .b
            .build_select(over, cap32, total, "tc")
            .unwrap()
            .into_int_value();
        let tel = if elem.get_bit_width() < 32 {
            self.b.build_int_truncate(tcl, elem, "te").unwrap()
        } else {
            tcl
        };
        self.b.build_store(p0, tel).unwrap();
        self.b
            .build_load(elem.array_type(want), pr, "sl")
            .unwrap()
            .into()
    }

    /// LEN/CONCAT over narrow and wide strings (IEC string functions)
    pub(crate) fn builtin_call(
        &mut self,
        ctx: &Ctx,
        b: tplc_mir::ty::BuiltinFn,
        args: &[(String, Expr)],
        ret_ty: &Type,
    ) -> BasicValueEnum<'ctx> {
        use tplc_mir::ty::BuiltinFn;
        match b {
            BuiltinFn::Left
            | BuiltinFn::Right
            | BuiltinFn::Mid
            | BuiltinFn::Find
            | BuiltinFn::Insert
            | BuiltinFn::Delete
            | BuiltinFn::Replace => self.str_slice_call(ctx, b, args, ret_ty),
            BuiltinFn::Len => {
                let av = self.expr(ctx, &args[0].1).into_array_value();
                let len = self
                    .b
                    .build_extract_value(av, 0, "len")
                    .unwrap()
                    .into_int_value();
                let want = llty(self.ctx, ret_ty).into_int_type();
                let (lb, wb) = (len.get_type().get_bit_width(), want.get_bit_width());
                if lb < wb {
                    self.b.build_int_z_extend(len, want, "lenz").unwrap().into()
                } else if lb > wb {
                    self.b.build_int_truncate(len, want, "lent").unwrap().into()
                } else {
                    len.into()
                }
            }
            BuiltinFn::Abs => {
                let v = self.expr(ctx, &args[0].1);
                if v.is_float_value() {
                    let f = v.into_float_value();
                    let neg = self.b.build_float_neg(f, "an").unwrap();
                    let lt = self
                        .b
                        .build_float_compare(
                            inkwell::FloatPredicate::OLT,
                            f,
                            self.ctx.f64_type().const_zero(),
                            "ac",
                        )
                        .unwrap();
                    // compare in the value's own width
                    let lt = if f.get_type() == self.ctx.f32_type() {
                        self.b
                            .build_float_compare(
                                inkwell::FloatPredicate::OLT,
                                f,
                                self.ctx.f32_type().const_zero(),
                                "ac",
                            )
                            .unwrap()
                    } else {
                        lt
                    };
                    self.b.build_select(lt, neg, f, "abs").unwrap()
                } else {
                    let i = v.into_int_value();
                    let neg = self.b.build_int_neg(i, "an").unwrap();
                    let lt = self
                        .b
                        .build_int_compare(IntPredicate::SLT, i, i.get_type().const_zero(), "ac")
                        .unwrap();
                    self.b.build_select(lt, neg, i, "abs").unwrap()
                }
            }
            BuiltinFn::Min | BuiltinFn::Max => {
                let a = self.expr(ctx, &args[0].1);
                let b2 = self.expr(ctx, &args[1].1);
                let is_min = b == BuiltinFn::Min;
                let pick = if a.is_float_value() {
                    let (x, y) = (a.into_float_value(), b2.into_float_value());
                    let p = if is_min {
                        inkwell::FloatPredicate::OLE
                    } else {
                        inkwell::FloatPredicate::OGE
                    };
                    self.b.build_float_compare(p, x, y, "cmp").unwrap()
                } else {
                    let (x, y) = (a.into_int_value(), b2.into_int_value());
                    let signed = matches!(ret_ty, Type::Int { signed: true, .. });
                    let p = match (is_min, signed) {
                        (true, true) => IntPredicate::SLE,
                        (true, false) => IntPredicate::ULE,
                        (false, true) => IntPredicate::SGE,
                        (false, false) => IntPredicate::UGE,
                    };
                    self.b.build_int_compare(p, x, y, "cmp").unwrap()
                };
                self.b.build_select(pick, a, b2, "mm").unwrap()
            }
            BuiltinFn::Sel => {
                let g = self.expr(ctx, &args[0].1);
                let a = self.expr(ctx, &args[1].1);
                let b2 = self.expr(ctx, &args[2].1);
                self.b
                    .build_select(g.into_int_value(), b2, a, "sel")
                    .unwrap()
            }
            BuiltinFn::Limit => {
                let mn = self.expr(ctx, &args[0].1);
                let v = self.expr(ctx, &args[1].1);
                let mx = self.expr(ctx, &args[2].1);
                if v.is_float_value() {
                    let (lo, x, hi) = (
                        mn.into_float_value(),
                        v.into_float_value(),
                        mx.into_float_value(),
                    );
                    let below = self
                        .b
                        .build_float_compare(inkwell::FloatPredicate::OLT, x, lo, "lb")
                        .unwrap();
                    let above = self
                        .b
                        .build_float_compare(inkwell::FloatPredicate::OGT, x, hi, "la")
                        .unwrap();
                    let lo_sel = self
                        .b
                        .build_select(below, lo, x, "ll")
                        .unwrap()
                        .into_float_value();
                    self.b.build_select(above, hi, lo_sel, "lim").unwrap()
                } else {
                    let (lo, x, hi) =
                        (mn.into_int_value(), v.into_int_value(), mx.into_int_value());
                    let signed = matches!(ret_ty, Type::Int { signed: true, .. });
                    let (bl, ab) = if signed {
                        (IntPredicate::SLT, IntPredicate::SGT)
                    } else {
                        (IntPredicate::ULT, IntPredicate::UGT)
                    };
                    let below = self.b.build_int_compare(bl, x, lo, "lb").unwrap();
                    let above = self.b.build_int_compare(ab, x, hi, "la").unwrap();
                    let lo_sel = self
                        .b
                        .build_select(below, lo, x, "ll")
                        .unwrap()
                        .into_int_value();
                    self.b.build_select(above, hi, lo_sel, "lim").unwrap()
                }
            }
            BuiltinFn::IsValidRef => {
                let v = self.expr(ctx, &args[0].1).into_int_value();
                let null = self
                    .b
                    .build_int_compare(
                        IntPredicate::EQ,
                        v,
                        v.get_type().const_int(u64::MAX, false),
                        "iv",
                    )
                    .unwrap();
                self.b.build_not(null, "ivn").unwrap().into()
            }
            BuiltinFn::Trunc => {
                /* sema rewrites TRUNC into a real→int Conv; a direct
                MIR use still lands here (fptosi truncates) */
                let v = self.expr(ctx, &args[0].1);
                let want = llty(self.ctx, ret_ty).into_int_type();
                self.b
                    .build_float_to_signed_int(v.into_float_value(), want, "tr")
                    .unwrap()
                    .into()
            }
            BuiltinFn::Round => {
                /* round half away from zero: fptosi(x ± 0.5) */
                let f = self.expr(ctx, &args[0].1).into_float_value();
                let f64t = self.ctx.f64_type();
                let v = if f.get_type() == self.ctx.f32_type() {
                    self.b.build_float_ext(f, f64t, "rw").unwrap()
                } else {
                    f
                };
                let pos = self
                    .b
                    .build_float_compare(inkwell::FloatPredicate::OGE, v, f64t.const_zero(), "rp")
                    .unwrap();
                let half = self
                    .b
                    .build_select(pos, f64t.const_float(0.5), f64t.const_float(-0.5), "rh")
                    .unwrap()
                    .into_float_value();
                let adj = self.b.build_float_add(v, half, "ra").unwrap();
                let want = llty(self.ctx, ret_ty).into_int_type();
                self.b
                    .build_float_to_signed_int(adj, want, "rr")
                    .unwrap()
                    .into()
            }
            BuiltinFn::Mux => {
                let k = self.expr(ctx, &args[0].1).into_int_value();
                let mut r = self.expr(ctx, &args[1].1);
                for i in 1..args.len() - 1 {
                    let cand = self.expr(ctx, &args[i + 1].1);
                    let cond = self
                        .b
                        .build_int_compare(
                            IntPredicate::EQ,
                            k,
                            k.get_type().const_int(i as u64, false),
                            "mc",
                        )
                        .unwrap();
                    r = self.b.build_select(cond, cand, r, "ms").unwrap();
                }
                r
            }
            BuiltinFn::Concat => {
                let av = self.expr(ctx, &args[0].1).into_array_value();
                let bv = self.expr(ctx, &args[1].1).into_array_value();
                let elem = av.get_type().get_element_type().into_int_type();
                let esz = (elem.get_bit_width() / 8) as u64;
                let cap = match ret_ty {
                    Type::Str { cap } | Type::WStr { cap } => *cap,
                    _ => 0,
                } as u32;
                let want = 1 + cap;
                let aty = av.get_type();
                let bty = bv.get_type();
                let pa = self.b.build_alloca(aty, "pa").unwrap();
                self.b.build_store(pa, av).unwrap();
                let pb = self.b.build_alloca(bty, "pb").unwrap();
                self.b.build_store(pb, bv).unwrap();
                let pr = self.b.build_alloca(elem.array_type(want), "pr").unwrap();

                /* a's whole array (len byte + data) lands first — fixed
                size — then b's units shift in after a's live length */
                let bytes = aty.len() as u64 * esz;
                let dst8 = self.b.build_pointer_cast(pr, self.i8p(), "d8").unwrap();
                let src8 = self.b.build_pointer_cast(pa, self.i8p(), "s8").unwrap();
                self.b
                    .build_memcpy(
                        dst8,
                        1,
                        src8,
                        1,
                        self.ctx.i64_type().const_int(bytes, false),
                    )
                    .unwrap();

                let i32t = self.ctx.i32_type();
                let la = self.as_i32(self.b.build_load(elem, pa, "la").unwrap().into_int_value());
                let lb = self.as_i32(self.b.build_load(elem, pb, "lb").unwrap().into_int_value());
                let sum = self.b.build_int_add(la, lb, "sum").unwrap();
                let capv = i32t.const_int(cap as u64, false);
                /* the len unit is unsigned: cap it at both the declared
                capacity and what the len unit can represent */
                let bits = elem.get_bit_width();
                let lim = if bits >= 32 {
                    capv
                } else {
                    let emax = i32t.const_int((1u64 << bits) - 1, false);
                    let cm = self
                        .b
                        .build_int_compare(IntPredicate::ULT, capv, emax, "cm")
                        .unwrap();
                    self.b
                        .build_select(cm, capv, emax, "lim")
                        .unwrap()
                        .into_int_value()
                };
                let over = self
                    .b
                    .build_int_compare(IntPredicate::UGT, sum, lim, "ov")
                    .unwrap();
                let total = self
                    .b
                    .build_select(over, lim, sum, "tot")
                    .unwrap()
                    .into_int_value();

                /* copy b[1..=lb] to pr[la+1..] */
                let di = self
                    .b
                    .build_int_add(la, i32t.const_int(1, false), "di")
                    .unwrap();
                self.copy_units(pr, di, pb, i32t.const_int(1, false), lb, elem, esz, "u");

                let p0 = self.b.build_pointer_cast(pr, self.i8p(), "p0").unwrap();
                let tot_e = if elem.get_bit_width() < 32 {
                    self.b.build_int_truncate(total, elem, "te").unwrap()
                } else {
                    total
                };
                self.b.build_store(p0, tot_e).unwrap();
                self.b
                    .build_load(elem.array_type(want), pr, "cat")
                    .unwrap()
                    .into()
            }
        }
    }

    pub(crate) fn fn_call(
        &mut self,
        ctx: &Ctx,
        fun: &str,
        args: &[(String, Expr)],
        ret_ty: &Type,
    ) -> BasicValueEnum<'ctx> {
        if let Some(b) = tplc_mir::ty::builtin_fn(fun) {
            return self.builtin_call(ctx, b, args, ret_ty);
        }
        let pou = self
            .mir
            .pous
            .iter()
            .find(|p| p.name == fun && p.kind == PouKind::Function)
            .unwrap_or_else(|| panic!("no FUNCTION {fun}"));
        let fbl = tplc_mir::layout::layout_members(&pou.iface, BTreeMap::new());
        let scratch = self
            .b
            .build_alloca(self.ctx.i8_type().array_type(fbl.size as u32), "fs")
            .unwrap();
        let inputs: Vec<&VarDecl> = pou
            .iface
            .iter()
            .filter(|v| matches!(v.kind, VarKind::Input | VarKind::InOut))
            .collect();
        for (i, (pname, pe)) in args.iter().enumerate() {
            let name = if pname.is_empty() {
                inputs.get(i).map(|v| v.name.as_str()).unwrap_or("")
            } else {
                pname.as_str()
            };
            if let Some((moff, mty)) = fbl.members.get(name).cloned() {
                let p = self.off_ptr(scratch, moff);
                // VAR_IN_OUT slot: pointer to the caller's lvalue
                if inputs
                    .iter()
                    .any(|v| v.kind == VarKind::InOut && v.name.eq_ignore_ascii_case(name))
                {
                    let ExprKind::Ref(r) = &pe.kind else {
                        panic!("inout {name} bound to non-reference")
                    };
                    let (op_ptr, _) = self.resolve(ctx, r);
                    let iv = self
                        .b
                        .build_ptr_to_int(op_ptr, self.ctx.i64_type(), "iop")
                        .unwrap();
                    self.b.build_store(p, iv).unwrap();
                    continue;
                }
                if matches!(mty, Type::Array { .. }) && self.agg_copy(ctx, pe, p, mty.size()) {
                    continue;
                }
                let v = self.expr(ctx, pe);
                let v = self.coerce(v, &pe.ty, &mty);
                self.b.build_store(p, v).unwrap();
            }
        }
        let callee = self.pou_fn(fun);
        let cs = self
            .b
            .build_call(callee, &[self.ctx_ptr.into(), scratch.into()], "fc")
            .unwrap();
        self.rc_check(cs);
        // return var = member named pou.name
        let (roff, rty) = fbl.members[&pou.name].clone();
        let rp = self.off_ptr(scratch, roff);
        self.b.build_load(llty(self.ctx, &rty), rp, "ret").unwrap()
    }

}
