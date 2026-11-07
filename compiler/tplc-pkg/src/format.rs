//! Wire constants and structures of the `.tplc` container — doc 01 §2.
//! All fields little-endian; layouts written/read explicitly (no repr(C)).

pub const MAGIC: &[u8; 8] = b"TPLCPKG\0";
pub const FORMAT_VERSION: u32 = 1;

/// Fixed header size: magic8 + ver4 + hdrsz4 + toccnt4 + flags4 + total8
/// + uuid64 + content_hash32 + header_hash32.
pub const HEADER_FIXED: usize = 160;
pub const TOC_ENTRY_SIZE: usize = 32;
pub const SECTION_ALIGN: u64 = 16;
pub const UUID_FIELD_LEN: usize = 64; // ASCII uuid text, NUL-padded

pub const F_SIGNED: u32 = 1 << 0;
pub const F_COMPRESSED: u32 = 1 << 1;

pub const SECF_REQUIRED: u32 = 1 << 0;
pub const SECF_COMPRESSED: u32 = 1 << 1;

/// Default size limits (doc 01 §2.2).
pub const MAX_FILE: u64 = 64 << 20;
pub const MAX_TOC: u32 = 64;
pub const MAX_IR: u64 = 32 << 20;
pub const MAX_TARGET_OBJ: u64 = 32 << 20;
pub const MAX_SYMMAP_CAT: u64 = 8 << 20;
pub const MAX_MANIFEST: u64 = 256 << 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum SectionType {
    Manifest = 0x0001,
    IrBitcode = 0x0002,
    TargetObj = 0x0010,
    Symmap = 0x0020,
    TypeCatalog = 0x0021,
    LayoutDesc = 0x0022,
    MigrateDesc = 0x0023,
    IoMap = 0x0030,
    SourceManifest = 0x0040,
    PlcopenXml = 0x0041,
    StSources = 0x0042,
    DebugLine = 0x0050,
    Signature = 0x0060,
    Toolchain = 0x0070,
}

impl SectionType {
    pub fn from_u32(v: u32) -> Option<Self> {
        Some(match v {
            0x0001 => Self::Manifest,
            0x0002 => Self::IrBitcode,
            0x0010 => Self::TargetObj,
            0x0020 => Self::Symmap,
            0x0021 => Self::TypeCatalog,
            0x0022 => Self::LayoutDesc,
            0x0023 => Self::MigrateDesc,
            0x0030 => Self::IoMap,
            0x0040 => Self::SourceManifest,
            0x0041 => Self::PlcopenXml,
            0x0042 => Self::StSources,
            0x0050 => Self::DebugLine,
            0x0060 => Self::Signature,
            0x0070 => Self::Toolchain,
            _ => return None,
        })
    }

    /// Per-type payload size limit (doc 01 §2.2); `None` = unbounded
    /// beyond the file limit.
    pub fn size_limit(self) -> Option<u64> {
        match self {
            Self::Manifest => Some(MAX_MANIFEST),
            Self::IrBitcode => Some(MAX_IR),
            Self::TargetObj => Some(MAX_TARGET_OBJ),
            Self::Symmap | Self::TypeCatalog => Some(MAX_SYMMAP_CAT),
            _ => None,
        }
    }
}

/// One TOC entry (32 B on the wire).
#[derive(Debug, Clone, Copy)]
pub struct TocEntry {
    pub section_type: u32,
    pub flags: u32,
    pub offset: u64,
    pub size: u64,
    pub section_version: u32,
    pub aux: u32,
}

impl TocEntry {
    pub fn encode(&self, out: &mut [u8; TOC_ENTRY_SIZE]) {
        out[0..4].copy_from_slice(&self.section_type.to_le_bytes());
        out[4..8].copy_from_slice(&self.flags.to_le_bytes());
        out[8..16].copy_from_slice(&self.offset.to_le_bytes());
        out[16..24].copy_from_slice(&self.size.to_le_bytes());
        out[24..28].copy_from_slice(&self.section_version.to_le_bytes());
        out[28..32].copy_from_slice(&self.aux.to_le_bytes());
    }

    pub fn decode(buf: &[u8]) -> Self {
        Self {
            section_type: u32::from_le_bytes(buf[0..4].try_into().unwrap()),
            flags: u32::from_le_bytes(buf[4..8].try_into().unwrap()),
            offset: u64::from_le_bytes(buf[8..16].try_into().unwrap()),
            size: u64::from_le_bytes(buf[16..24].try_into().unwrap()),
            section_version: u32::from_le_bytes(buf[24..28].try_into().unwrap()),
            aux: u32::from_le_bytes(buf[28..32].try_into().unwrap()),
        }
    }
}

/// Decoded fixed header (offsets 0..160).
#[derive(Debug, Clone)]
pub struct Header {
    pub format_version: u32,
    pub header_size: u32,
    pub toc_count: u32,
    pub flags: u32,
    pub total_size: u64,
    /// 64-byte field; UUID text is stored ASCII NUL-padded.
    pub uuid: [u8; UUID_FIELD_LEN],
    pub content_hash: [u8; 32],
    pub header_hash: [u8; 32],
}

impl Header {
    pub fn uuid_str(&self) -> &str {
        let end = self
            .uuid
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(UUID_FIELD_LEN);
        std::str::from_utf8(&self.uuid[..end]).unwrap_or("")
    }

    pub fn encode_into(&self, out: &mut [u8]) {
        debug_assert!(out.len() >= HEADER_FIXED);
        out[0..8].copy_from_slice(MAGIC);
        out[8..12].copy_from_slice(&self.format_version.to_le_bytes());
        out[12..16].copy_from_slice(&self.header_size.to_le_bytes());
        out[16..20].copy_from_slice(&self.toc_count.to_le_bytes());
        out[20..24].copy_from_slice(&self.flags.to_le_bytes());
        out[24..32].copy_from_slice(&self.total_size.to_le_bytes());
        out[32..96].copy_from_slice(&self.uuid);
        out[96..128].copy_from_slice(&self.content_hash);
        out[128..160].copy_from_slice(&self.header_hash);
    }

    pub fn decode(buf: &[u8]) -> Self {
        let mut uuid = [0u8; UUID_FIELD_LEN];
        uuid.copy_from_slice(&buf[32..96]);
        let mut content_hash = [0u8; 32];
        content_hash.copy_from_slice(&buf[96..128]);
        let mut header_hash = [0u8; 32];
        header_hash.copy_from_slice(&buf[128..160]);
        Self {
            format_version: u32::from_le_bytes(buf[8..12].try_into().unwrap()),
            header_size: u32::from_le_bytes(buf[12..16].try_into().unwrap()),
            toc_count: u32::from_le_bytes(buf[16..20].try_into().unwrap()),
            flags: u32::from_le_bytes(buf[20..24].try_into().unwrap()),
            total_size: u64::from_le_bytes(buf[24..32].try_into().unwrap()),
            uuid,
            content_hash,
            header_hash,
        }
    }
}
