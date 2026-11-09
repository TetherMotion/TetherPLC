//! `fbd-model` → `tplc-mir` lowering (doc 06 §4).
//!
//! Per POU: interface vars, FB instance decls from `fb` blocks,
//! synthesized located vars for `at` variable blocks, then each
//! network in declared order lowers to a stmt sequence in topo order.
//! Operator/function blocks fold into expressions at their consumers
//! (except conditional ops, which materialize a `__op_<id>` temp);
//! `fb` blocks become `Call` stmts; variable-out blocks become
//! `Assign` stmts; probed wires get a `__probe_<id>` field + store.

use fbd_model::*;
use std::collections::{BTreeMap, BTreeSet};
use tplc_mir::diag::Diag;
use tplc_mir::ir;
use tplc_mir::ir::{
    AtRegion, BinOp, Expr, ExprKind, IoBind, Lit, Prov, Ref, Stmt, StmtKind, Type, UnOp, VarDecl,
    VarKind,
};
use tplc_mir::ty::{compatible, convertible, literal_fits, unify_num};

use crate::resolve::{
    builtin_sig, cast_target, op_arity, op_in_index, op_in_name, op_kind, pou_sig, topo_order, Op,
    Sig,
};
use crate::ty::{parse_at, parse_lit, parse_type, ty_for_width};

/// FBD validation/diagnostic codes (doc 06 §7, §12).
pub mod codes {
    pub const TYPE_MISMATCH: &str = "TPLC_E_FBD001";
    pub const UNCONNECTED: &str = "TPLC_E_FBD002";
    pub const FAN_IN: &str = "TPLC_E_FBD003";
    pub const ALG_CYCLE: &str = "TPLC_E_FBD004";
    pub const INOUT_STORAGE: &str = "TPLC_E_FBD005";
    pub const UNKNOWN: &str = "TPLC_E_FBD006";
    pub const ORDER_COLLISION: &str = "TPLC_E_FBD007";
    pub const UNDECLARED: &str = "TPLC_E_FBD008";
    pub const ANY_UNRESOLVED: &str = "TPLC_E_FBD009";
    pub const ORPHAN: &str = "TPLC_W_FBD010";
    pub const BAD_JUMP: &str = "TPLC_E_FBD011";
    pub const INST_REUSE: &str = "TPLC_E_FBD012";
    pub const RECURSION: &str = "TPLC_E_FBD013";
    pub const INOUT_EXPR: &str = "TPLC_E_FBD014";
    pub const BAD_AT: &str = "TPLC_E_FBD_BAD_AT";
    pub const PARSE: &str = "TPLC_E_FBD_PARSE";
    pub const UNSUPPORTED: &str = "TPLC_E_FBD_UNSUPPORTED";
}

pub use tplc_mir::diag::Diag as LowerDiag;

pub struct LowerOutput {
    pub module: ir::Module,
    pub io_bindings: Vec<IoBind>,
    pub diags: Vec<Diag>,
}

/// Natural type of a literal (used when the pin type is unknown —
/// generic operator pins carry no signature type).
fn lit_ty(l: &Lit) -> Type {
    match l {
        Lit::Bool(_) => Type::Bool,
        Lit::Int(_) => Type::Int {
            signed: true,
            bits: 32,
        },
        Lit::UInt(_) => Type::Int {
            signed: false,
            bits: 32,
        },
        Lit::Real(_) => Type::Real { bits: 64 },
        Lit::TimeNs(_) => Type::Time,
        Lit::Str(_) => Type::Str { cap: 81 },
        Lit::WStr(_) => Type::WStr { cap: 160 },
        Lit::EnumVal { ty, .. } => Type::Enum {
            name: ty.clone(),
            values: vec![],
        },
    }
}

pub fn lower(project: &FbdProject) -> LowerOutput {
    let mut out = LowerOutput {
        module: ir::Module {
            pous: Vec::new(),
            gvls: Vec::new(),
            tasks: Vec::new(),
            checked_arith: false,
        },
        io_bindings: Vec::new(),
        diags: Vec::new(),
    };

    // FB type names resolvable as `Type::Fb` (user-declared POUs).
    let fb_names: BTreeSet<String> = project
        .pous
        .iter()
        .filter(|p| p.kind == PouKind::FunctionBlock)
        .map(|p| p.name.to_ascii_uppercase())
        .collect();
    let pou_by_name: BTreeMap<String, &Pou> = project
        .pous
        .iter()
        .map(|p| (p.name.to_ascii_uppercase(), p))
        .collect();

    // project-level globals → one GVL named GVL
    if !project.global_vars.is_empty() {
        let mut vars = Vec::new();
        for v in &project.global_vars {
            if let Some(vd) = decl_of(v, &fb_names, &mut out.diags) {
                vars.push(vd);
            }
        }
        out.module.gvls.push(ir::Gvl {
            name: "GVL".into(),
            vars,
            retain: false,
        });
    }

    for t in &project.tasks {
        out.module.tasks.push(ir::TaskDecl {
            name: t.name.clone(),
            period_ns: t.period_ns,
            wcet_hint_ns: 0,
            priority: t.priority.min(255) as u8,
            programs: t
                .programs
                .iter()
                .filter_map(|id| {
                    project
                        .pous
                        .iter()
                        .find(|p| &p.id == id || &p.name == id)
                        .map(|p| p.name.clone())
                })
                .collect(),
        });
    }

    for pou in &project.pous {
        let mut cx = Cx::new(pou, &fb_names, &pou_by_name, project);
        let p = cx.lower_pou();
        out.io_bindings.extend(cx.io_bindings);
        out.diags.extend(cx.diags);
        out.module.pous.push(p);
    }
    out
}

