//! `tplc-mir` — typed middle IR shared by the ST and FBD frontends,
//! the state-blob layout pass, and an eval interpreter used by spikes,
//! tests, and the future oracle harness. Doc 07 §2-3.

pub mod desc;
pub mod diag;
pub mod interp;
pub mod ir;
pub mod layout;
pub mod migrate;
pub mod ty;

pub use desc::{emit, Descriptors};
pub use diag::{Diag, Severity};
pub use interp::{Rt, Value};
pub use ir::*;
pub use layout::{layout_program, Layout};
