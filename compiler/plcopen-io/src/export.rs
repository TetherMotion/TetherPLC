use fbd_model::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportMode {
    /// TC6-only output for foreign tools; `tether:` constructs are
    /// degraded per Appendix E and each loss is reported.
    Strict,
    /// TC6 + `tether:` extensions; importing our own roundtrip file is
    /// lossless.
    Roundtrip,
}

/// A construct that strict export had to degrade or drop.
#[derive(Debug)]
pub struct LossNote {
    pub node: Option<NodeId>,
    pub what: &'static str,
}

const TC6_NS: &str = "http://www.plcopen.org/xml/tc6_0201";
const TETHER_NS: &str = "http://tether.dev/plc/tc6ext";

type Attrs<'a> = [(&'a str, String)];

pub fn export(p: &FbdProject, mode: ExportMode) -> (String, Vec<LossNote>) {
    let mut x = Xml::new();
    let mut losses = Vec::new();
    x.raw("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n");
    let mut ns = vec![("xmlns", TC6_NS.to_string())];
    if mode == ExportMode::Roundtrip {
        ns.push(("xmlns:tether", TETHER_NS.to_string()));
    }
    x.open("project", &ns);
    x.empty(
        "fileHeader",
        &[
            ("companyName", "Tether".into()),
            ("productName", "tplc".into()),
        ],
    );
    x.open("contentHeader", &[("name", p.project.clone())]);
    x.open("coordinateInfo", &[]);
    for tag in ["fbd", "ld", "sfc"] {
        x.open(tag, &[]);
        x.empty("scaling", &[("x", "1".into()), ("y", "1".into())]);
        x.close(tag);
    }
    x.close("coordinateInfo");
    x.close("contentHeader");

    x.open("types", &[]);
    x.open("dataTypes", &[]);
    for dt in &p.data_types {
        x.open("dataType", &[("name", dt.name.clone())]);
        if let Some(b) = dt.ty.get("base").and_then(|v| v.as_str()) {
            x.open("baseType", &[]);
            x.empty("derived", &[("name", b.into())]);
            x.close("baseType");
        }
        x.close("dataType");
    }
    x.close("dataTypes");

    x.open("pous", &[]);
    for pou in &p.pous {
        export_pou(&mut x, pou, mode, &mut losses);
    }
    x.close("pous");

    if !p.global_vars.is_empty() {
        x.open("globalVars", &[]);
        for v in &p.global_vars {
            export_iface_var(&mut x, v);
        }
        x.close("globalVars");
    }
    x.close("types");

    if mode == ExportMode::Roundtrip && !p.tasks.is_empty() {
        x.open("addData", &[]);
        x.open(
            "data",
            &[
                ("name", "tether:tasks".into()),
                ("handleUnknown", "implementation".into()),
            ],
        );
        x.open("content", &[]);
        for t in &p.tasks {
            x.open(
                "tether:task",
                &[
                    ("name", t.name.clone()),
                    ("period_ns", t.period_ns.to_string()),
                    ("priority", t.priority.to_string()),
                ],
            );
            for prog in &t.programs {
                x.empty("tether:program", &[("ref", prog.clone())]);
            }
            x.close("tether:task");
        }
        x.close("content");
        x.close("data");
        x.close("addData");
    } else if !p.tasks.is_empty() {
        losses.push(LossNote {
            node: None,
            what: "tasks dropped (strict)",
        });
    }

    x.close("project");
    (x.out, losses)
}

