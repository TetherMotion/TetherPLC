//! expr — split from sema.rs.
use super::*;

impl<'e> Checker<'e> {
    /// Bottom-up typing; inserts Conv nodes. Returns effective type.
    pub(crate) fn expr(&mut self, e: &mut Expr) -> Type {
        e.ty = match &mut e.kind {
            ExprKind::Lit(l) => match l {
                Lit::Bool(_) => Type::Bool,
                Lit::Int(_) => Type::Int {
                    signed: true,
                    bits: 32,
                },
                Lit::UInt(_) => Type::Int {
                    signed: false,
                    bits: 32,
                },
                Lit::Real(_) => Type::Real { bits: 64 },
                Lit::TimeNs(_) => Type::Time,
                Lit::Str(s) => Type::Str {
                    cap: s.len().max(1) as u16,
                },
                Lit::WStr(s) => Type::WStr {
                    cap: s.encode_utf16().count().max(1) as u16,
                },
                Lit::EnumVal { .. } => Type::Int {
                    signed: true,
                    bits: 32,
                },
            },
            ExprKind::Ref(r) => {
                // enum element fallback: Var(GREEN) → lit ordinal
                if let Ref::Var(name) = r {
                    if self.var_ty(name).is_none() {
                        if let Some(ord) = self.env.enum_ordinal(name) {
                            e.kind = ExprKind::Lit(Lit::Int(ord));
                            return Type::Int {
                                signed: true,
                                bits: 32,
                            };
                        }
                    }
                }
                self.ref_ty(r, &e.prov).unwrap_or(Type::Bool)
            }
            ExprKind::Un(op, a) => {
                let t = self.expr(a);
                match op {
                    UnOp::Not => Type::Bool,
                    UnOp::Neg => t,
                }
            }
            ExprKind::Bin(op, a, b) => {
                let mut ta = self.expr(a);
                let mut tb = self.expr(b);
                /* a reference participates as its pointee */
                if let Type::Ref { to } = ta.clone() {
                    if compatible(&to, &tb) || (is_numeric(&to) && is_numeric(&tb)) {
                        if let Some(t) = self.insert_deref(a) {
                            ta = t;
                        }
                    }
                }
                if let Type::Ref { to } = tb.clone() {
                    if compatible(&to, &ta) || (is_numeric(&to) && is_numeric(&ta)) {
                        if let Some(t) = self.insert_deref(b) {
                            tb = t;
                        }
                    }
                }
                use BinOp::*;
                match op {
                    // untyped literal adopts the other operand's type —
                    // `x + 1` on INT stays INT, no phantom i32 widening
                    Add | Sub | Mul | Div | Mod => {
                        let (la, lb) = (is_num_lit(a), is_num_lit(b));
                        match (la, lb) {
                            (true, false) if is_numeric(&tb) => tb,
                            (false, true) if is_numeric(&ta) => ta,
                            _ => unify_num(&ta, &tb),
                        }
                    }
                    Shl | Shr | Rol | Ror => ta,
                    Eq | Ne | Lt | Le | Gt | Ge => {
                        // STRING supports =/<> only (IEC 61131-3); an
                        // ordered compare or a mixed-type compare is a
                        // type error rather than a codegen surprise
                        let str_l = matches!(ta, Type::Str { .. } | Type::WStr { .. });
                        let str_r = matches!(tb, Type::Str { .. } | Type::WStr { .. });
                        if str_l || str_r {
                            if !(str_l && str_r) || !matches!(op, Eq | Ne) {
                                self.err(
                                    codes::TYPE,
                                    "STRING comparison supports = and <> only",
                                    &e.prov,
                                );
                            }
                        }
                        Type::Bool
                    }
                    And | Or | Xor | AndThen | OrElse => Type::Bool,
                }
            }
            ExprKind::RefTo { r } => {
                let t = self.ref_ty(r, &e.prov.clone());
                Type::Ref {
                    to: Box::new(t.unwrap_or(Type::Bool)),
                }
            }
            ExprKind::Conv { to, e } => {
                self.expr(e);
                to.clone()
            }
            ExprKind::Call { fun, args } => {
                /* X_TO_Y conversion functions become a Conv node so
                both engines share one conversion path */
                if let Some((from, to)) = tplc_mir::ty::conversion_fn(fun) {
                    if args.len() != 1 {
                        self.err(
                            codes::TYPE,
                            format!("{fun} takes exactly one value"),
                            &e.prov,
                        );
                        return to;
                    }
                    let t = self.expr(&mut args[0].1);
                    if !compatible(&t, &from) && !literal_fits(&args[0].1, &from) {
                        self.coerce(&mut args[0].1, &t, &from);
                    }
                    let inner = std::mem::replace(
                        &mut args[0].1,
                        Expr {
                            kind: ExprKind::Lit(Lit::Int(0)),
                            ty: Type::Bool,
                            prov: Prov::None,
                        },
                    );
                    e.kind = ExprKind::Conv {
                        to: to.clone(),
                        e: Box::new(inner),
                    };
                    e.ty = to.clone();
                    return to;
                }
                self.fn_ty(fun, args, &e.prov)
            }
        };
        e.ty.clone()
    }

