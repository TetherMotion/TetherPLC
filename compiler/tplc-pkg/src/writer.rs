//! `.tplc` package writer — produces spec-canonical bytes (doc 01 §2).

use crate::error::{PkgError, Result};
use crate::format::*;
use sha2::{Digest, Sha256};

struct PendingSection {
    toc: TocEntry, // offset filled at layout time
    data: Vec<u8>,
}

/// Builds a `.tplc` package. Sections may be added in any order; the TOC is
/// emitted sorted by section type (canonical order).
pub struct PackageWriter {
    uuid: [u8; UUID_FIELD_LEN],
    flags: u32,
    sections: Vec<PendingSection>,
}

impl Default for PackageWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl PackageWriter {
    pub fn new() -> Self {
        Self {
            uuid: [0u8; UUID_FIELD_LEN],
            flags: 0,
            sections: Vec::new(),
        }
    }

    /// Set the package UUID from its ASCII text form (typically a UUIDv7
    /// string, max 63 chars + NUL).
    pub fn uuid_str(&mut self, s: &str) -> Result<&mut Self> {
        let b = s.as_bytes();
        if b.len() > UUID_FIELD_LEN {
            return Err(PkgError::Limit);
        }
        self.uuid = [0u8; UUID_FIELD_LEN];
        self.uuid[..b.len()].copy_from_slice(b);
        Ok(self)
    }

    pub fn uuid_raw(&mut self, uuid: [u8; UUID_FIELD_LEN]) -> &mut Self {
        self.uuid = uuid;
        self
    }

    /// Header flag bits (e.g. F_SIGNED).
    pub fn flags(&mut self, flags: u32) -> &mut Self {
        self.flags = flags;
        self
    }

    /// Append a section. `required` sets SECF_REQUIRED (unknown-section
    /// rejection on readers that lack the type).
    pub fn add_section(
        &mut self,
        section_type: SectionType,
        data: impl Into<Vec<u8>>,
        required: bool,
        section_version: u32,
        aux: u32,
    ) -> Result<&mut Self> {
        let data = data.into();
        if let Some(lim) = section_type.size_limit() {
            if data.len() as u64 > lim {
                return Err(PkgError::Limit);
            }
        }
        self.sections.push(PendingSection {
            toc: TocEntry {
                section_type: section_type as u32,
                flags: if required { SECF_REQUIRED } else { 0 },
                offset: 0,
                size: data.len() as u64,
                section_version,
                aux,
            },
            data,
        });
        Ok(self)
    }

    /// Append a section with a raw (possibly unregistered) type id —
    /// forward-compat for newer toolchain versions writing sections this
    /// crate doesn't know yet.
    pub fn add_section_raw(
        &mut self,
        type_id: u32,
        data: impl Into<Vec<u8>>,
        required: bool,
        section_version: u32,
        aux: u32,
    ) -> Result<&mut Self> {
        let data = data.into();
        self.sections.push(PendingSection {
            toc: TocEntry {
                section_type: type_id,
                flags: if required { SECF_REQUIRED } else { 0 },
                offset: 0,
                size: data.len() as u64,
                section_version,
                aux,
            },
            data,
        });
        Ok(self)
    }

    /// Serialize. Returns the complete `.tplc` file bytes.
    pub fn build(mut self) -> Result<Vec<u8>> {
        if self.sections.len() > MAX_TOC as usize {
            return Err(PkgError::Limit);
        }
        // Canonical TOC order: sorted by section_type.
        self.sections.sort_by_key(|s| s.toc.section_type);

        let toc_bytes = self.sections.len() * TOC_ENTRY_SIZE;
        let header_size = align_up(HEADER_FIXED as u64 + toc_bytes as u64, SECTION_ALIGN) as usize;

        // Lay out payloads.
        let mut offset = header_size as u64;
        for s in &mut self.sections {
            s.toc.offset = offset;
            offset = align_up(offset + s.data.len() as u64, SECTION_ALIGN);
        }
        let total_size = offset;
        if total_size > MAX_FILE {
            return Err(PkgError::Limit);
        }

        // content_hash: SHA-256 over the raw TOC bytes then all section
        // payloads, canonical order (padding NOT hashed). Hashing the TOC
        // too closes the undetectable type/flags/aux tamper path
        // (D-IMPL-13); payloads alone would leave TOC semantics uncovered.
        let mut ch = Sha256::new();
        {
            let mut buf = [0u8; TOC_ENTRY_SIZE];
            for s in &self.sections {
                s.toc.encode(&mut buf);
                ch.update(buf);
            }
        }
        for s in &self.sections {
            ch.update(&s.data);
        }
        let content_hash: [u8; 32] = ch.finalize().into();

        let mut out = vec![0u8; total_size as usize];

        // Header with zeroed header_hash for hashing.
        let mut hdr = Header {
            format_version: FORMAT_VERSION,
            header_size: header_size as u32,
            toc_count: self.sections.len() as u32,
            flags: self.flags,
            total_size,
            uuid: self.uuid,
            content_hash,
            header_hash: [0u8; 32],
        };
        hdr.encode_into(&mut out[..HEADER_FIXED]);

        let header_hash: [u8; 32] = Sha256::digest(&out[..128]).into();
        hdr.header_hash = header_hash;
        hdr.encode_into(&mut out[..HEADER_FIXED]);

        // TOC.
        for (i, s) in self.sections.iter().enumerate() {
            let at = HEADER_FIXED + i * TOC_ENTRY_SIZE;
            s.toc.encode((&mut out[at..at + TOC_ENTRY_SIZE])
                .try_into()
                .unwrap());
        }

        // Payloads.
        for s in &self.sections {
            let at = s.toc.offset as usize;
            out[at..at + s.data.len()].copy_from_slice(&s.data);
        }

        Ok(out)
    }
}

pub(crate) fn align_up(v: u64, a: u64) -> u64 {
    (v + a - 1) & !(a - 1)
}