/// `IfaceVar` → `VarDecl` (type parse, init literal, at binding).
fn decl_of(v: &IfaceVar, fb_names: &BTreeSet<String>, diags: &mut Vec<Diag>) -> Option<VarDecl> {
    let ty = match parse_type(&v.ty, fb_names) {
        Ok(t) => t,
        Err(e) => {
            diags.push(Diag::error(
                codes::UNKNOWN,
                format!("{}: {e}", v.name),
                Prov::None,
            ));
            return None;
        }
    };
    let init = v.init.as_ref().map(|s| match parse_lit(s) {
        Ok(l) => Expr {
            kind: ExprKind::Lit(l),
            ty: ty.clone(),
            prov: Prov::None,
        },
        Err(e) => {
            diags.push(Diag::error(
                codes::PARSE,
                format!("init of {}: {e}", v.name),
                Prov::None,
            ));
            Expr {
                kind: ExprKind::Lit(Lit::Int(0)),
                ty: ty.clone(),
                prov: Prov::None,
            }
        }
    });
    let at = v.at.as_deref().and_then(parse_at);
    Some(VarDecl {
        name: v.name.clone(),
        kind: match v.kind {
            IfaceKind::Var => VarKind::Local,
            IfaceKind::VarInput => VarKind::Input,
            IfaceKind::VarOutput => VarKind::Output,
            IfaceKind::VarInOut => VarKind::InOut,
            IfaceKind::VarExternal => VarKind::External,
            IfaceKind::VarConstant => VarKind::Constant,
        },
        ty,
        init,
        retain: v.retain,
        at,
        probe: false,
        prov: Prov::None,
    })
}

/// Per-POU lowering context.
struct Cx<'a> {
    pou: &'a Pou,
    fb_names: &'a BTreeSet<String>,
    pous: &'a BTreeMap<String, &'a Pou>,
    project: &'a FbdProject,
    /// ordered var decls (iface + synthesized + instances + temps)
    vars: Vec<VarDecl>,
    var_by_name: BTreeMap<String, usize>,
    /// variable block id → vars index
    var_by_block: BTreeMap<NodeId, usize>,
    /// fb block id → instance var name
    inst_by_block: BTreeMap<NodeId, String>,
    /// canonical `at` string → vars index (shared located shadows)
    at_vars: BTreeMap<String, usize>,
    /// wire id → probe var name
    probes: BTreeMap<NodeId, String>,
    io_bindings: Vec<IoBind>,
    diags: Vec<Diag>,
    /// block id → resolved sig (fb/function blocks)
    sigs: BTreeMap<NodeId, Sig>,
    sheet: u32,
    /// blocks currently being folded (algebraic-cycle guard)
    expanding: BTreeSet<NodeId>,
}

impl<'a> Cx<'a> {
    fn new(
        pou: &'a Pou,
        fb_names: &'a BTreeSet<String>,
        pous: &'a BTreeMap<String, &'a Pou>,
        project: &'a FbdProject,
    ) -> Self {
        Cx {
            pou,
            fb_names,
            pous,
            project,
            vars: Vec::new(),
            var_by_name: BTreeMap::new(),
            var_by_block: BTreeMap::new(),
            inst_by_block: BTreeMap::new(),
            at_vars: BTreeMap::new(),
            probes: BTreeMap::new(),
            io_bindings: Vec::new(),
            diags: Vec::new(),
            sigs: BTreeMap::new(),
            sheet: 0,
            expanding: BTreeSet::new(),
        }
    }

    fn prov(&self, node: u64, pin: u16) -> Prov {
        Prov::Fbd {
            sheet: self.sheet,
            node: node as u32,
            pin,
        }
    }

    fn err(&mut self, code: &'static str, msg: impl Into<String>, node: u64) {
        let p = self.prov(node, 0);
        self.diags.push(Diag::error(code, msg, p));
    }

    fn warn(&mut self, code: &'static str, msg: impl Into<String>, node: u64) {
        let p = self.prov(node, 0);
        self.diags.push(Diag::warning(code, msg, p));
    }

    fn declare(&mut self, v: VarDecl) -> usize {
        let idx = self.vars.len();
        self.var_by_name.insert(v.name.to_ascii_uppercase(), idx);
        self.vars.push(v);
        idx
    }

    fn lower_pou(&mut self) -> ir::Pou {
        let kind = match self.pou.kind {
            PouKind::Program => ir::PouKind::Program,
            PouKind::FunctionBlock => ir::PouKind::FunctionBlock,
            PouKind::Function => ir::PouKind::Function,
        };

        // interface vars
        for v in &self.pou.interface {
            let mut diags = std::mem::take(&mut self.diags);
            let vd = decl_of(v, self.fb_names, &mut diags);
            self.diags = diags;
            if let Some(mut vd) = vd {
                if self.var_by_name.contains_key(&vd.name.to_ascii_uppercase()) {
                    self.err(codes::INST_REUSE, format!("duplicate var {}", vd.name), 0);
                    continue;
                }
                if let Some(at) = &vd.at {
                    self.io_bindings.push(IoBind {
                        name: format!("{}.{}", self.pou.name, vd.name),
                        at: at.clone(),
                        ty: vd.ty.clone(),
                    });
                }
                vd.prov = self.prov(0, 0);
                self.declare(vd);
            }
        }

        // pass 1: instance decls + sigs + located shadows + probes
        for ni in 0..self.pou.networks.len() {
            self.sheet = ni as u32;
            self.precollect(&self.pou.networks[ni]);
        }

        // pass 2: networks → stmts
        let mut stmts = Vec::new();
        for ni in 0..self.pou.networks.len() {
            self.sheet = ni as u32;
            self.lower_network(&self.pou.networks[ni], &mut stmts);
        }

        ir::Pou {
            kind,
            name: self.pou.name.clone(),
            iface: std::mem::take(&mut self.vars),
            body: ir::Body::Stmts(stmts),
            ret_ty: None,
            prov: self.prov(0, 0),
        }
    }

    /// First network pass: fb instance decls, sig resolution, located
    /// variable shadows, probe fields, exec_order collisions.
    fn precollect(&mut self, net: &Network) {
        if net.kind == NetworkKind::Ladder {
            self.err(codes::UNSUPPORTED, "ladder networks are schema-reserved", 0);
            return;
        }
        let mut orders: BTreeMap<u32, NodeId> = BTreeMap::new();
        for b in &net.blocks {
            if b.exec_order != 0 {
                if let Some(other) = orders.insert(b.exec_order, b.id) {
                    self.err(
                        codes::ORDER_COLLISION,
                        format!("exec_order {} on nodes {other} and {}", b.exec_order, b.id),
                        b.id,
                    );
                }
            }
            match b.kind {
                BlockKind::Fb => self.fb_decl(b),
                BlockKind::Function => self.fn_sig(b),
                BlockKind::Variable => self.var_decl(b),
                _ => {}
            }
        }
        for w in &net.wires {
            if w.probe {
                let name = format!("__probe_{}", w.id);
                self.probes.insert(w.id, name.clone());
                self.declare(VarDecl {
                    name,
                    kind: VarKind::Local,
                    ty: Type::Bool, // refined at store emission
                    init: None,
                    retain: false,
                    at: None,
                    probe: true,
                    prov: self.prov(w.id, 0),
                });
            }
        }
    }