    /// builtin function typing (LEN/CONCAT — IEC string functions)
    pub(crate) fn builtin_ty(&mut self, b: BuiltinFn, args: &mut [(String, Expr)], prov: &Prov) -> Type {
        match b {
            BuiltinFn::Len => {
                if args.len() != 1 {
                    self.err(codes::TYPE, "LEN takes exactly one string", prov);
                    return Type::Int {
                        signed: true,
                        bits: 16,
                    };
                }
                let t = self.expr(&mut args[0].1);
                match t {
                    Type::Str { .. } | Type::WStr { .. } => Type::Int {
                        signed: true,
                        bits: 16,
                    },
                    _ => {
                        self.err(codes::TYPE, "LEN expects STRING or WSTRING", prov);
                        Type::Int {
                            signed: true,
                            bits: 16,
                        }
                    }
                }
            }
            BuiltinFn::Abs => {
                if args.len() != 1 {
                    self.err(codes::TYPE, "ABS takes exactly one value", prov);
                    return Type::Int {
                        signed: true,
                        bits: 32,
                    };
                }
                let t = self.expr(&mut args[0].1);
                match t {
                    Type::Int { .. } | Type::Real { .. } | Type::Time => t,
                    _ => {
                        self.err(codes::TYPE, "ABS expects a numeric value", prov);
                        Type::Int {
                            signed: true,
                            bits: 32,
                        }
                    }
                }
            }
            BuiltinFn::Min | BuiltinFn::Max => {
                let name = if b == BuiltinFn::Min { "MIN" } else { "MAX" };
                if args.len() != 2 {
                    self.err(
                        codes::TYPE,
                        format!("{name} takes exactly two values"),
                        prov,
                    );
                    return Type::Int {
                        signed: true,
                        bits: 32,
                    };
                }
                let t1 = self.expr(&mut args[0].1);
                let t2 = self.expr(&mut args[1].1);
                let r = unify_num(&t1, &t2);
                if !is_numeric(&r) {
                    self.err(codes::TYPE, format!("{name} expects numeric values"), prov);
                    return Type::Int {
                        signed: true,
                        bits: 32,
                    };
                }
                self.coerce(&mut args[0].1, &t1, &r);
                self.coerce(&mut args[1].1, &t2, &r);
                r
            }
            BuiltinFn::Sel => {
                if args.len() != 3 {
                    self.err(codes::TYPE, "SEL takes G, IN0, IN1", prov);
                    return Type::Bool;
                }
                let tg = self.expr(&mut args[0].1);
                self.coerce(&mut args[0].1, &tg, &Type::Bool);
                let t0 = self.expr(&mut args[1].1);
                let t1 = self.expr(&mut args[2].1);
                let r = unify_num(&t0, &t1);
                if !is_numeric(&r) && !compatible(&t0, &t1) {
                    self.err(codes::TYPE, "SEL branches must share a type", prov);
                    return t0;
                }
                let r = if is_numeric(&r) { r } else { t0.clone() };
                self.coerce(&mut args[1].1, &t0, &r);
                self.coerce(&mut args[2].1, &t1, &r);
                r
            }
            BuiltinFn::Limit => {
                if args.len() != 3 {
                    self.err(codes::TYPE, "LIMIT takes MN, IN, MX", prov);
                    return Type::Int {
                        signed: true,
                        bits: 32,
                    };
                }
                let a = self.expr(&mut args[0].1);
                let i = self.expr(&mut args[1].1);
                let x = self.expr(&mut args[2].1);
                let r = unify_num(&unify_num(&a, &i), &x);
                if !is_numeric(&r) {
                    self.err(codes::TYPE, "LIMIT expects numeric values", prov);
                    return Type::Int {
                        signed: true,
                        bits: 32,
                    };
                }
                self.coerce(&mut args[0].1, &a, &r);
                self.coerce(&mut args[1].1, &i, &r);
                self.coerce(&mut args[2].1, &x, &r);
                r
            }
            BuiltinFn::IsValidRef => {
                if args.len() != 1 {
                    self.err(codes::TYPE, "ISVALIDREF takes one reference", prov);
                    return Type::Bool;
                }
                let t = self.expr(&mut args[0].1);
                if !matches!(t, Type::Ref { .. }) {
                    self.err(codes::TYPE, "ISVALIDREF expects a REFERENCE", prov);
                }
                Type::Bool
            }
            BuiltinFn::Trunc => {
                if args.len() != 1 {
                    self.err(codes::TYPE, "TRUNC takes one value", prov);
                    return Type::Int {
                        signed: true,
                        bits: 32,
                    };
                }
                let t = self.expr(&mut args[0].1);
                if !matches!(t, Type::Real { .. }) {
                    self.coerce(&mut args[0].1, &t, &Type::Real { bits: 64 });
                }
                Type::Int {
                    signed: true,
                    bits: 32,
                }
            }
            BuiltinFn::Round => {
                if args.len() != 1 {
                    self.err(codes::TYPE, "ROUND takes one value", prov);
                    return Type::Int {
                        signed: true,
                        bits: 32,
                    };
                }
                let t = self.expr(&mut args[0].1);
                if !matches!(t, Type::Real { .. }) {
                    self.coerce(&mut args[0].1, &t, &Type::Real { bits: 64 });
                }
                Type::Int {
                    signed: true,
                    bits: 32,
                }
            }
            BuiltinFn::Mux => {
                if args.len() < 2 {
                    self.err(codes::TYPE, "MUX takes K and at least one input", prov);
                    return Type::Int {
                        signed: true,
                        bits: 32,
                    };
                }
                let k = self.expr(&mut args[0].1);
                self.coerce(
                    &mut args[0].1,
                    &k,
                    &Type::Int {
                        signed: true,
                        bits: 32,
                    },
                );
                let mut r: Option<Type> = None;
                for i in 1..args.len() {
                    let t = self.expr(&mut args[i].1);
                    r = Some(match r {
                        None => t,
                        Some(prev) => {
                            let u = unify_num(&prev, &t);
                            if is_numeric(&u) {
                                u
                            } else {
                                prev
                            }
                        }
                    });
                }
                let r = r.unwrap_or(Type::Bool);
                for i in 1..args.len() {
                    let t = args[i].1.ty.clone();
                    self.coerce(&mut args[i].1, &t, &r);
                }
                r
            }
            BuiltinFn::Left | BuiltinFn::Right => {
                let name = if b == BuiltinFn::Left {
                    "LEFT"
                } else {
                    "RIGHT"
                };
                if args.len() != 2 {
                    self.err(codes::TYPE, format!("{name} takes IN, L"), prov);
                    return Type::Str { cap: 0 };
                }
                let t = self.expr(&mut args[0].1);
                let n = self.expr(&mut args[1].1);
                self.coerce(
                    &mut args[1].1,
                    &n,
                    &Type::Int {
                        signed: true,
                        bits: 32,
                    },
                );
                match t {
                    Type::Str { cap } => Type::Str { cap },
                    Type::WStr { cap } => Type::WStr { cap },
                    _ => {
                        self.err(codes::TYPE, format!("{name} expects a string"), prov);
                        Type::Str { cap: 0 }
                    }
                }
            }
            BuiltinFn::Mid => {
                if args.len() != 3 {
                    self.err(codes::TYPE, "MID takes IN, L, P", prov);
                    return Type::Str { cap: 0 };
                }
                let t = self.expr(&mut args[0].1);
                for k in 1..3 {
                    let n = self.expr(&mut args[k].1);
                    self.coerce(
                        &mut args[k].1,
                        &n,
                        &Type::Int {
                            signed: true,
                            bits: 32,
                        },
                    );
                }
                match t {
                    Type::Str { cap } => Type::Str { cap },
                    Type::WStr { cap } => Type::WStr { cap },
                    _ => {
                        self.err(codes::TYPE, "MID expects a string", prov);
                        Type::Str { cap: 0 }
                    }
                }
            }
            BuiltinFn::Find => {
                if args.len() != 2 {
                    self.err(codes::TYPE, "FIND takes IN1, IN2", prov);
                    return Type::Int {
                        signed: true,
                        bits: 16,
                    };
                }
                let t1 = self.expr(&mut args[0].1);
                let t2 = self.expr(&mut args[1].1);
                let ok = matches!(
                    (&t1, &t2),
                    (Type::Str { .. }, Type::Str { .. }) | (Type::WStr { .. }, Type::WStr { .. })
                );
                if !ok {
                    self.err(
                        codes::TYPE,
                        "FIND expects two strings of the same kind",
                        prov,
                    );
                }
                Type::Int {
                    signed: true,
                    bits: 16,
                }
            }
            BuiltinFn::Insert | BuiltinFn::Delete | BuiltinFn::Replace => {
                let (name, nargs) = match b {
                    BuiltinFn::Insert => ("INSERT", 3),
                    BuiltinFn::Delete => ("DELETE", 3),
                    _ => ("REPLACE", 4),
                };
                if args.len() != nargs {
                    self.err(codes::TYPE, format!("{name} takes {nargs} arguments"), prov);
                    return Type::Str { cap: 0 };
                }
                let t1 = self.expr(&mut args[0].1);
                /* INSERT/REPLACE carry a second string at index 1 */
                let ins_ty = if b == BuiltinFn::Delete {
                    None
                } else {
                    Some(self.expr(&mut args[1].1))
                };
                let start = if b == BuiltinFn::Delete { 1 } else { 2 };
                for k in start..nargs {
                    let n = self.expr(&mut args[k].1);
                    self.coerce(
                        &mut args[k].1,
                        &n,
                        &Type::Int {
                            signed: true,
                            bits: 32,
                        },
                    );
                }
                let (cap1, wide) = match &t1 {
                    Type::Str { cap } => (*cap, false),
                    Type::WStr { cap } => (*cap, true),
                    _ => {
                        self.err(codes::TYPE, format!("{name} expects a string"), prov);
                        return Type::Str { cap: 0 };
                    }
                };
                let ins_cap = match &ins_ty {
                    Some(Type::Str { cap }) | Some(Type::WStr { cap })
                        if wide == matches!(ins_ty, Some(Type::WStr { .. })) =>
                    {
                        *cap
                    }
                    Some(_) => {
                        self.err(
                            codes::TYPE,
                            format!("{name}: second argument must be the same string kind"),
                            prov,
                        );
                        0
                    }
                    None => 0,
                };
                let cap = if b == BuiltinFn::Delete {
                    cap1
                } else {
                    cap1.saturating_add(ins_cap)
                };
                if wide {
                    Type::WStr { cap }
                } else {
                    Type::Str { cap }
                }
            }
            BuiltinFn::Concat => {
                if args.len() != 2 {
                    self.err(codes::TYPE, "CONCAT takes exactly two strings", prov);
                    return Type::Str { cap: 0 };
                }
                let t1 = self.expr(&mut args[0].1);
                let t2 = self.expr(&mut args[1].1);
                match (t1, t2) {
                    (Type::Str { cap: c1 }, Type::Str { cap: c2 }) => Type::Str {
                        cap: c1.saturating_add(c2),
                    },
                    (Type::WStr { cap: c1 }, Type::WStr { cap: c2 }) => Type::WStr {
                        cap: c1.saturating_add(c2),
                    },
                    _ => {
                        self.err(
                            codes::TYPE,
                            "CONCAT expects two strings of the same kind",
                            prov,
                        );
                        Type::Str { cap: 0 }
                    }
                }
            }
        }
    }

