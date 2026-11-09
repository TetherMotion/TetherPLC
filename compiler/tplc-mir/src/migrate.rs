//! Online-change layout diff (doc 08 §3–§5): decode a predecessor's
//! LAYD/SMAP/TYPC sections, expand both sides to leaf fields, join on
//! qualified name, and classify every pair as copy / transform /
//! default-init / dropped / deny. The result feeds the `migrate_desc`
//! package section and the emitted `tplc_migrate` body.

use std::collections::BTreeMap;
use tplc_pkg::sections::Layd;

/* ---------------- wire decoders (doc 01 §6/§8) ---------------- */

fn u32at(b: &[u8], o: usize) -> u32 {
    u32::from_le_bytes(b[o..o + 4].try_into().unwrap())
}
fn u64at(b: &[u8], o: usize) -> u64 {
    u64::from_le_bytes(b[o..o + 8].try_into().unwrap())
}

#[derive(Debug, Clone)]
pub struct SmapEnt {
    pub name: String,
    pub kind: u8,
    pub flags: u8,
    pub region: u8,
    pub type_ref: u32,
    pub offset: u64,
    pub size: u32,
}

/// SMAP section (32 B header + count × 64 B entries + provs + strtab).
pub fn decode_smap(buf: &[u8]) -> Option<Vec<SmapEnt>> {
    if buf.len() < 32 || &buf[..4] != b"SMAP" {
        return None;
    }
    let count = u32at(buf, 8) as usize;
    let strtab_off = u32at(buf, 12) as usize;
    let strtab_len = u32at(buf, 16) as usize;
    if strtab_off + strtab_len > buf.len() || 32 + count * 64 > buf.len() {
        return None;
    }
    let strs = &buf[strtab_off..strtab_off + strtab_len];
    let name = |off: u32| -> String {
        let o = off as usize;
        if o >= strs.len() {
            return String::new();
        }
        let end = strs[o..]
            .iter()
            .position(|&c| c == 0)
            .unwrap_or(strs.len() - o);
        String::from_utf8_lossy(&strs[o..o + end]).into_owned()
    };
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let e = &buf[32 + i * 64..];
        out.push(SmapEnt {
            name: name(u32at(e, 0)),
            kind: e[4],
            flags: e[5],
            region: e[6],
            type_ref: u32at(e, 8),
            offset: u64at(e, 12),
            size: u32at(e, 20),
        });
    }
    Some(out)
}

/* ---------------- TYPC canonical signatures ---------------- */

pub const TAG_BOOL: u8 = 0;
pub const TAG_STRING: u8 = 15;
pub const TAG_ARRAY: u8 = 18;
pub const TAG_STRUCT: u8 = 19;
pub const TAG_ENUM: u8 = 20;
pub const TAG_FB: u8 = 21;
/// REFERENCE TO: u64 blob offset (never survives a layout change)
pub const TAG_REF: u8 = 17;

#[derive(Debug, Clone)]
pub struct TypeInfo {
    pub tag: u8,
    pub size: u32,
    /// canonical signature — structural identity across packages
    pub sig: Vec<u8>,
    /// member list (name, member type idx, offset) for struct/fb
    pub members: Vec<(String, u32, u32)>,
    /// array: elem type idx + (lo, hi) of dim 0
    pub elem: u32,
    pub dims: (i32, i32),
}

struct TypcView<'a> {
    count: usize,
    entries: &'a [u8],
    strs: &'a [u8],
}

impl<'a> TypcView<'a> {
    fn str(&self, off: u32) -> String {
        let o = off as usize;
        if o >= self.strs.len() {
            return String::new();
        }
        let end = self.strs[o..]
            .iter()
            .position(|&c| c == 0)
            .unwrap_or(self.strs.len() - o);
        String::from_utf8_lossy(&self.strs[o..o + end]).into_owned()
    }
}