    /// FB block → instance var decl + sig (FBD006/012/013 checks).
    fn fb_decl(&mut self, b: &Block) {
        let tref = b.type_ref.to_ascii_uppercase();
        if tref == self.pou.name.to_ascii_uppercase() {
            self.err(
                codes::RECURSION,
                format!("{} instantiates itself", self.pou.name),
                b.id,
            );
            return;
        }
        let sig =
            builtin_sig(&tref).or_else(|| self.pous.get(&tref).map(|p| pou_sig(p, self.fb_names)));
        let Some(sig) = sig else {
            self.err(
                codes::UNKNOWN,
                format!("unknown FB type {}", b.type_ref),
                b.id,
            );
            return;
        };
        let inst = match &b.instance {
            Some(n) => n.clone(),
            None => format!("__fb_{}", b.id),
        };
        if self.var_by_name.contains_key(&inst.to_ascii_uppercase()) {
            self.err(
                codes::INST_REUSE,
                format!("instance name {inst} reused"),
                b.id,
            );
            return;
        }
        self.sigs.insert(b.id, sig);
        self.inst_by_block.insert(b.id, inst.clone());
        self.declare(VarDecl {
            name: inst,
            kind: VarKind::Local,
            ty: Type::Fb { name: tref },
            init: None,
            retain: false,
            at: None,
            probe: false,
            prov: self.prov(b.id, 0),
        });
    }

    /// FUNCTION block → sig only (no instance state).
    fn fn_sig(&mut self, b: &Block) {
        let tref = b.type_ref.to_ascii_uppercase();
        match self.pous.get(&tref) {
            Some(p) if p.kind == PouKind::Function => {
                self.sigs.insert(b.id, pou_sig(p, self.fb_names));
            }
            _ => self.err(
                codes::UNKNOWN,
                format!("unknown FUNCTION {}", b.type_ref),
                b.id,
            ),
        }
    }

    /// Variable block → located shadow or iface binding (FBD008).
    fn var_decl(&mut self, b: &Block) {
        let at_p = b.at.as_deref().and_then(|s| {
            let a = parse_at(s);
            if a.is_none() {
                self.err(codes::BAD_AT, format!("malformed AT {s}"), b.id);
            }
            a
        });
        match (&b.name, at_p) {
            (Some(n), atp) => {
                let un = n.to_ascii_uppercase();
                if let Some(&idx) = self.var_by_name.get(&un) {
                    if let Some(a) = &atp {
                        let v = &mut self.vars[idx];
                        match &v.at {
                            Some(prev) if prev != a => self.err(
                                codes::BAD_AT,
                                format!("{n} bound to conflicting AT"),
                                b.id,
                            ),
                            None => {
                                v.at = Some(a.clone());
                                self.io_bindings.push(IoBind {
                                    name: format!("{}.{}", self.pou.name, v.name),
                                    at: a.clone(),
                                    ty: v.ty.clone(),
                                });
                            }
                            _ => {}
                        }
                    }
                    self.var_by_block.insert(b.id, idx);
                    return;
                }
                if self.gvl_member(n).is_some() && atp.is_none() {
                    return; // global — resolved in the expr path
                }
                // undeclared name is only a declaration when `at` present
                let Some(a) = atp else {
                    self.err(codes::UNDECLARED, format!("undeclared name {n}"), b.id);
                    return;
                };
                let idx = self.declare(VarDecl {
                    name: n.clone(),
                    kind: VarKind::Local,
                    ty: ty_for_width(a.width),
                    init: None,
                    retain: false,
                    at: Some(a.clone()),
                    probe: false,
                    prov: self.prov(b.id, 0),
                });
                self.io_bindings.push(IoBind {
                    name: format!("{}.{}", self.pou.name, self.vars[idx].name),
                    at: a,
                    ty: self.vars[idx].ty.clone(),
                });
                self.var_by_block.insert(b.id, idx);
            }
            (None, Some(a)) => {
                let key = format!("{:?}{}x{}", a.region, a.byte, a.bit.unwrap_or(255));
                let idx = match self.at_vars.get(&key) {
                    Some(&i) => i,
                    None => {
                        let name = format!(
                            "__io_{}_{}{}",
                            region_ch(a.region),
                            a.byte,
                            a.bit.map(|b| format!("_{b}")).unwrap_or_default()
                        );
                        let i = self.declare(VarDecl {
                            name: name.clone(),
                            kind: VarKind::Local,
                            ty: ty_for_width(a.width),
                            init: None,
                            retain: false,
                            at: Some(a.clone()),
                            probe: false,
                            prov: self.prov(b.id, 0),
                        });
                        self.io_bindings.push(IoBind {
                            name: format!("{}.{}", self.pou.name, name),
                            at: a,
                            ty: self.vars[i].ty.clone(),
                        });
                        self.at_vars.insert(key, i);
                        i
                    }
                };
                self.var_by_block.insert(b.id, idx);
            }
            (None, None) => self.err(
                codes::UNDECLARED,
                "variable block with neither name nor at",
                b.id,
            ),
        }
    }

    /// `(GVL, member)` if `n` names a global (`GVL.x` or bare member).
    fn gvl_member(&self, n: &str) -> Option<(String, String)> {
        for g in &self.project.global_vars {
            if g.name.eq_ignore_ascii_case(n) {
                return Some(("GVL".into(), g.name.clone()));
            }
        }
        if let Some((g, m)) = n.split_once('.') {
            if g.eq_ignore_ascii_case("GVL")
                && self
                    .project
                    .global_vars
                    .iter()
                    .any(|v| v.name.eq_ignore_ascii_case(m))
            {
                return Some(("GVL".into(), m.to_string()));
            }
        }
        None
    }