    pub(crate) fn fn_ty(&mut self, fun: &mut String, args: &mut [(String, Expr)], prov: &Prov) -> Type {
        if let Some(b) = builtin_fn(fun) {
            return self.builtin_ty(b, args, prov);
        }
        let Some(pou) = self.env.pous.get(&fun.to_ascii_uppercase()) else {
            self.err(codes::UNKNOWN_NAME, format!("no FUNCTION {fun}"), prov);
            return Type::Bool;
        };
        let inputs: Vec<VarDecl> = pou
            .iface
            .iter()
            .filter(|v| matches!(v.kind, VarKind::Input | VarKind::InOut))
            .cloned()
            .collect();
        let mut bound: BTreeSet<String> = BTreeSet::new();
        for (i, (pname, pe)) in args.iter_mut().enumerate() {
            let arg_ty = self.expr(pe);
            let target = if pname.is_empty() {
                inputs.get(i).map(|v| (v.name.clone(), v.ty.clone()))
            } else {
                inputs
                    .iter()
                    .find(|v| v.name.eq_ignore_ascii_case(pname))
                    .map(|v| (v.name.clone(), v.ty.clone()))
            };
            match target {
                Some((n, t)) => {
                    bound.insert(n.to_ascii_uppercase());
                    *pname = n;
                    self.coerce(pe, &arg_ty, &t);
                }
                None => self.err(
                    codes::UNKNOWN_NAME,
                    format!("{fun} has no param {pname}"),
                    prov,
                ),
            }
        }
        // call shape: FUNCTION inputs without a default are required
        let missing: Vec<&str> = inputs
            .iter()
            .filter(|v| v.kind == VarKind::Input && v.init.is_none())
            .filter(|v| !bound.contains(&v.name.to_ascii_uppercase()))
            .map(|v| v.name.as_str())
            .collect();
        if !missing.is_empty() {
            self.diags.push(
                Diag::error(
                    codes::TYPE,
                    format!(
                        "{fun} missing required argument{} {}",
                        if missing.len() > 1 { "s" } else { "" },
                        missing.join(", ")
                    ),
                    prov.clone(),
                )
                .with_help("FUNCTION inputs without defaults must be bound"),
            );
        }
        pou.ret_ty.clone().unwrap_or(Type::Bool)
    }

