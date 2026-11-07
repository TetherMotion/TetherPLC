//! `tplc-st` — the IEC Structured Text frontend (P1-02).
//!
//! `compile()` turns ST sources into `tplc-mir` + io_bindings +
//! diagnostics. The parser itself is ruSTy behind the `rusty` feature
//! (DEC-2026-01: raw AST only, never pre_process); its types never
//! escape this crate.

pub mod diag;
pub mod sema;

#[cfg(feature = "rusty")]
mod rusty_lower;

pub use diag::{codes, Diag, Severity};
use tplc_mir as mir;

/// An `AT`-bound variable: qualified name + image address + declared
/// type → becomes an `io_binding` in the package descriptor.
#[derive(Debug, Clone)]
pub struct IoBind {
    /// qualified name `POU.var`
    pub name: String,
    pub at: mir::At,
    pub ty: mir::Type,
}

#[derive(Debug)]
pub struct FrontendOutput {
    pub module: mir::Module,
    pub io_bindings: Vec<IoBind>,
    pub diags: Vec<Diag>,
}

impl FrontendOutput {
    pub fn has_errors(&self) -> bool {
        self.diags.iter().any(|d| d.severity == Severity::Error)
    }
}

/// Compile ST sources through the ruSTy parser into `tplc-mir`.
/// Requires the `rusty` feature.
#[cfg(feature = "rusty")]
pub fn compile(sources: &[(&str, &str)]) -> FrontendOutput {
    rusty_lower::compile(sources)
}