    // ---- network lowering ----

    fn lower_network(&mut self, net: &Network, stmts: &mut Vec<Stmt>) {
        if net.kind == NetworkKind::Ladder {
            return;
        }
        if net.blocks.is_empty() {
            return;
        }
        let order = match topo_order(net) {
            Ok(o) => o,
            Err(stuck) => {
                self.err(
                    codes::ALG_CYCLE,
                    format!("algebraic loop through nodes {stuck:?}"),
                    stuck[0],
                );
                return;
            }
        };
        let blocks: BTreeMap<NodeId, &Block> = net.blocks.iter().map(|b| (b.id, b)).collect();
        let mut effectful = 0usize;

        // jump checks (FBD011); jumps have no mir target today
        let labels: BTreeSet<&str> = net.labels.iter().map(|l| l.name.as_str()).collect();
        for j in &net.jumps {
            if labels.contains(j.target.as_str()) {
                self.err(
                    codes::UNSUPPORTED,
                    format!("jump to {} — goto lowering not implemented", j.target),
                    j.id,
                );
            } else {
                self.err(
                    codes::BAD_JUMP,
                    format!("jump to nonexistent label {}", j.target),
                    j.id,
                );
            }
        }

        for id in &order {
            let b = blocks[id];
            let pre = stmts.len();
            self.emit_block(net, b, &blocks, stmts);
            if stmts.len() > pre {
                effectful += 1;
            }
            // probe stores for wires sourced at this block (the value
            // just produced — matches "store after production", §4.3)
            for w in &net.wires {
                if w.src.block == *id && self.probes.contains_key(&w.id) {
                    if let Some(e) = self.src_expr(&w.src, net, &blocks) {
                        let pname = self.probes[&w.id].clone();
                        self.fix_probe_ty(&pname, &e.ty.clone());
                        stmts.push(Stmt {
                            kind: StmtKind::Assign {
                                lhs: Ref::Var(pname),
                                rhs: e,
                            },
                            prov: self.prov(w.id, 0),
                        });
                    }
                }
            }
        }

        // unconditional returns end the POU after this network's work
        for r in &net.returns {
            stmts.push(Stmt {
                kind: StmtKind::Return(None),
                prov: self.prov(r.id, 0),
            });
            effectful += 1;
        }
        if effectful == 0 {
            self.warn(codes::ORPHAN, "network produces no effects", 0);
        }
    }

    fn fix_probe_ty(&mut self, name: &str, ty: &Type) {
        if let Some(&idx) = self.var_by_name.get(&name.to_ascii_uppercase()) {
            self.vars[idx].ty = ty.clone();
        }
    }

    /// Emit one block's statements in topo position.
    fn emit_block(
        &mut self,
        net: &Network,
        b: &Block,
        blocks: &BTreeMap<NodeId, &Block>,
        stmts: &mut Vec<Stmt>,
    ) {
        match b.kind {
            BlockKind::Variable => self.emit_var_block(net, b, blocks, stmts),
            BlockKind::Fb => self.emit_fb(net, b, blocks, stmts),
            BlockKind::Operator => self.emit_op(net, b, blocks, stmts),
            BlockKind::Function | BlockKind::JumpTarget => {}
        }
    }

    /// variable out/inout → Assign to the resolved storage.
    fn emit_var_block(
        &mut self,
        net: &Network,
        b: &Block,
        blocks: &BTreeMap<NodeId, &Block>,
        stmts: &mut Vec<Stmt>,
    ) {
        if matches!(b.role, Some(VarRole::In)) {
            return;
        }
        let Some(lhs) = self.var_ref(b) else {
            return;
        };
        let Some(e) = self.dst_input(net, b, 0, blocks) else {
            self.warn(codes::ORPHAN, "out variable has no input", b.id);
            return;
        };
        let lt = self.ref_ty(&lhs);
        self.check_coerce(&e, &lt, b.id);
        stmts.push(Stmt {
            kind: StmtKind::Assign { lhs, rhs: e },
            prov: self.prov(b.id, 0),
        });
    }

    /// fb block → Call stmt (EN-gated when EN is bound).
    fn emit_fb(
        &mut self,
        net: &Network,
        b: &Block,
        blocks: &BTreeMap<NodeId, &Block>,
        stmts: &mut Vec<Stmt>,
    ) {
        let Some(inst) = self.inst_by_block.get(&b.id).cloned() else {
            return;
        };
        let sig = self.sigs.get(&b.id).cloned().unwrap_or_default();
        let mut args: Vec<(String, Expr)> = Vec::new();
        for (i, (pname, pty)) in sig.ins.iter().enumerate() {
            match self.dst_input(net, b, i as u32, blocks) {
                Some(e) => {
                    if sig.inouts.contains(&(i as u32)) {
                        match &e.kind {
                            // storage ref — legal inout binding
                            ExprKind::Ref(_) => {}
                            ExprKind::Lit(_) => self.err(
                                codes::INOUT_STORAGE,
                                format!("inout pin {pname} bound to literal"),
                                b.id,
                            ),
                            _ => self.err(
                                codes::INOUT_EXPR,
                                format!("inout pin {pname} fed by expression"),
                                b.id,
                            ),
                        }
                    }
                    self.check_coerce(&e, pty, b.id);
                    args.push((pname.clone(), e));
                }
                None => {
                    if let Some(p) = b.params.get(pname) {
                        match parse_lit(p) {
                            Ok(l) => args.push((
                                pname.clone(),
                                Expr {
                                    kind: ExprKind::Lit(l),
                                    ty: pty.clone(),
                                    prov: self.prov(b.id, i as u16),
                                },
                            )),
                            Err(e) => self.err(codes::PARSE, format!("param {pname}: {e}"), b.id),
                        }
                    }
                    // unbound fb inputs hold their blob value (default)
                }
            }
        }
        // EN: named pin or the index just past the last in-pin
        let en = self
            .dst_input(net, b, sig.ins.len() as u32, blocks)
            .or_else(|| self.dst_input(net, b, u32::MAX, blocks));
        let mut body = vec![Stmt {
            kind: StmtKind::Call {
                inst: Ref::Var(inst.clone()),
                args,
            },
            prov: self.prov(b.id, 0),
        }];
        // ENO var holds EN result for wired consumers
        if self.eno_used(net, b, &sig) {
            let eno = self.eno_var(b.id);
            let p = self.prov(b.id, 0);
            let set_eno = |v: bool| Stmt {
                kind: StmtKind::Assign {
                    lhs: Ref::Var(eno.clone()),
                    rhs: Expr {
                        kind: ExprKind::Lit(Lit::Bool(v)),
                        ty: Type::Bool,
                        prov: p.clone(),
                    },
                },
                prov: p.clone(),
            };
            match en {
                Some(cond) => stmts.push(Stmt {
                    kind: StmtKind::If {
                        branches: vec![(cond, {
                            let mut v = body;
                            v.push(set_eno(true));
                            v
                        })],
                        else_: vec![set_eno(false)],
                    },
                    prov: self.prov(b.id, 0),
                }),
                None => {
                    stmts.append(&mut body);
                    stmts.push(set_eno(true));
                }
            }
            return;
        }
        match en {
            Some(cond) => stmts.push(Stmt {
                kind: StmtKind::If {
                    branches: vec![(cond, body)],
                    else_: vec![],
                },
                prov: self.prov(b.id, 0),
            }),
            None => stmts.append(&mut body),
        }
    }

