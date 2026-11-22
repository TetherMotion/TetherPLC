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
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use tplc_mir::diag::Diag;
use tplc_mir::ir;
use tplc_mir::ir::{
    AtRegion, BinOp, Expr, ExprKind, IoBind, Lit, Prov, Ref, Stmt, StmtKind, Type, UnOp, VarDecl,
    VarKind,
};
use tplc_mir::ty::{compatible, convertible, literal_fits, unify_num};

use crate::resolve::{
    builtin_sig, cast_target, device_sig, op_arity, op_in_index, op_in_name, op_kind, pou_sig,
    topo_order, Op, Sig,
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

/// Variable block carrying PLCopen data-tap semantics
/// (`inVariable`/`outVariable`/`inOutVariable` → `flow: data`)
/// rather than power-flow contact/coil semantics. Data taps lower
/// through the FBD variable-block path — no rung gating.
mod decls;
mod emit;
mod expr;
mod ladder;

fn is_data_var(b: &Block) -> bool {
    b.kind == BlockKind::Variable && b.flow == Some(VarFlow::Data)
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
    lower_mixed(project, &[])
}

/// Lower with an extern POU table — POUs compiled from other sources
/// (mixed FBD+ST projects). Extern FBs and FUNCTIONs are resolvable
/// as block types/callables; their signatures feed pin binding and
/// type checks exactly like in-project decl POUs.
pub fn lower_mixed(project: &FbdProject, externs: &[ir::Pou]) -> LowerOutput {
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
    let mut fb_names: BTreeSet<String> = project
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
    // extern POUs (e.g. ST-defined FBs/FUNCTIONs in mixed projects)
    let ext_pous: BTreeMap<String, &ir::Pou> = externs
        .iter()
        .map(|p| (p.name.to_ascii_uppercase(), p))
        .collect();
    fb_names.extend(
        externs
            .iter()
            .filter(|p| p.kind == ir::PouKind::FunctionBlock)
            .map(|p| p.name.to_ascii_uppercase()),
    );

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
        let kind = match t.kind.to_ascii_lowercase().as_str() {
            "cyclic" => ir::TaskKind::Cyclic,
            "freewheeling" => ir::TaskKind::Freewheeling,
            "event" => ir::TaskKind::Event,
            other => {
                out.diags.push(tplc_mir::diag::Diag::error(
                    "TPLC_E_TASK_KIND",
                    format!("task {}: unknown kind '{other}'", t.name),
                    tplc_mir::ir::Prov::None,
                ));
                ir::TaskKind::Cyclic
            }
        };
        let trig = if kind == ir::TaskKind::Event {
            match &t.trigger {
                Some(tr) => {
                    let tt = match tr.trig.to_ascii_lowercase().as_str() {
                        "rise" => ir::TaskTrig::Rise,
                        "level" => ir::TaskTrig::Level,
                        "manual" => ir::TaskTrig::Manual,
                        other => {
                            out.diags.push(tplc_mir::diag::Diag::error(
                                "TPLC_E_TASK_TRIG",
                                format!(
                                    "task {}: unknown trigger type '{other}'",
                                    t.name
                                ),
                                tplc_mir::ir::Prov::None,
                            ));
                            ir::TaskTrig::Rise
                        }
                    };
                    let sig = tr.signal.clone().unwrap_or_default();
                    if tt != ir::TaskTrig::Manual && sig.trim().is_empty() {
                        out.diags.push(tplc_mir::diag::Diag::error(
                            "TPLC_E_TASK_TRIG",
                            format!("task {}: trigger needs a signal", t.name),
                            tplc_mir::ir::Prov::None,
                        ));
                    }
                    Some((tt, sig))
                }
                None => {
                    out.diags.push(tplc_mir::diag::Diag::error(
                        "TPLC_E_TASK_TRIG",
                        format!("event task {} has no trigger", t.name),
                        tplc_mir::ir::Prov::None,
                    ));
                    None
                }
            }
        } else {
            None
        };
        out.module.tasks.push(ir::TaskDecl {
            name: t.name.clone(),
            period_ns: t.period_ns,
            wcet_hint_ns: 0,
            priority: t.priority.min(255) as u8,
            kind,
            trig,
            programs: t
                .programs
                .iter()
                .map(|id| {
                    project
                        .pous
                        .iter()
                        .find(|p| &p.id == id || &p.name == id)
                        .map(|p| p.name.clone())
                        // program defined in another language (mixed
                        // FBD+ST) — keep the name for sema to resolve
                        .unwrap_or_else(|| id.clone())
                })
                .collect(),
        });
    }

    for pou in &project.pous {
        let mut cx = Cx::new(pou, &fb_names, &pou_by_name, &ext_pous, project);
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

/// `Sig` of an extern MIR POU (ST-compiled) — same binding view as
/// `pou_sig`: inputs+inouts in decl order, then outputs; FUNCTIONs get
/// their return value as implicit first output.
fn mir_sig(pou: &ir::Pou) -> Sig {
    let mut sig = Sig::default();
    for v in &pou.iface {
        match v.kind {
            VarKind::Input => sig.ins.push((v.name.clone(), v.ty.clone())),
            VarKind::InOut => {
                sig.inouts.insert(sig.ins.len() as u32);
                sig.ins.push((v.name.clone(), v.ty.clone()));
            }
            VarKind::Output => sig.outs.push((v.name.clone(), v.ty.clone())),
            _ => {}
        }
    }
    if pou.kind == ir::PouKind::Function {
        sig.outs.insert(
            0,
            (pou.name.clone(), pou.ret_ty.clone().unwrap_or(Type::Bool)),
        );
    }
    sig
}

/// Per-POU lowering context.
struct Cx<'a> {
    pou: &'a Pou,
    fb_names: &'a BTreeSet<String>,
    pous: &'a BTreeMap<String, &'a Pou>,
    ext_pous: &'a BTreeMap<String, &'a ir::Pou>,
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
    /// block id → resolved sig (fb/function/device blocks)
    sigs: BTreeMap<NodeId, Sig>,
    /// (device block id, pin name upper) → vars index of the located
    /// channel var the pin aliases
    dev_vars: BTreeMap<(NodeId, String), usize>,
    sheet: u32,
    /// blocks currently being folded (algebraic-cycle guard)
    expanding: BTreeSet<NodeId>,
    /// ladder wire-topology: variable block id → gated flow expr.
    /// src_expr consults this for ladder nets so contacts feed gated
    /// (not raw) values into data pins; populated per network by
    /// lower_ladder_wired in topo order.
    ld_gated: HashMap<NodeId, Expr>,
}

impl<'a> Cx<'a> {
    fn new(
        pou: &'a Pou,
        fb_names: &'a BTreeSet<String>,
        pous: &'a BTreeMap<String, &'a Pou>,
        ext_pous: &'a BTreeMap<String, &'a ir::Pou>,
        project: &'a FbdProject,
    ) -> Self {
        Cx {
            pou,
            fb_names,
            pous,
            ext_pous,
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
            dev_vars: BTreeMap::new(),
            sheet: 0,
            expanding: BTreeSet::new(),
            ld_gated: HashMap::new(),
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

        // FUNCTION return value — implicit local var named after the
        // POU (mirrors the ST frontend): `variable-out` blocks named
        // `pou.name` then write the return value.
        let ret_ty = self
            .pou
            .ret_ty
            .as_ref()
            .map(|t| parse_type(t, self.fb_names).unwrap_or(Type::Bool));
        if kind == ir::PouKind::Function {
            let ty = ret_ty.clone().unwrap_or(Type::Bool);
            if self
                .var_by_name
                .contains_key(&self.pou.name.to_ascii_uppercase())
            {
                self.err(
                    codes::INST_REUSE,
                    format!("interface var shadows return value {}", self.pou.name),
                    0,
                );
            } else {
                self.declare(VarDecl {
                    name: self.pou.name.clone(),
                    kind: VarKind::Local,
                    ty,
                    init: None,
                    retain: false,
                    at: None,
                    probe: false,
                    prov: Prov::Generated,
                });
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
            ret_ty,
            prov: self.prov(0, 0),
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

/* ---------------- inline ST blocks ---------------- */

/// Rewrite `st` blocks into generated FUNCTION_BLOCK sources for mixed
/// FBD+ST compilation. Returns `(source_name, source_text)` pairs —
/// the caller appends them to the project's ST source list and compiles
/// before lowering. Each `st` block mutates into an `fb` block whose
/// `type_ref` is the generated POU name, so wire/param/EN-ENO binding
/// works unchanged.
///
/// Pins: `block.pins` entries (VarInput/VarOutput/VarInOut/Var) become
/// the generated FB's interface; `params["CODE"]` is the body — full
/// IEC 61131-3 ST, including FB calls and member/array writes (which
/// is also how aggregate config like `FB_AtcProfile.SEG` is authored).
pub fn expand_st_blocks(project: &mut FbdProject) -> Vec<(String, String)> {
    let mut srcs = Vec::new();
    for pou in &mut project.pous {
        for net in &mut pou.networks {
            for b in &mut net.blocks {
                if b.kind != BlockKind::St {
                    continue;
                }
                let name = format!(
                    "__fbd_st_{}_{}",
                    pou.name
                        .chars()
                        .map(|c| if c.is_ascii_alphanumeric() || c == '_' {
                            c
                        } else {
                            '_'
                        })
                        .collect::<String>(),
                    b.id
                );
                srcs.push((format!("{name}.st"), gen_st_fb_source(&name, b)));
                b.kind = BlockKind::Fb;
                b.type_ref = name;
            }
        }
    }
    srcs
}

/// Generated FB source for one `st` block: declared pin groups + the
/// CODE body verbatim (parsed by the normal ST frontend — syntax and
/// type errors carry that file's provenance).
fn gen_st_fb_source(name: &str, b: &Block) -> String {
    let mut s = format!("FUNCTION_BLOCK {name}\n");
    for (kw, want) in [
        ("VAR_INPUT", IfaceKind::VarInput),
        ("VAR_OUTPUT", IfaceKind::VarOutput),
        ("VAR_IN_OUT", IfaceKind::VarInOut),
        ("VAR", IfaceKind::Var),
    ] {
        let pins: Vec<&IfaceVar> = b.pins.iter().filter(|p| p.kind == want).collect();
        if pins.is_empty() {
            continue;
        }
        s.push_str(kw);
        s.push('\n');
        for v in pins {
            s.push_str("    ");
            s.push_str(&v.name);
            s.push_str(" : ");
            s.push_str(&v.ty);
            if let Some(i) = &v.init {
                s.push_str(" := ");
                s.push_str(i);
            }
            s.push_str(";\n");
        }
        s.push_str("END_VAR\n");
    }
    let code = b.params.get("CODE").map(String::as_str).unwrap_or("");
    s.push_str(code);
    s.push_str("\nEND_FUNCTION_BLOCK\n");
    s
}
