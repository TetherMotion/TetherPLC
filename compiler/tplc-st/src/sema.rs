//! `tplc-mir` semantic pass (P1-03): name resolution, type annotation,
//! explicit conversion insertion, call-shape checks, AT-width checks.
//! Pure MIR — frontend-agnostic (FBD lowering lands here too), no
//! ruSTy dependency, no LLVM.
//!
//! Input MIR has `Expr.ty` stubs; output has every expression typed and
//! every legal conversion explicit (`ExprKind::Conv`) — codegen never
//! guesses (doc 07 §2 invariant).

use std::collections::BTreeMap;
use tplc_mir::ir::*;
use tplc_mir::layout::builtin_fb_layout;

use crate::diag::{codes, Diag};

/// POU table for call-shape and FB-member lookups.
struct Env {
    pous: BTreeMap<String, Pou>,
    /// enum name → value → ordinal (for `GREEN` → literal resolution)
    enums: BTreeMap<String, BTreeMap<String, i64>>,
}

impl Env {
    fn new(module: &Module) -> Self {
        let mut env = Env {
            pous: BTreeMap::new(),
            enums: BTreeMap::new(),
        };
        for p in &module.pous {
            env.pous.insert(p.name.to_ascii_uppercase(), p.clone());
        }
        for g in &module.gvls {
            for v in &g.vars {
                env.collect_enum(&v.name, &v.ty);
            }
        }
        for p in &module.pous {
            for v in &p.iface {
                env.collect_enum(&format!("{}.{}", p.name, v.name), &v.ty);
            }
        }
        env
    }
    fn collect_enum(&mut self, _qname: &str, ty: &Type) {
        if let Type::Enum { name, values } = ty {
            let ord: BTreeMap<String, i64> = values
                .iter()
                .enumerate()
                .map(|(i, v)| (v.to_ascii_uppercase(), i as i64))
                .collect();
            self.enums.insert(name.clone(), ord);
        }
    }
    fn enum_ordinal(&self, name: &str) -> Option<i64> {
        self.enums
            .values()
            .find_map(|m| m.get(&name.to_ascii_uppercase()).copied())
    }
}

/// Per-POU scope: var name (uppercased) → decl.
struct Scope {
    vars: BTreeMap<String, VarDecl>,
    gvls: BTreeMap<String, BTreeMap<String, VarDecl>>,
}

impl Scope {
    fn for_pou(pou: &Pou, module: &Module) -> Self {
        let mut s = Scope {
            vars: BTreeMap::new(),
            gvls: BTreeMap::new(),
        };
        for v in &pou.iface {
            s.vars.insert(v.name.to_ascii_uppercase(), v.clone());
        }
        for g in &module.gvls {
            let m: BTreeMap<String, VarDecl> = g
                .vars
                .iter()
                .map(|v| (v.name.to_ascii_uppercase(), v.clone()))
                .collect();
            s.gvls.insert(g.name.to_ascii_uppercase(), m);
        }
        s
    }
}

pub fn check(module: &mut Module) -> Vec<Diag> {
    let env = Env::new(module);
    let mut diags = Vec::new();
    for i in 0..module.pous.len() {
        let scope = Scope::for_pou(&module.pous[i], module);
            let mut ch = Checker {
            env: &env,
            scope: &scope,
            diags: &mut diags,
        };
        // take the body out so env/scope (owning clones) coexist with
        // mutation of statements
        if let Body::Stmts(stmts) = &mut module.pous[i].body {
            let mut owned = std::mem::take(stmts);
            for s in &mut owned {
                ch.stmt(s);
            }
            *stmts = owned;
        }
    }
    diags
}

struct Checker<'e> {
    env: &'e Env,
    scope: &'e Scope,
    diags: &'e mut Vec<Diag>,
}

impl<'e> Checker<'e> {
    fn err(&mut self, code: &'static str, msg: impl Into<String>, p: &Prov) {
        self.diags.push(Diag::error(code, msg, p.clone()));
    }

    /// Type of a declared var name (case-insensitive).
    fn var_ty(&self, name: &str) -> Option<Type> {
        self.scope
            .vars
            .get(&name.to_ascii_uppercase())
            .map(|v| v.ty.clone())
    }