fn export_pou(x: &mut Xml, pou: &Pou, mode: ExportMode, losses: &mut Vec<LossNote>) {
    let kind = match pou.kind {
        PouKind::Program => "program",
        PouKind::FunctionBlock => "functionBlock",
        PouKind::Function => "function",
    };
    x.open(
        "pou",
        &[("name", pou.name.clone()), ("pouType", kind.into())],
    );

    x.open("interface", &[]);
    // FB instances that are not already interface vars are declared as
    // localVars (Appendix E)
    let declared: std::collections::HashSet<&str> =
        pou.interface.iter().map(|v| v.name.as_str()).collect();
    for (tag, kind) in [
        ("inputVars", IfaceKind::VarInput),
        ("outputVars", IfaceKind::VarOutput),
        ("localVars", IfaceKind::Var),
        ("externalVars", IfaceKind::VarExternal),
        ("inOutVars", IfaceKind::VarInOut),
    ] {
        let vars: Vec<_> = pou.interface.iter().filter(|v| v.kind == kind).collect();
        let mut inst_vars: Vec<(&str, &str)> = Vec::new();
        if kind == IfaceKind::Var {
            for net in &pou.networks {
                for b in &net.blocks {
                    if b.kind == BlockKind::Fb {
                        if let Some(inst) = &b.instance {
                            if !declared.contains(inst.as_str()) {
                                inst_vars.push((inst, &b.type_ref));
                            }
                        }
                    }
                }
            }
            inst_vars.sort();
            inst_vars.dedup();
        }
        if vars.is_empty() && inst_vars.is_empty() {
            continue;
        }
        x.open(tag, &[]);
        for v in vars {
            export_iface_var(x, v);
        }
        for (name, ty) in inst_vars {
            x.empty("variable", &[("name", name.into()), ("type", ty.into())]);
        }
        x.close(tag);
    }
    x.close("interface");

    x.open("body", &[]);
    x.open("FBD", &[]);
    for net in &pou.networks {
        match mode {
            ExportMode::Roundtrip => {
                let mut a = vec![("id", net.id.clone()), ("kind", net_kind(net.kind).into())];
                if !net.label.is_empty() {
                    a.push(("label", net.label.clone()));
                }
                x.open("tether:network", &a);
            }
            ExportMode::Strict => {
                // flatten; a comment marker keeps the boundary readable
                x.open("comment", &[("localId", marker_id(&net.id).to_string())]);
                x.empty("position", &[("x", "0".into()), ("y", "0".into())]);
                x.open("content", &[]);
                x.open("xhtml", &[("xmlns", "http://www.w3.org/1999/xhtml".into())]);
                x.text(&format!("--- net {} ---", esc(&net.id)));
                x.close("xhtml");
                x.close("content");
                x.close("comment");
                losses.push(LossNote {
                    node: None,
                    what: "network flattened to comment marker",
                });
            }
        }
        export_network(x, net, mode, losses);
        if mode == ExportMode::Roundtrip {
            x.close("tether:network");
        }
    }
    x.close("FBD");
    x.close("body");
    x.close("pou");
}

/// deterministic localId for strict-mode net markers, in a vendor band
/// that cannot collide with model u32 ids.
fn marker_id(net_id: &str) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in net_id.bytes() {
        h = (h ^ b as u64).wrapping_mul(0x100000001b3);
    }
    (h % 900_000_000) + 1_000_000_000
}

fn net_kind(k: NetworkKind) -> &'static str {
    match k {
        NetworkKind::Fbd => "fbd",
        NetworkKind::Ladder => "ladder",
    }
}

fn export_iface_var(x: &mut Xml, v: &IfaceVar) {
    let mut a = vec![("name", v.name.clone()), ("type", v.ty.clone())];
    if let Some(at) = &v.at {
        a.push(("address", at.clone()));
    }
    match &v.init {
        None => x.empty("variable", &a),
        Some(init) => {
            x.open("variable", &a);
            x.open("initialValue", &[]);
            x.empty("simpleValue", &[("value", init.clone())]);
            x.close("initialValue");
            x.close("variable");
        }
    }
}

/// dst port key for the by-dst wire index: pin name or `#index`.
fn port_key(p: &PortSel) -> String {
    match p {
        PortSel::Name(n) => n.clone(),
        PortSel::Index(i) => format!("#{i}"),
    }
}

type WireIndex<'a> = std::collections::HashMap<(NodeId, String), Vec<&'a Wire>>;

fn export_network(x: &mut Xml, net: &Network, mode: ExportMode, losses: &mut Vec<LossNote>) {
    let mut by_dst: WireIndex = std::collections::HashMap::new();
    for w in &net.wires {
        for d in &w.dsts {
            by_dst
                .entry((d.block, port_key(&d.port)))
                .or_default()
                .push(w);
        }
    }
    // variable-role blocks are dst ends; index "#0" for their single pin
    for w in &net.wires {
        for d in &w.dsts {
            if net
                .blocks
                .iter()
                .any(|b| b.id == d.block && b.kind == BlockKind::Variable)
            {
                by_dst.entry((d.block, String::new())).or_default().push(w);
            }
        }
    }

    for b in &net.blocks {
        export_block(x, net, b, &by_dst, mode, losses);
    }
    for c in &net.comments {
        x.open("comment", &[("localId", c.id.to_string())]);
        x.empty(
            "position",
            &[("x", c.pos.x.to_string()), ("y", c.pos.y.to_string())],
        );
        x.open("content", &[]);
        x.open("xhtml", &[("xmlns", "http://www.w3.org/1999/xhtml".into())]);
        x.text(&esc(&c.text));
        x.close("xhtml");
        x.close("content");
        x.close("comment");
    }
    for l in &net.labels {
        x.open(
            "label",
            &[("localId", l.id.to_string()), ("label", l.name.clone())],
        );
        x.empty(
            "position",
            &[("x", l.pos.x.to_string()), ("y", l.pos.y.to_string())],
        );
        x.close("label");
    }
    for j in &net.jumps {
        x.open(
            "jump",
            &[("localId", j.id.to_string()), ("label", j.target.clone())],
        );
        x.empty(
            "position",
            &[("x", j.pos.x.to_string()), ("y", j.pos.y.to_string())],
        );
        x.close("jump");
    }
    for r in &net.returns {
        x.open("return", &[("localId", r.id.to_string())]);
        x.empty(
            "position",
            &[("x", r.pos.x.to_string()), ("y", r.pos.y.to_string())],
        );
        x.close("return");
    }
}

