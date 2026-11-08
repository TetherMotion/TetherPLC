//! Descriptor emission (P1-04): `layout_desc` field linkage +
//! `symmap` (SMAP) + `type_catalog` (TYPC) section payloads from a
//! computed `Layout` (doc 01 §6/§7). The runtime parses LAYD only;
//! SMAP/TYPC are debugger/force/watch metadata — emitted here as
//! canonical bytes for the package writer.

use crate::ir::*;
use crate::layout::{builtin_fb_layout, FbLayout, Layout, REGION_RETAIN};
use std::collections::BTreeMap;

const NONE: u32 = 0xFFFF_FFFF;

fn le32(v: u32, b: &mut Vec<u8>) {
    b.extend_from_slice(&v.to_le_bytes());
}
fn le64(v: u64, b: &mut Vec<u8>) {
    b.extend_from_slice(&v.to_le_bytes());
}
fn lei64(v: i64, b: &mut Vec<u8>) {
    b.extend_from_slice(&v.to_le_bytes());
}

struct Strtab {
    map: BTreeMap<String, u32>,
    buf: Vec<u8>,
}
impl Strtab {
    fn new() -> Self {
        Strtab {
            map: BTreeMap::new(),
            buf: vec![0],
        } // 0 = ""
    }
    fn intern(&mut self, s: &str) -> u32 {
        if let Some(&o) = self.map.get(s) {
            return o;
        }
        let o = self.buf.len() as u32;
        self.buf.extend_from_slice(s.as_bytes());
        self.buf.push(0);
        self.map.insert(s.to_string(), o);
        o
    }
}

// ---------- type_catalog (doc 01 §6.6) ----------
// Canonical entry (56 B):
//   +0  tag u8, flags u8, dims u16
//   +4  name_off u32          (section-local strtab)
//   +8  elem u32              (array elem type idx; else NONE)
//   +12 member_start u32      (member-table start; else NONE)
//   +16 member_count u32
//   +20 dim_lo i32, +24 dim_hi i32   (first dim; multi-dim reserved)
//   +28 size u32, +32 align u32
//   +36..+56 reserved
// ENUM entries (tag 20) are followed immediately by their tail:
//   count u32, then count × (name_off u32, ordinal i64)
// Member-table record (16 B): name_off u32, type_idx u32,
//   offset u32, flags u32(bit0 retain bit1 const)

fn tag_of(ty: &Type) -> u8 {
    match ty {
        Type::Bool => 0,
        Type::Int {
            signed: true,
            bits: 8,
        } => 1,
        Type::Int {
            signed: true,
            bits: 16,
        } => 2,
        Type::Int {
            signed: true,
            bits: 32,
        } => 3,
        Type::Int {
            signed: true,
            bits: 64,
        } => 4,
        Type::Int {
            signed: false,
            bits: 8,
        } => 5,
        Type::Int {
            signed: false,
            bits: 16,
        } => 6,
        Type::Int {
            signed: false,
            bits: 32,
        } => 7,
        Type::Int {
            signed: false,
            bits: 64,
        } => 8,
        Type::Real { bits: 32 } => 9,
        Type::Real { bits: 64 } => 10,
        Type::Time => 11,
        Type::Str { .. } => 15,
        Type::Array { .. } => 18,
        Type::Struct { .. } => 19,
        Type::Enum { .. } => 20,
        Type::Fb { .. } => 21,
        Type::Int { signed, bits } => match (signed, bits) {
            (true, 8) => 1,
            (true, 16) => 2,
            (true, 32) => 3,
            (true, _) => 4,
            (false, 8) => 5,
            (false, 16) => 6,
            (false, 32) => 7,
            (false, _) => 8,
        },
        Type::Real { bits } => {
            if *bits == 32 {
                9
            } else {
                10
            }
        }
    }
}

struct Catalog<'a> {
    types: Vec<Type>,
    member_records: Vec<(String, u32, u32, u32)>,
    member_ranges: BTreeMap<usize, (u32, u32)>, // type idx → (start, count)
    fbl: &'a BTreeMap<String, FbLayout>,
}

impl<'a> Catalog<'a> {
    fn new(fbl: &'a BTreeMap<String, FbLayout>) -> Self {
        Catalog {
            types: Vec::new(),
            member_records: Vec::new(),
            member_ranges: BTreeMap::new(),
            fbl,
        }
    }