    /// Rewrite a reference expression into an explicit deref read
    /// (`r` → `r^`) — IEC auto-deref in value contexts
    pub(crate) fn insert_deref(&mut self, e: &mut Expr) -> Option<Type> {
        let to = match &e.ty {
            Type::Ref { to } => (**to).clone(),
            _ => return None,
        };
        if let ExprKind::Ref(r) = &mut e.kind {
            let inner = std::mem::replace(r, Ref::Var("?".into()));
            *r = Ref::Deref(Box::new(inner));
            e.ty = to.clone();
            Some(to)
        } else {
            None
        }
    }

    /// Insert a Conv node around `e` if the matrix allows implicit
    /// conversion from→to; narrowing conversions also lint.
    pub(crate) fn coerce(&mut self, e: &mut Expr, from: &Type, to: &Type) {
        // untyped literals fold at codegen — legal wherever they fit
        if compatible(from, to) || literal_fits(e, to) {
            return;
        }
        /* auto-deref (IEC REFERENCE TO): a reference used where its
        pointee is expected reads through the reference */
        if let Type::Ref { to: pointee } = from {
            if **pointee == *to && self.insert_deref(e).is_some() {
                return;
            }
        }
        if convertible(from, to) {
            if narrows(from, to) {
                self.diags.push(Diag::warning(
                    wcodes::IMPLICIT_NARROW,
                    format!("implicit narrowing {from:?} → {to:?}"),
                    e.prov.clone(),
                ));
            }
            let inner = std::mem::replace(
                e,
                Expr {
                    kind: ExprKind::Lit(Lit::Int(0)),
                    ty: Type::Bool,
                    prov: Prov::None,
                },
            );
            *e = Expr {
                kind: ExprKind::Conv {
                    to: to.clone(),
                    e: Box::new(inner),
                },
                ty: to.clone(),
                prov: Prov::Generated,
            };
        } else {
            self.diags.push(
                Diag::error(
                    codes::TYPE,
                    format!("expected {to:?}, found {from:?}"),
                    e.prov.clone(),
                )
                .with_help("no implicit conversion — cast explicitly or use a matching type"),
            );
        }
    }

}