/// Decode TYPC into canonical type info per index. `member_tab_off` is
/// read from the header; members are (name, type_idx, offset) rows.
pub fn decode_typc(buf: &[u8]) -> Option<Vec<TypeInfo>> {
    if buf.len() < 24 || &buf[..4] != b"TYPC" {
        return None;
    }
    let count = u32at(buf, 8) as usize;
    let mtab_off = u32at(buf, 12) as usize;
    let strtab_off = u32at(buf, 16) as usize;
    if 24 + count * 56 > buf.len() || mtab_off > buf.len() || strtab_off > buf.len() {
        return None;
    }
    let v = TypcView {
        count,
        entries: &buf[24..24 + count * 56],
        strs: &buf[strtab_off..],
    };
    let member = |i: usize| -> (String, u32, u32) {
        let o = mtab_off + i * 16;
        if o + 16 > buf.len() {
            return (String::new(), u32::MAX, 0);
        }
        (v.str(u32at(buf, o)), u32at(buf, o + 4), u32at(buf, o + 8))
    };
    let mut out = vec![
        TypeInfo {
            tag: 0,
            size: 0,
            sig: vec![],
            members: vec![],
            elem: u32::MAX,
            dims: (0, 0),
        };
        count
    ];
    for (i, o) in out.iter_mut().enumerate() {
        let e = &v.entries[i * 56..];
        o.tag = e[0];
        o.size = u32at(e, 28);
        let mstart = u32at(e, 12) as usize;
        let mcount = u32at(e, 16) as usize;
        o.elem = u32at(e, 8);
        o.dims = (u32at(e, 20) as i32, u32at(e, 24) as i32);
        if o.tag == TAG_STRUCT || o.tag == TAG_FB {
            for k in 0..mcount {
                if mstart != u32::MAX as usize {
                    o.members.push(member(mstart + k));
                }
            }
        }
    }
    // signatures: recurse with memo; arrays fold elem sig + bounds,
    // structs fold member (name + sig + offset), enums fold ordinals
    fn sig_of(i: usize, v: &TypcView, out: &[TypeInfo], depth: usize) -> Vec<u8> {
        if i >= v.count || depth > 16 {
            return vec![0xFF];
        }
        let e = &v.entries[i * 56..];
        let t = &out[i];
        let mut s = vec![t.tag];
        s.extend_from_slice(&t.size.to_le_bytes());
        match t.tag {
            TAG_ARRAY => {
                let ei = u32at(e, 8) as usize;
                s.extend_from_slice(&u32at(e, 20).to_le_bytes());
                s.extend_from_slice(&u32at(e, 24).to_le_bytes());
                s.extend_from_slice(&sig_of(ei, v, out, depth + 1));
            }
            TAG_STRUCT | TAG_FB => {
                for (mn, mi, mo) in &t.members {
                    s.extend_from_slice(mn.as_bytes());
                    s.push(0);
                    s.extend_from_slice(&mo.to_le_bytes());
                    s.extend_from_slice(&sig_of(*mi as usize, v, out, depth + 1));
                }
            }
            _ => {}
        }
        s
    }
    let sigs: Vec<Vec<u8>> = (0..count).map(|i| sig_of(i, &v, &out, 0)).collect();
    for (i, s) in sigs.into_iter().enumerate() {
        out[i].sig = s;
    }
    Some(out)
}

/* ---------------- leaf expansion ---------------- */

/// One migratable byte range: a scalar field or a leaf member of a
/// struct/FB instance. `abs_off` is the state-blob byte offset.
#[derive(Debug, Clone)]
pub struct Leaf {
    pub name: String,
    pub abs_off: u64,
    pub size: u64,
    pub tag: u8,
    /// elem sig + elem size for arrays (bounds-grow detection)
    pub elem_sig: Vec<u8>,
    pub elem_size: u64,
    pub lo: i32,
    pub hi: i32,
    pub sig: Vec<u8>,
    /// index into the LAYD field table containing this leaf
    pub field_idx: u32,
    pub retain: bool,
    /// SMAP kind of the containing entry (1 var, 7 fb-instance)
    pub kind: u8,
}

impl Leaf {
    /// array bounds grew with identical element type (doc 08 §3.2)
    pub fn arr_grow_to(&self, new: &Leaf) -> bool {
        self.tag == TAG_ARRAY
            && new.tag == TAG_ARRAY
            && self.elem_sig == new.elem_sig
            && self.elem_size > 0
            && new.size >= self.size
            && (new.lo, new.hi) != (self.lo, self.hi)
    }
}

/// Expand a LAYD field table into leaves, joining each field to its
/// SMAP entry (qualified name) and TYPC type. Struct/FB-typed fields
/// decompose to members recursively; arrays stay whole-field leaves.
pub fn leaf_fields(layd: &Layd, smap: &[SmapEnt], typc: &[TypeInfo]) -> Vec<Leaf> {
    let mut out = Vec::new();
    for (fi, f) in layd.fields.iter().enumerate() {
        let se = f.sym_entry as usize;
        if se >= smap.len() {
            continue;
        }
        let ent = &smap[se];
        if ent.kind == 10 {
            continue; // io channel — image-bound, not blob state
        }
        let ti = f.type_ref as usize;
        let retain = ent.flags & 1 != 0 || ent.region == 3;
        expand(
            &mut out, ent, ti, f.offset, f.size, fi as u32, retain, typc, 0,
        );
    }
    out
}

