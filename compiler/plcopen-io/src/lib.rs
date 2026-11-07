//! `plcopen-io` — PLCopen TC6 XML exchange surface for `fbd-model`
//! (doc 06 §5 / Appendix E).
//!
//! Two export modes: `strict` emits schema-valid TC6 only (vendor
//! extensions dropped, losses reported); `roundtrip` additionally
//! carries `tether:`-namespaced elements so an import of our own
//! export is lossless. Import is lossy-tolerant: unknown vendor
//! namespaces are skipped with import notes; unsupported constructs
//! surface as `XmlNote`s rather than guesses.

mod export;
mod import;

pub use export::{export, ExportMode, LossNote};
pub use import::{import, ImportReport, XmlNote};
