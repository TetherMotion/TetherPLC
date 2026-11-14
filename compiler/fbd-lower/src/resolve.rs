//! Block signatures (builtin FBs, operators, user POUs) and the
//! deterministic topological order computation.

use fbd_model::{Block, BlockKind, Network, NodeId, Pou, PouKind, VarRole};
use std::collections::{BTreeMap, BTreeSet};
use tplc_mir::ir::Type;

/// Ordered pin signature: inputs then outputs (signature order is the
/// `PortSel::Index` space — separate index spaces per direction).
#[derive(Debug, Clone, Default)]
pub struct Sig {
    pub ins: Vec<(String, Type)>,
    pub outs: Vec<(String, Type)>,
    /// in-pin indexes declared VAR_IN_OUT — their source must be a
    /// resolvable storage reference, not a computed expression.
    pub inouts: BTreeSet<u32>,
}

fn s(n: &str, t: Type) -> (String, Type) {
    (n.into(), t)
}
const BOOL: Type = Type::Bool;
const TIME: Type = Type::Time;
const DINT: Type = Type::Int {
    signed: true,
    bits: 32,
};

/// Ordered signature for builtin FBs (mirrors `builtin_fb_layout`
/// members, which are alphabet-sorted and lose signature order).
pub fn builtin_sig(name: &str) -> Option<Sig> {
    let (ins, outs) = match name {
        "TON" | "TOF" | "TP" => (
            vec![s("IN", BOOL), s("PT", TIME)],
            vec![s("Q", BOOL), s("ET", TIME)],
        ),
        "R_TRIG" | "F_TRIG" => (vec![s("CLK", BOOL)], vec![s("Q", BOOL)]),
        "CTU" => (
            vec![s("CU", BOOL), s("R", BOOL), s("PV", DINT)],
            vec![s("QU", BOOL), s("CV", DINT)],
        ),
        "CTD" => (
            vec![s("CD", BOOL), s("LD", BOOL), s("PV", DINT)],
            vec![s("QD", BOOL), s("CV", DINT)],
        ),
        "CTUD" => (
            vec![
                s("CU", BOOL),
                s("CD", BOOL),
                s("R", BOOL),
                s("LD", BOOL),
                s("PV", DINT),
            ],
            vec![s("QU", BOOL), s("QD", BOOL), s("CV", DINT)],
        ),
        "SR" => (vec![s("S1", BOOL), s("R", BOOL)], vec![s("Q1", BOOL)]),
        "RS" => (vec![s("S", BOOL), s("R1", BOOL)], vec![s("Q1", BOOL)]),
        _ => return None,
    };
    Some(Sig {
        ins,
        outs,
        inouts: BTreeSet::new(),
    })
}

/// Signature of a user-declared FB or FUNCTION POU from its interface
/// (declaration order; InOut counts as an input pin for binding).
pub fn pou_sig(pou: &Pou, fb_names: &BTreeSet<String>) -> Sig {
    let mut sig = Sig::default();
    for v in &pou.interface {
        let ty = crate::ty::parse_type(&v.ty, fb_names).unwrap_or(Type::Bool);
        match v.kind {
            fbd_model::IfaceKind::VarInput => sig.ins.push((v.name.clone(), ty)),
            fbd_model::IfaceKind::VarInOut => {
                sig.inouts.insert(sig.ins.len() as u32);
                sig.ins.push((v.name.clone(), ty));
            }
            fbd_model::IfaceKind::VarOutput => sig.outs.push((v.name.clone(), ty)),
            _ => {}
        }
    }
    if pou.kind == PouKind::Function {
        // FUNCTION return value is an implicit first output
        let rt = pou
            .ret_ty
            .as_ref()
            .and_then(|t| crate::ty::parse_type(t, fb_names).ok())
            .unwrap_or(Type::Bool);
        sig.outs.insert(0, (pou.name.clone(), rt));
    }
    sig
}

