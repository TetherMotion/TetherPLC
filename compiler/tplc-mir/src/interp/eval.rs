//! eval — split from interp.rs.
use super::*;

impl<'a> Rt<'a> {
    /// checked-arith: int Add/Sub/Mul must keep the mathematical
    /// result inside the expr type's range — latches -20 on overflow
    pub(crate) fn chk_int_op(&mut self, op: BinOp, l: &Value, r: &Value, ty: &Type) -> bool {
        if !self.module.checked_arith || !matches!(ty, Type::Int { .. }) {
            return false;
        }
        if matches!(l, Value::F(_) | Value::Time(_)) || matches!(r, Value::F(_) | Value::Time(_)) {
            return false;
        }
        let (x, y) = (val_i128(l), val_i128(r));
        let v = match op {
            BinOp::Add => x.checked_add(y),
            BinOp::Sub => x.checked_sub(y),
            BinOp::Mul => x.checked_mul(y),
            _ => return false,
        };
        let Some((lo, hi)) = int_range(ty) else {
            return false;
        };
        match v {
            Some(v) if v >= lo && v <= hi => false,
            _ => {
                self.fault = Some(TPLC_TASK_E_OVERFLOW);
                true
            }
        }
    }

    /// checked-arith: signed INT_MIN / -1 overflows — latches -20
    pub(crate) fn chk_div_overflow(&mut self, l: &Value, r: &Value, ty: &Type) -> bool {
        if !self.module.checked_arith {
            return false;
        }
        let Some((lo, _)) = int_range(ty) else {
            return false;
        };
        if !matches!(ty, Type::Int { signed: true, .. })
            || matches!(l, Value::F(_) | Value::Time(_))
            || matches!(r, Value::F(_) | Value::Time(_))
            || r.as_i64() != -1
        {
            return false;
        }
        if val_i128(l) == lo {
            self.fault = Some(TPLC_TASK_E_OVERFLOW);
            return true;
        }
        false
    }

    /// checked-arith: does `v` fall outside `to`'s representable
    /// range? covers int→int (width/signedness) and real→int;
    /// other conversion pairs never fault
    pub(crate) fn conv_out_of_range(&self, v: &Value, to: &Type) -> bool {
        let Type::Int { .. } = to else {
            return false;
        };
        let Some((lo, hi)) = int_range(to) else {
            return false;
        };
        match v {
            Value::F(x) => {
                /* f64 bounds [lo, hi+1): hi+1 is a power-of-two
                boundary, exact in f64; NaN fails both compares */
                !x.is_finite() || *x < lo as f64 || *x >= (hi + 1) as f64
            }
            Value::Str(_) | Value::WStr(_) | Value::Ref(_) => false,
            _ => {
                let x = val_i128(v);
                x < lo || x > hi
            }
        }
    }