    fn index_of(&mut self, ty: &Type) -> u32 {
        if let Some(i) = self.types.iter().position(|t| t == ty) {
            return i as u32;
        }
        let idx = self.types.len() as u32;
        self.types.push(ty.clone());
        match ty {
            Type::Struct { name: _, fields } => {
                let start = self.member_records.len() as u32;
                let mut off = 0u64;
                for (n, t) in fields {
                    off = (off + t.align() - 1) & !(t.align() - 1);
                    let ti = self.index_of(t);
                    self.member_records.push((n.clone(), ti, off as u32, 0));
                    off += t.size();
                }
                self.member_ranges
                    .insert(idx as usize, (start, fields.len() as u32));
            }
            Type::Fb { name } => {
                if let Some(fbl) = self
                    .fbl
                    .get(name)
                    .cloned()
                    .or_else(|| builtin_fb_layout(name))
                {
                    let start = self.member_records.len() as u32;
                    for (mn, (moff, mty)) in &fbl.members {
                        let ti = self.index_of(mty);
                        self.member_records.push((mn.clone(), ti, *moff as u32, 0));
                    }
                    self.member_ranges
                        .insert(idx as usize, (start, fbl.members.len() as u32));
                }
            }
            Type::Array { elem, .. } => {
                self.index_of(elem);
            }
            _ => {}
        }
        idx
    }

    fn emit(&self) -> Vec<u8> {
        let mut st = Strtab::new();
        let mut member_table = Vec::new();
        for (n, tidx, off, fl) in &self.member_records {
            le32(st.intern(n), &mut member_table);
            le32(*tidx, &mut member_table);
            le32(*off, &mut member_table);
            le32(*fl, &mut member_table);
        }
        let count = self.types.len() as u32;
        let hdr = 24usize;
        // entries are 56 B each + enum tails appended inline
        let mut enum_tail_bytes = 0usize;
        for t in &self.types {
            if let Type::Enum { values, .. } = t {
                enum_tail_bytes += 4 + values.len() * 12;
            }
        }
        let entries_bytes = count as usize * 56 + enum_tail_bytes;
        let member_table_off = hdr + entries_bytes;
        let strtab_off = member_table_off + member_table.len();

        let mut out = Vec::new();
        le32(u32::from_le_bytes(*b"TYPC"), &mut out);
        le32(1, &mut out);
        le32(count, &mut out);
        le32(member_table_off as u32, &mut out);
        le32(strtab_off as u32, &mut out);
        le32(0, &mut out); // strtab size computed by reader (end of section)

        for (i, t) in self.types.iter().enumerate() {
            let mut e = Vec::new();
            let name = match t {
                Type::Struct { name, .. } | Type::Enum { name, .. } | Type::Fb { name } => {
                    st.intern(name)
                }
                _ => st.intern(""),
            };
            let (elem, dims, lo, hi) = match t {
                Type::Array { elem, lo, hi } => (
                    self.types
                        .iter()
                        .position(|x| x == elem.as_ref())
                        .unwrap_or(0) as u32,
                    1,
                    *lo,
                    *hi,
                ),
                _ => (NONE, 0, 0i64, 0i64),
            };
            let (ms, mc) = self.member_ranges.get(&i).copied().unwrap_or((NONE, 0));
            e.push(tag_of(t));
            e.push(0);
            e.extend_from_slice(&(dims as u16).to_le_bytes());
            le32(name, &mut e);
            le32(elem, &mut e);
            le32(ms, &mut e);
            le32(mc, &mut e);
            e.extend_from_slice(&(lo as i32).to_le_bytes());
            e.extend_from_slice(&(hi as i32).to_le_bytes());
            le32(t.size() as u32, &mut e);
            le32(t.align() as u32, &mut e);
            e.resize(56, 0);
            out.extend_from_slice(&e);
            if let Type::Enum { values, .. } = t {
                le32(values.len() as u32, &mut out);
                for (ord, v) in values.iter().enumerate() {
                    le32(st.intern(v), &mut out);
                    lei64(ord as i64, &mut out);
                }
            }
        }
        out.extend_from_slice(&member_table);
        out.extend_from_slice(&st.buf);
        out
    }
}

// ---------- symmap (doc 01 §6.1–6.5) ----------

const SK_VAR: u8 = 1;
const SK_FB_INSTANCE: u8 = 7;
const SK_PROGRAM_INSTANCE: u8 = 8;
const SK_IO_CHANNEL: u8 = 10;

struct SymEntry {
    name: String,
    kind: u8,
    flags: u8,
    region: u8,
    access: u8,
    type_ref: u32,
    offset: u64,
    size: u32,
    align: u32,
    parent: u32,
    first_child: u32,
    next_sibling: u32,
    provenance: u32,
}