    /// Does any wire/assoc read this fb's ENO (index ≥ outs.len()
    /// or a pin named ENO)?
    fn eno_used(&self, net: &Network, b: &Block, sig: &Sig) -> bool {
        net.wires.iter().any(|w| {
            w.src.block == b.id
                && match &w.src.port {
                    PortSel::Index(i) => *i as usize >= sig.outs.len(),
                    PortSel::Name(n) => n.eq_ignore_ascii_case("ENO"),
                }
        })
    }

    /// If-expansion operators emit their temp computation here; pure
    /// ops fold at consumers and emit nothing.
    fn emit_op(
        &mut self,
        net: &Network,
        b: &Block,
        blocks: &BTreeMap<NodeId, &Block>,
        stmts: &mut Vec<Stmt>,
    ) {
        let Some(op) = op_kind(&b.type_ref) else {
            if cast_target(&b.type_ref).is_none() {
                self.err(
                    codes::UNKNOWN,
                    format!("unknown operator {}", b.type_ref),
                    b.id,
                );
            }
            return;
        };
        if !matches!(
            op,
            Op::Sel | Op::Min | Op::Max | Op::Limit | Op::Abs | Op::Mux
        ) {
            return;
        }
        let ins = self.op_inputs(net, b, blocks);
        let Some((exprs, rty)) = self.typed_inputs(net, b, blocks, op, &ins) else {
            return;
        };
        let tname = format!("__op_{}", b.id);
        self.declare(VarDecl {
            name: tname.clone(),
            kind: VarKind::Local,
            ty: rty,
            init: None,
            retain: false,
            at: None,
            probe: false,
            prov: self.prov(b.id, 0),
        });
        let p = self.prov(b.id, 0);
        let lhs = Ref::Var(tname);
        let assign = |e: Expr| Stmt {
            kind: StmtKind::Assign {
                lhs: lhs.clone(),
                rhs: e,
            },
            prov: p.clone(),
        };
        let lit0 = || Expr {
            kind: ExprKind::Lit(Lit::Int(0)),
            ty: Type::Int {
                signed: true,
                bits: 32,
            },
            prov: p.clone(),
        };
        let e = |i: usize| exprs.get(i).cloned().unwrap_or_else(lit0);
        let cmp = |op: BinOp, a: Expr, b2: Expr| Expr {
            kind: ExprKind::Bin(op, Box::new(a), Box::new(b2)),
            ty: Type::Bool,
            prov: p.clone(),
        };
        match op {
            Op::Sel => stmts.push(Stmt {
                kind: StmtKind::If {
                    branches: vec![(e(0), vec![assign(e(2))])],
                    else_: vec![assign(e(1))],
                },
                prov: p,
            }),
            Op::Min | Op::Max => {
                let cmpop = if op == Op::Min { BinOp::Lt } else { BinOp::Gt };
                stmts.push(assign(e(0)));
                for (i, _) in exprs.iter().enumerate().skip(1) {
                    stmts.push(Stmt {
                        kind: StmtKind::If {
                            branches: vec![(
                                cmp(
                                    cmpop,
                                    e(i),
                                    Expr {
                                        kind: ExprKind::Ref(lhs.clone()),
                                        ty: exprs[i].ty.clone(),
                                        prov: p.clone(),
                                    },
                                ),
                                vec![assign(e(i))],
                            )],
                            else_: vec![],
                        },
                        prov: p.clone(),
                    });
                }
            }
            Op::Limit => stmts.push(Stmt {
                kind: StmtKind::If {
                    branches: vec![
                        (cmp(BinOp::Lt, e(1), e(0)), vec![assign(e(0))]),
                        (cmp(BinOp::Gt, e(1), e(2)), vec![assign(e(2))]),
                    ],
                    else_: vec![assign(e(1))],
                },
                prov: p,
            }),
            Op::Abs => stmts.push(Stmt {
                kind: StmtKind::If {
                    branches: vec![(
                        cmp(BinOp::Lt, e(0), lit0()),
                        vec![assign(Expr {
                            kind: ExprKind::Un(UnOp::Neg, Box::new(e(0))),
                            ty: exprs.first().map(|x| x.ty.clone()).unwrap_or(Type::Bool),
                            prov: p.clone(),
                        })],
                    )],
                    else_: vec![assign(e(0))],
                },
                prov: p,
            }),
            Op::Mux => {
                let mut branches = Vec::new();
                for i in 1..exprs.len() {
                    branches.push((
                        cmp(
                            BinOp::Eq,
                            e(0),
                            Expr {
                                kind: ExprKind::Lit(Lit::Int((i - 1) as i64)),
                                ty: Type::Int {
                                    signed: true,
                                    bits: 32,
                                },
                                prov: p.clone(),
                            },
                        ),
                        vec![assign(e(i))],
                    ));
                }
                stmts.push(Stmt {
                    kind: StmtKind::If {
                        branches,
                        // K out of range → first input (documented v1 rule)
                        else_: vec![assign(e(1))],
                    },
                    prov: p,
                });
            }
            _ => {}
        }
    }