/// Signature of a `device` block from its channel pins — same
/// direction convention as `st` blocks: var_input/var_in_out → in
/// pins, var_output/var_in_out → out pins. Pin types come from the
/// declared `type` string; `at` bindings are checked separately.
pub fn device_sig(b: &Block, fb_names: &BTreeSet<String>) -> Sig {
    let mut sig = Sig::default();
    for v in &b.pins {
        let ty = crate::ty::parse_type(&v.ty, fb_names).unwrap_or(Type::Bool);
        match v.kind {
            fbd_model::IfaceKind::VarInput => sig.ins.push((v.name.clone(), ty)),
            fbd_model::IfaceKind::VarInOut => {
                sig.inouts.insert(sig.ins.len() as u32);
                sig.ins.push((v.name.clone(), ty.clone()));
                sig.outs.push((v.name.clone(), ty));
            }
            fbd_model::IfaceKind::VarOutput => sig.outs.push((v.name.clone(), ty)),
            _ => {}
        }
    }
    sig
}

/// Operator classification (clean-room block library, doc 06 §10).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    /// variadic n-input fold: AND OR XOR ADD MUL
    Fold(tplc_mir::ir::BinOp),
    /// binary: SUB DIV MOD GT GE LT LE EQ NE SHL SHR ROL ROR
    Bin(tplc_mir::ir::BinOp),
    Not,
    Move,
    Sel,
    Min,
    Max,
    Limit,
    Abs,
    Mux,
}

pub fn op_kind(type_ref: &str) -> Option<Op> {
    use tplc_mir::ir::BinOp as B;
    Some(match type_ref {
        "AND" => Op::Fold(B::And),
        "OR" => Op::Fold(B::Or),
        "XOR" => Op::Fold(B::Xor),
        "ADD" => Op::Fold(B::Add),
        "MUL" => Op::Fold(B::Mul),
        "SUB" => Op::Bin(B::Sub),
        "DIV" => Op::Bin(B::Div),
        "MOD" => Op::Bin(B::Mod),
        "GT" => Op::Bin(B::Gt),
        "GE" => Op::Bin(B::Ge),
        "LT" => Op::Bin(B::Lt),
        "LE" => Op::Bin(B::Le),
        "EQ" => Op::Bin(B::Eq),
        "NE" => Op::Bin(B::Ne),
        "SHL" => Op::Bin(B::Shl),
        "SHR" => Op::Bin(B::Shr),
        "ROL" => Op::Bin(B::Rol),
        "ROR" => Op::Bin(B::Ror),
        "NOT" => Op::Not,
        "MOVE" => Op::Move,
        "SEL" => Op::Sel,
        "MIN" => Op::Min,
        "MAX" => Op::Max,
        "LIMIT" => Op::Limit,
        "ABS" => Op::Abs,
        "MUX" => Op::Mux,
        _ => return None,
    })
}

/// `X_TO_Y` cast blocks → explicit `Conv`.
pub fn cast_target(type_ref: &str) -> Option<&str> {
    type_ref.split_once("_TO_").map(|(_, to)| to)
}

/// Named in-pin → operand index for shaped operators (index space for
/// `PortSel::Index` is always positional 0..n).
pub fn op_in_index(op: Op, name: &str) -> Option<u32> {
    let idx = |n: u32| Some(n);
    match op {
        Op::Sel => match name {
            "G" => idx(0),
            "IN0" => idx(1),
            "IN1" => idx(2),
            _ => None,
        },
        Op::Limit => match name {
            "MN" => idx(0),
            "IN" => idx(1),
            "MX" => idx(2),
            _ => None,
        },
        Op::Mux => match name {
            "K" => idx(0),
            _ => name.strip_prefix("IN")?.parse().ok().map(|n: u32| n + 1),
        },
        Op::Bin(_) | Op::Fold(_) | Op::Min | Op::Max => {
            name.strip_prefix("IN")?.parse().ok().map(Some)?
        }
        Op::Not | Op::Move | Op::Abs => match name {
            "IN" | "IN0" => idx(0),
            _ => None,
        },
    }
}

