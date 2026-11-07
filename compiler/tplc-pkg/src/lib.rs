//! `tplc-pkg` — reader/writer for the `.tplc` deployment package and the
//! binary section formats defined by plan doc 01.
//!
//! The container guarantees: header-first parse, per-file SHA-256 integrity
//! (header hash + content hash), canonical sorted TOC, 16-byte aligned
//! sections, bounded sizes. See `docs/plc/plan/01-artifact-format.md`.

pub mod error;
pub mod format;
pub mod reader;
pub mod sections;
pub mod writer;

pub use error::{PkgError, Result};
pub use format::{SectionType, TocEntry};
pub use reader::{Package, Section};
pub use sections::{Layd, LaydField, LaydRegion, LaydRegionKind, Migd, MigdRule};
pub use writer::PackageWriter;
