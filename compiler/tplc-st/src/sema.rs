//! `tplc-mir` semantic pass (P1-03): name resolution, type annotation,
//! explicit conversion insertion, call-shape checks, AT-width checks.
//! Pure MIR — frontend-agnostic (FBD lowering lands here too), no
//! ruSTy dependency, no LLVM.
//!
//! Input MIR has `Expr.ty` stubs; output has every expression typed and
//! every legal conversion explicit (`ExprKind::Conv`) — codegen never
//! guesses (doc 07 §2 invariant).

use std::collections::{BTreeMap, BTreeSet};
use tplc_mir::ir::*;
use tplc_mir::layout::builtin_fb_layout;
use tplc_mir::ty::{builtin_fn, compatible, convertible, literal_fits, narrows,
                   unify_num, BuiltinFn};

use crate::diag::{codes, wcodes, Diag};

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
        decl_lints(&module.pous[i], &scope, &mut diags);
        let mut ch = Checker {
            env: &env,
            scope: &scope,
            diags: &mut diags,
            used: BTreeSet::new(),
        };
        // take the body out so env/scope (owning clones) coexist with
        // mutation of statements
        if let Body::Stmts(stmts) = &mut module.pous[i].body {
            let mut owned = std::mem::take(stmts);
            ch.stmts(&mut owned);
            *stmts = owned;
        }
        // unused-var lint: locals never resolved anywhere (skip
        // AT-bound — those are driven by the image, not the code)
        for v in &module.pous[i].iface {
            if v.kind == VarKind::Local
                && v.at.is_none()
                && !ch.used.contains(&v.name.to_ascii_uppercase())
            {
                ch.diags.push(Diag::warning(
                    wcodes::UNUSED_VAR,
                    format!("variable `{}` is never used", v.name),
                    v.prov.clone(),
                ));
            }
        }
    }
    diags
}

struct Checker<'e> {
    env: &'e Env,
    scope: &'e Scope,
    diags: &'e mut Vec<Diag>,
    /// canonical (uppercased) local names resolved during the walk
    used: BTreeSet<String>,
}

/// Declaration-level lints: shadowing of a visible outer name, and
/// `VAR_EXTERNAL` declarations coupling the POU to global state.
fn decl_lints(pou: &Pou, scope: &Scope, diags: &mut Vec<Diag>) {
    for v in &pou.iface {
        if v.kind == VarKind::External {
            diags.push(Diag::warning(
                wcodes::EXTERNAL,
                format!("external variable `{}` couples POU to global state", v.name),
                v.prov.clone(),
            ));
            continue;
        }
        let un = v.name.to_ascii_uppercase();
        if scope.gvls.contains_key(&un) {
            diags.push(Diag::warning(
                wcodes::SHADOW,
                format!(
                    "`{}` shadows global list {un} — `GVL.` access resolves to the local",
                    v.name
                ),
                v.prov.clone(),
            ));
        }
    }
}

impl<'e> Checker<'e> {
    fn err(&mut self, code: &'static str, msg: impl Into<String>, p: &Prov) {
        self.diags.push(Diag::error(code, msg, p.clone()));
    }

    /// closest declared name within edit-distance 2 (case-insensitive)
    fn did_you_mean(&self, name: &str) -> Option<String> {
        let cand = |n: &str| -> Option<usize> {
            let d = levenshtein(&name.to_ascii_lowercase(), &n.to_ascii_lowercase());
            (d <= 2).then_some(d)
        };
        self.scope
            .vars
            .keys()
            .chain(self.scope.gvls.keys())
            .filter_map(|n| cand(n).map(|d| (d, n.clone())))
            .min_by_key(|(d, _)| *d)
            .map(|(_, n)| n)
    }

    /// Type of a declared var name (case-insensitive).
    fn var_ty(&self, name: &str) -> Option<Type> {
        self.scope
            .vars
            .get(&name.to_ascii_uppercase())
            .map(|v| v.ty.clone())
    }