/// Operand index → canonical param name (reverse of `op_in_index`).
/// Shaped ops use their IEC operand names: SEL data pins are 0-based
/// `IN0`/`IN1` behind `G`; MUX likewise behind `K`.
pub fn op_in_name(op: Op, i: u32) -> String {
    match (op, i) {
        (Op::Sel, 0) => "G".into(),
        (Op::Sel, n) => format!("IN{}", n - 1),
        (Op::Limit, 0) => "MN".into(),
        (Op::Limit, 1) => "IN".into(),
        (Op::Limit, 2) => "MX".into(),
        (Op::Mux, 0) => "K".into(),
        (Op::Mux, n) => format!("IN{}", n - 1),
        (Op::Not | Op::Move | Op::Abs, _) => "IN".into(),
        _ => format!("IN{i}"),
    }
}

/// Fixed in-pin arity for non-variadic ops (`None` = variadic ≥1).
pub fn op_arity(op: Op) -> Option<u32> {
    Some(match op {
        Op::Not | Op::Move | Op::Abs => 1,
        Op::Bin(_) => 2,
        Op::Sel | Op::Limit => 3,
        Op::Fold(_) | Op::Min | Op::Max | Op::Mux => return None,
    })
}

/// Deterministic topo order of a network's effectful blocks.
///
/// Kahn's algorithm; the frontier key is
/// `(exec_order==0, exec_order, y, x, id)` — explicit `exec_order`
/// blocks first (sorted by it), then canonical position order.
/// A second pass releases nodes blocked only by edges into FB blocks
/// (feedback through state is legal — the read sees previous-cycle
/// values); any node still blocked is part of an algebraic cycle and
/// reported in `Err`.
pub fn topo_order(net: &Network) -> Result<Vec<NodeId>, Vec<NodeId>> {
    let blocks: BTreeMap<NodeId, &Block> = net.blocks.iter().map(|b| (b.id, b)).collect();
    let mut indeg: BTreeMap<NodeId, usize> = blocks.keys().map(|id| (*id, 0)).collect();
    let mut edge_into_fb: BTreeMap<NodeId, BTreeSet<NodeId>> = BTreeMap::new();
    for w in &net.wires {
        for d in &w.dsts {
            if !blocks.contains_key(&d.block) || !blocks.contains_key(&w.src.block) {
                continue;
            }
            // a device block may legitimately wire its own pins
            // (e.g. AI0 → AO0 on one terminal) — each pin aliases a
            // distinct located var, so there is no ordering cycle
            if w.src.block == d.block && blocks[&d.block].kind == BlockKind::Device {
                continue;
            }
            *indeg.entry(d.block).or_insert(0) += 1;
            edge_into_fb.entry(w.src.block).or_default().insert(d.block);
        }
    }
    // wireless `assoc` sources name variables, not blocks — they
    // create no block-level ordering dependency.
    let key = |id: NodeId| -> (u8, u32, u32, u32, NodeId) {
        let b = blocks[&id];
        (
            if b.exec_order == 0 { 1 } else { 0 },
            b.exec_order,
            b.pos.y,
            b.pos.x,
            id,
        )
    };
    let mut done: Vec<NodeId> = Vec::new();
    let mut frontier: Vec<NodeId> = indeg
        .iter()
        .filter(|(_, d)| **d == 0)
        .map(|(id, _)| *id)
        .collect();
    frontier.sort_by_key(|id| std::cmp::Reverse(key(*id)));
    while let Some(id) = frontier.pop() {
        if done.contains(&id) {
            continue;
        }
        done.push(id);
        for next in edge_into_fb.get(&id).cloned().unwrap_or_default() {
            if let Some(d) = indeg.get_mut(&next) {
                *d = d.saturating_sub(1);
                if *d == 0 {
                    frontier.push(next);
                }
            }
        }
        frontier.sort_by_key(|id| std::cmp::Reverse(key(*id)));
    }
    if done.len() == blocks.len() {
        return Ok(done);
    }
    // cycle pass: drop edges INTO fb blocks (feedback through state),
    // re-run over the remaining subgraph.
    let mut stuck: Vec<NodeId> = blocks
        .keys()
        .filter(|id| !done.contains(id))
        .copied()
        .collect();
    // Full adjacency of the stuck subgraph — reachability decides
    // whether a store's outgoing edge actually closes a cycle (edges
    // to merely-downstream nodes must keep ordering: they read the
    // NEW value).
    let mut adj_all: BTreeMap<NodeId, BTreeSet<NodeId>> = BTreeMap::new();
    for w in &net.wires {
        for d in &w.dsts {
            if stuck.contains(&w.src.block)
                && stuck.contains(&d.block)
                && !(w.src.block == d.block && blocks[&d.block].kind == BlockKind::Device)
            {
                adj_all.entry(w.src.block).or_default().insert(d.block);
            }
        }
    }
    let reaches = |from: NodeId, to: NodeId| -> bool {
        let mut seen: BTreeSet<NodeId> = BTreeSet::new();
        let mut stack = vec![from];
        while let Some(n) = stack.pop() {
            if n == to {
                return true;
            }
            if seen.insert(n) {
                if let Some(ns) = adj_all.get(&n) {
                    stack.extend(ns.iter().copied());
                }
            }
        }
        false
    };
    let mut indeg2: BTreeMap<NodeId, usize> = stuck.iter().map(|id| (*id, 0)).collect();
    let mut fwd2: BTreeMap<NodeId, BTreeSet<NodeId>> = BTreeMap::new();
    for w in &net.wires {
        for d in &w.dsts {
            if !stuck.contains(&w.src.block) || !stuck.contains(&d.block) {
                continue;
            }
            if w.src.block == d.block && blocks[&d.block].kind == BlockKind::Device {
                continue;
            }
            if blocks[&d.block].kind == BlockKind::Fb {
                continue; // soft edge — breaks at state element
            }
            // Variable stores are state too: a register's read-port
            // inside a cycle sees the pre-store value (the store
            // itself still runs after its producers — incoming edges
            // are kept). Only edges that close a cycle are dropped.
            if is_var_store(blocks[&w.src.block]) && reaches(d.block, w.src.block) {
                continue;
            }
            *indeg2.entry(d.block).or_insert(0) += 1;
            fwd2.entry(w.src.block).or_default().insert(d.block);
        }
    }
    let mut f2: Vec<NodeId> = indeg2
        .iter()
        .filter(|(_, d)| **d == 0)
        .map(|(id, _)| *id)
        .collect();
    f2.sort_by_key(|id| std::cmp::Reverse(key(*id)));
    let mut extra: Vec<NodeId> = Vec::new();
    while let Some(id) = f2.pop() {
        extra.push(id);
        for next in fwd2.get(&id).cloned().unwrap_or_default() {
            if let Some(d) = indeg2.get_mut(&next) {
                *d = d.saturating_sub(1);
                if *d == 0 {
                    f2.push(next);
                }
            }
        }
        f2.sort_by_key(|id| std::cmp::Reverse(key(*id)));
    }
    stuck.retain(|id| !extra.contains(id));
    if !stuck.is_empty() {
        return Err(stuck);
    }
    // legal feedback: append released nodes in pass-2 order (their
    // position is after all independent work — reads see the values
    // of this cycle, inputs fed by their own outputs see last cycle's).
    done.extend(extra);
    Ok(done)
}

/// Variable block that writes storage — `emit_var_block`'s store
/// arm (everything except a pure `in` source). Its read-port may
/// close a feedback cycle: consumers inside the cycle see the
/// pre-store value, exactly like reads of an FB instance's fields.
fn is_var_store(b: &Block) -> bool {
    b.kind == BlockKind::Variable && !matches!(b.role, Some(VarRole::In))
}
