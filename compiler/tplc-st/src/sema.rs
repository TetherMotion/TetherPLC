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
use tplc_mir::ty::{
    builtin_fn, compatible, convertible, literal_fits, narrows, unify_num, BuiltinFn,
};

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

/// Declaration-level lints: duplicate identifiers (names are
/// case-insensitive — `r` and `R` collide and would silently alias in
/// the scope map), shadowing of a visible outer name, and
/// `VAR_EXTERNAL` declarations coupling the POU to global state.
fn decl_lints(pou: &Pou, scope: &Scope, diags: &mut Vec<Diag>) {
    let mut seen: BTreeSet<String> = BTreeSet::new();
    for v in &pou.iface {
        let un = v.name.to_ascii_uppercase();
        if !seen.insert(un.clone()) {
            diags.push(Diag::error(
                codes::DUP_NAME,
                format!(
                    "duplicate identifier `{un}` — IEC names are case-insensitive"
                ),
                v.prov.clone(),
            ));
            continue;
        }
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


mod resolve;
mod expr;
mod stmt;


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
