//! `tplc-codegen` — `tplc-mir` → LLVM IR (P1-05, doc 07 §4).
//! Feature `llvm` gates inkwell/LLVM-21 so the default workspace build
//! stays link-light.

#[cfg(feature = "llvm")]
pub mod llvm_emit;
