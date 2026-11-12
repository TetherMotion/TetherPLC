//! `fbd-lower` — `fbd-model` → `tplc-mir` (doc 06 §4).
//!
//! Per network: block→block dependency graph, deterministic topo sort
//! (explicit `exec_order` first, then y, x, id), algebraic-cycle
//! rejection, then statement emission. Operator/function blocks fold
//! into expressions at their consumers; FB blocks become `Call`
//! stmts; variable blocks become `Ref`/`Assign`; probed wires get a
//! named internal field + store.

mod lower;
mod resolve;
mod ty;

pub use lower::{codes, expand_st_blocks, lower, lower_mixed, LowerDiag, LowerOutput};