/// Emitted package metadata for one compiled program.
pub struct Descriptors {
    /// LAYD with sym_entry + type_ref indices wired
    pub layd: tplc_pkg::sections::Layd,
    pub symmap: Vec<u8>,
    pub typecat: Vec<u8>,
    /// layd_bytes || symmap || typecat — layout_hash preimage order
    /// (doc 01 §6.5)
    pub payload_for_hash: Vec<u8>,
}

/// Emit descriptors for one program instance's layout.
/// `io` = (qualified var name, at, image-offset) channel entries.
/// `prov_of` resolves a qualified field name → declaration provenance.
pub fn emit(
    layout: &mut Layout,
    program: &Pou,
    io: &[(String, At, u64)],
    prov_of: &dyn Fn(&str) -> Option<Prov>,
) -> Descriptors {
    let mut cat = Catalog::new(&layout.fb_types);
    let mut st = Strtab::new();
    let mut provs: Vec<(u32, u32, u32, u8)> = Vec::new();
    let mut prov_index = |p: Option<Prov>| -> u32 {
        match p {
            Some(Prov::St { file, offset, len }) => {
                let i = provs.len() as u32;
                provs.push((file, offset, len, 0));
                i
            }
            Some(Prov::Fbd { sheet, node, pin }) => {
                let i = provs.len() as u32;
                provs.push((sheet, node, pin as u32, 1));
                i
            }
            _ => NONE,
        }
    };

    let mut entries: Vec<SymEntry> = Vec::new();

    // entry 0: program instance root
    entries.push(SymEntry {
        name: program.name.clone(),
        kind: SK_PROGRAM_INSTANCE,
        flags: 0,
        region: 0,
        access: 0,
        type_ref: NONE,
        offset: 0,
        size: layout.total_size as u32,
        align: layout.align as u32,
        parent: NONE,
        first_child: NONE,
        next_sibling: NONE,
        provenance: NONE,
    });

    // layd field index ↔ field map iteration order are identical
    let field_list: Vec<(String, crate::layout::FieldInfo)> = layout
        .fields
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    // type_ref wiring on LAYD fields happens below while walking them.
    let mut top_entries: Vec<u32> = Vec::new();
    let mut off_to_sym: BTreeMap<u64, u32> = BTreeMap::new();

    for (qname, f) in field_list.iter() {
        if qname.contains('.') {
            continue; // GVL-qualified handled with their pseudo-parent later
        }
        let is_fb = f.field_kind == 1;
        let tref = cat.index_of(&f.ty);
        let idx = entries.len() as u32;
        off_to_sym.insert(f.offset, idx);
        entries.push(SymEntry {
            name: qname.clone(),
            kind: if is_fb { SK_FB_INSTANCE } else { SK_VAR },
            flags: (f.retain as u8) | ((is_fb as u8) << 4),
            region: if f.retain { REGION_RETAIN } else { 0 },
            // access: 0 rw, 1 ro (VAR_CONSTANT), 2 wo marker (%Q
            // channel entries only)
            access: if f.is_const { 1 } else { 0 },
            type_ref: tref,
            offset: f.offset,
            size: f.size as u32,
            align: f.ty.align() as u32,
            parent: 0,
            first_child: NONE,
            next_sibling: NONE,
            provenance: prov_index(prov_of(qname)),
        });
        top_entries.push(idx);

        // FB instance → member children (one level; nested FB members
        // flatten under the same parent for v1)
        if let Type::Fb { name } = &f.ty {
            if let Some(fbl) = layout
                .fb_types
                .get(name)
                .cloned()
                .or_else(|| builtin_fb_layout(name))
            {
                let mut child_idxs = Vec::new();
                for (mn, (moff, mty)) in &fbl.members {
                    if mn.starts_with("__") {
                        continue; // bookkeeping is internal, not symmap
                    }
                    let cidx = entries.len() as u32;
                    entries.push(SymEntry {
                        name: format!("{qname}.{mn}"),
                        kind: SK_VAR,
                        flags: (f.retain as u8) | 2, // forced-capable leaf
                        region: if f.retain { REGION_RETAIN } else { 0 },
                        access: if f.is_const { 1 } else { 0 },
                        type_ref: cat.index_of(mty),
                        offset: f.offset + *moff,
                        size: mty.size() as u32,
                        align: mty.align() as u32,
                        parent: idx,
                        first_child: NONE,
                        next_sibling: NONE,
                        provenance: NONE,
                    });
                    child_idxs.push(cidx);
                }
                if let Some(&first) = child_idxs.first() {
                    entries[idx as usize].first_child = first;
                    for (ci, &c) in child_idxs.iter().enumerate() {
                        if let Some(&n) = child_idxs.get(ci + 1) {
                            entries[c as usize].next_sibling = n;
                        }
                    }
                }
            }
        }
    }
    // sibling chain among top-level entries
    for (i, &r) in top_entries.iter().enumerate() {
        if let Some(&next) = top_entries.get(i + 1) {
            entries[r as usize].next_sibling = next;
        }
    }
    if let Some(&first) = top_entries.first() {
        entries[0].first_child = first;
    }

    // io channel entries
    for (name, at, img_off) in io {
        entries.push(SymEntry {
            name: name.clone(),
            kind: SK_IO_CHANNEL,
            flags: 0,
            region: match at.region {
                AtRegion::I => 1,
                AtRegion::Q => 2,
                AtRegion::M => 3,
            },
            access: if at.region == AtRegion::Q { 2 } else { 0 },
            type_ref: NONE,
            offset: *img_off,
            size: match at.width {
                'X' => 1,
                'B' => 1,
                'W' => 2,
                'D' => 4,
                'L' => 8,
                _ => 1,
            },
            align: 1,
            parent: NONE,
            first_child: NONE,
            next_sibling: NONE,
            provenance: NONE,
        });
    }

    // sort non-root entries by (region, offset) per §6.5
    let (head, tail) = entries.split_at_mut(1);
    tail.sort_by_key(|e| (e.region, e.offset));
    let _ = head;

    // provenance table bytes
    let mut prov_bytes = Vec::new();
    le32(provs.len() as u32, &mut prov_bytes);
    for (f, o, l, k) in &provs {
        le32(*f, &mut prov_bytes);
        le32(*o, &mut prov_bytes);
        le32(*l, &mut prov_bytes);
        prov_bytes.push(*k);
        prov_bytes.extend_from_slice(&[0, 0, 0]);
    }

    for e in &entries {
        st.intern(&e.name);
    }
    let count = entries.len() as u32;
    let hdr = 32usize;
    let prov_off = hdr + count as usize * 64;
    let strtab_off = prov_off + prov_bytes.len();

    let mut out = Vec::new();
    le32(u32::from_le_bytes(*b"SMAP"), &mut out);
    le32(1, &mut out);
    le32(count, &mut out);
    le32(strtab_off as u32, &mut out);
    le32(st.buf.len() as u32, &mut out);
    le32(0, &mut out); // typedesc_base
    le32(if provs.is_empty() { 0 } else { 1 }, &mut out); // flags
    le32(0, &mut out);

    for e in &entries {
        let mut eb = Vec::new();
        le32(st.map[&e.name], &mut eb);
        eb.push(e.kind);
        eb.push(e.flags);
        eb.push(e.region);
        eb.push(e.access);
        le32(e.type_ref, &mut eb);
        le64(e.offset, &mut eb);
        le32(e.size, &mut eb);
        le32(e.align, &mut eb);
        le32(e.parent, &mut eb);
        le32(e.first_child, &mut eb);
        le32(e.next_sibling, &mut eb);
        le32(e.provenance, &mut eb);
        le32(NONE, &mut eb);
        eb.resize(64, 0);
        out.extend_from_slice(&eb);
    }
    out.extend_from_slice(&prov_bytes);
    out.extend_from_slice(&st.buf);

    let typecat = cat.emit();

    // LAYD wiring: match each field-table row to its field by unique
    // blob offset → symmap entry index + type catalog index. (LAYD
    // order is region-bucket order, symmap emission order is name
    // order — offsets are the join key.)
    let off_to_ty: BTreeMap<u64, u32> = field_list
        .iter()
        .map(|(_, f)| (f.offset, cat.index_of(&f.ty)))
        .collect();
    for lf in layout.layd.fields.iter_mut() {
        if let Some(&s) = off_to_sym.get(&lf.offset) {
            lf.sym_entry = s;
        }
        if let Some(&t) = off_to_ty.get(&lf.offset) {
            lf.type_ref = t;
        }
    }

    let mut payload_for_hash = Vec::new();
    payload_for_hash.extend_from_slice(&layout.layd.encode());
    payload_for_hash.extend_from_slice(&out);
    payload_for_hash.extend_from_slice(&typecat);

    Descriptors {
        layd: layout.layd.clone(),
        symmap: out,
        typecat,
        payload_for_hash,
    }
}
