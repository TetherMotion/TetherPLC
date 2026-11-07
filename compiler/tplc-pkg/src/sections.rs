//! Codecs for the binary section formats the runtime consumes directly:
//! `layout_desc` (LAYD, doc 01 §7) and `migrate_desc` (MIGD, doc 01 §8).
//! Byte layouts are identical to `tests/abi/desc_formats.h` — the ABI
//! conformance module parses these buffers in C, so this codec must
//! produce/consume exactly the same bytes.

use crate::error::{PkgError, Result};

fn need(buf: &[u8], at: usize, n: usize) -> Result<()> {
    if at > buf.len() || n > buf.len() - at {
        return Err(PkgError::SectionFormat);
    }
    Ok(())
}

fn u32at(buf: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(buf[at..at + 4].try_into().unwrap())
}
fn u64at(buf: &[u8], at: usize) -> u64 {
    u64::from_le_bytes(buf[at..at + 8].try_into().unwrap())
}
fn u16at(buf: &[u8], at: usize) -> u16 {
    u16::from_le_bytes(buf[at..at + 2].try_into().unwrap())
}

/* ------------------------------------------------------------------ */
/* LAYD                                                                */
/* ------------------------------------------------------------------ */

pub const LAYD_MAGIC: u32 = u32::from_le_bytes(*b"LAYD");
pub const LAYD_VERSION: u32 = 1;
pub const LAYD_HEADER: usize = 24;
pub const LAYD_REGION: usize = 24;
pub const LAYD_FIELD: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaydRegionKind {
    Global = 0,
    Fb = 1,
    TaskLocal = 2,
    Retain = 3,
    Internal = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LaydRegion {
    pub kind: u8,
    pub align: u32,
    pub offset: u64,
    pub size: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LaydField {
    pub sym_entry: u32,
    pub field_kind: u8,
    pub flags: u8,
    pub gen: u16,
    pub offset: u64,
    pub size: u64,
    pub type_ref: u32,
}

/// Decoded `layout_desc` section.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layd {
    pub version: u32,
    pub total_size: u64,
    pub regions: Vec<LaydRegion>,
    pub fields: Vec<LaydField>,
}

impl Layd {
    pub fn encode(&self) -> Vec<u8> {
        let mut o = Vec::with_capacity(
            LAYD_HEADER + self.regions.len() * LAYD_REGION + 4 + self.fields.len() * LAYD_FIELD,
        );
        o.extend_from_slice(&LAYD_MAGIC.to_le_bytes());
        o.extend_from_slice(&self.version.to_le_bytes());
        o.extend_from_slice(&(self.regions.len() as u32).to_le_bytes());
        o.extend_from_slice(&0u32.to_le_bytes());
        o.extend_from_slice(&self.total_size.to_le_bytes());
        for r in &self.regions {
            o.push(r.kind);
            o.extend_from_slice(&[0; 3]);
            o.extend_from_slice(&r.align.to_le_bytes());
            o.extend_from_slice(&r.offset.to_le_bytes());
            o.extend_from_slice(&r.size.to_le_bytes());
        }
        o.extend_from_slice(&(self.fields.len() as u32).to_le_bytes());
        for f in &self.fields {
            o.extend_from_slice(&f.sym_entry.to_le_bytes());
            o.push(f.field_kind);
            o.push(f.flags);
            o.extend_from_slice(&f.gen.to_le_bytes());
            o.extend_from_slice(&f.offset.to_le_bytes());
            o.extend_from_slice(&f.size.to_le_bytes());
            o.extend_from_slice(&f.type_ref.to_le_bytes());
            o.extend_from_slice(&0u32.to_le_bytes());
        }
        o
    }

    pub fn decode(buf: &[u8]) -> Result<Self> {
        need(buf, 0, LAYD_HEADER)?;
        if u32at(buf, 0) != LAYD_MAGIC {
            return Err(PkgError::SectionFormat);
        }
        let version = u32at(buf, 4);
        if version > LAYD_VERSION {
            return Err(PkgError::Version);
        }
        let region_count = u32at(buf, 8) as usize;
        let total_size = u64at(buf, 16);
        let mut at = LAYD_HEADER;
        let mut regions = Vec::with_capacity(region_count);
        for _ in 0..region_count {
            need(buf, at, LAYD_REGION)?;
            regions.push(LaydRegion {
                kind: buf[at],
                align: u32at(buf, at + 4),
                offset: u64at(buf, at + 8),
                size: u64at(buf, at + 16),
            });
            at += LAYD_REGION;
        }
        need(buf, at, 4)?;
        let field_count = u32at(buf, at) as usize;
        at += 4;
        let mut fields = Vec::with_capacity(field_count);
        for _ in 0..field_count {
            need(buf, at, LAYD_FIELD)?;
            fields.push(LaydField {
                sym_entry: u32at(buf, at),
                field_kind: buf[at + 4],
                flags: buf[at + 5],
                gen: u16at(buf, at + 6),
                offset: u64at(buf, at + 8),
                size: u64at(buf, at + 16),
                type_ref: u32at(buf, at + 24),
            });
            at += LAYD_FIELD;
        }
        Ok(Self {
            version,
            total_size,
            regions,
            fields,
        })
    }
}

/* ------------------------------------------------------------------ */
/* MIGD                                                                */
/* ------------------------------------------------------------------ */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MigdRule {
    /// 0 copy, 1 transform, 2 default-init, 3 dropped, 4 deny, 5 alias-rename
    pub kind: u8,
    /// 0 info, 1 degraded, 2 deny
    pub severity: u8,
    pub transform_id: u16,
    pub old_field: u32,
    pub new_field: u32,
    pub aux: u32,
}

/// Decoded `migrate_desc` section — predecessor hash/version, rule table,
/// embedded predecessor LAYD (doc 01 §8).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Migd {
    pub predecessor_layout_hash: [u8; 32],
    pub predecessor_version: Vec<u8>,
    pub rules: Vec<MigdRule>,
    pub predecessor_layd: Layd,
}

