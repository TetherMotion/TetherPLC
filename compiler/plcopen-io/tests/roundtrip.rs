use plcopen_io::*;

/// same doc as the fbd-model test fixture — the doc-06 Appendix D
/// example exercising every mapped construct.
const FILL_STATION: &str = r##"{
  "schema": "fbd.v1",
  "project": "fill_station",
  "pous": [{
    "id": "pou_fill", "name": "FILL_CTL", "kind": "program",
    "interface": [
      {"name":"speed","kind":"var_input","type":"INT","init":"100"},
      {"name":"done","kind":"var_output","type":"BOOL"}
    ],
    "networks": [{
      "id": "net_1", "kind": "fbd", "label": "start logic",
      "blocks": [{
        "id": 12, "kind": "fb", "type_ref": "TON",
        "instance": "T_Start", "pos": {"x": 8, "y": 4},
        "params": {"PT": "T#200ms"}, "exec_order": 0
      },{
        "id": 13, "kind": "variable", "role": "in",
        "name": "StartBtn", "at": "%IX0.0", "pos": {"x": 1, "y": 4}
      },{
        "id": 14, "kind": "operator", "type_ref": "AND",
        "pos": {"x": 20, "y": 6}
      },{
        "id": 15, "kind": "variable", "role": "out",
        "name": "MotorRun", "at": "%QX0.0", "pos": {"x": 30, "y": 6}
      }],
      "wires": [
        {"id": 40, "src": {"block": 13, "port": 0},
         "dsts": [{"block": 12, "port": 0}]},
        {"id": 41, "src": {"block": 12, "port": "Q"},
         "dsts": [{"block": 14, "port": 0}], "probe": true},
        {"id": 42, "src": {"block": 14, "port": 0},
         "dsts": [{"block": 15, "port": 0}]},
        {"id": 43, "src": {"block": 99, "port": 0},
         "dsts": [{"block": 14, "port": 1}]}
      ],
      "comments": [{"id":50,"pos":{"x":2,"y":0},"text":"debounce"}],
      "labels":   [{"id":60,"pos":{"x":25,"y":2},"name":"L1"}],
      "jumps":    [{"id":61,"pos":{"x":26,"y":8},"target":"L1"}]
    }]
  }],
  "tasks": [{"name":"MAIN","period_ns":1000000,"programs":["pou_fill"]}]
}"##;

fn model() -> fbd_model::FbdProject {
    fbd_model::from_json(FILL_STATION).unwrap()
}

#[test]
fn roundtrip_lossless() {
    let p = model();
    let (xml, losses) = export(&p, ExportMode::Roundtrip);
    assert!(
        losses.is_empty(),
        "roundtrip export must be lossless: {losses:?}"
    );
    eprintln!("{xml}");

    let rep = import(&xml).expect("reimport");
    let q = rep.project;
    assert_eq!(q.pous.len(), 1);
    let pou = &q.pous[0];
    assert_eq!(pou.name, "FILL_CTL");
    assert_eq!(pou.kind, fbd_model::PouKind::Program);
    assert_eq!(pou.interface.len(), 2);
    assert_eq!(pou.interface[0].name, "speed");
    assert_eq!(pou.interface[0].init.as_deref(), Some("100"));

    assert_eq!(pou.networks.len(), 1);
    let net = &pou.networks[0];
    assert_eq!(net.id, "net_1");
    assert_eq!(net.label, "start logic");
    assert_eq!(net.blocks.len(), 4);
    assert_eq!(net.wires.len(), 4);
    assert_eq!(net.comments.len(), 1);
    assert_eq!(net.labels.len(), 1);
    assert_eq!(net.jumps.len(), 1);

    let ton = net.blocks.iter().find(|b| b.id == 12).unwrap();
    assert_eq!(ton.kind, fbd_model::BlockKind::Fb);
    assert_eq!(ton.instance.as_deref(), Some("T_Start"));
    assert_eq!(ton.params["PT"], "T#200ms");
    assert_eq!((ton.pos.x, ton.pos.y), (8, 4));

    let probe_wire = net.wires.iter().find(|w| w.probe).unwrap();
    assert_eq!(probe_wire.src.block, 12);
    assert_eq!(probe_wire.src.port, fbd_model::PortSel::Name("Q".into()));
    assert_eq!(probe_wire.id, 41, "tether:wire preserves wire id");

    let start = net.blocks.iter().find(|b| b.id == 13).unwrap();
    assert_eq!(start.at.as_deref(), Some("%IX0.0"));
    assert_eq!(start.name.as_deref(), Some("StartBtn"));

    assert_eq!(q.tasks.len(), 1);
    assert_eq!(q.tasks[0].name, "MAIN");
    assert_eq!(q.tasks[0].programs, vec!["pou_fill".to_string()]);
}

