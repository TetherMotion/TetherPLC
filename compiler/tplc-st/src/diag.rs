//! Frontend diagnostics — product-layer shape (doc 05 §12): code,
//! severity, primary span, message, optional help. ruSTy parse diags
//! map into this type at the adapter boundary so downstream tooling
//! never sees foreign diagnostic types.

use tplc_mir::Prov;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Note,
}

#[derive(Debug, Clone)]
pub struct Diag {
    pub code: &'static str,
    pub severity: Severity,
    pub message: String,
    pub help: Option<String>,
    pub span: Prov,
}

impl Diag {
    pub fn error(code: &'static str, message: impl Into<String>, span: Prov) -> Self {
        Diag {
            code,
            severity: Severity::Error,
            message: message.into(),
            help: None,
            span,
        }
    }
    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }
    pub fn warning(code: &'static str, message: impl Into<String>, span: Prov) -> Self {
        Diag {
            code,
            severity: Severity::Warning,
            message: message.into(),
            help: None,
            span,
        }
    }
}

/// Error codes (doc 05 §12 registry — frontend block).
pub mod codes {
    pub const PARSE: &str = "TPLC_E_FRONT_PARSE";
    pub const UNSUPPORTED: &str = "TPLC_E_FRONT_UNSUPPORTED";
    pub const UNKNOWN_TYPE: &str = "TPLC_E_FRONT_UNKNOWN_TYPE";
    pub const UNKNOWN_NAME: &str = "TPLC_E_FRONT_UNKNOWN_NAME";
    pub const BAD_AT: &str = "TPLC_E_FRONT_BAD_AT";
    pub const DUP_NAME: &str = "TPLC_E_FRONT_DUP_NAME";
}
