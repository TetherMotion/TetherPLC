//! Type coercion matrix (doc 05 §12) — shared by sema and the FBD
//! lowering so both frontends apply identical conversion rules.

use crate::ir::{Expr, ExprKind, Lit, Type};

/// from→to loses range/precision (implicit-narrow lint).
pub fn narrows(from: &Type, to: &Type) -> bool {
    match (from, to) {
        (Type::Int { bits: f, .. }, Type::Int { bits: t, .. }) => t < f,
        (Type::Real { bits: f }, Type::Real { bits: t }) => t < f,
        (Type::Real { .. }, Type::Int { .. }) => true,
        (Type::Int { .. } | Type::Time, Type::Bool) => true,
        (Type::Time, Type::Int { bits, .. }) => *bits < 64,
        _ => false,
    }
}

pub fn same_class(a: &Type, b: &Type) -> bool {
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

/// Builtin functions the frontend, interpreter and codegen all know.
/// IEC 61131-3 §2.5.1.5 string functions; scalar builtins (ABS/MIN/
/// SEL) still resolve as user FUNCTIONs only.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuiltinFn {
    Len,
    Concat,
    Abs,
    Min,
    Max,
    Sel,
    Limit,
    Left,
    Right,
    Mid,
    Find,
    Insert,
    Delete,
    Replace,
}

pub fn builtin_fn(name: &str) -> Option<BuiltinFn> {
    match name.to_ascii_uppercase().as_str() {
        "LEN" => Some(BuiltinFn::Len),
        "CONCAT" => Some(BuiltinFn::Concat),
        "ABS" => Some(BuiltinFn::Abs),
        "MIN" => Some(BuiltinFn::Min),
        "MAX" => Some(BuiltinFn::Max),
        "SEL" => Some(BuiltinFn::Sel),
        "LIMIT" => Some(BuiltinFn::Limit),
        "LEFT" => Some(BuiltinFn::Left),
        "RIGHT" => Some(BuiltinFn::Right),
        "MID" => Some(BuiltinFn::Mid),
        "FIND" => Some(BuiltinFn::Find),
        "INSERT" => Some(BuiltinFn::Insert),
        "DELETE" => Some(BuiltinFn::Delete),
        "REPLACE" => Some(BuiltinFn::Replace),
        _ => None,
    }
}

pub fn compatible(a: &Type, b: &Type) -> bool {
    if a == b {
        return true;
    }
    // enum values pass where ints are expected
    matches!(a, Type::Enum { .. }) && matches!(b, Type::Int { .. })
}

/// An int literal expr assigns to any numeric/time/enum target — the
/// literal carries no declared width; codegen folds it into `to`.
pub fn literal_fits(e: &Expr, to: &Type) -> bool {
    matches!(e.kind, ExprKind::Lit(Lit::Int(_) | Lit::UInt(_)))
        && matches!(
            to,
            Type::Int { .. } | Type::Real { .. } | Type::Time | Type::Enum { .. }
        )
}

pub fn convertible(a: &Type, b: &Type) -> bool {
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
            | (Type::WStr { .. }, Type::WStr { .. })
    )
}

/// Numeric result type of a binary op (doc 05 coercion matrix).
pub fn unify_num(a: &Type, b: &Type) -> Type {
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
    if let (
        Type::Int {
            signed: sa,
            bits: ba,
        },
        Type::Int {
            signed: sb,
            bits: bb,
        },
    ) = (a, b)
    {
        return Type::Int {
            signed: *sa && *sb,
            bits: (*ba).max(*bb),
        };
    }
    if same_class(a, b) {
        return a.clone();
    }
    Type::Int {
        signed: true,
        bits: 32,
    }
}