#[test]
fn strict_export_degrades_reported() {
    let p = model();
    let (xml, losses) = export(&p, ExportMode::Strict);
    eprintln!("{xml}");
    assert!(!xml.contains("tether:"), "strict must not emit vendor ns");
    assert!(xml.contains("--- net net_1 ---"), "flatten marker");
    assert!(!losses.is_empty());
    assert!(losses.iter().any(|l| l.what.contains("network")));
    assert!(losses.iter().any(|l| l.what.contains("tasks")));
}

#[test]
fn strict_reimport_recovers_structure() {
    let p = model();
    let (xml, _) = export(&p, ExportMode::Strict);
    let rep = import(&xml).expect("reimport strict");
    let net = &rep.project.pous[0].networks[0];
    // blocks/wires/comments survive; probe flags and wire ids are
    // dropped by strict mode (wire id regenerated)
    assert_eq!(net.blocks.len(), 4);
    assert_eq!(net.wires.len(), 4);
    assert!(!net.wires.iter().any(|w| w.probe));
    // tasks dropped in strict
    assert!(rep.project.tasks.is_empty());
}

#[test]
fn beremiz_style_flat_fbd_imports() {
    // a foreign-tool-shaped doc: no tether ns, flat FBD, expressions
    // on variable pins
    let xml = r##"<?xml version="1.0"?>
<project xmlns="http://www.plcopen.org/xml/tc6_0201">
  <types><pous>
    <pou name="BLINK" pouType="program">
      <interface>
        <localVars><variable name="ton0" type="TON"/></localVars>
      </interface>
      <body><FBD>
        <inVariable localId="1">
          <position x="10" y="20"/>
          <connectionPointOut><expression>%IX0.0</expression></connectionPointOut>
        </inVariable>
        <block localId="2" typeName="TON" instanceName="ton0">
          <position x="100" y="20"/>
          <inputVariables>
            <variable formalParameter="IN">
              <connectionPointIn>
                <connection refLocalId="1"/>
              </connectionPointIn>
            </variable>
            <variable formalParameter="PT">
              <connectionPointIn>
                <expression>T#500ms</expression>
              </connectionPointIn>
            </variable>
          </inputVariables>
          <outputVariables>
            <variable formalParameter="Q"><connectionPointOut/></variable>
          </outputVariables>
        </block>
        <outVariable localId="3">
          <position x="200" y="20"/>
          <connectionPointIn>
            <connection refLocalId="2" formalParameter="Q"/>
          </connectionPointIn>
          <expression>%QX0.0</expression>
        </outVariable>
      </FBD></body>
    </pou>
  </pous></types>
</project>"##;
    let rep = import(xml).expect("foreign import");
    let pou = &rep.project.pous[0];
    assert_eq!(pou.name, "BLINK");
    // the ton0 localVar decl folds into the block's instance field —
    // interface holds only what blocks don't own
    assert_eq!(pou.interface.len(), 0);
    let net = &pou.networks[0];
    assert_eq!(net.blocks.len(), 3);
    let ton = net
        .blocks
        .iter()
        .find(|b| b.instance.as_deref() == Some("ton0"))
        .unwrap();
    assert_eq!(ton.params["PT"], "T#500ms");
    assert_eq!(net.wires.len(), 2);
    let w_in = net.wires.iter().find(|w| w.src.block == 1).unwrap();
    assert_eq!(w_in.dsts[0].block, 2);
    let w_q = net.wires.iter().find(|w| w.src.block == 2).unwrap();
    assert_eq!(w_q.dsts[0].block, 3);
    let outv = net.blocks.iter().find(|b| b.id == 3).unwrap();
    assert_eq!(outv.at.as_deref(), Some("%QX0.0"));
}
