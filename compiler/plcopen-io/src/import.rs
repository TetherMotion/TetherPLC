use fbd_model::*;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::BTreeMap;

/// Non-fatal import gap — unsupported/skipped constructs surface here,
/// never as silent drops (doc 06 §5).
#[derive(Debug)]
pub struct XmlNote {
    pub what: String,
}

#[derive(Debug)]
pub struct ImportReport {
    pub project: FbdProject,
    pub notes: Vec<XmlNote>,
}

#[derive(Debug)]
pub enum ImportError {
    Xml(quick_xml::Error),
    /// not a TC6 project document
    NotProject,
}

impl std::fmt::Display for ImportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImportError::Xml(e) => write!(f, "{e}"),
            ImportError::NotProject => write!(f, "not a PLCopen project document"),
        }
    }
}
impl std::error::Error for ImportError {}
impl From<quick_xml::Error> for ImportError {
    fn from(e: quick_xml::Error) -> Self {
        ImportError::Xml(e)
    }
}

pub fn import(src: &str) -> Result<ImportReport, ImportError> {
    let mut r = Reader::from_str(src);
    r.config_mut().trim_text(true);
    let mut st = P::default();
    let mut buf = Vec::new();
    loop {
        match r.read_event_into(&mut buf)? {
            Event::Start(e) => st.start(&e),
            Event::Empty(e) => {
                st.start(&e);
                st.end(e.name().as_ref());
            }
            Event::End(e) => st.end(e.name().as_ref()),
            Event::Text(e) => st.text(&e),
            Event::CData(e) => st.cdata(&e),
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }
    st.finish()
}

/// operator mnemonics — blocks with these type_names and no instance
/// are `operator` kind, not `function`.
const OPERATORS: &[&str] = &[
    "ADD", "SUB", "MUL", "DIV", "MOD", "AND", "OR", "XOR", "NOT", "NAND", "NOR", "GT", "GE", "LT",
    "LE", "EQ", "NE", "SEL", "MUX", "MIN", "MAX", "LIMIT", "ABS", "SHL", "SHR", "ROL", "ROR",
    "MOVE",
];

/// which annotation a `<position>` belongs to — set at annotation
/// start, consumed by the position element.
#[derive(Clone, Copy)]
enum AnnTarget {
    Comment(usize),
    Label(usize),
    Jump(usize),
    Return(usize),
}

#[derive(Default)]
struct P {
    notes: Vec<XmlNote>,
    project_name: String,
    pous: Vec<Pou>,
    tasks: Vec<Task>,
    gvl: Vec<IfaceVar>,
    dtypes: Vec<DataTypeDecl>,

    stack: Vec<String>,
    cur_pou: Option<Pou>,
    cur_net: Option<Network>,
    cur_block: Option<Block>,
    cur_pin: Option<String>,
    in_cpi: bool,
    cpi_negated: bool,
    cpi_has_conn: bool,
    cpi_expr: String,
    in_expr: bool,
    expr_text: String,
    comment_text: String,
    ann: Option<AnnTarget>,
    pending_var: Option<IfaceVar>,
    in_adddata: bool,
    cur_task: Option<Task>,
    conns: Vec<Conn>,
    alloc: IdAlloc,
}

struct Conn {
    src: PortRef,
    dst: PortRef,
    probe: bool,
    wire_id: Option<NodeId>,
    negated: bool,
}

impl P {
    /// strip namespace prefix → local name.
    fn local(name: &[u8]) -> &str {
        let s = std::str::from_utf8(name).unwrap_or("");
        s.rsplit(':').next().unwrap_or(s)
    }

    /// attribute by local name — `tether:wire` matches `wire`.
    fn attr(e: &quick_xml::events::BytesStart, name: &str) -> Option<String> {
        e.attributes()
            .filter_map(|a| a.ok())
            .find(|a| {
                let k = std::str::from_utf8(a.key.as_ref()).unwrap_or("");
                k == name || k.rsplit(':').next() == Some(name)
            })
            .and_then(|a| String::from_utf8(a.value.to_vec()).ok())
    }

    /// parent element — the current tag is pushed only at the end of
    /// `start()`, so the stack top during the match is the parent.
    fn parent(&self) -> &str {
        self.stack.last().map(|s| s.as_str()).unwrap_or("")
    }

    fn depth_tag(&self, tag: &str) -> bool {
        self.stack.iter().any(|t| t == tag)
    }

    fn local_id(&mut self, e: &quick_xml::events::BytesStart) -> NodeId {
        Self::attr(e, "localId")
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or_else(|| self.alloc.alloc())
    }

    fn auto_net_id(&self) -> u64 {
        self.cur_pou
            .as_ref()
            .map(|p| p.networks.len() as u64 + 1)
            .unwrap_or(1)
    }

    fn new_net(&self, id: String) -> Network {
        Network {
            id,
            kind: NetworkKind::Fbd,
            label: String::new(),
            blocks: vec![],
            wires: vec![],
            assocs: vec![],
            comments: vec![],
            labels: vec![],
            jumps: vec![],
            returns: vec![],
        }
    }

    fn ensure_net(&mut self) {
        if self.cur_net.is_none() && self.cur_pou.is_some() {
            self.cur_net = Some(self.new_net(format!("net_{}", self.auto_net_id())));
        }
    }

    fn pos_of(e: &quick_xml::events::BytesStart) -> Pos {
        Pos {
            x: Self::attr(e, "x")
                .and_then(|s| s.parse::<f64>().ok())
                .unwrap_or(0.0) as u32,
            y: Self::attr(e, "y")
                .and_then(|s| s.parse::<f64>().ok())
                .unwrap_or(0.0) as u32,
        }
    }

    fn start(&mut self, e: &quick_xml::events::BytesStart) {
        let tag = Self::local(e.name().as_ref()).to_string();
        let in_fbdish = self.depth_tag("FBD");
        match tag.as_str() {
            "contentHeader" => {
                if let Some(n) = Self::attr(e, "name") {
                    if self.project_name.is_empty() {
                        self.project_name = n;
                    }
                }
            }
            "pou" => {
                self.cur_pou = Some(Pou {
                    id: String::new(),
                    name: Self::attr(e, "name").unwrap_or_default(),
                    kind: match Self::attr(e, "pouType").as_deref() {
                        Some("functionBlock") => PouKind::FunctionBlock,
                        Some("function") => PouKind::Function,
                        _ => PouKind::Program,
                    },
                    interface: vec![],
                    networks: vec![],
                    en_eno: "relaxed".into(),
                });
            }
            "FBD" => self.ensure_net(),
            "network" if in_fbdish => {
                // tether:network (prefix stripped) — roundtrip wrap
                let net = Network {
                    kind: match Self::attr(e, "kind").as_deref() {
                        Some("ladder") => NetworkKind::Ladder,
                        _ => NetworkKind::Fbd,
                    },
                    label: Self::attr(e, "label").unwrap_or_default(),
                    ..self.new_net(
                        Self::attr(e, "id")
                            .unwrap_or_else(|| format!("net_{}", self.auto_net_id())),
                    )
                };
                if let Some(prev) = self.cur_net.replace(net) {
                    if !net_is_empty(&prev) {
                        if let Some(pou) = &mut self.cur_pou {
                            pou.networks.push(prev);
                        }
                    }
                }
            }
            "block" if in_fbdish => {
                let tn = Self::attr(e, "typeName").unwrap_or_default();
                let inst = Self::attr(e, "instanceName");
                let kind = if inst.is_some() {
                    BlockKind::Fb
                } else if OPERATORS.contains(&tn.to_ascii_uppercase().as_str()) {
                    BlockKind::Operator
                } else {
                    BlockKind::Function
                };
                self.cur_block = Some(Block {
                    id: self.local_id(e),
                    kind,
                    type_ref: tn,
                    instance: inst,
                    pos: Pos::default(),
                    params: BTreeMap::new(),
                    exec_order: Self::attr(e, "executionOrderId")
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(0),
                    negated: vec![],
                    comment: String::new(),
                    role: None,
                    name: None,
                    at: None,
                });
            }
            "inVariable" | "outVariable" | "inOutVariable" if in_fbdish => {
                let role = match tag.as_str() {
                    "inVariable" => VarRole::In,
                    "outVariable" => VarRole::Out,
                    _ => VarRole::InOut,
                };
                self.cur_block = Some(Block {
                    id: self.local_id(e),
                    kind: BlockKind::Variable,
                    type_ref: String::new(),
                    instance: None,
                    pos: Pos::default(),
                    params: BTreeMap::new(),
                    exec_order: 0,
                    negated: vec![],
                    comment: String::new(),
                    role: Some(role),
                    name: Self::attr(e, "name"),
                    at: None,
                });
            }
            "variable" => match self.parent() {
                "inputVariables" | "inOutVariables" | "outputVariables" => {
                    self.cur_pin = Self::attr(e, "formalParameter");
                }
                "inputVars" | "outputVars" | "localVars" | "externalVars" | "inOutVars"
                | "globalVars" => {
                    let kind = match self.parent() {
                        "inputVars" => IfaceKind::VarInput,
                        "outputVars" => IfaceKind::VarOutput,
                        "externalVars" => IfaceKind::VarExternal,
                        "inOutVars" => IfaceKind::VarInOut,
                        _ => IfaceKind::Var,
                    };
                    self.pending_var = Some(IfaceVar {
                        name: Self::attr(e, "name").unwrap_or_default(),
                        kind,
                        ty: Self::attr(e, "type").unwrap_or_default(),
                        init: None,
                        at: Self::attr(e, "address"),
                        retain: false,
                    });
                }
                _ => {}
            },
            "type" if self.pending_var.is_some() => {}
            "derived" => {
                if let Some(v) = &mut self.pending_var {
                    if v.ty.is_empty() {
                        if let Some(n) = Self::attr(e, "name") {
                            v.ty = n;
                        }
                    }
                }
            }
            "simpleValue" => {
                if self.depth_tag("initialValue") {
                    if let Some(v) = &mut self.pending_var {
                        v.init = Self::attr(e, "value");
                    }
                }
            }
            "connectionPointIn" => {
                self.in_cpi = true;
                self.cpi_negated = Self::attr(e, "negated").as_deref() == Some("true");
                self.cpi_has_conn = false;
                self.cpi_expr.clear();
            }
            "connection" => {
                if !self.in_cpi {
                    // connectionPointOut-side links are the src end —
                    // the dst side already carries the full ref
                    self.stack.push(tag);
                    return;
                }
                let ref_id = Self::attr(e, "refLocalId").and_then(|s| s.parse::<u64>().ok());
                if let Some(src_block) = ref_id {
                    let src_port = Self::attr(e, "formalParameter")
                        .map(PortSel::Name)
                        .unwrap_or(PortSel::Index(0));
                    let dst = match (&self.cur_block, &self.cur_pin) {
                        (Some(b), Some(pin)) => PortRef {
                            block: b.id,
                            port: pin_index(pin)
                                .map(PortSel::Index)
                                .unwrap_or_else(|| PortSel::Name(pin.clone())),
                        },
                        (Some(b), None) => PortRef {
                            block: b.id,
                            port: PortSel::Index(0),
                        },
                        _ => {
                            self.stack.push(tag);
                            return;
                        }
                    };
                    self.conns.push(Conn {
                        src: PortRef {
                            block: src_block,
                            port: src_port,
                        },
                        dst,
                        probe: Self::attr(e, "probe").as_deref() == Some("true"),
                        wire_id: Self::attr(e, "wire").and_then(|s| s.parse().ok()),
                        negated: self.cpi_negated,
                    });
                    self.cpi_has_conn = true;
                }
            }
            "expression" => {
                self.in_expr = true;
                self.expr_text.clear();
            }
            "position" => {
                let p = Self::pos_of(e);
                if let Some(b) = &mut self.cur_block {
                    b.pos = p;
                } else if let (Some(n), Some(a)) = (&mut self.cur_net, self.ann) {
                    match a {
                        AnnTarget::Comment(i) => {
                            if let Some(c) = n.comments.get_mut(i) {
                                c.pos = p;
                            }
                        }
                        AnnTarget::Label(i) => {
                            if let Some(l) = n.labels.get_mut(i) {
                                l.pos = p;
                            }
                        }
                        AnnTarget::Jump(i) => {
                            if let Some(j) = n.jumps.get_mut(i) {
                                j.pos = p;
                            }
                        }
                        AnnTarget::Return(i) => {
                            if let Some(r) = n.returns.get_mut(i) {
                                r.pos = p;
                            }
                        }
                    }
                }
            }
            "comment" if in_fbdish || self.depth_tag("network") => {
                self.ensure_net();
                let id = self.local_id(e);
                if let Some(n) = &mut self.cur_net {
                    n.comments.push(Comment {
                        id,
                        pos: Pos::default(),
                        text: String::new(),
                    });
                    self.ann = Some(AnnTarget::Comment(n.comments.len() - 1));
                    self.comment_text.clear();
                }
            }
            "label" if in_fbdish || self.depth_tag("network") => {
                self.ensure_net();
                let id = self.local_id(e);
                if let Some(n) = &mut self.cur_net {
                    n.labels.push(Label {
                        id,
                        pos: Pos::default(),
                        name: Self::attr(e, "label").unwrap_or_default(),
                    });
                    self.ann = Some(AnnTarget::Label(n.labels.len() - 1));
                }
            }
            "jump" if in_fbdish || self.depth_tag("network") => {
                self.ensure_net();
                let id = self.local_id(e);
                if let Some(n) = &mut self.cur_net {
                    n.jumps.push(Jump {
                        id,
                        pos: Pos::default(),
                        target: Self::attr(e, "label").unwrap_or_default(),
                    });
                    self.ann = Some(AnnTarget::Jump(n.jumps.len() - 1));
                }
            }
            "return" if in_fbdish || self.depth_tag("network") => {
                self.ensure_net();
                let id = self.local_id(e);
                if let Some(n) = &mut self.cur_net {
                    n.returns.push(ReturnMarker {
                        id,
                        pos: Pos::default(),
                    });
                    self.ann = Some(AnnTarget::Return(n.returns.len() - 1));
                }
            }
            "addData" => self.in_adddata = true,
            "task" if self.in_adddata => {
                self.cur_task = Some(Task {
                    name: Self::attr(e, "name").unwrap_or_default(),
                    period_ns: Self::attr(e, "period_ns")
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(0),
                    programs: vec![],
                    priority: Self::attr(e, "priority")
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(0),
                });
            }
            "program" if self.in_adddata => {
                if let (Some(t), Some(r)) = (&mut self.cur_task, Self::attr(e, "ref")) {
                    t.programs.push(r);
                }
            }
            "dataType" => {
                if let Some(n) = Self::attr(e, "name") {
                    self.dtypes.push(DataTypeDecl {
                        name: n,
                        ty: serde_json::Value::Null,
                    });
                }
            }
            // known-but-unsupported body languages: note once
            "SFC" | "LD" | "ST" if self.depth_tag("body") => {
                self.notes.push(XmlNote {
                    what: format!("body language <{tag}> not imported (FBD only)"),
                });
            }
            _ => {}
        }
        self.stack.push(tag);
    }

    fn end(&mut self, name: &[u8]) {
        let tag = Self::local(name);
        match tag {
            "pou" => {
                if let Some(mut pou) = self.cur_pou.take() {
                    if pou.id.is_empty() {
                        pou.id = format!("pou_{}", pou.name.to_lowercase());
                    }
                    if let Some(net) = self.cur_net.take() {
                        pou.networks.push(net);
                    }
                    self.merge_wires(&mut pou);
                    dedupe_instance_decls(&mut pou);
                    self.pous.push(pou);
                }
            }
            "network" | "FBD" => {
                // tether:network closes a segment; flat FBD keeps its
                // implicit net open until pou end
                if tag == "network" {
                    if let Some(net) = self.cur_net.take() {
                        if let Some(pou) = &mut self.cur_pou {
                            pou.networks.push(net);
                        }
                    }
                }
            }
            "block" | "inVariable" | "outVariable" | "inOutVariable" => {
                if let Some(b) = self.cur_block.take() {
                    if let Some(n) = &mut self.cur_net {
                        n.blocks.push(b);
                    }
                }
            }
            "connectionPointIn" => {
                self.in_cpi = false;
                if !self.cpi_has_conn && !self.cpi_expr.is_empty() {
                    if let (Some(b), Some(pin)) = (&mut self.cur_block, &self.cur_pin) {
                        b.params
                            .insert(pin.clone(), self.cpi_expr.trim().to_string());
                    }
                }
                self.cur_pin = None;
            }
            "expression" => {
                self.in_expr = false;
                let t = self.expr_text.trim().to_string();
                if self.in_cpi {
                    self.cpi_expr = t;
                } else if let Some(b) = &mut self.cur_block {
                    if b.kind == BlockKind::Variable && !t.is_empty() {
                        // %I/%Q expressions are direct addresses;
                        // anything else is the variable name
                        if t.starts_with('%') {
                            if b.at.is_none() {
                                b.at = Some(t);
                            }
                        } else if b.name.is_none() {
                            b.name = Some(t);
                        }
                    }
                }
            }
            "variable" => {
                if let Some(v) = self.pending_var.take() {
                    if let Some(pou) = &mut self.cur_pou {
                        pou.interface.push(v);
                    } else {
                        self.gvl.push(v);
                    }
                }
            }
            "comment" => {
                // strict-export marker: `--- net <id> ---` closes the
                // current segment and opens the named one
                let mut marker: Option<String> = None;
                if let Some(n) = &mut self.cur_net {
                    if let Some(c) = n.comments.last_mut() {
                        if c.text.is_empty() {
                            c.text = self.comment_text.trim().to_string();
                        }
                        if let Some(id) = marker_net(&c.text) {
                            marker = Some(id.to_string());
                        }
                    }
                    if marker.is_some() {
                        n.comments.pop();
                    }
                }
                if let Some(id) = marker {
                    if let Some(net) = self.cur_net.take() {
                        if !net_is_empty(&net) {
                            if let Some(pou) = &mut self.cur_pou {
                                pou.networks.push(net);
                            }
                        }
                    }
                    self.cur_net = Some(self.new_net(id));
                }
            }
            "addData" => self.in_adddata = false,
            "task" => {
                if let Some(t) = self.cur_task.take() {
                    self.tasks.push(t);
                }
            }
            _ => {}
        }
        if let Some(pos) = self.stack.iter().rposition(|t| *t == tag) {
            self.stack.remove(pos);
        }
    }

    fn text(&mut self, e: &quick_xml::events::BytesText) {
        if let Ok(t) = e.xml_content() {
            if self.in_expr {
                self.expr_text.push_str(&t);
            }
            if self.depth_tag("content") || self.depth_tag("xhtml") {
                self.comment_text.push_str(&t);
            }
        }
    }

    fn cdata(&mut self, _e: &quick_xml::events::BytesCData) {}

    /// merge dst-side connections into wires: roundtrip files carry
    /// `tether:wire` ids (exact merge key); foreign files merge by
    /// source endpoint (fan-out). Negated dsts mark the block pin.
    fn merge_wires(&mut self, pou: &mut Pou) {
        let mut wires: Vec<Wire> = Vec::new();
        let mut key_of: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        for c in self.conns.drain(..) {
            let key = match c.wire_id {
                Some(id) => format!("w{id}"),
                None => format!("s{}:{:?}", c.src.block, c.src.port),
            };
            let idx = *key_of.entry(key).or_insert_with(|| {
                wires.push(Wire {
                    id: c.wire_id.unwrap_or_else(|| self.alloc.alloc()),
                    src: c.src.clone(),
                    dsts: vec![],
                    probe: c.probe,
                });
                wires.len() - 1
            });
            let neg_idx = match (&c.dst.port, c.negated) {
                (PortSel::Index(i), true) => Some(*i),
                _ => None,
            };
            let dst_block = c.dst.block;
            wires[idx].dsts.push(c.dst);
            if c.probe {
                wires[idx].probe = true;
            }
            if let Some(pin_i) = neg_idx {
                if let Some(net) = pou
                    .networks
                    .iter_mut()
                    .find(|n| n.blocks.iter().any(|b| b.id == dst_block))
                {
                    if let Some(b) = net.blocks.iter_mut().find(|b| b.id == dst_block) {
                        if !b.negated.contains(&pin_i) {
                            b.negated.push(pin_i);
                        }
                    }
                }
            }
        }
        for w in wires {
            if let Some(net) = pou
                .networks
                .iter_mut()
                .find(|n| n.blocks.iter().any(|b| b.id == w.src.block))
            {
                net.wires.push(w);
            } else if let Some(net) = pou.networks.last_mut() {
                net.wires.push(w);
            }
        }
    }

    fn finish(self) -> Result<ImportReport, ImportError> {
        if self.pous.is_empty() && self.gvl.is_empty() {
            return Err(ImportError::NotProject);
        }
        Ok(ImportReport {
            project: FbdProject {
                schema: SCHEMA_ID.into(),
                project: if self.project_name.is_empty() {
                    "imported".into()
                } else {
                    self.project_name
                },
                pous: self.pous,
                tasks: self.tasks,
                data_types: self.dtypes,
                global_vars: self.gvl,
            },
            notes: self.notes,
        })
    }
}

/// TC6 carries FB instances as localVars (`<variable name type>`).
/// Drop the decl when it exactly mirrors a block's (instance, type)
/// pair and has no extra data — the block already owns it. Foreign
/// decls (no matching instance, or AT/init/constant extras) stay.
fn dedupe_instance_decls(pou: &mut Pou) {
    let insts: Vec<(&str, &str)> = pou
        .networks
        .iter()
        .flat_map(|n| n.blocks.iter())
        .filter(|b| b.kind == BlockKind::Fb)
        .filter_map(|b| b.instance.as_deref().map(|i| (i, b.type_ref.as_str())))
        .collect();
    pou.interface.retain(|v| {
        !(v.kind == IfaceKind::Var
            && v.at.is_none()
            && v.init.is_none()
            && !v.retain
            && insts.contains(&(v.name.as_str(), v.ty.as_str())))
    });
}

fn net_is_empty(n: &Network) -> bool {
    n.blocks.is_empty()
        && n.wires.is_empty()
        && n.comments.is_empty()
        && n.labels.is_empty()
        && n.jumps.is_empty()
        && n.returns.is_empty()
        && n.assocs.is_empty()
}

/// `--- net <id> ---` strict-export marker → network id.
fn marker_net(text: &str) -> Option<&str> {
    text.trim().strip_prefix("--- net ")?.strip_suffix(" ---")
}

/// `IN3` → index 3 (our indexed-pin export convention); a bare `IN`
/// is a real FB pin name, not an index.
fn pin_index(pin: &str) -> Option<u32> {
    pin.strip_prefix("IN")?.parse().ok()
}
