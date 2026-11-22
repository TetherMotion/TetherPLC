//! Tree-walking MIR interpreter over a byte blob. Proves that the MIR +
//! layout pair is executable: refs resolve to `base + offset`, the same
//! addressing model generated code will use (doc 07 §3.4). Function
//! calls get call-scoped scratch memory (locals are transient, not in
//! the state blob — same split codegen will make). Eval/oracle-side
//! tool, not a shipping runtime.

use crate::ir::*;
use crate::layout::{builtin_fb_layout, layout_members, FbLayout, Layout};
use crate::ty::{builtin_fn, BuiltinFn};
use std::collections::BTreeMap;

/// Offsets at or above this base index into `scratch` instead of `blob`
/// — marks call-scoped storage in the ctx maps.
const SCRATCH: u64 = 1 << 48;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Bool(bool),
    I(i64),
    U(u64),
    F(f64),
    Time(i64), // ns
    Str(Vec<u8>),
    /// wide string: UTF-16 code units
    WStr(Vec<u16>),
    /// reference: blob byte offset (u64::MAX = null)
    Ref(u64),
}

impl Value {
    pub fn truthy(&self) -> bool {
        matches!(self, Value::Bool(true))
    }
    pub fn as_i64(&self) -> i64 {
        match self {
            Value::I(v) => *v,
            Value::U(v) => *v as i64,
            Value::F(v) => *v as i64,
            Value::Time(v) => *v,
            Value::Bool(v) => *v as i64,
            Value::Str(_) | Value::WStr(_) | Value::Ref(_) => 0,
        }
    }
    pub fn as_f64(&self) -> f64 {
        match self {
            Value::I(v) => *v as f64,
            Value::U(v) => *v as f64,
            Value::F(v) => *v,
            Value::Time(v) => *v as f64,
            _ => 0.0,
        }
    }
}

#[derive(Debug)]
pub enum Flow {
    Normal,
    /// IEC EXIT — breaks the innermost loop
    Exit,
    Return(Value),
    Abort(i32),
}

/// member name → (offset, type). Offsets ≥ SCRATCH index `scratch`.
pub type Ctx = BTreeMap<String, (u64, Type)>;

/// Build the program-level ctx: top-level var name → (blob offset, ty).
/// `k` is a GVL field (`GVL.x`) — global storage resolvable by
/// qualified name from every program scope.
fn is_gvl_field(layout: &Layout, k: &str) -> bool {
    let Some((p, _)) = k.split_once('.') else {
        return false;
    };
    layout.gvls.iter().any(|g| g.eq_ignore_ascii_case(p))
}

pub fn program_ctx(layout: &Layout) -> Ctx {
    layout
        .fields
        .iter()
        .filter(|(k, _)| !k.contains('.') || is_gvl_field(layout, k))
        .map(|(k, f)| (k.clone(), (f.offset, f.ty.clone())))
        .collect()
}

/// Build the ctx for one program in a (possibly multi-program) layout:
/// fields qualified `PROG.name` by `layout_module` are stripped back to
/// bare names; falls back to the flat program ctx when no qualified
/// fields exist (single-program layouts keep bare names).
pub fn pou_scope(layout: &Layout, prog: &str) -> Ctx {
    let prefix = format!("{}.", prog.to_ascii_uppercase());
    let m: Ctx = layout
        .fields
        .iter()
        .filter(|(k, _)| k.to_ascii_uppercase().starts_with(&prefix))
        .map(|(k, f)| (k[prefix.len()..].to_string(), (f.offset, f.ty.clone())))
        .collect();
    if m.is_empty() {
        program_ctx(layout)
    } else {
        // programs also see GVL storage under its qualified name
        let mut m = m;
        for (k, f) in &layout.fields {
            if is_gvl_field(layout, k) {
                m.insert(k.clone(), (f.offset, f.ty.clone()));
            }
        }
        m
    }
}

/// Build the ctx for an FB body: member → (member offset, ty), resolved
/// relative to the instance base.
pub fn fb_ctx(fbl: &FbLayout) -> Ctx {
    fbl.members
        .iter()
        .map(|(k, (o, t))| (k.clone(), (*o, t.clone())))
        .collect()
}

pub struct Rt<'a> {
    pub blob: &'a mut [u8],
    pub layout: &'a Layout,
    pub module: &'a Module,
    pub now_ns: u64,
    pub cycle: u64,
    pub abort_flag: Option<&'a dyn Fn() -> bool>,
    scratch: Vec<u8>,
    /// latched runtime fault (null/OOB reference deref); exec turns
    /// it into Flow::Abort so the sim faults like the runtime does
    fault: Option<i32>,
}