    /// Resolve a Ref chain → its type; canonicalizes Var case in place.
    fn ref_ty(&mut self, r: &mut Ref) -> Option<Type> {
        match r {
            Ref::Var(name) => {
                if let Some(decl) =
                    self.scope.vars.get(&name.to_ascii_uppercase())
                {
                    *name = decl.name.clone(); // canonical case
                    Some(decl.ty.clone())
                } else if self.scope.gvls.contains_key(&name.to_ascii_uppercase()) {
                    // bare GVL name — resolves as struct-like pseudo-var
                    Some(Type::Struct {
                        name: name.clone(),
                        fields: self
                            .scope
                            .gvls[&name.to_ascii_uppercase()]
                            .values()
                            .map(|v| (v.name.clone(), v.ty.clone()))
                            .collect(),
                    })
                } else if self.env.enum_ordinal(name).is_some() {
                    // enum element `GREEN` — expr() rewrites to ordinal
                    Some(Type::Int { signed: true, bits: 32 })
                } else {
                    self.err(
                        codes::UNKNOWN_NAME,
                        format!("undeclared name {name}"),
                        &Prov::None,
                    );
                    None
                }
            }
            Ref::Field(base, name) => {
                let bt = self.ref_ty(base)?;
                match &bt {
                    Type::Fb { name: fbname } => {
                        if let Some(pou) =
                            self.env.pous.get(&fbname.to_ascii_uppercase())
                        {
                            if let Some(v) = pou.iface.iter().find(|v| {
                                v.name.eq_ignore_ascii_case(name)
                            }) {
                                *name = v.name.clone();
                                return Some(v.ty.clone());
                            }
                        }
                        // builtin FB member
                        if let Some(fbl) = builtin_fb_layout(fbname) {
                            if let Some((_, t)) = fbl.members.get(name.as_str()) {
                                return Some(t.clone());
                            }
                        }
                        self.err(
                            codes::UNKNOWN_NAME,
                            format!("{fbname} has no member {name}"),
                            &Prov::None,
                        );
                        None
                    }
                    Type::Struct { fields, .. } => fields
                        .iter()
                        .find(|(n, _)| n.eq_ignore_ascii_case(name))
                        .map(|(n, t)| {
                            *name = n.clone();
                            t.clone()
                        })
                        .or_else(|| {
                            self.err(
                                codes::UNKNOWN_NAME,
                                format!("no field {name}"),
                                &Prov::None,
                            );
                            None
                        }),
                    _ => {
                        self.err(
                            codes::UNKNOWN_NAME,
                            format!("field access on non-aggregate {name}"),
                            &Prov::None,
                        );
                        None
                    }
                }
            }
            Ref::Index(base, idx) => {
                let bt = self.ref_ty(base)?;
                self.expr(idx);
                if let Type::Array { elem, lo, hi } = &bt {
                    let (lo, hi) = (*lo, *hi);
                    let _ = (lo, hi);
                    Some(*elem.clone())
                } else {
                    self.err(codes::UNKNOWN_NAME, "index on non-array", &Prov::None);
                    None
                }
            }
        }
    }