    /// Ordered input indexes present on an operator block.
    fn op_inputs(
        &mut self,
        net: &Network,
        b: &Block,
        _blocks: &BTreeMap<NodeId, &Block>,
    ) -> BTreeSet<u32> {
        let mut set = BTreeSet::new();
        for w in &net.wires {
            for d in &w.dsts {
                if d.block == b.id {
                    set.insert(self.in_index(b, &d.port));
                }
            }
        }
        for a in &net.assocs {
            if a.dst.block == b.id {
                set.insert(self.in_index(b, &a.dst.port));
            }
        }
        if let Some(op) = op_kind(&b.type_ref) {
            for pname in b.params.keys() {
                if let Some(i) = op_in_index(op, &pname.to_ascii_uppercase()) {
                    set.insert(i);
                }
            }
        }
        set
    }

    /// `dst_input`: the expr feeding `b`'s input index `want` — wire,
    /// assoc, or param literal. Reports FBD002/003.
    fn dst_input(
        &mut self,
        net: &Network,
        b: &Block,
        want: u32,
        blocks: &BTreeMap<NodeId, &Block>,
    ) -> Option<Expr> {
        let mut srcs: Vec<Expr> = Vec::new();
        for w in &net.wires {
            for d in &w.dsts {
                if d.block != b.id || self.in_index(b, &d.port) != want {
                    continue;
                }
                let Some(mut e) = self.src_expr(&w.src, net, blocks) else {
                    continue;
                };
                // probed wires read the probe field, not the fold
                if let Some(pn) = self.probes.get(&w.id) {
                    e = Expr {
                        kind: ExprKind::Ref(Ref::Var(pn.clone())),
                        ty: e.ty.clone(),
                        prov: self.prov(w.id, 0),
                    };
                }
                if b.negated.contains(&want) {
                    e = Expr {
                        kind: ExprKind::Un(UnOp::Not, Box::new(e)),
                        ty: Type::Bool,
                        prov: self.prov(d.block, want as u16),
                    };
                }
                srcs.push(e);
            }
        }
        for a in &net.assocs {
            if a.dst.block != b.id || self.in_index(b, &a.dst.port) != want {
                continue;
            }
            let un = a.src_name.to_ascii_uppercase();
            let r = if let Some(&idx) = self.var_by_name.get(&un) {
                Ref::Var(self.vars[idx].name.clone())
            } else if let Some((g, m)) = self.gvl_member(&a.src_name) {
                Ref::Field(Box::new(Ref::Var(g)), m)
            } else {
                self.err(
                    codes::UNDECLARED,
                    format!("assoc source {}", a.src_name),
                    b.id,
                );
                continue;
            };
            let ty = self.ref_ty(&r);
            srcs.push(Expr {
                kind: ExprKind::Ref(r),
                ty,
                prov: self.prov(a.dst.block, want as u16),
            });
        }
        if srcs.len() > 1 {
            self.err(
                codes::FAN_IN,
                format!("input {want} has {} sources", srcs.len()),
                b.id,
            );
        }
        if let Some(e) = srcs.into_iter().next() {
            return Some(e);
        }
        let pname = self.in_name(b, want);
        if let Some(p) = pname.as_ref().and_then(|n| b.params.get(n)) {
            let want_ty = self.in_ty(b, want);
            return match parse_lit(p) {
                Ok(l) => Some(Expr {
                    kind: ExprKind::Lit(l.clone()),
                    ty: want_ty.unwrap_or_else(|| lit_ty(&l)),
                    prov: self.prov(b.id, want as u16),
                }),
                Err(e) => {
                    self.err(codes::PARSE, format!("param: {e}"), b.id);
                    None
                }
            };
        }
        None
    }

    /// Map a dst `PortSel` on `b` to an input index.
    fn in_index(&self, b: &Block, sel: &PortSel) -> u32 {
        match sel {
            PortSel::Index(i) => *i,
            PortSel::Name(n) => {
                let un = n.to_ascii_uppercase();
                if un == "EN" {
                    return u32::MAX;
                }
                match b.kind {
                    BlockKind::Fb | BlockKind::Function => self
                        .sigs
                        .get(&b.id)
                        .and_then(|s| {
                            s.ins
                                .iter()
                                .position(|(pn, _)| pn.eq_ignore_ascii_case(&un))
                                .map(|i| i as u32)
                        })
                        .unwrap_or(u32::MAX),
                    BlockKind::Operator => op_kind(&b.type_ref)
                        .and_then(|op| op_in_index(op, &un))
                        .unwrap_or(u32::MAX),
                    _ => 0,
                }
            }
        }
    }

    /// Name of input pin `want` on `b` (for param lookup).
    fn in_name(&self, b: &Block, want: u32) -> Option<String> {
        match b.kind {
            BlockKind::Fb | BlockKind::Function => self
                .sigs
                .get(&b.id)
                .and_then(|s| s.ins.get(want as usize))
                .map(|(n, _)| n.clone()),
            BlockKind::Operator => op_kind(&b.type_ref).map(|op| op_in_name(op, want)),
            _ => None,
        }
    }

    /// Declared type of input pin `want` on `b`.
    fn in_ty(&self, b: &Block, want: u32) -> Option<Type> {
        self.sigs
            .get(&b.id)
            .and_then(|s| s.ins.get(want as usize))
            .map(|(_, t)| t.clone())
    }