    /// Resolve a Ref chain → its type; canonicalizes Var case in place.
    fn ref_ty(&mut self, r: &mut Ref, prov: &Prov) -> Option<Type> {
        match r {
            Ref::Var(name) => {
                if let Some(decl) = self.scope.vars.get(&name.to_ascii_uppercase()) {
                    *name = decl.name.clone(); // canonical case
                    self.used.insert(name.to_ascii_uppercase());
                    Some(decl.ty.clone())
                } else if self.scope.gvls.contains_key(&name.to_ascii_uppercase()) {
                    // bare GVL name — resolves as struct-like pseudo-var;
                    // implicit global access is a lint hit
                    self.diags.push(Diag::warning(
                        wcodes::EXTERNAL,
                        format!("implicit global access `{name}` — declare VAR_EXTERNAL"),
                        prov.clone(),
                    ));
                    Some(Type::Struct {
                        name: name.clone(),
                        fields: self.scope.gvls[&name.to_ascii_uppercase()]
                            .values()
                            .map(|v| (v.name.clone(), v.ty.clone()))
                            .collect(),
                    })
                } else if self.env.enum_ordinal(name).is_some() {
                    // enum element `GREEN` — expr() rewrites to ordinal
                    Some(Type::Int {
                        signed: true,
                        bits: 32,
                    })
                } else {
                    let mut d = Diag::error(
                        codes::UNKNOWN_NAME,
                        format!("undeclared name {name}"),
                        prov.clone(),
                    );
                    if let Some(sug) = self.did_you_mean(name) {
                        d = d.with_help(format!("did you mean `{sug}`?"));
                    }
                    self.diags.push(d);
                    None
                }
            }
            Ref::Field(base, name) => {
                let bt = self.ref_ty(base, prov)?;
                match &bt {
                    Type::Fb { name: fbname } => {
                        if let Some(pou) = self.env.pous.get(&fbname.to_ascii_uppercase()) {
                            if let Some(v) =
                                pou.iface.iter().find(|v| v.name.eq_ignore_ascii_case(name))
                            {
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
                            prov,
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
                            self.err(codes::UNKNOWN_NAME, format!("no field {name}"), prov);
                            None
                        }),
                    _ => {
                        self.err(
                            codes::UNKNOWN_NAME,
                            format!("field access `.{name}` on non-aggregate"),
                            prov,
                        );
                        None
                    }
                }
            }
            Ref::Index(base, idx) => {
                let bt = self.ref_ty(base, prov)?;
                self.expr(idx);
                if let Type::Array { elem, lo, hi } = &bt {
                    let (lo, hi) = (*lo, *hi);
                    let _ = (lo, hi);
                    Some(*elem.clone())
                } else {
                    self.err(codes::UNKNOWN_NAME, "index on non-array", prov);
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
                let ta = self.expr(a);
                let tb = self.expr(b);
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
                        let str_l = matches!(
                            ta,
                            Type::Str { .. } | Type::WStr { .. }
                        );
                        let str_r = matches!(
                            tb,
                            Type::Str { .. } | Type::WStr { .. }
                        );
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
            ExprKind::Conv { to, e } => {
                self.expr(e);
                to.clone()
            }
            ExprKind::Call { fun, args } => self.fn_ty(fun, args, &e.prov),
        };
        e.ty.clone()
    }

    /// builtin function typing (LEN/CONCAT — IEC string functions)
    fn builtin_ty(&mut self, b: BuiltinFn, args: &mut [(String, Expr)],
                  prov: &Prov) -> Type {
        match b {
            BuiltinFn::Len => {
                if args.len() != 1 {
                    self.err(codes::TYPE, "LEN takes exactly one string", prov);
                    return Type::Int { signed: true, bits: 16 };
                }
                let t = self.expr(&mut args[0].1);
                match t {
                    Type::Str { .. } | Type::WStr { .. } => {
                        Type::Int { signed: true, bits: 16 }
                    }
                    _ => {
                        self.err(codes::TYPE,
                                 "LEN expects STRING or WSTRING", prov);
                        Type::Int { signed: true, bits: 16 }
                    }
                }
            }
            BuiltinFn::Concat => {
                if args.len() != 2 {
                    self.err(codes::TYPE,
                             "CONCAT takes exactly two strings", prov);
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
                        self.err(codes::TYPE,
                                 "CONCAT expects two strings of the same kind",
                                 prov);
                        Type::Str { cap: 0 }
                    }
                }
            }
        }
    }

    fn fn_ty(&mut self, fun: &mut String, args: &mut [(String, Expr)], prov: &Prov) -> Type {
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

    /// Insert a Conv node around `e` if the matrix allows implicit
    /// conversion from→to; narrowing conversions also lint.
    fn coerce(&mut self, e: &mut Expr, from: &Type, to: &Type) {
        // untyped literals fold at codegen — legal wherever they fit
        if compatible(from, to) || literal_fits(e, to) {
            return;
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

    /// Statement list — tracks dead code after unconditional
    /// Return/Exit/Continue within the list.
    fn stmts(&mut self, list: &mut [Stmt]) {
        let mut dead = false;
        for s in list {
            if dead {
                self.diags.push(Diag::warning(
                    wcodes::DEAD_CODE,
                    "unreachable statement",
                    s.prov.clone(),
                ));
                // keep checking — more diags inside are still useful
            }
            self.stmt(s);
            dead |= matches!(
                s.kind,
                StmtKind::Return(_) | StmtKind::Exit | StmtKind::Continue
            );
        }
    }

    fn stmt(&mut self, s: &mut Stmt) {
        match &mut s.kind {
            StmtKind::Assign { lhs, rhs } => {
                let lt = self.ref_ty(lhs, &s.prov);
                let rt = self.expr(rhs);
                if let Some(lt) = lt {
                    self.coerce(rhs, &rt, &lt);
                }
            }
            StmtKind::If { branches, else_ } => {
                for (c, b) in branches {
                    self.cond(c);
                    self.stmts(b);
                }
                self.stmts(else_);
            }
            StmtKind::Case { sel, arms, else_ } => {
                self.expr(sel);
                let mut covered: Vec<(i64, i64)> = Vec::new();
                for (sels, b) in arms {
                    for cs in sels.iter_mut() {
                        match cs {
                            CaseSel::Lit(e) => {
                                self.expr(e);
                                if let Some(v) = const_int(e) {
                                    self.case_cover(&mut covered, v, v, &e.prov);
                                }
                            }
                            CaseSel::Range(a, b) => {
                                self.expr(a);
                                self.expr(b);
                                if let (Some(lo), Some(hi)) = (const_int(a), const_int(b)) {
                                    self.case_cover(&mut covered, lo, hi, &a.prov);
                                }
                            }
                        }
                    }
                    self.stmts(b);
                }
                self.stmts(else_);
            }
            StmtKind::For {
                var,
                start,
                end,
                step,
                body,
            } => {
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
                self.stmts(body);
            }
            StmtKind::While { cond, body } | StmtKind::Repeat { cond, body } => {
                self.cond(cond);
                self.stmts(body);
            }
            StmtKind::Call { inst, args } => {
                let it = self.ref_ty(inst, &s.prov);
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
                        if let Some((cn, ct)) =
                            members.iter().find(|(n, _)| n.eq_ignore_ascii_case(pname))
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
                    self.err(codes::UNKNOWN_NAME, "call on non-FB-instance ref", &s.prov);
                }
            }
            StmtKind::Return(e) => {
                if let Some(x) = e {
                    self.expr(x);
                }
            }
            StmtKind::Exit | StmtKind::Continue => {
                self.diags.push(Diag::warning(
                    wcodes::NO_JUMPS,
                    "loop jump — prefer structured conditions",
                    s.prov.clone(),
                ));
            }
        }
    }

    /// Condition position: must be BOOL (IEC strict — no Int→Bool here).
    fn cond(&mut self, e: &mut Expr) {
        let t = self.expr(e);
        if t != Type::Bool {
            self.diags.push(Diag::error(
                codes::TYPE,
                format!("expected Bool condition, found {t:?}"),
                e.prov.clone(),
            ));
        }
    }

    /// Record a case-selector interval; overlap → unreachable-case lint.
    fn case_cover(&mut self, covered: &mut Vec<(i64, i64)>, lo: i64, hi: i64, prov: &Prov) {
        if covered.iter().any(|&(l, h)| lo <= h && l <= hi) {
            self.diags.push(Diag::warning(
                wcodes::UNREACHABLE_CASE,
                format!("case selector {lo}..{hi} overlaps an earlier arm"),
                prov.clone(),
            ));
        }
        covered.push((lo, hi));
    }
}

/// Constant-fold an integer literal expr (post-`expr()` typing).
fn const_int(e: &Expr) -> Option<i64> {
    match &e.kind {
        ExprKind::Lit(Lit::Int(v)) => Some(*v),
        ExprKind::Lit(Lit::UInt(v)) => Some(*v as i64),
        _ => None,
    }
}

/// Untyped literal exprs (no declared width of their own).
fn is_num_lit(e: &Expr) -> bool {
    matches!(
        e.kind,
        ExprKind::Lit(Lit::Int(_) | Lit::UInt(_) | Lit::Real(_))
    )
}

fn is_numeric(t: &Type) -> bool {
    matches!(t, Type::Int { .. } | Type::Real { .. } | Type::Time)
}

/// classic DP edit distance (did-you-mean)
fn levenshtein(a: &str, b: &str) -> usize {
    let (a, b): (Vec<char>, Vec<char>) = (a.chars().collect(), b.chars().collect());
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    for (i, ca) in a.iter().enumerate() {
        let mut cur = vec![i + 1];
        for (j, cb) in b.iter().enumerate() {
            cur.push(
                (prev[j] + usize::from(ca != cb))
                    .min(prev[j + 1] + 1)
                    .min(cur[j] + 1),
            );
        }
        prev = cur;
    }
    prev[b.len()]
}