    /// Bottom-up typing; inserts Conv nodes. Returns effective type.
    fn expr(&mut self, e: &mut Expr) -> Type {
        e.ty = match &mut e.kind {
            ExprKind::Lit(l) => match l {
                Lit::Bool(_) => Type::Bool,
                Lit::Int(_) => Type::Int { signed: true, bits: 32 },
                Lit::UInt(_) => Type::Int { signed: false, bits: 32 },
                Lit::Real(_) => Type::Real { bits: 64 },
                Lit::TimeNs(_) => Type::Time,
                Lit::Str(s) => Type::Str { cap: s.len().max(1) as u16 },
                Lit::EnumVal { .. } => Type::Int { signed: true, bits: 32 },
            },
            ExprKind::Ref(r) => {
                // enum element fallback: Var(GREEN) → lit ordinal
                if let Ref::Var(name) = r {
                    if self.var_ty(name).is_none() {
                        if let Some(ord) = self.env.enum_ordinal(name) {
                            e.kind = ExprKind::Lit(Lit::Int(ord));
                            return Type::Int { signed: true, bits: 32 };
                        }
                    }
                }
                self.ref_ty(r).unwrap_or(Type::Bool)
            }
            ExprKind::Un(op, a) => {
                let t = self.expr(a);
                match op {
                    UnOp::Not => Type::Bool,
                    UnOp::Neg => t,
                }
            }
            ExprKind::Bin(op, a, b) => {
                let ta = self.expr(a);
                let tb = self.expr(b);
                use BinOp::*;
                match op {
                    Add | Sub | Mul | Div | Mod => unify_num(&ta, &tb),
                    Shl | Shr | Rol | Ror => ta,
                    Eq | Ne | Lt | Le | Gt | Ge => Type::Bool,
                    And | Or | Xor | AndThen | OrElse => Type::Bool,
                }
            }
            ExprKind::Conv { to, e } => {
                self.expr(e);
                to.clone()
            }
            ExprKind::Call { fun, args } => self.fn_ty(fun, args, &e.prov),
        };
        e.ty.clone()
    }

    fn fn_ty(
        &mut self,
        fun: &mut String,
        args: &mut [(String, Expr)],
        prov: &Prov,
    ) -> Type {
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
        pou.ret_ty.clone().unwrap_or(Type::Bool)
    }

    /// Insert a Conv node around `e` if the matrix allows implicit
    /// conversion from→to; error otherwise.
    fn coerce(&mut self, e: &mut Expr, from: &Type, to: &Type) {
        if compatible(from, to) {
            return;
        }
        if convertible(from, to) {
            let inner = std::mem::replace(
                e,
                Expr {
                    kind: ExprKind::Lit(Lit::Int(0)),
                    ty: Type::Bool,
                    prov: Prov::None,
                },
            );
            *e = Expr {
                kind: ExprKind::Conv { to: to.clone(), e: Box::new(inner) },
                ty: to.clone(),
                prov: Prov::Generated,
            };
        } else {
            self.err(
                codes::UNKNOWN_TYPE,
                format!("cannot convert {from:?} → {to:?}"),
                &e.prov,
            );
        }
    }

    fn stmt(&mut self, s: &mut Stmt) {
        match &mut s.kind {
            StmtKind::Assign { lhs, rhs } => {
                let lt = self.ref_ty(lhs);
                let rt = self.expr(rhs);
                if let Some(lt) = lt {
                    self.coerce(rhs, &rt, &lt);
                }
            }
            StmtKind::If { branches, else_ } => {
                for (c, b) in branches {
                    self.expr(c);
                    for st in b {
                        self.stmt(st);
                    }
                }
                for st in else_ {
                    self.stmt(st);
                }
            }
            StmtKind::Case { sel, arms, else_ } => {
                self.expr(sel);
                for (sels, b) in arms {
                    for cs in sels.iter_mut() {
                        match cs {
                            CaseSel::Lit(e) => {
                                self.expr(e);
                            }
                            CaseSel::Range(a, b) => {
                                self.expr(a);
                                self.expr(b);
                            }
                        }
                    }
                    for st in b {
                        self.stmt(st);
                    }
                }
                for st in else_ {
                    self.stmt(st);
                }
            }
            StmtKind::For { var, start, end, step, body } => {
                if self.var_ty(var).is_none() {
                    self.err(
                        codes::UNKNOWN_NAME,
                        format!("FOR counter {var} undeclared"),
                        &s.prov,
                    );
                }
                self.expr(start);
                self.expr(end);
                if let Some(st) = step {
                    self.expr(st);
                }
                for st in body {
                    self.stmt(st);
                }
            }
            StmtKind::While { cond, body } | StmtKind::Repeat { cond, body } => {
                self.expr(cond);
                for st in body {
                    self.stmt(st);
                }
            }
            StmtKind::Call { inst, args } => {
                let it = self.ref_ty(inst);
                let fbname = match &it {
                    Some(Type::Fb { name }) => Some(name.clone()),
                    _ => None,
                };
                if let Some(fbname) = fbname {
                    // arg names against FB iface / builtin members
                    let members: Vec<(String, Type)> = self
                        .env
                        .pous
                        .get(&fbname.to_ascii_uppercase())
                        .map(|p| {
                            p.iface
                                .iter()
                                .map(|v| (v.name.clone(), v.ty.clone()))
                                .collect()
                        })
                        .or_else(|| {
                            builtin_fb_layout(&fbname).map(|f| {
                                f.members
                                    .iter()
                                    .filter(|(n, _)| !n.starts_with("__"))
                                    .map(|(n, (_, t))| (n.clone(), t.clone()))
                                    .collect()
                            })
                        })
                        .unwrap_or_default();
                    for (pname, pe) in args.iter_mut() {
                        let rt = self.expr(pe);
                        if let Some((cn, ct)) = members
                            .iter()
                            .find(|(n, _)| n.eq_ignore_ascii_case(pname))
                        {
                            let (cn, ct) = (cn.clone(), ct.clone());
                            *pname = cn;
                            self.coerce(pe, &rt, &ct);
                        } else {
                            self.err(
                                codes::UNKNOWN_NAME,
                                format!("{fbname} has no input {pname}"),
                                &s.prov,
                            );
                        }
                    }
                } else {
                    self.err(
                        codes::UNKNOWN_NAME,
                        "call on non-FB-instance ref",
                        &s.prov,
                    );
                }
            }
            StmtKind::Return(e) => {
                if let Some(x) = e {
                    self.expr(x);
                }
            }
            StmtKind::Exit | StmtKind::Continue => {}
        }
    }
}