    /// Expr for a source port (`wire.src`).
    fn src_expr(
        &mut self,
        pref: &PortRef,
        net: &Network,
        blocks: &BTreeMap<NodeId, &Block>,
    ) -> Option<Expr> {
        let b = blocks.get(&pref.block).copied()?;
        match b.kind {
            BlockKind::Variable => {
                let r = self.var_ref(b)?;
                let ty = self.ref_ty(&r);
                Some(Expr {
                    kind: ExprKind::Ref(r),
                    ty,
                    prov: self.prov(b.id, 0),
                })
            }
            BlockKind::Fb => {
                let sig = self.sigs.get(&b.id).cloned().unwrap_or_default();
                let i = match &pref.port {
                    PortSel::Index(i) => *i,
                    PortSel::Name(n) => sig
                        .outs
                        .iter()
                        .position(|(pn, _)| pn.eq_ignore_ascii_case(n))
                        .map(|i| i as u32)
                        .unwrap_or(u32::MAX),
                };
                if i as usize >= sig.outs.len() {
                    // past-the-end index or ENO → the eno flag var
                    let eno = self.eno_var(b.id);
                    return Some(Expr {
                        kind: ExprKind::Ref(Ref::Var(eno)),
                        ty: Type::Bool,
                        prov: self.prov(b.id, i as u16),
                    });
                }
                let inst = self.inst_by_block.get(&b.id)?.clone();
                let (pn, pty) = sig.outs[i as usize].clone();
                Some(Expr {
                    kind: ExprKind::Ref(Ref::Field(Box::new(Ref::Var(inst)), pn)),
                    ty: pty,
                    prov: self.prov(b.id, i as u16),
                })
            }
            BlockKind::Function => {
                if !self.expanding.insert(b.id) {
                    self.err(codes::ALG_CYCLE, format!("cycle at node {}", b.id), b.id);
                    return None;
                }
                let sig = self.sigs.get(&b.id).cloned().unwrap_or_default();
                let mut args = Vec::new();
                for (i, (pn, _)) in sig.ins.iter().enumerate() {
                    match self.dst_input(net, b, i as u32, blocks) {
                        Some(e) => args.push((pn.clone(), e)),
                        None => self.err(
                            codes::UNCONNECTED,
                            format!("{} input {pn} unconnected", b.type_ref),
                            b.id,
                        ),
                    }
                }
                self.expanding.remove(&b.id);
                Some(Expr {
                    kind: ExprKind::Call {
                        fun: b.type_ref.clone(),
                        args,
                    },
                    ty: Type::Bool, // sema retypes (functions declare ret_ty)
                    prov: self.prov(b.id, 0),
                })
            }
            BlockKind::Operator => self.op_expr(net, b, blocks),
            BlockKind::JumpTarget => None,
        }
    }

    /// Operator block → expr (pure fold or materialized temp ref).
    fn op_expr(
        &mut self,
        net: &Network,
        b: &Block,
        blocks: &BTreeMap<NodeId, &Block>,
    ) -> Option<Expr> {
        if !self.expanding.insert(b.id) {
            self.err(codes::ALG_CYCLE, format!("cycle at node {}", b.id), b.id);
            return None;
        }
        let tref = b.type_ref.clone();
        let p = self.prov(b.id, 0);
        let r = (|| {
            if let Some(to) = cast_target(&tref) {
                let e = self.dst_input(net, b, 0, blocks)?;
                let ty = parse_type(to, self.fb_names).ok()?;
                return Some(Expr {
                    kind: ExprKind::Conv {
                        to: ty.clone(),
                        e: Box::new(e),
                    },
                    ty,
                    prov: p.clone(),
                });
            }
            let op = op_kind(&tref)?;
            match op {
                Op::Sel | Op::Min | Op::Max | Op::Limit | Op::Abs | Op::Mux => {
                    // temp materialized by emit_op
                    Some(Expr {
                        kind: ExprKind::Ref(Ref::Var(format!("__op_{}", b.id))),
                        ty: Type::Bool,
                        prov: p.clone(),
                    })
                }
                Op::Move => self.dst_input(net, b, 0, blocks),
                Op::Not => {
                    let e = self.dst_input(net, b, 0, blocks)?;
                    if !matches!(e.ty, Type::Bool) {
                        self.err(
                            codes::ANY_UNRESOLVED,
                            format!("{} input 0 needs BOOL", b.type_ref),
                            b.id,
                        );
                    }
                    Some(Expr {
                        kind: ExprKind::Un(UnOp::Not, Box::new(e)),
                        ty: Type::Bool,
                        prov: p.clone(),
                    })
                }
                Op::Fold(binop) | Op::Bin(binop) => {
                    let ins = self.op_inputs(net, b, blocks);
                    let mut exprs = Vec::new();
                    for i in &ins {
                        match self.dst_input(net, b, *i, blocks) {
                            Some(e) => exprs.push(e),
                            None => self.err(
                                codes::UNCONNECTED,
                                format!("{} input {i} unconnected", b.type_ref),
                                b.id,
                            ),
                        }
                    }
                    if let Some(2) = op_arity(op) {
                        if exprs.len() != 2 {
                            self.err(
                                codes::UNCONNECTED,
                                format!("{} needs 2 inputs, has {}", b.type_ref, exprs.len()),
                                b.id,
                            );
                        }
                    }
                    self.check_op_class(op, b, &exprs);
                    let mut it = exprs.into_iter();
                    let mut acc = it.next()?;
                    for e in it {
                        let ty = unify_num(&acc.ty, &e.ty);
                        acc = Expr {
                            kind: ExprKind::Bin(binop, Box::new(acc), Box::new(e)),
                            ty,
                            prov: p.clone(),
                        };
                    }
                    Some(acc)
                }
            }
        })();
        self.expanding.remove(&b.id);
        r
    }

    /// Operator input exprs + result type for If-expansion ops.
    /// `exprs[i]` is pin index i (positional — gaps are FBD002 errors
    /// filled with a literal-0 placeholder so lowering can continue).
    fn typed_inputs(
        &mut self,
        net: &Network,
        b: &Block,
        blocks: &BTreeMap<NodeId, &Block>,
        op: Op,
        ins: &BTreeSet<u32>,
    ) -> Option<(Vec<Expr>, Type)> {
        // required pin count: fixed-arity ops or the highest wired pin
        let need = match op_arity(op) {
            Some(n) => n,
            None => ins.iter().max().copied().unwrap_or(0) + 1,
        };
        let mut exprs: Vec<Expr> = Vec::new();
        for i in 0..need {
            match self.dst_input(net, b, i, blocks) {
                Some(e) => exprs.push(e),
                None => {
                    self.err(
                        codes::UNCONNECTED,
                        format!("{} input {i} unconnected", b.type_ref),
                        b.id,
                    );
                    exprs.push(Expr {
                        kind: ExprKind::Lit(Lit::Int(0)),
                        ty: Type::Bool,
                        prov: self.prov(b.id, i as u16),
                    });
                }
            }
        }
        if exprs.is_empty() {
            return None;
        }
        self.check_op_class(op, b, &exprs);
        let rty = match op {
            Op::Sel | Op::Mux => exprs
                .iter()
                .skip(1)
                .map(|e| e.ty.clone())
                .reduce(|a, t| unify_num(&a, &t))
                .unwrap_or(Type::Bool),
            Op::Min | Op::Max => exprs
                .iter()
                .map(|e| e.ty.clone())
                .reduce(|a, t| unify_num(&a, &t))
                .unwrap_or(Type::Bool),
            Op::Limit => exprs[1].ty.clone(),
            Op::Abs => exprs[0].ty.clone(),
            _ => Type::Bool,
        };
        Some((exprs, rty))
    }