impl Migd {
    pub fn encode(&self) -> Vec<u8> {
        let layd = self.predecessor_layd.encode();
        let ver_len = self.predecessor_version.len() as u32;
        let body_start = 32 + 2 + ver_len as usize;
        let pad = (4 - (body_start % 4)) % 4;
        let mut o = Vec::with_capacity(body_start + pad + 4 + self.rules.len() * 16 + layd.len());
        o.extend_from_slice(&self.predecessor_layout_hash);
        o.extend_from_slice(&(self.predecessor_version.len() as u16).to_le_bytes());
        o.extend_from_slice(&self.predecessor_version);
        o.resize(o.len() + pad, 0);
        o.extend_from_slice(&(self.rules.len() as u32).to_le_bytes());
        for r in &self.rules {
            o.push(r.kind);
            o.push(r.severity);
            o.extend_from_slice(&r.transform_id.to_le_bytes());
            o.extend_from_slice(&r.old_field.to_le_bytes());
            o.extend_from_slice(&r.new_field.to_le_bytes());
            o.extend_from_slice(&r.aux.to_le_bytes());
        }
        o.extend_from_slice(&layd);
        o
    }

    /// `migrate_desc_size` is passed separately by the runtime; decode takes
    /// the exact buffer.
    pub fn decode(buf: &[u8]) -> Result<Self> {
        need(buf, 0, 34)?;
        let mut pred_hash = [0u8; 32];
        pred_hash.copy_from_slice(&buf[..32]);
        let vlen = u16at(buf, 32) as usize;
        let mut at = 34;
        need(buf, at, vlen)?;
        let pred_version = buf[at..at + vlen].to_vec();
        at += vlen;
        at += (4 - (at % 4)) % 4;
        need(buf, at, 4)?;
        let rule_count = u32at(buf, at) as usize;
        at += 4;
        let mut rules = Vec::with_capacity(rule_count);
        for _ in 0..rule_count {
            need(buf, at, 16)?;
            rules.push(MigdRule {
                kind: buf[at],
                severity: buf[at + 1],
                transform_id: u16at(buf, at + 2),
                old_field: u32at(buf, at + 4),
                new_field: u32at(buf, at + 8),
                aux: u32at(buf, at + 12),
            });
            at += 16;
        }
        let predecessor_layd = Layd::decode(&buf[at..])?;
        Ok(Self {
            predecessor_layout_hash: pred_hash,
            predecessor_version: pred_version,
            rules,
            predecessor_layd,
        })
    }
}