    // ---- expressions ----
    pub fn eval(&mut self, ctx: &Ctx, e: &Expr) -> Value {
        match &e.kind {
            ExprKind::Lit(l) => match l {
                Lit::Bool(v) => Value::Bool(*v),
                Lit::Int(v) => Value::I(*v),
                Lit::UInt(v) => Value::U(*v),
                Lit::Real(v) => Value::F(*v),
                Lit::TimeNs(v) => Value::Time(*v),
                Lit::Str(s) => Value::Str(s.as_bytes().to_vec()),
                Lit::WStr(s) => Value::WStr(s.encode_utf16().collect()),
                Lit::EnumVal { val, .. } => Value::I(val.parse().unwrap_or(0)),
            },
            ExprKind::Ref(r) => self.read_ref(ctx, r),
            ExprKind::Un(op, a) => {
                let v = self.eval(ctx, a);
                match op {
                    UnOp::Neg => match v {
                        Value::F(x) => Value::F(-x),
                        x => {
                            if self.module.checked_arith {
                                if let Some((lo, hi)) = int_range(&e.ty) {
                                    let nv = -val_i128(&x);
                                    if nv < lo || nv > hi {
                                        self.fault = Some(TPLC_TASK_E_OVERFLOW);
                                        return Value::I(0);
                                    }
                                }
                            }
                            Value::I(x.as_i64().wrapping_neg())
                        }
                    },
                    UnOp::Not => Value::Bool(!v.truthy()),
                }
            }
            ExprKind::Bin(op, a, b) => {
                use BinOp::*;
                match op {
                    AndThen => {
                        if !self.eval(ctx, a).truthy() {
                            return Value::Bool(false);
                        }
                        return Value::Bool(self.eval(ctx, b).truthy());
                    }
                    OrElse => {
                        if self.eval(ctx, a).truthy() {
                            return Value::Bool(true);
                        }
                        return Value::Bool(self.eval(ctx, b).truthy());
                    }
                    _ => {}
                }
                let l = self.eval(ctx, a);
                let r = self.eval(ctx, b);
                match op {
                    Add | Sub | Mul => {
                        if self.chk_int_op(*op, &l, &r, &e.ty) {
                            Value::I(0)
                        } else {
                            match op {
                                Add => arith(l, r, |x, y| x.wrapping_add(y), |x, y| x + y),
                                Sub => arith(l, r, |x, y| x.wrapping_sub(y), |x, y| x - y),
                                _ => arith(l, r, |x, y| x.wrapping_mul(y), |x, y| x * y),
                            }
                        }
                    }
                    Div => {
                        if r.as_i64() == 0 && !matches!(l, Value::F(_)) && !matches!(r, Value::F(_))
                        {
                            /* int div-by-zero: checked mode faults
                            like the JIT (-19); otherwise saturate.
                            Float operands keep IEEE semantics. */
                            if self.module.checked_arith {
                                self.fault = Some(TPLC_TASK_E_DIVZERO);
                            }
                            Value::I(0)
                        } else if self.chk_div_overflow(&l, &r, &e.ty) {
                            Value::I(0)
                        } else {
                            /* U/U divides in u64 — as_i64 would wrap
                            large dividends (matches udiv codegen) */
                            match (&l, &r) {
                                (Value::U(a), Value::U(b))
                                    if matches!(e.ty, Type::Int { signed: false, .. }) =>
                                {
                                    Value::U(if *op == Div {
                                        a.wrapping_div(*b)
                                    } else {
                                        a.wrapping_rem(*b)
                                    })
                                }
                                _ => arith(l, r, |x, y| x.wrapping_div(y), |x, y| x / y),
                            }
                        }
                    }
                    Mod => {
                        if r.as_i64() == 0 && !matches!(l, Value::F(_)) && !matches!(r, Value::F(_))
                        {
                            /* wrapping_rem panics on a zero divisor:
                            guard it here; checked mode faults (-19) */
                            if self.module.checked_arith {
                                self.fault = Some(TPLC_TASK_E_DIVZERO);
                            }
                            Value::I(0)
                        } else if self.chk_div_overflow(&l, &r, &e.ty) {
                            Value::I(0)
                        } else {
                            match (&l, &r) {
                                (Value::U(a), Value::U(b))
                                    if matches!(e.ty, Type::Int { signed: false, .. }) =>
                                {
                                    Value::U(a.wrapping_rem(*b))
                                }
                                _ => arith(l, r, |x, y| x.wrapping_rem(y), |x, y| x % y),
                            }
                        }
                    }
                    Eq => Value::Bool(cmp(&l, &r) == 0),
                    Ne => Value::Bool(cmp(&l, &r) != 0),
                    Lt => Value::Bool(cmp(&l, &r) < 0),
                    Le => Value::Bool(cmp(&l, &r) <= 0),
                    Gt => Value::Bool(cmp(&l, &r) > 0),
                    Ge => Value::Bool(cmp(&l, &r) >= 0),
                    And => Value::Bool(l.truthy() & r.truthy()),
                    Or => Value::Bool(l.truthy() | r.truthy()),
                    Xor => Value::Bool(l.truthy() ^ r.truthy()),
                    Shl => Value::I(l.as_i64() << (r.as_i64() & 63)),
                    Shr => Value::I(l.as_i64() >> (r.as_i64() & 63)),
                    Rol | Ror => Value::I(l.as_i64()),
                    AndThen | OrElse => unreachable!(),
                }
            }
            ExprKind::Conv { to, e } => {
                let v = self.eval(ctx, e);
                if self.module.checked_arith && self.conv_out_of_range(&v, to) {
                    self.fault = Some(TPLC_TASK_E_RANGE);
                    return Value::I(0);
                }
                match to {
                    Type::Bool => Value::Bool(v.truthy()),
                    Type::Int { signed: true, .. } => Value::I(v.as_i64()),
                    Type::Int { signed: false, .. } => Value::U(v.as_i64() as u64),
                    Type::Real { .. } => Value::F(v.as_f64()),
                    Type::Time => Value::Time(v.as_i64()),
                    _ => v,
                }
            }
            ExprKind::RefTo { r } => {
                let (off, _) = self.resolve(ctx, r);
                Value::Ref(off)
            }
            ExprKind::Call { fun, args } => self.fn_call(ctx, fun, args),
        }
    }