#[allow(clippy::too_many_arguments)]
fn expand(
    out: &mut Vec<Leaf>,
    ent: &SmapEnt,
    ti: usize,
    abs_off: u64,
    size: u64,
    field_idx: u32,
    retain: bool,
    typc: &[TypeInfo],
    depth: usize,
) {
    if depth > 8 || ti >= typc.len() {
        return;
    }
    let t = &typc[ti];
    if (t.tag == TAG_STRUCT || t.tag == TAG_FB) && !t.members.is_empty() {
        for (mn, mi, mo) in &t.members {
            let mt = &typc[*mi as usize];
            let sub = SmapEnt {
                name: format!("{}.{}", ent.name, mn),
                kind: 1,
                flags: ent.flags,
                region: ent.region,
                type_ref: *mi,
                offset: abs_off + *mo as u64,
                size: mt.size,
            };
            expand(
                out,
                &sub,
                *mi as usize,
                abs_off + *mo as u64,
                mt.size as u64,
                field_idx,
                retain,
                typc,
                depth + 1,
            );
        }
        return;
    }
    let (elem_sig, elem_size, lo, hi) = if t.tag == TAG_ARRAY {
        let e = &typc[t.elem as usize];
        (e.sig.clone(), e.size as u64, t.dims.0, t.dims.1)
    } else {
        (vec![], 0, 0, 0)
    };
    out.push(Leaf {
        name: ent.name.clone(),
        abs_off,
        size,
        tag: t.tag,
        elem_sig,
        elem_size,
        lo,
        hi,
        sig: t.sig.clone(),
        field_idx,
        retain,
        kind: ent.kind,
    });
}

/* ---------------- diff ---------------- */

/// Stable transform identities for `migrate_desc.transform_id` and the
/// generated `tplc_migrate` body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transform {
    IntWiden = 1,    // signed chain SINT→INT→DINT→LINT
    UintWiden = 2,   // unsigned chain USINT→…→ULINT
    RealWiden = 3,   // REAL→LREAL
    StringTrunc = 4, // STRING(n)→STRING(m), m<n → DEGRADED
    ArrayGrow = 5,   // bounds grow: copy prefix, init tail → DEGRADED
}

#[derive(Debug, Clone)]
pub enum Rule {
    Copy {
        old_off: u64,
        new_off: u64,
        size: u64,
        old_field: u32,
        new_field: u32,
    },
    Transform {
        id: Transform,
        old_off: u64,
        new_off: u64,
        old_size: u64,
        new_size: u64,
        old_tag: u8,
        new_tag: u8,
        old_field: u32,
        new_field: u32,
    },
    DefaultInit {
        name: String,
        new_field: u32,
    },
    Dropped {
        name: String,
        retain: bool,
        old_field: u32,
        /// the field is a REFERENCE: its offset is layout-relative,
        /// so a surviving ref is reset to null, never copied
        ref_reset: bool,
    },
    Deny {
        reason: u32,
        detail: String,
        field: u32,
    },
    Alias {
        old_name: String,
        new_name: String,
        old_field: u32,
        new_field: u32,
    },
}

/// deny reason codes (doc 08 §5)
pub const DENY_FB_TYPE: u32 = 1;
pub const DENY_NARROW: u32 = 2;
pub const DENY_TASK: u32 = 3;
pub const DENY_AT_SPACE: u32 = 4;
pub const DENY_ABI: u32 = 5;
pub const DENY_RETAIN_SHRINK: u32 = 6;
pub const DENY_KIND: u32 = 7;
pub const DENY_UNKNOWN: u32 = 8;

#[derive(Debug, Default)]
pub struct Plan {
    pub rules: Vec<Rule>,
    pub degraded: bool,
    pub deny: bool,
    /// reference fields whose offsets stay valid (layout unchanged):
    /// (name, new field index)
    pub refs_preserved: Vec<(String, u32)>,
    /// reference fields that must be nulled (layout moved)
    pub refs_reset: Vec<(String, u32)>,
}

/// true when every field that survives by name keeps its exact offset,
/// size and type signature, and no field was dropped — a reference
/// offset stays valid across such a change
fn layout_stable(old: &Side, new: &Side) -> bool {
    let new_by: std::collections::HashMap<&str, &Leaf> =
        new.leaves.iter().map(|l| (l.name.as_str(), l)).collect();
    for ol in &old.leaves {
        match new_by.get(ol.name.as_str()) {
            Some(nl) => {
                if nl.abs_off != ol.abs_off || nl.size != ol.size || nl.sig != ol.sig {
                    return false;
                }
            }
            None => return false,
        }
    }
    true
}