fn export_block(
    x: &mut Xml,
    net: &Network,
    b: &Block,
    by_dst: &WireIndex,
    mode: ExportMode,
    losses: &mut Vec<LossNote>,
) {
    match b.kind {
        BlockKind::Variable => export_var_block(x, b, by_dst, mode, losses),
        BlockKind::JumpTarget => {
            if let Some(n) = &b.name {
                x.open(
                    "label",
                    &[("localId", b.id.to_string()), ("label", n.clone())],
                );
                x.empty(
                    "position",
                    &[("x", b.pos.x.to_string()), ("y", b.pos.y.to_string())],
                );
                x.close("label");
            }
        }
        _ => export_call_block(x, net, b, by_dst, mode, losses),
    }
}

fn export_var_block(
    x: &mut Xml,
    b: &Block,
    by_dst: &WireIndex,
    mode: ExportMode,
    losses: &mut Vec<LossNote>,
) {
    let role = b.role.unwrap_or(VarRole::In);
    let tag = match role {
        VarRole::In => "inVariable",
        VarRole::Out => "outVariable",
        VarRole::InOut => "inOutVariable",
    };
    let name_expr =
        b.at.clone()
            .unwrap_or_else(|| b.name.clone().unwrap_or_default());
    let mut a = vec![("localId", b.id.to_string())];
    if !b.negated.is_empty() {
        a.push(("negated", "true".into()));
    }
    if mode == ExportMode::Roundtrip {
        if b.at.is_some() {
            if let Some(n) = &b.name {
                a.push(("tether:name", n.clone()));
            }
        }
    } else if b.at.is_some() && b.name.is_some() {
        losses.push(LossNote {
            node: Some(b.id),
            what: "friendly name dropped (strict uses %address)",
        });
    }
    x.open(tag, &a);
    x.empty(
        "position",
        &[("x", b.pos.x.to_string()), ("y", b.pos.y.to_string())],
    );
    // connectionPointIn carries incoming wires for any role — LD
    // contacts are role=in yet receive flow from upstream elements,
    // so emit whenever the dst index has entries (FBD inputs stay
    // empty and unchanged)
    let has_in = by_dst.contains_key(&(b.id, "#0".to_string()))
        || by_dst.contains_key(&(b.id, String::new()));
    if role != VarRole::In || has_in {
        x.open("connectionPointIn", &[]);
        // by_dst indexes variable dsts under both "#0" and "" — emit
        // each wire once or reimport sees duplicate connections
        let mut emitted: std::collections::HashSet<*const Wire> =
            std::collections::HashSet::new();
        for key in ["#0".to_string(), String::new()] {
            if let Some(ws) = by_dst.get(&(b.id, key)) {
                for w in ws {
                    if emitted.insert(*w) {
                        conn_elem(x, &w.src, wire_attr(w, mode));
                    }
                }
            }
        }
        x.close("connectionPointIn");
    }
    if role != VarRole::Out {
        x.open("connectionPointOut", &[]);
        x.open("expression", &[]);
        x.text(&esc(&name_expr));
        x.close("expression");
        x.close("connectionPointOut");
    }
    if role == VarRole::Out {
        x.open("expression", &[]);
        x.text(&esc(&name_expr));
        x.close("expression");
    }
    x.close(tag);
}

