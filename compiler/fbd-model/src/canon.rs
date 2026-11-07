use crate::model::{FbdProject, SCHEMA_ID};
use std::fmt;

#[derive(Debug)]
pub enum ModelError {
    Json(serde_json::Error),
    /// top-level `schema` field missing or not "fbd.v1".
    SchemaVersion(String),
    /// duplicate node id inside a POU (provenance key violation).
    DupNodeId(u64),
}

impl fmt::Display for ModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModelError::Json(e) => write!(f, "{e}"),
            ModelError::SchemaVersion(s) => {
                write!(f, "unsupported schema {s:?} (want {SCHEMA_ID:?})")
            }
            ModelError::DupNodeId(id) => write!(f, "duplicate node id {id}"),
        }
    }
}

impl std::error::Error for ModelError {}

impl From<serde_json::Error> for ModelError {
    fn from(e: serde_json::Error) -> Self {
        ModelError::Json(e)
    }
}

/// Parse a `fbd.v1` document. Forward-compat: unknown keys are ignored
/// (serde default); a missing/foreign `schema` field is an error —
/// migrations live here once v2 exists.
pub fn from_json(src: &str) -> Result<FbdProject, ModelError> {
    let p: FbdProject = serde_json::from_str(src)?;
    if p.schema != SCHEMA_ID {
        return Err(ModelError::SchemaVersion(p.schema));
    }
    check_node_ids(&p)?;
    Ok(p)
}

/// Canonical serialization: struct field order + BTreeMap keys are
/// fixed by the model types, so output is byte-identical for identical
/// models. Trailing newline per repo convention.
pub fn to_canonical_json(p: &FbdProject) -> String {
    let mut s = serde_json::to_string_pretty(p).expect("serialize");
    s.push('\n');
    s
}

/// Every node id inside a POU must be unique — ids key provenance
/// (symmap, diagnostics, breakpoints), so a collision is data loss.
fn check_node_ids(p: &FbdProject) -> Result<(), ModelError> {
    let mut seen = std::collections::HashSet::new();
    for pou in &p.pous {
        seen.clear();
        for net in &pou.networks {
            for id in net.node_ids() {
                if !seen.insert(*id) {
                    return Err(ModelError::DupNodeId(*id));
                }
            }
        }
    }
    Ok(())
}