    /// FBD009 — generic `ANY_*` pins must resolve to a type in the
    /// operator's class: BOOL ops take BOOL, arith ops take
    /// Int/Real/Time, shifts take Int, SEL.G is BOOL, MUX.K is Int.
    fn check_op_class(&mut self, op: Op, b: &Block, exprs: &[Expr]) {
        // literals classify by payload — param literals carry a Bool
        // placeholder ty until coerced at the pin
        let num = |e: &Expr| match &e.kind {
            ExprKind::Lit(Lit::Int(_) | Lit::UInt(_) | Lit::Real(_) | Lit::TimeNs(_)) => true,
            ExprKind::Lit(_) => false,
            _ => matches!(e.ty, Type::Int { .. } | Type::Real { .. } | Type::Time),
        };
        let int = |e: &Expr| match &e.kind {
            ExprKind::Lit(Lit::Int(_) | Lit::UInt(_)) => true,
            ExprKind::Lit(_) => false,
            _ => matches!(e.ty, Type::Int { .. }),
        };
        let boolean = |e: &Expr| match &e.kind {
            ExprKind::Lit(Lit::Bool(_)) => true,
            ExprKind::Lit(_) => false,
            _ => matches!(e.ty, Type::Bool),
        };
        let mut bad = |cond: bool, pin: usize, want: &str| {
            if cond {
                self.err(
                    codes::ANY_UNRESOLVED,
                    format!("{} input {pin} needs {want}", b.type_ref),
                    b.id,
                );
            }
        };
        match op {
            Op::Fold(BinOp::And | BinOp::Or | BinOp::Xor) | Op::Not => {
                for (i, e) in exprs.iter().enumerate() {
                    bad(!boolean(e), i, "BOOL");
                }
            }
            Op::Bin(BinOp::Shl | BinOp::Shr | BinOp::Rol | BinOp::Ror) => {
                for (i, e) in exprs.iter().enumerate() {
                    bad(!int(e), i, "ANY_INT");
                }
            }
            Op::Sel => {
                for (i, e) in exprs.iter().enumerate() {
                    if i == 0 {
                        bad(!boolean(e), i, "BOOL");
                    } else {
                        bad(!num(e), i, "ANY_NUM");
                    }
                }
            }
            Op::Mux => {
                for (i, e) in exprs.iter().enumerate() {
                    if i == 0 {
                        bad(!int(e), i, "ANY_INT");
                    } else {
                        bad(!num(e), i, "ANY_NUM");
                    }
                }
            }
            // ANY operand ops: EQ/NE accept every same-class pair
            Op::Move | Op::Bin(BinOp::Eq | BinOp::Ne) => {}
            _ => {
                for (i, e) in exprs.iter().enumerate() {
                    bad(!num(e), i, "ANY_NUM");
                }
            }
        }
    }

    /// `eno` internal var for an fb block (lazily declared).
    fn eno_var(&mut self, blk: NodeId) -> String {
        let name = format!("__eno_{blk}");
        if !self.var_by_name.contains_key(&name.to_ascii_uppercase()) {
            self.declare(VarDecl {
                name: name.clone(),
                kind: VarKind::Local,
                ty: Type::Bool,
                init: None,
                retain: false,
                at: None,
                probe: false,
                prov: Prov::Generated,
            });
        }
        name
    }

    /// `Ref` for a variable block's storage.
    fn var_ref(&mut self, b: &Block) -> Option<Ref> {
        if let Some(&idx) = self.var_by_block.get(&b.id) {
            return Some(Ref::Var(self.vars[idx].name.clone()));
        }
        let n = b.name.as_ref()?;
        if let Some(&idx) = self.var_by_name.get(&n.to_ascii_uppercase()) {
            return Some(Ref::Var(self.vars[idx].name.clone()));
        }
        self.gvl_member(n)
            .map(|(g, m)| Ref::Field(Box::new(Ref::Var(g)), m))
    }

    fn ref_ty(&self, r: &Ref) -> Type {
        match r {
            Ref::Var(n) => self
                .var_by_name
                .get(&n.to_ascii_uppercase())
                .map(|&i| self.vars[i].ty.clone())
                .unwrap_or(Type::Bool),
            Ref::Field(inner, m) => {
                if let Ref::Var(g) = inner.as_ref() {
                    if g.eq_ignore_ascii_case("GVL") {
                        for gv in &self.project.global_vars {
                            if gv.name.eq_ignore_ascii_case(m) {
                                return parse_type(&gv.ty, self.fb_names).unwrap_or(Type::Bool);
                            }
                        }
                    }
                }
                Type::Bool
            }
            Ref::Index(inner, _) => self.ref_ty(inner),
            Ref::Deref(inner) => match self.ref_ty(inner) {
                Type::Ref { to } => *to,
                _ => Type::Bool,
            },
        }
    }

    /// FBD001 check: src expr → dst pin type must be coercible.
    fn check_coerce(&mut self, e: &Expr, want: &Type, node: u64) {
        let ok = compatible(&e.ty, want) || convertible(&e.ty, want) || literal_fits(e, want);
        if !ok {
            self.diags.push(
                Diag::error(
                    codes::TYPE_MISMATCH,
                    format!("expected {want:?}, found {:?}", e.ty),
                    self.prov(node, 0),
                )
                .with_help("no implicit conversion — insert a cast block"),
            );
        }
    }
}

fn region_ch(r: AtRegion) -> char {
    match r {
        AtRegion::I => 'i',
        AtRegion::Q => 'q',
        AtRegion::M => 'm',
    }
}
