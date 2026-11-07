//! `.tplc` package reader — verifies integrity then exposes sections
//! zero-copy. Implements every reader invariant from doc 01 §2.

use crate::error::{PkgError, Result};
use crate::format::*;
use sha2::{Digest, Sha256};

/// A verified section payload view into the parsed package buffer.
#[derive(Debug, Clone, Copy)]
pub struct Section<'a> {
    pub section_type: u32,
    pub flags: u32,
    pub section_version: u32,
    pub aux: u32,
    pub data: &'a [u8],
}

impl<'a> Section<'a> {
    pub fn required(&self) -> bool {
        self.flags & SECF_REQUIRED != 0
    }
    pub fn compressed(&self) -> bool {
        self.flags & SECF_COMPRESSED != 0
    }
    pub fn known_type(&self) -> Option<SectionType> {
        SectionType::from_u32(self.section_type)
    }
}

/// Parsed + verified package. Cheap to construct: one pass over the TOC and
/// two SHA-256 passes; payload slices borrow from the input.
#[derive(Debug)]
pub struct Package<'a> {
    pub header: Header,
    toc: Vec<TocEntry>,
    data: &'a [u8],
}

impl<'a> Package<'a> {
    /// Parse and fully verify (header hash, content hash, all invariants).
    pub fn parse(data: &'a [u8]) -> Result<Self> {
        if data.len() < HEADER_FIXED {
            return Err(PkgError::Magic);
        }
        if &data[0..8] != MAGIC {
            return Err(PkgError::Magic);
        }
        let header = Header::decode(&data[..HEADER_FIXED]);

        if header.format_version > FORMAT_VERSION {
            return Err(PkgError::Version);
        }
        if header.total_size != data.len() as u64 || data.len() as u64 > MAX_FILE {
            return Err(PkgError::Limit);
        }
        if header.toc_count > MAX_TOC {
            return Err(PkgError::Toc);
        }

        // header_hash covers bytes 0..128.
        let hh: [u8; 32] = Sha256::digest(&data[..128]).into();
        if hh != header.header_hash {
            return Err(PkgError::HashHeader);
        }

        let want_header = crate::writer::align_up(
            HEADER_FIXED as u64 + header.toc_count as u64 * 32,
            SECTION_ALIGN,
        );
        if header.header_size as u64 != want_header || data.len() < header.header_size as usize {
            return Err(PkgError::Toc);
        }

        // TOC.
        let mut toc = Vec::with_capacity(header.toc_count as usize);
        for i in 0..header.toc_count as usize {
            let at = HEADER_FIXED + i * TOC_ENTRY_SIZE;
            toc.push(TocEntry::decode(&data[at..at + TOC_ENTRY_SIZE]));
        }

        // TOC invariants: sorted, non-overlapping, aligned, in-bounds, sized.
        let mut prev_end = header.header_size as u64;
        let mut prev_type = 0u32;
        for (i, e) in toc.iter().enumerate() {
            if i > 0 && e.section_type <= prev_type {
                return Err(PkgError::Toc); // strictly increasing = sorted, unique
            }
            prev_type = e.section_type;
            if e.offset % SECTION_ALIGN != 0
                || e.offset < prev_end
                || e.offset > header.total_size
                || e.size > header.total_size - e.offset
            {
                return Err(PkgError::Toc);
            }
            if let Some(t) = SectionType::from_u32(e.section_type) {
                if let Some(lim) = t.size_limit() {
                    if e.size > lim {
                        return Err(PkgError::Limit);
                    }
                }
            } else if e.flags & SECF_REQUIRED != 0 {
                return Err(PkgError::UnknownSection);
            }
            prev_end = e.offset + e.size;
        }

        // content_hash over raw TOC bytes then payloads in TOC order
        // (matches writer; D-IMPL-13).
        let mut ch = Sha256::new();
        ch.update(&data[HEADER_FIXED..HEADER_FIXED + toc.len() * TOC_ENTRY_SIZE]);
        for e in &toc {
            ch.update(&data[e.offset as usize..(e.offset + e.size) as usize]);
        }
        let ch: [u8; 32] = ch.finalize().into();
        if ch != header.content_hash {
            return Err(PkgError::HashContent);
        }

        Ok(Self { header, toc, data })
    }

    /// All TOC entries in canonical order.
    pub fn toc(&self) -> &[TocEntry] {
        &self.toc
    }

    /// First section of a given type.
    pub fn section(&self, t: SectionType) -> Option<Section<'a>> {
        self.sections_of(t as u32).next()
    }

    /// All sections of a given type (e.g. multiple `target_obj`).
    pub fn sections_of(&self, type_id: u32) -> impl Iterator<Item = Section<'a>> + '_ {
        let data = self.data;
        self.toc
            .iter()
            .filter(move |e| e.section_type == type_id)
            .map(move |e| Section {
                section_type: e.section_type,
                flags: e.flags,
                section_version: e.section_version,
                aux: e.aux,
                data: &data[e.offset as usize..(e.offset + e.size) as usize],
            })
    }

    pub fn sections(&self) -> impl Iterator<Item = Section<'a>> + '_ {
        let data = self.data;
        self.toc.iter().map(move |e| Section {
            section_type: e.section_type,
            flags: e.flags,
            section_version: e.section_version,
            aux: e.aux,
            data: &data[e.offset as usize..(e.offset + e.size) as usize],
        })
    }

    /// Structural check for loadability (doc 01 §2.1): manifest, symmap,
    /// type_catalog, layout_desc, ≥1 target_obj, and ir_bitcode unless the
    /// manifest declares `ir_stripped`. Format-level only — semantic
    /// requires/ABI/target checks belong to the host loader.
    pub fn validate_for_load(&self, ir_stripped: bool) -> Result<()> {
        for t in [
            SectionType::Manifest,
            SectionType::Symmap,
            SectionType::TypeCatalog,
            SectionType::LayoutDesc,
        ] {
            if self.section(t).is_none() {
                return Err(PkgError::Toc);
            }
        }
        if !ir_stripped && self.section(SectionType::IrBitcode).is_none() {
            return Err(PkgError::Toc);
        }
        if self
            .sections_of(SectionType::TargetObj as u32)
            .next()
            .is_none()
        {
            return Err(PkgError::TargetMismatch);
        }
        Ok(())
    }

    /// Convenience: the manifest as UTF-8 JSON text.
    pub fn manifest_json(&self) -> Result<&'a str> {
        let s = self.section(SectionType::Manifest).ok_or(PkgError::Toc)?;
        std::str::from_utf8(s.data).map_err(|_| PkgError::SectionFormat)
    }
}
