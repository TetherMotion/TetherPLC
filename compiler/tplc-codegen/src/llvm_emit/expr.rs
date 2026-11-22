//! expr — split from llvm_emit.rs.
use super::*;

impl<'ctx> Gen<'ctx> {
    // ---- expressions ----
    pub(crate) fn expr(&mut self, ctx: &Ctx, e: &Expr) -> BasicValueEnum<'ctx> {
        match &e.kind {
            // literals materialize at the expr's declared type — sema
            // already fixed the type, codegen never re-guesses
            ExprKind::Lit(l) => {
                let lt = llty(self.ctx, &e.ty);
                match l {
                    Lit::Bool(v) => self.ctx.bool_type().const_int(*v as u64, false).into(),
                    Lit::Int(v) => lt.into_int_type().const_int(*v as u64, true).into(),
                    Lit::UInt(v) => lt.into_int_type().const_int(*v, false).into(),
                    Lit::Real(v) => lt.into_float_type().const_float(*v).into(),
                    Lit::TimeNs(v) => lt.into_int_type().const_int(*v as u64, true).into(),
                    Lit::EnumVal { .. } => lt.into_int_type().const_zero().into(),
                    Lit::Str(s) => {
                        // STRING literal → [1+cap x i8] {len, bytes, 0…}
                        // (a bare i8 zero would only clear the len byte)
                        let cap = match &e.ty {
                            Type::Str { cap } => *cap as usize,
                            _ => s.len(),
                        };
                        let i8t = self.ctx.i8_type();
                        let n = s.len().min(cap);
                        let mut els: Vec<inkwell::values::IntValue<'ctx>> =
                            Vec::with_capacity(1 + cap);
                        els.push(i8t.const_int(n as u64, false));
                        for b in s.as_bytes().iter().take(n) {
                            els.push(i8t.const_int(*b as u64, false));
                        }
                        while els.len() < 1 + cap {
                            els.push(i8t.const_zero());
                        }
                        i8t.const_array(&els).into()
                    }
                    Lit::WStr(s) => {
                        // WSTRING literal → [1+cap x i16] {len, units, 0…}
                        let cap = match &e.ty {
                            Type::WStr { cap } => *cap as usize,
                            _ => s.encode_utf16().count(),
                        };
                        let i16t = self.ctx.i16_type();
                        let units: Vec<u16> = s.encode_utf16().collect();
                        let n = units.len().min(cap);
                        let mut els: Vec<inkwell::values::IntValue<'ctx>> =
                            Vec::with_capacity(1 + cap);
                        els.push(i16t.const_int(n as u64, false));
                        for u in units.iter().take(n) {
                            els.push(i16t.const_int(*u as u64, false));
                        }
                        while els.len() < 1 + cap {
                            els.push(i16t.const_zero());
                        }
                        i16t.const_array(&els).into()
                    }
                }
            }
            ExprKind::RefTo { r } => self.ref_offset(ctx, r).into(),
            ExprKind::Ref(r) => self.read(ctx, r),
            ExprKind::Un(op, a) => {
                let v = self.expr(ctx, a);
                match op {
                    UnOp::Neg => {
                        if v.is_float_value() {
                            self.b
                                .build_float_neg(v.into_float_value(), "n")
                                .unwrap()
                                .into()
                        } else {
                            let iv = v.into_int_value();
                            /* checked-arith: negating the signed
                            minimum (or any nonzero unsigned)
                            leaves the type's range → -20 */
                            if self.mir.checked_arith {
                                if let Type::Int { signed, bits } = &e.ty {
                                    let bad = if *signed {
                                        let minv = if *bits >= 64 {
                                            i64::MIN
                                        } else {
                                            -(1i64 << (*bits as u32 - 1))
                                        };
                                        self.b
                                            .build_int_compare(
                                                IntPredicate::EQ,
                                                iv,
                                                iv.get_type().const_int(minv as u64, true),
                                                "mn",
                                            )
                                            .unwrap()
                                    } else {
                                        self.b
                                            .build_int_compare(
                                                IntPredicate::NE,
                                                iv,
                                                iv.get_type().const_zero(),
                                                "nz",
                                            )
                                            .unwrap()
                                    };
                                    let f =
                                        self.b.get_insert_block().unwrap().get_parent().unwrap();
                                    let ok_bb = self.ctx.append_basic_block(f, "neg.ok");
                                    self.b
                                        .build_conditional_branch(
                                            bad,
                                            self.overflow_bb.unwrap(),
                                            ok_bb,
                                        )
                                        .unwrap();
                                    self.b.position_at_end(ok_bb);
                                }
                            }
                            self.b.build_int_neg(iv, "n").unwrap().into()
                        }
                    }
                    UnOp::Not => self.b.build_not(v.into_int_value(), "not").unwrap().into(),
                }
            }
            ExprKind::Bin(op, a, b) => self.bin(ctx, *op, a, b, &e.ty),
            ExprKind::Conv { to, e } => {
                let v = self.expr(ctx, e);
                self.conv(v, &e.ty, to)
            }
            ExprKind::Call { fun, args } => self.fn_call(ctx, fun, args, &e.ty),
        }
    }

    pub(crate) fn conv(&mut self, v: BasicValueEnum<'ctx>, from: &Type, to: &Type) -> BasicValueEnum<'ctx> {
        match (from, to) {
            (Type::Bool, Type::Int { .. }) => self
                .b
                .build_int_z_extend(v.into_int_value(), llty(self.ctx, to).into_int_type(), "c")
                .unwrap()
                .into(),
            (Type::Int { .. }, Type::Bool) => self
                .b
                .build_int_compare(
                    IntPredicate::NE,
                    v.into_int_value(),
                    llty(self.ctx, from).into_int_type().const_zero(),
                    "c",
                )
                .unwrap()
                .into(),
            (Type::Int { signed: s, .. }, Type::Real { .. }) => {
                if *s {
                    self.b
                        .build_signed_int_to_float(
                            v.into_int_value(),
                            llty(self.ctx, to).into_float_type(),
                            "c",
                        )
                        .unwrap()
                        .into()
                } else {
                    self.b
                        .build_unsigned_int_to_float(
                            v.into_int_value(),
                            llty(self.ctx, to).into_float_type(),
                            "c",
                        )
                        .unwrap()
                        .into()
                }
            }
            (Type::Real { .. }, Type::Int { signed: ts, .. }) => {
                let f = v.into_float_value();
                let dt = llty(self.ctx, to).into_int_type();
                if self.mir.checked_arith {
                    if let Some((lo, hi)) = int128_range(to) {
                        /* [lo, hi+1) as f64 bounds — hi+1 is a
                        power-of-two boundary, exact; NaN fails both
                        ordered compares */
                        let ft = f.get_type();
                        let ge = self
                            .b
                            .build_float_compare(
                                FloatPredicate::OGE,
                                f,
                                ft.const_float(lo as f64),
                                "rlo",
                            )
                            .unwrap();
                        let lt = self
                            .b
                            .build_float_compare(
                                FloatPredicate::OLT,
                                f,
                                ft.const_float((hi + 1) as f64),
                                "rhi",
                            )
                            .unwrap();
                        let ok = self.b.build_and(ge, lt, "rok").unwrap();
                        let fr = self.b.get_insert_block().unwrap().get_parent().unwrap();
                        let cont = self.ctx.append_basic_block(fr, "cv.ok");
                        self.b
                            .build_conditional_branch(ok, cont, self.range_bb.unwrap())
                            .unwrap();
                        self.b.position_at_end(cont);
                    }
                }
                if *ts {
                    self.b.build_float_to_signed_int(f, dt, "c").unwrap().into()
                } else {
                    self.b
                        .build_float_to_unsigned_int(f, dt, "c")
                        .unwrap()
                        .into()
                }
            }
            (Type::Real { bits: fb }, Type::Real { bits: tb }) if fb != tb => {
                let dt = llty(self.ctx, to).into_float_type();
                if tb > fb {
                    self.b
                        .build_float_ext(v.into_float_value(), dt, "c")
                        .unwrap()
                        .into()
                } else {
                    self.b
                        .build_float_trunc(v.into_float_value(), dt, "c")
                        .unwrap()
                        .into()
                }
            }
            (Type::Int { signed: s, .. }, Type::Int { signed: d, .. }) => {
                let dt = llty(self.ctx, to).into_int_type();
                let iv = v.into_int_value();
                let sw = iv.get_type().get_bit_width();
                let dw = dt.get_bit_width();
                let res = if dw > sw {
                    if *s {
                        self.b.build_int_s_extend(iv, dt, "c").unwrap()
                    } else {
                        self.b.build_int_z_extend(iv, dt, "c").unwrap()
                    }
                } else if dw < sw {
                    self.b.build_int_truncate(iv, dt, "c").unwrap()
                } else {
                    iv
                };
                /* checked-arith: the mathematical value (source
                signedness) must fit the target range — widening to
                i128 makes the compare exact for every width pair,
                including same-width signedness flips */
                if self.mir.checked_arith && (*s != *d || sw != dw) {
                    if let Some((lo, hi)) = int128_range(to) {
                        let i128t = self.ctx.i128_type();
                        let wide = if *s {
                            self.b.build_int_s_extend(iv, i128t, "cv").unwrap()
                        } else {
                            self.b.build_int_z_extend(iv, i128t, "cv").unwrap()
                        };
                        let lok = self
                            .b
                            .build_int_compare(
                                IntPredicate::SGE,
                                wide,
                                i128_const(self.ctx, lo),
                                "rlo",
                            )
                            .unwrap();
                        let hik = self
                            .b
                            .build_int_compare(
                                IntPredicate::SLE,
                                wide,
                                i128_const(self.ctx, hi),
                                "rhi",
                            )
                            .unwrap();
                        let ok = self.b.build_and(lok, hik, "rok").unwrap();
                        let f = self.b.get_insert_block().unwrap().get_parent().unwrap();
                        let cont = self.ctx.append_basic_block(f, "cv.ok");
                        self.b
                            .build_conditional_branch(ok, cont, self.range_bb.unwrap())
                            .unwrap();
                        self.b.position_at_end(cont);
                    }
                }
                res.into()
            }
            (Type::Real { .. }, Type::Real { bits }) => {
                let dt = llty(self.ctx, to).into_float_type();
                if *bits == 32 {
                    self.b
                        .build_float_trunc(v.into_float_value(), dt, "c")
                        .unwrap()
                        .into()
                } else {
                    self.b
                        .build_float_ext(v.into_float_value(), dt, "c")
                        .unwrap()
                        .into()
                }
            }
            _ => v,
        }
    }

    /// numeric coercion for operand-width plumbing (sema owns semantic
    /// conversions; LLVM just needs equal-width operands)
    pub(crate) fn coerce(&mut self, v: BasicValueEnum<'ctx>, from: &Type, to: &Type) -> BasicValueEnum<'ctx> {
        if v.is_int_value() && matches!(to, Type::Int { .. } | Type::Time | Type::Enum { .. }) {
            let dt = llty(self.ctx, to).into_int_type();
            let iv = v.into_int_value();
            let (sw, dw) = (iv.get_type().get_bit_width(), dt.get_bit_width());
            if dw > sw {
                let sext = matches!(from, Type::Int { signed: true, .. });
                return if sext {
                    self.b.build_int_s_extend(iv, dt, "w").unwrap().into()
                } else {
                    self.b.build_int_z_extend(iv, dt, "w").unwrap().into()
                };
            }
            if dw < sw {
                return self.b.build_int_truncate(iv, dt, "w").unwrap().into();
            }
        }
        if v.is_float_value() && matches!(to, Type::Real { .. }) {
            let dt = llty(self.ctx, to).into_float_type();
            let fv = v.into_float_value();
            if fv.get_type() != dt {
                return self.b.build_float_ext(fv, dt, "w").unwrap().into();
            }
        }
        v
    }

    /// common numeric type for comparisons (wider wins; signed wins ties)
    pub(crate) fn cmp_ty(a: &Type, b: &Type) -> Type {
        let bits = |t: &Type| match t {
            Type::Int { bits, .. } => *bits,
            Type::Real { bits } => *bits,
            Type::Bool => 1,
            _ => 64,
        };
        if matches!(a, Type::Real { .. }) || matches!(b, Type::Real { .. }) {
            return Type::Real {
                bits: bits(a).max(bits(b)).max(32),
            };
        }
        let (ba, bb) = (bits(a), bits(b));
        let w = ba.max(bb);
        let signed = matches!(a, Type::Int { signed: true, .. })
            || matches!(b, Type::Int { signed: true, .. })
            || matches!(a, Type::Time)
            || matches!(b, Type::Time);
        if matches!(a, Type::Time) || matches!(b, Type::Time) {
            return Type::Time;
        }
        Type::Int { signed, bits: w }
    }

    pub(crate) fn bin(
        &mut self,
        ctx: &Ctx,
        op: BinOp,
        a: &Expr,
        b: &Expr,
        e_ty: &Type,
    ) -> BasicValueEnum<'ctx> {
        use BinOp::*;
        match op {
            AndThen | OrElse => {
                // short-circuit via blocks
                let l_bb = self.b.get_insert_block().unwrap();
                let f = l_bb.get_parent().unwrap();
                let l = self.expr(ctx, a).into_int_value();
                let l_bb = self.b.get_insert_block().unwrap();
                let rhs_bb = self.ctx.append_basic_block(f, "sc.rhs");
                let done_bb = self.ctx.append_basic_block(f, "sc.done");
                if op == AndThen {
                    self.b.build_conditional_branch(l, rhs_bb, done_bb).unwrap();
                } else {
                    self.b.build_conditional_branch(l, done_bb, rhs_bb).unwrap();
                }
                self.b.position_at_end(rhs_bb);
                let r = self.expr(ctx, b).into_int_value();
                self.b.build_unconditional_branch(done_bb).unwrap();
                self.b.position_at_end(done_bb);
                let phi = self.b.build_phi(self.ctx.bool_type(), "sc").unwrap();
                phi.add_incoming(&[(&l, l_bb), (&r, rhs_bb)]);
                return phi.as_basic_value();
            }
            _ => {}
        }
        // STRING =/<> : length byte then code-unit compare within the
        // shorter string (sema rejects ordered compares). Unrolled —
        // caps are small and this keeps the scan path libc-free.
        if matches!(op, Eq | Ne)
            && (matches!(a.ty, Type::Str { .. } | Type::WStr { .. })
                || matches!(b.ty, Type::Str { .. } | Type::WStr { .. }))
        {
            let av = self.expr(ctx, a).into_array_value();
            let bv = self.expr(ctx, b).into_array_value();
            let cap = av.get_type().len().min(bv.get_type().len()) as usize;
            let elem = av.get_type().get_element_type().into_int_type();
            let la = self
                .b
                .build_extract_value(av, 0, "la")
                .unwrap()
                .into_int_value();
            let lb = self
                .b
                .build_extract_value(bv, 0, "lb")
                .unwrap()
                .into_int_value();
            let mut neq = self
                .b
                .build_int_compare(IntPredicate::NE, la, lb, "lneq")
                .unwrap();
            for i in 1..cap {
                let ca = self
                    .b
                    .build_extract_value(av, i as u32, "ca")
                    .unwrap()
                    .into_int_value();
                let cb = self
                    .b
                    .build_extract_value(bv, i as u32, "cb")
                    .unwrap()
                    .into_int_value();
                let d = self
                    .b
                    .build_int_compare(IntPredicate::NE, ca, cb, "cd")
                    .unwrap();
                let idx = elem.const_int((i - 1) as u64, false);
                let ina = self
                    .b
                    .build_int_compare(IntPredicate::ULT, idx, la, "ia")
                    .unwrap();
                let inb = self
                    .b
                    .build_int_compare(IntPredicate::ULT, idx, lb, "ib")
                    .unwrap();
                let act = self.b.build_and(ina, inb, "act").unwrap();
                let m = self.b.build_and(d, act, "m").unwrap();
                neq = self.b.build_or(neq, m, "no").unwrap();
            }
            return if op == Eq {
                self.b.build_not(neq, "seq").unwrap().into()
            } else {
                neq.into()
            };
        }
        let target = if matches!(op, Eq | Ne | Lt | Le | Gt | Ge) {
            Self::cmp_ty(&a.ty, &b.ty)
        } else {
            e_ty.clone()
        };
        let lv = self.expr(ctx, a);
        let l = self.coerce(lv, &a.ty, &target);
        let rv = self.expr(ctx, b);
        let r = self.coerce(rv, &b.ty, &target);
        let isf = l.is_float_value();
        if isf {
            let (l, r) = (l.into_float_value(), r.into_float_value());
            return match op {
                Add => self.b.build_float_add(l, r, "f").unwrap().into(),
                Sub => self.b.build_float_sub(l, r, "f").unwrap().into(),
                Mul => self.b.build_float_mul(l, r, "f").unwrap().into(),
                Div => self.b.build_float_div(l, r, "f").unwrap().into(),
                Mod => self.b.build_float_rem(l, r, "f").unwrap().into(),
                Eq => self.fcmp(FloatPredicate::OEQ, l, r),
                Ne => self.fcmp(FloatPredicate::ONE, l, r),
                Lt => self.fcmp(FloatPredicate::OLT, l, r),
                Le => self.fcmp(FloatPredicate::OLE, l, r),
                Gt => self.fcmp(FloatPredicate::OGT, l, r),
                Ge => self.fcmp(FloatPredicate::OGE, l, r),
                _ => self.b.build_float_add(l, r, "f").unwrap().into(),
            };
        }
        let (l, r) = (l.into_int_value(), r.into_int_value());
        if self.mir.checked_arith
            && matches!(op, Add | Sub | Mul)
            && matches!(target, Type::Int { .. })
        {
            if let Some(v) = self.checked_bin(&target, op, l, r) {
                return v;
            }
        }
        match op {
            Add => self.b.build_int_add(l, r, "i").unwrap().into(),
            Sub => self.b.build_int_sub(l, r, "i").unwrap().into(),
            Mul => self.b.build_int_mul(l, r, "i").unwrap().into(),
            Div | Mod => {
                /* div/rem by zero is UB for both signed and unsigned
                variants, and sdiv/srem INT_MIN/-1 is poison too: a
                select guard cannot protect the op, and neither can
                a branch→phi merge: instcombine folds that back
                into `select(dz, 0, sdiv)` which x86 lowers to an
                unconditional idiv that still traps. Clamp the
                divisor to a provably safe value, divide, then
                select 0 on dz — for MIN/-1 the clamped divisor 1
                yields the wrapped results (MIN / 0) for free.
                Checked mode keeps real control flow to the -19/-20
                exits (no merge, so nothing folds the div loose). */
                let unsigned = matches!(target, Type::Int { signed: false, .. });
                let zero = r.get_type().const_zero();
                let dz = self
                    .b
                    .build_int_compare(IntPredicate::EQ, r, zero, "dz")
                    .unwrap();
                let ominus1 = if unsigned {
                    None
                } else {
                    let minv = match target {
                        Type::Int { signed: true, bits } if bits < 64 => {
                            -(1i64 << (bits as u32 - 1))
                        }
                        _ => i64::MIN, // 64-bit ints, Time, Enum ride i64
                    };
                    let is_min = self
                        .b
                        .build_int_compare(
                            IntPredicate::EQ,
                            l,
                            r.get_type().const_int(minv as u64, true),
                            "mn",
                        )
                        .unwrap();
                    let is_neg1 = self
                        .b
                        .build_int_compare(
                            IntPredicate::EQ,
                            r,
                            r.get_type().const_int((-1i64) as u64, true),
                            "n1",
                        )
                        .unwrap();
                    Some(self.b.build_and(is_min, is_neg1, "o1").unwrap())
                };
                if self.mir.checked_arith {
                    let f = self.b.get_insert_block().unwrap().get_parent().unwrap();
                    let ok_bb = self.ctx.append_basic_block(f, "div.ok");
                    self.b
                        .build_conditional_branch(dz, self.divzero_bb.unwrap(), ok_bb)
                        .unwrap();
                    self.b.position_at_end(ok_bb);
                    if let Some(o1) = ominus1 {
                        let f2 = self.b.get_insert_block().unwrap().get_parent().unwrap();
                        let ok2 = self.ctx.append_basic_block(f2, "div.ok2");
                        self.b
                            .build_conditional_branch(o1, self.overflow_bb.unwrap(), ok2)
                            .unwrap();
                        self.b.position_at_end(ok2);
                    }
                    return (if op == Div {
                        if unsigned {
                            self.b.build_int_unsigned_div(l, r, "d").unwrap()
                        } else {
                            self.b.build_int_signed_div(l, r, "d").unwrap()
                        }
                    } else {
                        if unsigned {
                            self.b.build_int_unsigned_rem(l, r, "m").unwrap()
                        } else {
                            self.b.build_int_signed_rem(l, r, "m").unwrap()
                        }
                    })
                    .into();
                }
                let one: BasicValueEnum = r.get_type().const_int(1, false).into();
                let rv: BasicValueEnum = r.into();
                let mut sr = self
                    .b
                    .build_select(dz, one, rv, "sd")
                    .unwrap()
                    .into_int_value();
                if let Some(o1) = ominus1 {
                    let one2: BasicValueEnum = sr.get_type().const_int(1, false).into();
                    let srv: BasicValueEnum = sr.into();
                    sr = self
                        .b
                        .build_select(o1, one2, srv, "sd2")
                        .unwrap()
                        .into_int_value();
                }
                let q = if op == Div {
                    if unsigned {
                        self.b.build_int_unsigned_div(l, sr, "d").unwrap()
                    } else {
                        self.b.build_int_signed_div(l, sr, "d").unwrap()
                    }
                } else {
                    if unsigned {
                        self.b.build_int_unsigned_rem(l, sr, "m").unwrap()
                    } else {
                        self.b.build_int_signed_rem(l, sr, "m").unwrap()
                    }
                };
                let z: BasicValueEnum = q.get_type().const_zero().into();
                let qv: BasicValueEnum = q.into();
                return self.b.build_select(dz, z, qv, "sel").unwrap();
            }
            Eq => self.icmp(IntPredicate::EQ, l, r),
            Ne => self.icmp(IntPredicate::NE, l, r),
            Lt => self.icmp(IntPredicate::SLT, l, r),
            Le => self.icmp(IntPredicate::SLE, l, r),
            Gt => self.icmp(IntPredicate::SGT, l, r),
            Ge => self.icmp(IntPredicate::SGE, l, r),
            And => self.b.build_and(l, r, "a").unwrap().into(),
            Or => self.b.build_or(l, r, "o").unwrap().into(),
            Xor => self.b.build_xor(l, r, "x").unwrap().into(),
            Shl => self.b.build_left_shift(l, r, "s").unwrap().into(),
            Shr => self.b.build_right_shift(l, r, true, "s").unwrap().into(),
            Rol | Ror => l.into(),
            AndThen | OrElse => unreachable!(),
        }
    }

    /// checked-arith: int Add/Sub/Mul via the *.with.overflow
    /// intrinsic — branches to the -20 exit on overflow and returns
    /// the wrapped result on the fallthrough path. None when the
    /// intrinsic is unavailable (caller emits the plain op).
    pub(crate) fn checked_bin(
        &mut self,
        target: &Type,
        op: BinOp,
        l: IntValue<'ctx>,
        r: IntValue<'ctx>,
    ) -> Option<BasicValueEnum<'ctx>> {
        let signed = matches!(target, Type::Int { signed: true, .. });
        let name = match (op, signed) {
            (BinOp::Add, true) => "llvm.sadd.with.overflow",
            (BinOp::Add, false) => "llvm.uadd.with.overflow",
            (BinOp::Sub, true) => "llvm.ssub.with.overflow",
            (BinOp::Sub, false) => "llvm.usub.with.overflow",
            (BinOp::Mul, true) => "llvm.smul.with.overflow",
            _ => "llvm.umul.with.overflow",
        };
        let it = l.get_type();
        let intr =
            Intrinsic::find(name).and_then(|i| i.get_declaration(&self.module, &[it.into()]))?;
        let call = self.b.build_call(intr, &[l.into(), r.into()], "ov").ok()?;
        let bve = match call.try_as_basic_value() {
            inkwell::values::ValueKind::Basic(b) => b,
            _ => return None,
        };
        let sv = bve.into_struct_value();
        let res = self.b.build_extract_value(sv, 0, "ovr").ok()?;
        let flag = self
            .b
            .build_extract_value(sv, 1, "ovf")
            .ok()?
            .into_int_value();
        let f = self.b.get_insert_block().unwrap().get_parent().unwrap();
        let ok_bb = self.ctx.append_basic_block(f, "ovf.ok");
        self.b
            .build_conditional_branch(flag, self.overflow_bb?, ok_bb)
            .ok()?;
        self.b.position_at_end(ok_bb);
        Some(res)
    }

    pub(crate) fn icmp(
        &mut self,
        p: IntPredicate,
        l: IntValue<'ctx>,
        r: IntValue<'ctx>,
    ) -> BasicValueEnum<'ctx> {
        self.b.build_int_compare(p, l, r, "c").unwrap().into()
    }

    pub(crate) fn fcmp(
        &mut self,
        p: FloatPredicate,
        l: inkwell::values::FloatValue<'ctx>,
        r: inkwell::values::FloatValue<'ctx>,
    ) -> BasicValueEnum<'ctx> {
        self.b.build_float_compare(p, l, r, "c").unwrap().into()
    }

}