    /// FUNCTION invocation: inputs bind into scratch, body runs with
    /// call-scoped ctx, return value read from the name-var.
    /// LEFT/RIGHT/MID/INSERT/DELETE/REPLACE over narrow and wide
    /// strings. Positions are 1-based (IEC) and clamped; L<=0 yields
    /// the empty string, P past the end clamps to the end.
    pub(crate) fn str_op(&mut self, caller: &Ctx, b: BuiltinFn, args: &[(String, Expr)]) -> Value {
        let ev = |rt: &mut Self, i: usize| -> Value {
            args.get(i)
                .map(|(_, e)| rt.eval(caller, e))
                .unwrap_or(Value::I(0))
        };
        match b {
            BuiltinFn::Left | BuiltinFn::Right => {
                let s = ev(self, 0);
                let l = ev(self, 1).as_i64();
                match s {
                    Value::Str(x) => Value::Str(slice_lr(&x, l, b == BuiltinFn::Left)),
                    Value::WStr(x) => Value::WStr(slice_lr(&x, l, b == BuiltinFn::Left)),
                    o => o,
                }
            }
            BuiltinFn::Mid => {
                let s = ev(self, 0);
                let l = ev(self, 1).as_i64();
                let p = ev(self, 2).as_i64();
                match s {
                    Value::Str(x) => Value::Str(slice_mid(&x, l, p)),
                    Value::WStr(x) => Value::WStr(slice_mid(&x, l, p)),
                    o => o,
                }
            }
            BuiltinFn::Insert => {
                let s = ev(self, 0);
                let ins = ev(self, 1);
                let p = ev(self, 2).as_i64();
                match (s, ins) {
                    (Value::Str(a), Value::Str(i)) => Value::Str(insert_at(&a, &i, p)),
                    (Value::WStr(a), Value::WStr(i)) => Value::WStr(insert_at(&a, &i, p)),
                    _ => Value::Str(Vec::new()),
                }
            }
            BuiltinFn::Delete => {
                let s = ev(self, 0);
                let l = ev(self, 1).as_i64();
                let p = ev(self, 2).as_i64();
                match s {
                    Value::Str(x) => Value::Str(delete_at(&x, l, p)),
                    Value::WStr(x) => Value::WStr(delete_at(&x, l, p)),
                    o => o,
                }
            }
            BuiltinFn::Replace => {
                let s = ev(self, 0);
                let ins = ev(self, 1);
                let l = ev(self, 2).as_i64();
                let p = ev(self, 3).as_i64();
                match (s, ins) {
                    (Value::Str(a), Value::Str(i)) => {
                        Value::Str(insert_at(&delete_at(&a, l, p), &i, p))
                    }
                    (Value::WStr(a), Value::WStr(i)) => {
                        Value::WStr(insert_at(&delete_at(&a, l, p), &i, p))
                    }
                    _ => Value::Str(Vec::new()),
                }
            }
            _ => unreachable!(),
        }
    }

