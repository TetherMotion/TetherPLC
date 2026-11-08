//! State-blob layout algorithm (doc 07 §3).
//!
//! Per program instance: collect fields into regions
//! `[global | fb_instances | task_local | retain | internal]`, order
//! each region by qualified name, assign aligned offsets, emit the
//! `layout_desc` (LAYD) field table.

use crate::ir::*;
use std::collections::BTreeMap;
use tplc_pkg::sections::{Layd, LaydField, LaydRegion};

pub const REGION_GLOBAL: u8 = 0;
pub const REGION_FB: u8 = 1;
pub const REGION_TASK_LOCAL: u8 = 2;
pub const REGION_RETAIN: u8 = 3;
pub const REGION_INTERNAL: u8 = 4;

#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub offset: u64,
    pub size: u64,
    pub ty: Type,
    pub retain: bool,
    pub is_const: bool,
    pub field_kind: u8, // 0 var, 1 fb_inst, 4 retain_wrapper
}

/// Layout of one FB *type* (doc 07 §3.2): iface + internal vars +
/// bookkeeping, declaration order.
#[derive(Debug, Clone)]
pub struct FbLayout {
    pub size: u64,
    pub align: u64,
    /// member name → (offset, type). Includes builtin bookkeeping fields.
    pub members: BTreeMap<String, (u64, Type)>,
}

/// Complete blob layout for one program instance.
#[derive(Debug)]
pub struct Layout {
    pub total_size: u64,
    pub fields: BTreeMap<String, FieldInfo>, // qualified path → leaf field
    pub fb_types: BTreeMap<String, FbLayout>,
    pub retain_offset: u64,
    pub retain_size: u64,
    pub align: u64,
    pub layd: Layd,
}

/// Builtin FB bookkeeping layouts (doc 07 §3.2 rt fields). The names are
/// reserved (`__` prefix) and live inside the instance sub-object.
pub fn builtin_fb_layout(name: &str) -> Option<FbLayout> {
    let mut m = BTreeMap::new();
    let mut push = |o: &mut u64, n: &str, t: Type| {
        let a = t.align();
        *o = (*o + a - 1) & !(a - 1);
        m.insert(n.to_string(), (*o, t.clone()));
        *o += t.size();
    };
    let mut off = 0u64;
    match name {
        "TON" | "TOF" | "TP" => {
            push(&mut off, "IN", Type::Bool);
            push(&mut off, "PT", Type::Time);
            push(&mut off, "Q", Type::Bool);
            push(&mut off, "ET", Type::Time);
            push(&mut off, "__in_last", Type::Bool);
            push(&mut off, "__start", Type::Time);
            push(&mut off, "__run", Type::Bool);
        }
        "R_TRIG" | "F_TRIG" => {
            push(&mut off, "CLK", Type::Bool);
            push(&mut off, "Q", Type::Bool);
            push(&mut off, "__clk_last", Type::Bool);
        }
        "CTU" | "CTD" | "CTUD" => {
            push(&mut off, "CU", Type::Bool);
            push(&mut off, "CD", Type::Bool);
            push(&mut off, "R", Type::Bool);
            push(&mut off, "LD", Type::Bool);
            push(
                &mut off,
                "PV",
                Type::Int {
                    signed: true,
                    bits: 32,
                },
            );
            push(&mut off, "QU", Type::Bool);
            push(&mut off, "QD", Type::Bool);
            push(
                &mut off,
                "CV",
                Type::Int {
                    signed: true,
                    bits: 32,
                },
            );
            push(&mut off, "__cu_last", Type::Bool);
            push(&mut off, "__cd_last", Type::Bool);
        }
        "RS" | "SR" => {
            push(&mut off, "S", Type::Bool);
            push(&mut off, "R1", Type::Bool);
            push(&mut off, "S1", Type::Bool);
            push(&mut off, "R", Type::Bool);
            push(&mut off, "Q1", Type::Bool);
        }
        _ => return None,
    }
    let align = 8u64;
    Some(FbLayout {
        size: (off + align - 1) & !(align - 1),
        align,
        members: m,
    })
}

/// Lay out an ordered VarDecl list as a struct (declaration order,
/// natural alignment ≤8). Used for FB types, FUNCTION call scopes, and
/// STRUCT member tables. VAR_IN_OUT gets no stored field (call-scoped
/// binding, doc 07 §3.2).
pub fn layout_members(vars: &[VarDecl], fb_types: BTreeMap<String, FbLayout>) -> FbLayout {
    let mut members = BTreeMap::new();
    let mut off = 0u64;
    let mut align = 1u64;
    for v in vars {
        if v.kind == VarKind::InOut {
            continue;
        }
        let (size, a) = if let Type::Fb { name } = &v.ty {
            let sub = fb_types
                .get(name)
                .cloned()
                .or_else(|| builtin_fb_layout(name))
                .unwrap_or_else(|| panic!("unknown FB type {name}"));
            (sub.size, sub.align)
        } else {
            (v.ty.size(), v.ty.align())
        };
        align = align.max(a);
        off = (off + a - 1) & !(a - 1);
        members.insert(v.name.clone(), (off, v.ty.clone()));
        off += size;
    }
    let size = (off + align - 1) & !(align - 1);
    FbLayout {
        size,
        align,
        members,
    }
}