fn export_call_block(
    x: &mut Xml,
    net: &Network,
    b: &Block,
    by_dst: &WireIndex,
    mode: ExportMode,
    losses: &mut Vec<LossNote>,
) {
    let mut a = vec![
        ("localId", b.id.to_string()),
        ("typeName", b.type_ref.clone()),
    ];
    if let Some(inst) = &b.instance {
        a.push(("instanceName", inst.clone()));
    }
    if b.exec_order != 0 {
        a.push(("executionOrderId", b.exec_order.to_string()));
    }
    x.open("block", &a);
    x.empty(
        "position",
        &[("x", b.pos.x.to_string()), ("y", b.pos.y.to_string())],
    );

    // input pins = union of wired dst ports + literal params
    let mut ins: Vec<String> = by_dst
        .keys()
        .filter(|k| k.0 == b.id)
        .map(|k| k.1.clone())
        .collect();
    ins.extend(b.params.keys().cloned());
    ins.sort();
    ins.dedup();

    if !ins.is_empty() {
        x.open("inputVariables", &[]);
        for pin in &ins {
            match pin.strip_prefix('#') {
                Some(idx) => {
                    // indexed pin → no formalParameter; TC6 position is
                    // element order, so emit an empty named-less var
                    x.open("variable", &[("formalParameter", format!("IN{idx}"))]);
                }
                None => {
                    x.open("variable", &[("formalParameter", pin.clone())]);
                }
            }
            let neg = pin
                .strip_prefix('#')
                .and_then(|s| s.parse::<u32>().ok())
                .map(|i| b.negated.contains(&i))
                .unwrap_or(false);
            if neg {
                x.open("connectionPointIn", &[("negated", "true".into())]);
            } else {
                x.open("connectionPointIn", &[]);
            }
            let key = (b.id, pin.clone());
            if let Some(ws) = by_dst.get(&key) {
                for w in ws {
                    conn_elem(x, &w.src, wire_attr(w, mode));
                }
            } else if let Some(lit) = b.params.get(pin) {
                x.open("expression", &[]);
                x.text(&esc(lit));
                x.close("expression");
            } else {
                losses.push(LossNote {
                    node: Some(b.id),
                    what: "unconnected input serialized empty",
                });
            }
            x.close("connectionPointIn");
            x.close("variable");
        }
        x.close("inputVariables");
    }

    // output pins present in the file = union of wire src ports
    let mut outs: Vec<String> = net
        .wires
        .iter()
        .filter(|w| w.src.block == b.id)
        .map(|w| port_key(&w.src.port))
        .collect();
    outs.sort();
    outs.dedup();
    if !outs.is_empty() {
        x.open("outputVariables", &[]);
        for pin in &outs {
            match pin.strip_prefix('#') {
                Some(_) => x.open("variable", &[]),
                None => x.open("variable", &[("formalParameter", pin.clone())]),
            }
            x.empty("connectionPointOut", &[]);
            x.close("variable");
        }
        x.close("outputVariables");
    }
    x.close("block");
}

/// roundtrip extras carried on a `<connection>`: wire id (fan-out
/// merge key on reimport) and probe flag. Both dropped in strict mode.
fn wire_attr(w: &Wire, mode: ExportMode) -> Option<(NodeId, bool)> {
    match mode {
        ExportMode::Roundtrip => Some((w.id, w.probe)),
        ExportMode::Strict => None,
    }
}

/// `<connection refLocalId="src" formalParameter="srcpin" tether:wire tether:probe/>`
fn conn_elem(x: &mut Xml, src: &PortRef, rt: Option<(NodeId, bool)>) {
    let mut a = vec![("refLocalId", src.block.to_string())];
    if let PortSel::Name(n) = &src.port {
        a.push(("formalParameter", n.clone()));
    }
    if let Some((id, probe)) = rt {
        a.push(("tether:wire", id.to_string()));
        if probe {
            a.push(("tether:probe", "true".into()));
        }
    }
    x.empty("connection", &a);
}

fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// minimal pretty XML writer — indent stack + single escape point.
struct Xml {
    out: String,
    depth: usize,
}

impl Xml {
    fn new() -> Self {
        Xml {
            out: String::new(),
            depth: 0,
        }
    }
    fn pad(&mut self) {
        for _ in 0..self.depth {
            self.out.push_str("  ");
        }
    }
    fn raw(&mut self, s: &str) {
        self.out.push_str(s);
        if !s.ends_with('\n') {
            self.out.push('\n');
        }
    }
    fn open(&mut self, tag: &str, attrs: &Attrs) {
        self.pad();
        self.out.push('<');
        self.out.push_str(tag);
        for (k, v) in attrs {
            self.out.push(' ');
            self.out.push_str(k);
            self.out.push_str("=\"");
            self.out.push_str(&esc(v));
            self.out.push('"');
        }
        self.out.push_str(">\n");
        self.depth += 1;
    }
    fn empty(&mut self, tag: &str, attrs: &Attrs) {
        self.pad();
        self.out.push('<');
        self.out.push_str(tag);
        for (k, v) in attrs {
            self.out.push(' ');
            self.out.push_str(k);
            self.out.push_str("=\"");
            self.out.push_str(&esc(v));
            self.out.push('"');
        }
        self.out.push_str("/>\n");
    }
    fn text(&mut self, s: &str) {
        self.pad();
        self.out.push_str(s);
        self.out.push('\n');
    }
    fn close(&mut self, tag: &str) {
        self.depth = self.depth.saturating_sub(1);
        self.pad();
        self.out.push_str("</");
        self.out.push_str(tag);
        self.out.push_str(">\n");
    }
}