mod access;
mod eval;
mod exec;


fn arith(l: Value, r: Value, fi: fn(i64, i64) -> i64, ff: fn(f64, f64) -> f64) -> Value {
    match (&l, &r) {
        (Value::F(_), _) | (_, Value::F(_)) => Value::F(ff(l.as_f64(), r.as_f64())),
        (Value::Time(_), _) | (_, Value::Time(_)) => Value::Time(fi(l.as_i64(), r.as_i64())),
        (Value::U(_), Value::U(_)) => Value::U(fi(l.as_i64(), r.as_i64()) as u64),
        _ => Value::I(fi(l.as_i64(), r.as_i64())),
    }
}

/// inclusive (lo, hi) of an int type as i128 — checked-arith range
/// tests compare mathematical results against this
fn int_range(ty: &Type) -> Option<(i128, i128)> {
    match ty {
        Type::Int { signed: true, bits } => {
            let b = *bits as u32;
            Some((-(1i128 << (b - 1)), (1i128 << (b - 1)) - 1))
        }
        Type::Int {
            signed: false,
            bits,
        } => Some((0, (1i128 << *bits as u32) - 1)),
        _ => None,
    }
}

/// mathematical value of an int-family Value widened to i128 —
/// Value::U keeps its unsigned magnitude, the rest go through i64
fn val_i128(v: &Value) -> i128 {
    match v {
        Value::U(x) => *x as i128,
        Value::I(x) | Value::Time(x) => *x as i128,
        Value::Bool(x) => *x as i128,
        Value::F(x) => *x as i128,
        _ => 0,
    }
}

/// first 1-based position of `needle` in `hay`, 0 when absent
fn find_sub<T: PartialEq>(hay: &[T], needle: &[T]) -> usize {
    if needle.is_empty() || needle.len() > hay.len() {
        return 0;
    }
    for i in 0..=hay.len() - needle.len() {
        if &hay[i..i + needle.len()] == needle {
            return i + 1;
        }
    }
    0
}

fn slice_lr<T: Copy>(s: &[T], l: i64, left: bool) -> Vec<T> {
    if l <= 0 {
        return Vec::new();
    }
    let n = (l as usize).min(s.len());
    if left {
        s[..n].to_vec()
    } else {
        s[s.len() - n..].to_vec()
    }
}

fn slice_mid<T: Copy>(s: &[T], l: i64, p: i64) -> Vec<T> {
    if l <= 0 || p < 1 {
        return Vec::new();
    }
    let start = ((p - 1) as usize).min(s.len());
    let n = (l as usize).min(s.len() - start);
    s[start..start + n].to_vec()
}

fn insert_at<T: Copy>(s: &[T], ins: &[T], p: i64) -> Vec<T> {
    let pos = ((p.max(1) - 1) as usize).min(s.len());
    let mut out = Vec::with_capacity(s.len() + ins.len());
    out.extend_from_slice(&s[..pos]);
    out.extend_from_slice(ins);
    out.extend_from_slice(&s[pos..]);
    out
}

fn delete_at<T: Copy>(s: &[T], l: i64, p: i64) -> Vec<T> {
    if l <= 0 || p < 1 {
        return s.to_vec();
    }
    let start = ((p - 1) as usize).min(s.len());
    let n = (l as usize).min(s.len() - start);
    let mut out = s[..start].to_vec();
    out.extend_from_slice(&s[start + n..]);
    out
}

fn cmp(l: &Value, r: &Value) -> i8 {
    if matches!(l, Value::F(_)) || matches!(r, Value::F(_)) {
        let (x, y) = (l.as_f64(), r.as_f64());
        return if x < y {
            -1
        } else if x > y {
            1
        } else {
            0
        };
    }
    match (l, r) {
        (Value::Bool(a), Value::Bool(b)) => (*a as i8) - (*b as i8),
        (Value::WStr(a), Value::WStr(b)) => match a.cmp(b) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        },
        (Value::Str(a), Value::Str(b)) => {
            // lexicographic over bytes (IEC defines =/<> only; the
            // ordering is what a memcmp-style engine would produce)
            match a.cmp(b) {
                std::cmp::Ordering::Less => -1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
            }
        }
        _ => {
            let (a, b) = (l.as_i64(), r.as_i64());
            if a < b {
                -1
            } else if a > b {
                1
            } else {
                0
            }
        }
    }
}