/// Layout an FB *type* from its MIR declaration (declaration order).
fn layout_fb(pou: &Pou, fb_types: &mut BTreeMap<String, FbLayout>) -> FbLayout {
    if let Some(b) = builtin_fb_layout(&pou.name) {
        return b;
    }
    if let Some(l) = fb_types.get(&pou.name) {
        return l.clone();
    }
    layout_members(&pou.iface, fb_types.clone())
}

fn layout_fb_named(name: &str, fb_types: &mut BTreeMap<String, FbLayout>) -> FbLayout {
    if let Some(l) = fb_types.get(name) {
        return l.clone();
    }
    if let Some(b) = builtin_fb_layout(name) {
        fb_types.insert(name.to_string(), b.clone());
        return b;
    }
    panic!("unknown FB type {name}");
}

/// Collect one program's leaf fields (qualified paths) into regions.
pub fn layout_program(module: &Module, program: &Pou) -> Layout {
    let mut fb_types: BTreeMap<String, FbLayout> = BTreeMap::new();
    // Pre-layout all declared FB types (decl-order within each type).
    for pou in &module.pous {
        if pou.kind == PouKind::FunctionBlock {
            let l = layout_fb(pou, &mut fb_types);
            fb_types.insert(pou.name.clone(), l);
        }
    }

    // Region → (qualified name, decl) pending list.
    let mut buckets: [Vec<(&VarDecl, String)>; 5] = Default::default();
    for gvl in &module.gvls {
        for v in &gvl.vars {
            let region = if v.retain || gvl.retain {
                REGION_RETAIN
            } else {
                REGION_GLOBAL
            };
            buckets[region as usize].push((v, format!("{}.{}", gvl.name, v.name)));
        }
    }
    for v in &program.iface {
        if v.kind == VarKind::InOut {
            continue;
        }
        let region = if v.retain {
            REGION_RETAIN
        } else if matches!(v.ty, Type::Fb { .. }) {
            REGION_FB
        } else {
            REGION_GLOBAL
        };
        buckets[region as usize].push((v, v.name.clone()));
    }

    // Name-sort within each region; assign offsets (doc 07 §3.3).
    let mut fields = BTreeMap::new();
    let mut off = 0u64;
    let max_align = 1u64;
    let mut layd_fields = Vec::new();
    let mut region_ranges: [(u64, u64, u32); 5] = [(0, 0, 0); 5];
    for (ri, bucket) in buckets.iter_mut().enumerate() {
        bucket.sort_by(|a, b| a.1.cmp(&b.1));
        let region_start = off;
        let mut region_align = 1u64;
        for (v, qname) in bucket.iter() {
            let is_fb = matches!(v.ty, Type::Fb { .. });
            let (size, a) = if let Type::Fb { name } = &v.ty {
                let sub = layout_fb_named(name, &mut fb_types);
                (sub.size, sub.align)
            } else {
                (v.ty.size(), v.ty.align())
            };
            region_align = region_align.max(a);
            off = (off + a - 1) & !(a - 1);
            let is_const = v.kind == VarKind::Constant;
            fields.insert(
                qname.clone(),
                FieldInfo {
                    offset: off,
                    size,
                    ty: v.ty.clone(),
                    retain: v.retain,
                    is_const,
                    field_kind: if is_fb { 1 } else { 0 },
                },
            );
            layd_fields.push(LaydField {
                sym_entry: u32::MAX,
                field_kind: if is_fb { 1 } else { 0 },
                // bit0 retain, bit1 const (read-only, not writeable
                // or forceable through the debug plane)
                flags: (v.retain as u8) | ((is_const as u8) << 1),
                gen: 0,
                offset: off,
                size,
                type_ref: u32::MAX,
            });
            off += size;
        }
        region_ranges[ri] = (region_start, off - region_start, region_align as u32);
    }

    let total = (off + max_align - 1) & !(max_align - 1);
    let (retain_offset, retain_size, _) = region_ranges[REGION_RETAIN as usize];

    let regions: Vec<LaydRegion> = region_ranges
        .iter()
        .enumerate()
        .filter(|(_, (_, size, _))| *size > 0)
        .map(|(kind, (start, size, al))| LaydRegion {
            kind: kind as u8,
            align: *al,
            offset: *start,
            size: *size,
        })
        .collect();

    Layout {
        total_size: total.max(1),
        fields,
        fb_types,
        retain_offset,
        retain_size,
        align: max_align,
        layd: Layd {
            version: 1,
            total_size: total.max(1),
            regions,
            fields: layd_fields,
        },
    }
}
