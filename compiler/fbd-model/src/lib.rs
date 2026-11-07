//! `fbd-model` — the `fbd.v1` document model (doc 06 §2/§6).
//!
//! The JSON file is the working surface (editor native format,
//! git-friendly); PLCopen XML is the exchange surface (handled by
//! `plcopen-io`). Canonical form: struct field order is declaration
//! order, maps are `BTreeMap`, so identical models serialize to
//! identical bytes.

mod canon;
mod model;
mod nodeid;

pub use canon::{from_json, to_canonical_json, ModelError};
pub use model::*;
pub use nodeid::IdAlloc;