pub struct Side {
    pub leaves: Vec<Leaf>,
    /// retain region (offset, size) for DENY6
    pub retain_region: (u64, u64),
}

fn widen_int(from: u8, to: u8, signed: bool) -> bool {
    let (lo, hi) = if signed { (1, 4) } else { (5, 8) };
    from >= lo && from <= hi && to >= lo && to <= hi && to > from
}

/// Allowed (from_tag, to_tag) → Transform per doc 08 §3.2.
fn transform_for(old: &Leaf, new: &Leaf) -> Option<Transform> {
    if widen_int(old.tag, new.tag, true) {
        return Some(Transform::IntWiden);
    }
    if widen_int(old.tag, new.tag, false) {
        return Some(Transform::UintWiden);
    }
    if old.tag == 9 && new.tag == 10 {
        return Some(Transform::RealWiden);
    }
    if old.tag == TAG_STRING && new.tag == TAG_STRING {
        return Some(Transform::StringTrunc);
    }
    if old.tag == TAG_ARRAY && new.tag == TAG_ARRAY && old.arr_grow_to(new) {
        return Some(Transform::ArrayGrow);
    }
    None
}

/// Diff two sides. `renames` = explicit alias pairs (old → new).
/// `allow_retain_loss` demotes DENY6 to a DEGRADED report.
pub fn diff(old: &Side, new: &Side, renames: &[(String, String)], allow_retain_loss: bool) -> Plan {
    let mut plan = Plan::default();
    let old_by_name: BTreeMap<&str, &Leaf> =
        old.leaves.iter().map(|l| (l.name.as_str(), l)).collect();
    let new_by_name: BTreeMap<&str, &Leaf> =
        new.leaves.iter().map(|l| (l.name.as_str(), l)).collect();
    let alias_of: BTreeMap<&str, &str> = renames
        .iter()
        .map(|(a, b)| (a.as_str(), b.as_str()))
        .collect();
    /* a reference offset stays valid only when nothing moved */
    let stable = layout_stable(old, new);

    for nl in &new.leaves {
        let lookup = renames
            .iter()
            .find(|(_, n)| n == &nl.name)
            .map(|(o, _)| o.as_str())
            .unwrap_or(nl.name.as_str());
        let ol = old_by_name.get(lookup).copied();
        match ol {
            Some(ol) => {
                if lookup != nl.name {
                    plan.rules.push(Rule::Alias {
                        old_name: lookup.to_string(),
                        new_name: nl.name.clone(),
                        old_field: ol.field_idx,
                        new_field: nl.field_idx,
                    });
                }
                /* a reference stores a blob offset: it survives when
                the referenced layout did not move (every surviving
                field keeps its offset and nothing was dropped), and
                must be nulled otherwise — a stale offset would point
                into the predecessor's layout */
                if ol.tag == TAG_REF || nl.tag == TAG_REF {
                    if stable && ol.sig == nl.sig && ol.size == nl.size {
                        plan.refs_preserved.push((nl.name.clone(), nl.field_idx));
                        plan.rules.push(Rule::Copy {
                            old_off: ol.abs_off,
                            new_off: nl.abs_off,
                            size: nl.size,
                            old_field: ol.field_idx,
                            new_field: nl.field_idx,
                        });
                    } else {
                        plan.degraded = true;
                        plan.refs_reset.push((nl.name.clone(), nl.field_idx));
                        plan.rules.push(Rule::Dropped {
                            name: nl.name.clone(),
                            retain: ol.retain,
                            old_field: ol.field_idx,
                            ref_reset: true,
                        });
                    }
                    continue;
                }
                if ol.sig == nl.sig && ol.size == nl.size {
                    plan.rules.push(Rule::Copy {
                        old_off: ol.abs_off,
                        new_off: nl.abs_off,
                        size: nl.size,
                        old_field: ol.field_idx,
                        new_field: nl.field_idx,
                    });
                } else if ol.kind != nl.kind {
                    plan.rules.push(Rule::Deny {
                        reason: DENY_KIND,
                        detail: format!("{}: kind changed ({} → {})", nl.name, ol.kind, nl.kind),
                        field: nl.field_idx,
                    });
                    plan.deny = true;
                } else if let Some(t) = transform_for(ol, nl) {
                    if matches!(t, Transform::StringTrunc | Transform::ArrayGrow) {
                        plan.degraded = true;
                    }
                    plan.rules.push(Rule::Transform {
                        id: t,
                        old_off: ol.abs_off,
                        new_off: nl.abs_off,
                        old_size: ol.size,
                        new_size: nl.size,
                        old_tag: ol.tag,
                        new_tag: nl.tag,
                        old_field: ol.field_idx,
                        new_field: nl.field_idx,
                    });
                } else {
                    plan.rules.push(Rule::Deny {
                        reason: DENY_NARROW,
                        detail: format!(
                            "{}: type change not in transform table \
                             (tag {} → {})",
                            nl.name, ol.tag, nl.tag
                        ),
                        field: nl.field_idx,
                    });
                    plan.deny = true;
                }
            }
            None => plan.rules.push(Rule::DefaultInit {
                name: nl.name.clone(),
                new_field: nl.field_idx,
            }),
        }
    }
    // dropped: old leaves with no new home (alias targets count as
    // their new home)
    for ol in &old.leaves {
        let target = alias_of
            .get(ol.name.as_str())
            .copied()
            .unwrap_or(ol.name.as_str());
        if !new_by_name.contains_key(target) {
            // a dropped *member* inside a surviving FB instance is a
            // type change (DENY2), not a dropped field
            if let Some(dot) = ol.name.rfind('.') {
                let parent = &ol.name[..dot];
                if new
                    .leaves
                    .iter()
                    .any(|nl| nl.name == parent || nl.name.starts_with(&format!("{}.", parent)))
                {
                    plan.rules.push(Rule::Deny {
                        reason: DENY_NARROW,
                        detail: format!(
                            "{}: member removed from preserved \
                             instance",
                            ol.name
                        ),
                        field: ol.field_idx,
                    });
                    plan.deny = true;
                    continue;
                }
            }
            plan.degraded = true;
            plan.rules.push(Rule::Dropped {
                name: ol.name.clone(),
                retain: ol.retain,
                old_field: ol.field_idx,
                ref_reset: ol.tag == TAG_REF,
            });
        }
    }
    // DENY6: retain region shrink below used
    let old_retain_used: u64 = old.leaves.iter().filter(|l| l.retain).map(|l| l.size).sum();
    if old_retain_used > new.retain_region.1 {
        if allow_retain_loss {
            plan.degraded = true;
        } else {
            plan.deny = true;
        }
        plan.rules.push(Rule::Deny {
            reason: DENY_RETAIN_SHRINK,
            detail: format!(
                "retain region shrinks below used \
                 ({} B used → {} B region)",
                old_retain_used, new.retain_region.1
            ),
            field: u32::MAX,
        });
    }
    plan
}

