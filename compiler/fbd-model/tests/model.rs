use fbd_model::*;

/// The doc-06 Appendix D example, as a literal JSON doc.
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
        "params": {"PT": "T#200ms"}, "exec_order": 0,
        "comment": ""
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

#[test]
fn round_trip_lossless() {
    let p = from_json(FILL_STATION).expect("parse");
    assert_eq!(p.project, "fill_station");
    let net = &p.pous[0].networks[0];
    assert_eq!(net.blocks.len(), 4);
    assert_eq!(net.wires.len(), 4);
    assert!(net.wires[1].probe);
    assert_eq!(
        net.wires[0].src,
        PortRef {
            block: 13,
            port: PortSel::Index(0)
        }
    );
    assert_eq!(
        net.wires[1].src,
        PortRef {
            block: 12,
            port: PortSel::Name("Q".into())
        }
    );
    assert_eq!(net.blocks[0].params["PT"], "T#200ms");
    assert_eq!(net.comments[0].text, "debounce");
    assert_eq!(net.jumps[0].target, "L1");

    // serialize → parse → serialize: stable both ways
    let c1 = to_canonical_json(&p);
    let p2 = from_json(&c1).expect("re-parse canonical");
    let c2 = to_canonical_json(&p2);
    assert_eq!(c1, c2, "canonical form must be byte-stable");
}

#[test]
fn canonical_byte_stability() {
    // two parses of the same doc serialize identically; field order
    // in output is declaration order regardless of input order
    let scrambled = FILL_STATION.replace(
        "\"id\": \"pou_fill\", \"name\": \"FILL_CTL\", \"kind\": \"program\"",
        "\"kind\": \"program\", \"id\": \"pou_fill\", \"name\": \"FILL_CTL\"",
    );
    let a = to_canonical_json(&from_json(FILL_STATION).unwrap());
    let b = to_canonical_json(&from_json(&scrambled).unwrap());
    assert_eq!(a, b);
    assert!(a.ends_with("}\n"));
}

#[test]
fn forward_compat_unknown_keys() {
    let with_extra = FILL_STATION.replace(
        "\"kind\": \"fbd\"",
        "\"kind\": \"fbd\", \"future_field\": [1,2,3]",
    );
    let p = from_json(&with_extra).expect("unknown keys must be ignored");
    assert_eq!(p.pous[0].networks[0].kind, NetworkKind::Fbd);
}

#[test]
fn schema_version_enforced() {
    let bad = FILL_STATION.replace("fbd.v1", "fbd.v99");
    match from_json(&bad) {
        Err(ModelError::SchemaVersion(s)) => assert_eq!(s, "fbd.v99"),
        other => panic!("want SchemaVersion, got {other:?}"),
    }
}

#[test]
fn duplicate_node_id_rejected() {
    // blocks 14 and 15 share id 14
    let dup = FILL_STATION.replace(
        "\"id\": 15, \"kind\": \"variable\", \"role\": \"out\"",
        "\"id\": 14, \"kind\": \"variable\", \"role\": \"out\"",
    );
    match from_json(&dup) {
        Err(ModelError::DupNodeId(14)) => {}
        other => panic!("want DupNodeId(14), got {other:?}"),
    }
}

#[test]
fn id_alloc_monotonic_over_loaded() {
    let p = from_json(FILL_STATION).unwrap();
    let mut a = IdAlloc::new();
    a.over(p.pous[0].networks[0].node_ids());
    // max id in doc is 61 → next alloc must be ≥62
    let n1 = a.alloc();
    assert!(n1 >= 62, "got {n1}");
    assert!(a.alloc() > n1, "monotonic");
}