    /// LEN/CONCAT over narrow and wide strings (IEC string functions)
    pub(crate) fn builtin_call(&mut self, caller: &Ctx, b: BuiltinFn, args: &[(String, Expr)]) -> Value {
        match b {
            BuiltinFn::Len => {
                let v = args
                    .first()
                    .map(|(_, e)| self.eval(caller, e))
                    .unwrap_or(Value::I(0));
                let n = match v {
                    Value::Str(s) => s.len(),
                    Value::WStr(s) => s.len(),
                    _ => 0,
                };
                Value::I(n as i64)
            }
            BuiltinFn::Abs => {
                let v = args
                    .first()
                    .map(|(_, e)| self.eval(caller, e))
                    .unwrap_or(Value::I(0));
                match v {
                    Value::F(x) => Value::F(x.abs()),
                    Value::U(x) => Value::U(if (x as i64) < 0 {
                        (x as i64).wrapping_neg() as u64
                    } else {
                        x
                    }),
                    Value::Time(x) => Value::Time(x.wrapping_abs()),
                    Value::I(x) => Value::I(x.wrapping_abs()),
                    other => other,
                }
            }
            BuiltinFn::Min | BuiltinFn::Max => {
                let a = args
                    .first()
                    .map(|(_, e)| self.eval(caller, e))
                    .unwrap_or(Value::I(0));
                let b2 = args
                    .get(1)
                    .map(|(_, e)| self.eval(caller, e))
                    .unwrap_or(Value::I(0));
                let take_a = if matches!(a, Value::F(_)) || matches!(b2, Value::F(_)) {
                    let (x, y) = (a.as_f64(), b2.as_f64());
                    if b == BuiltinFn::Min {
                        x <= y
                    } else {
                        x >= y
                    }
                } else {
                    let (x, y) = (a.as_i64(), b2.as_i64());
                    if b == BuiltinFn::Min {
                        x <= y
                    } else {
                        x >= y
                    }
                };
                if take_a {
                    a
                } else {
                    b2
                }
            }
            BuiltinFn::Sel => {
                let g = args
                    .first()
                    .map(|(_, e)| self.eval(caller, e))
                    .map(|v| v.truthy())
                    .unwrap_or(false);
                let idx = if g { 2 } else { 1 };
                args.get(idx)
                    .map(|(_, e)| self.eval(caller, e))
                    .unwrap_or(Value::I(0))
            }
            BuiltinFn::Limit => {
                let mn = args
                    .first()
                    .map(|(_, e)| self.eval(caller, e))
                    .unwrap_or(Value::I(0));
                let v = args
                    .get(1)
                    .map(|(_, e)| self.eval(caller, e))
                    .unwrap_or(Value::I(0));
                let mx = args
                    .get(2)
                    .map(|(_, e)| self.eval(caller, e))
                    .unwrap_or(Value::I(0));
                if matches!(v, Value::F(_))
                    || matches!(mn, Value::F(_))
                    || matches!(mx, Value::F(_))
                {
                    let (lo, x, hi) = (mn.as_f64(), v.as_f64(), mx.as_f64());
                    Value::F(if x < lo {
                        lo
                    } else if x > hi {
                        hi
                    } else {
                        x
                    })
                } else {
                    let (lo, x, hi) = (mn.as_i64(), v.as_i64(), mx.as_i64());
                    Value::I(if x < lo {
                        lo
                    } else if x > hi {
                        hi
                    } else {
                        x
                    })
                }
            }
            BuiltinFn::Left
            | BuiltinFn::Right
            | BuiltinFn::Mid
            | BuiltinFn::Insert
            | BuiltinFn::Delete
            | BuiltinFn::Replace => self.str_op(caller, b, args),
            BuiltinFn::Find => {
                let a = args.first().map(|(_, e)| self.eval(caller, e));
                let b2 = args.get(1).map(|(_, e)| self.eval(caller, e));
                let pos = match (a, b2) {
                    (Some(Value::Str(x)), Some(Value::Str(y))) => find_sub(&x, &y),
                    (Some(Value::WStr(x)), Some(Value::WStr(y))) => find_sub(&x, &y),
                    _ => 0,
                };
                Value::I(pos as i64)
            }
            BuiltinFn::IsValidRef => {
                let v = args
                    .first()
                    .map(|(_, e)| self.eval(caller, e))
                    .unwrap_or(Value::Ref(u64::MAX));
                Value::Bool(match v {
                    Value::Ref(o) => o != u64::MAX,
                    _ => false,
                })
            }
            BuiltinFn::Trunc => {
                let v = args
                    .first()
                    .map(|(_, e)| self.eval(caller, e))
                    .unwrap_or(Value::I(0));
                Value::I(v.as_f64().trunc() as i64)
            }
            BuiltinFn::Round => {
                let v = args
                    .first()
                    .map(|(_, e)| self.eval(caller, e))
                    .unwrap_or(Value::I(0));
                Value::I(v.as_f64().round() as i64)
            }
            BuiltinFn::Mux => {
                let k = args
                    .first()
                    .map(|(_, e)| self.eval(caller, e).as_i64())
                    .unwrap_or(0);
                let idx = 1 + k as usize;
                args.get(idx)
                    .map(|(_, e)| self.eval(caller, e))
                    .unwrap_or(Value::I(0))
            }
            BuiltinFn::Concat => {
                let a = args.first().map(|(_, e)| self.eval(caller, e));
                let b2 = args.get(1).map(|(_, e)| self.eval(caller, e));
                match (a, b2) {
                    (Some(Value::Str(x)), Some(Value::Str(y))) => Value::Str([x, y].concat()),
                    (Some(Value::WStr(x)), Some(Value::WStr(y))) => Value::WStr([x, y].concat()),
                    _ => Value::Str(Vec::new()),
                }
            }
        }
    }

