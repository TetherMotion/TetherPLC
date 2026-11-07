//! Frontend diagnostics — the shared `Diag`/`Severity` types live in
//! `tplc-mir` so the ST and FBD frontends emit one diagnostic surface;
//! this module re-exports them and owns the ST code registry.

pub use tplc_mir::diag::{Diag, Severity};

/// Error codes (doc 05 §12 registry — frontend block).
pub mod codes {
    pub const PARSE: &str = "TPLC_E_ST_PARSE";
    pub const UNSUPPORTED: &str = "TPLC_E_ST_UNSUPPORTED";
    pub const UNKNOWN_TYPE: &str = "TPLC_E_ST_UNKNOWN_TYPE";
    pub const UNKNOWN_NAME: &str = "TPLC_E_ST_UNKNOWN_NAME";
    pub const TYPE: &str = "TPLC_E_ST_TYPE";
    pub const BAD_AT: &str = "TPLC_E_ST_BAD_AT";
    pub const DUP_NAME: &str = "TPLC_E_ST_DUP_NAME";
}

/// Lint warnings (doc 05 §12 — `TPLC_W_ST_*`, also stable API).
pub mod wcodes {
    pub const UNUSED_VAR: &str = "TPLC_W_ST_UNUSED_VAR";
    pub const SHADOW: &str = "TPLC_W_ST_SHADOW";
    pub const UNREACHABLE_CASE: &str = "TPLC_W_ST_UNREACHABLE_CASE";
    pub const DEAD_CODE: &str = "TPLC_W_ST_DEAD_CODE";
    pub const IMPLICIT_NARROW: &str = "TPLC_W_ST_IMPLICIT_NARROW";
    pub const EXTERNAL: &str = "TPLC_W_ST_EXTERNAL";
    pub const NO_JUMPS: &str = "TPLC_W_ST_NO_JUMPS";
}