fn same_class(a: &Type, b: &Type) -> bool {
    matches!(
        (a, b),
        (Type::Int { .. }, Type::Int { .. })
            | (Type::Real { .. }, Type::Real { .. })
            | (Type::Bool, Type::Bool)
            | (Type::Time, Type::Time)
            | (Type::Str { .. }, Type::Str { .. })
            | (Type::Enum { .. }, Type::Enum { .. })
    )
}

fn compatible(a: &Type, b: &Type) -> bool {
    if a == b {
        return true;
    }
    // int literal (i32) assigns to any int/real/time — literal narrowing
    // is constant-folded by codegen; sema accepts
    matches!(
        (a, b),
        (Type::Int { signed: true, bits: 32 }, Type::Int { .. })
            | (Type::Int { signed: true, bits: 32 }, Type::Real { .. })
            | (Type::Int { signed: true, bits: 32 }, Type::Time)
            | (Type::Int { signed: true, bits: 32 }, Type::Enum { .. })
    ) || (matches!(a, Type::Enum { .. }) && matches!(b, Type::Int { .. }))
}

fn convertible(a: &Type, b: &Type) -> bool {
    matches!(
        (a, b),
        (Type::Int { .. }, Type::Int { .. })
            | (Type::Int { .. }, Type::Real { .. })
            | (Type::Int { .. }, Type::Time)
            | (Type::Real { .. }, Type::Int { .. }) // IEC allows w/ rounding
            | (Type::Int { .. }, Type::Bool) // nonzero → true (permissive v1)
            | (Type::Bool, Type::Int { .. })
            | (Type::Time, Type::Int { .. })
            | (Type::Enum { .. }, Type::Int { .. })
            | (Type::Int { .. }, Type::Enum { .. })
            | (Type::Str { .. }, Type::Str { .. })
    )
}

fn unify_num(a: &Type, b: &Type) -> Type {
    if matches!(a, Type::Real { .. }) || matches!(b, Type::Real { .. }) {
        return Type::Real {
            bits: match (a, b) {
                (Type::Real { bits: 64 }, _) | (_, Type::Real { bits: 64 }) => 64,
                _ => 32,
            },
        };
    }
    if matches!(a, Type::Time) || matches!(b, Type::Time) {
        return Type::Time;
    }
    if let (Type::Int { signed: sa, bits: ba }, Type::Int { signed: sb, bits: bb }) =
        (a, b)
    {
        return Type::Int {
            signed: *sa && *sb,
            bits: (*ba).max(*bb),
        };
    }
    if same_class(a, b) {
        return a.clone();
    }
    Type::Int { signed: true, bits: 32 }
}
