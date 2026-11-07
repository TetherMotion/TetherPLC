//! Package error codes — mirrors the TPLC_E_PKG_* block owned by doc 01 §13.

use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PkgError {
    Magic = -200,
    Version = -201,
    Toc = -202,
    HashHeader = -203,
    HashContent = -204,
    UnknownSection = -205,
    Limit = -206,
    SigRequired = -207,
    SigInvalid = -208,
    SigUntrusted = -209,
    HiddenState = -210,
    SymmapRange = -211,
    TargetMismatch = -212,
    AbiMismatch = -213,
    RequiresUnmet = -214,
    /// Malformed section-internal payload (bad magic, short buffer, bad version).
    SectionFormat = -215,
}

impl PkgError {
    pub fn code(self) -> i32 {
        self as i32
    }
}

impl fmt::Display for PkgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Magic => "bad package magic",
            Self::Version => "unsupported package format version",
            Self::Toc => "malformed table of contents",
            Self::HashHeader => "header hash mismatch",
            Self::HashContent => "content hash mismatch",
            Self::UnknownSection => "unknown required section",
            Self::Limit => "package size limit exceeded",
            Self::SigRequired => "signature required",
            Self::SigInvalid => "signature invalid",
            Self::SigUntrusted => "signer untrusted",
            Self::HiddenState => "target object contains non-whitelist writable state",
            Self::SymmapRange => "symmap entry out of range",
            Self::TargetMismatch => "no target object matches this host",
            Self::AbiMismatch => "package ABI version unsupported",
            Self::RequiresUnmet => "package resource requirements exceed image",
            Self::SectionFormat => "section payload malformed",
        };
        f.write_str(s)
    }
}

impl std::error::Error for PkgError {}

pub type Result<T> = std::result::Result<T, PkgError>;