    pub(crate) fn fn_call(&mut self, caller: &Ctx, fun: &str, args: &[(String, Expr)]) -> Value {
        if let Some(b) = builtin_fn(fun) {
            return self.builtin_call(caller, b, args);
        }
        let pou = self
            .module
            .pous
            .iter()
            .find(|p| p.name == fun && p.kind == PouKind::Function)
            .unwrap_or_else(|| panic!("no FUNCTION {fun}"));
        // evaluate args lazily — inout operands are lvalues, not values
        // (aggregate operands have no Value representation at all)
        // lay out fn vars (inputs + locals + implicit return var)
        let fbl = layout_members(&pou.iface, self.layout.fb_types.clone());
        let base = SCRATCH + self.scratch.len() as u64;
        self.scratch
            .resize(self.scratch.len() + fbl.size as usize, 0);
        let mut ctx = fb_ctx(&fbl);
        for v in ctx.values_mut() {
            v.0 += base;
        }
        // positional + named binding
        let inputs: Vec<&VarDecl> = pou
            .iface
            .iter()
            .filter(|v| matches!(v.kind, VarKind::Input | VarKind::InOut))
            .collect();
        // inouts alias caller storage: bound names remap into ctx,
        // unbound names fault on access like an unresolved var
        for v in pou.iface.iter().filter(|v| v.kind == VarKind::InOut) {
            ctx.remove(v.name.as_str());
        }
        for (i, (pname, pe)) in args.iter().enumerate() {
            let name = if pname.is_empty() {
                inputs.get(i).map(|v| v.name.clone())
            } else {
                Some(pname.clone())
            };
            if let Some(n) = name {
                let is_inout = inputs
                    .iter()
                    .any(|v| v.kind == VarKind::InOut && v.name.eq_ignore_ascii_case(&n));
                if is_inout {
                    let ExprKind::Ref(r) = &pe.kind else {
                        panic!("inout {n} bound to non-reference")
                    };
                    let decl = inputs
                        .iter()
                        .find(|v| v.name.eq_ignore_ascii_case(&n))
                        .unwrap()
                        .name
                        .clone();
                    let (o, t) = self.resolve(caller, r);
                    ctx.insert(decl, (o, t));
                } else if let Some((o, t)) = ctx.get(&n).cloned() {
                    let v = self.eval(caller, pe);
                    self.wr(o, &t, &v);
                }
            }
        }
        if let Body::Stmts(stmts) = &pou.body {
            self.exec_all(&ctx, stmts);
        }
        ctx.get(&pou.name)
            .map(|(o, t)| {
                let (o, t) = (*o, t.clone());
                self.rd(o, &t)
            })
            .unwrap_or(Value::I(0))
    }

}