/* ---------------- codegen-facing emit form ---------------- */

/// A single straight-line operation `tplc_migrate` performs.
#[derive(Debug, Clone)]
pub enum MigOp {
    /// memcpy(new_blob+new_off, old_blob+old_off, size)
    Copy {
        old_off: u64,
        new_off: u64,
        size: u64,
    },
    /// load old field (old_tag), cast, store at new_off (new_tag)
    Widen {
        old_off: u64,
        new_off: u64,
        old_tag: u8,
        new_tag: u8,
    },
    /// memcpy min(old,new) bytes (string trunc / array prefix)
    CopyMin {
        old_off: u64,
        new_off: u64,
        old_size: u64,
        new_size: u64,
    },
}

/// Everything codegen needs to emit `tplc_migrate`.
pub struct MigrateEmit {
    pub predecessor_hash: [u8; 32],
    pub ops: Vec<MigOp>,
    pub degraded: bool,
}

/// Lower a Plan to straight-line ops; deny rules mean the caller must
/// not reach codegen at all (compile-time gate).
pub fn plan_to_emit(plan: &Plan) -> MigrateEmit {
    let mut ops = Vec::new();
    for r in &plan.rules {
        match *r {
            Rule::Copy {
                old_off,
                new_off,
                size,
                ..
            } => ops.push(MigOp::Copy {
                old_off,
                new_off,
                size,
            }),
            Rule::Transform {
                id,
                old_off,
                new_off,
                old_size,
                new_size,
                old_tag,
                new_tag,
                ..
            } => match id {
                Transform::IntWiden | Transform::UintWiden | Transform::RealWiden => {
                    ops.push(MigOp::Widen {
                        old_off,
                        new_off,
                        old_tag,
                        new_tag,
                    })
                }
                Transform::StringTrunc | Transform::ArrayGrow => ops.push(MigOp::CopyMin {
                    old_off,
                    new_off,
                    old_size,
                    new_size,
                }),
            },
            _ => {}
        }
    }
    MigrateEmit {
        predecessor_hash: [0; 32], // caller stamps the real hash
        ops,
        degraded: plan.degraded,
    }
}
