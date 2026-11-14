use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const SCHEMA_ID: &str = "fbd.v1";

/// Stable u64 — THE provenance key; never reused after delete within a POU.
pub type NodeId = u64;

/* ---------------- project ---------------- */

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct FbdProject {
    pub schema: String,
    pub project: String,
    #[serde(default)]
    pub pous: Vec<Pou>,
    #[serde(default)]
    pub tasks: Vec<Task>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data_types: Vec<DataTypeDecl>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub global_vars: Vec<IfaceVar>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Task {
    pub name: String,
    pub period_ns: u64,
    #[serde(default)]
    pub programs: Vec<String>,
    #[serde(default)]
    pub priority: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DataTypeDecl {
    pub name: String,
    /// IEC type expression (structured types keep their TC6/JSON
    /// sub-form; scalar names pass through).
    pub ty: serde_json::Value,
}

/* ---------------- pou ---------------- */

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum PouKind {
    Program,
    FunctionBlock,
    Function,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Pou {
    pub id: String,
    pub name: String,
    pub kind: PouKind,
    #[serde(default)]
    pub interface: Vec<IfaceVar>,
    #[serde(default)]
    pub networks: Vec<Network>,
    /// FUNCTION return type (IEC type expression, e.g. "LREAL").
    /// Required for `kind == "function"` POUs to carry a value back to
    /// callers; ignored for program/function_block.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ret_ty: Option<String>,
    /// EN/ENO config: "relaxed" (default) | "strict-iec" (doc 06 §4.5).
    #[serde(default = "default_en_eno")]
    pub en_eno: String,
}

fn default_en_eno() -> String {
    "relaxed".into()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum IfaceKind {
    Var,
    VarInput,
    VarOutput,
    VarInOut,
    VarExternal,
    VarConstant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct IfaceVar {
    pub name: String,
    pub kind: IfaceKind,
    /// IEC type name/expression string ("INT", "TON", "ARRAY[0..9] OF BOOL").
    #[serde(rename = "type")]
    pub ty: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub init: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub at: Option<String>,
    #[serde(default)]
    pub retain: bool,
}

/* ---------------- network ---------------- */

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum NetworkKind {
    Fbd,
    /// iter-2 hook (doc 06 §11): schema-reserved, lowering rejects.
    Ladder,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Network {
    pub id: String,
    #[serde(default = "default_net_kind")]
    pub kind: NetworkKind,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub blocks: Vec<Block>,
    #[serde(default)]
    pub wires: Vec<Wire>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assocs: Vec<Assoc>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comments: Vec<Comment>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<Label>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jumps: Vec<Jump>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub returns: Vec<ReturnMarker>,
}

fn default_net_kind() -> NetworkKind {
    NetworkKind::Fbd
}

impl Network {
    /// every provenance id in this network — feed to `IdAlloc::over`
    /// when loading a document.
    pub fn node_ids(&self) -> impl Iterator<Item = &NodeId> {
        self.blocks
            .iter()
            .map(|b| &b.id)
            .chain(self.wires.iter().map(|w| &w.id))
            .chain(self.comments.iter().map(|c| &c.id))
            .chain(self.labels.iter().map(|l| &l.id))
            .chain(self.jumps.iter().map(|j| &j.id))
            .chain(self.returns.iter().map(|r| &r.id))
    }
}

/* ---------------- blocks ---------------- */

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum BlockKind {
    Fb,
    Function,
    Operator,
    Variable,
    JumpTarget,
    /// Inline Structured-Text block: `pins` declares typed
    /// VAR_INPUT/VAR_OUTPUT pins, `params["CODE"]` holds the ST body.
    /// `tplcc` expands each ST block into a generated FUNCTION_BLOCK
    /// (`__fbd_st_*`) compiled with the project's ST sources; the FBD
    /// network then calls it like any other FB.
    St,
    /// I/O device block (fieldbus terminal, e.g. a Beckhoff EK remote
    /// I/O station). `type_ref` names the device, `pins` declares the
    /// channels: var_output pins are device→logic outputs (block out
    /// pins), var_input pins are logic→device inputs (block in pins).
    /// Every pin carries an `at` process-image binding (`%I`/`%Q`/`%M`)
    /// so each channel aliases a located var — wires on the pins read
    /// and write the image directly, no call is emitted.
    Device,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum VarRole {
    In,
    Out,
    InOut,
}

/// How a variable block participates in ladder-rung wires.
/// Absent = power flow (IEC `<contact>`/`<coil>`): the block gates
/// BOOL rung flow. `Data` = PLCopen `inVariable`/`outVariable`/
/// `inOutVariable` element: a typed data tap lowered exactly like an
/// FBD variable block — no gating, wires carry values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum VarFlow {
    Data,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Block {
    pub id: NodeId,
    pub kind: BlockKind,
    /// FB type name | function signature | operator mnemonic.
    /// Empty for variable blocks.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub type_ref: String,
    /// required for fb kind; synthesized by lowering when absent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    #[serde(default)]
    pub pos: Pos,
    /// literal/customized parameters keyed by pin name ("PT":"T#200ms").
    /// For `st` blocks, `params["CODE"]` holds the ST statement body.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub params: BTreeMap<String, String>,
    /// typed pin declarations for `st` blocks (VarInput/VarOutput
    /// entries become the generated FB's interface).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pins: Vec<IfaceVar>,
    /// explicit order override; 0 = auto topo (doc 06 §4.1).
    #[serde(default)]
    pub exec_order: u32,
    /// pin indexes carrying IEC negation markers.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub negated: Vec<u32>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub comment: String,
    /* variable-block fields */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<VarRole>,
    /// ladder-only flow class — see `VarFlow`. Ignored in FBD nets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flow: Option<VarFlow>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub at: Option<String>,
    /// device blocks: bus PDO endpoint key (e.g. `el1014.c1`) the
    /// deploy map binds `pdo=` channels to. Transport metadata — the
    /// compiler ignores it; the WebUI deploy-map generator consumes it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}

/// grid units — positions in the model are integers; canvas floats
/// are render-layer only (doc 06 §4.2 determinism).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Pos {
    pub x: u32,
    pub y: u32,
}

/* ---------------- wires ---------------- */

/// pin selector: signature-order index or FB pin name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum PortSel {
    Index(u32),
    Name(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PortRef {
    pub block: NodeId,
    pub port: PortSel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Wire {
    pub id: NodeId,
    pub src: PortRef,
    #[serde(default)]
    pub dsts: Vec<PortRef>,
    /// debug probe marker (doc 06 §4.3) — opt-in blob field + store.
    #[serde(default, skip_serializing_if = "is_false")]
    pub probe: bool,
}

fn is_false(b: &bool) -> bool {
    !*b
}

/// wireless association (PLCopen formalParameter+variable form):
/// a named source feeds a pin without a drawn wire.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Assoc {
    /// variable/element name resolved against POU interface/GVL.
    pub src_name: String,
    pub dst: PortRef,
}

/* ---------------- annotations ---------------- */

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Comment {
    pub id: NodeId,
    #[serde(default)]
    pub pos: Pos,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Label {
    pub id: NodeId,
    #[serde(default)]
    pub pos: Pos,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Jump {
    pub id: NodeId,
    #[serde(default)]
    pub pos: Pos,
    pub target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ReturnMarker {
    pub id: NodeId,
    #[serde(default)]
    pub pos: Pos,
}
