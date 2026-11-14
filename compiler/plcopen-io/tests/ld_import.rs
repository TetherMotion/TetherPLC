//! PLCopen `<LD>` body import — contacts/coils become variable
//! blocks, `connectionPointIn` links become wires, power-rail
//! sources are dropped so fed elements evaluate rail-powered.

use fbd_model::{NetworkKind, VarRole};
use plcopen_io::import;

/// (start OR hold) AND NOT stop -> run — same topology as
/// tests/ld/03_parallel so the wire graph must match it.
fn ld_xml() -> String {
    std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/ld_basic.xml"),
    )
    .expect("ld_basic.xml fixture missing")
}

#[test]
fn ld_body_imports_as_ladder_network() {
    let rep = import(&ld_xml()).expect("import failed");
    assert_eq!(rep.project.pous.len(), 1);
    let pou = &rep.project.pous[0];
    assert_eq!(pou.networks.len(), 1, "one LD body -> one network");
    let net = &pou.networks[0];
    assert_eq!(net.kind, NetworkKind::Ladder);

    // 3 contacts + 1 coil; rails emit no blocks
    assert_eq!(net.blocks.len(), 4);
    let contacts: Vec<_> = net
        .blocks
        .iter()
        .filter(|b| b.role == Some(VarRole::In))
        .collect();
    let coils: Vec<_> = net
        .blocks
        .iter()
        .filter(|b| b.role == Some(VarRole::Out))
        .collect();
    assert_eq!(contacts.len(), 3);
    assert_eq!(coils.len(), 1);

    // % addresses land on `at`; NC contact is negated
    let ats: Vec<_> = net.blocks.iter().map(|b| b.at.as_deref()).collect();
    for a in ["%IX0.0", "%IX0.1", "%IX0.2", "%QX0.0"] {
        assert!(ats.contains(&Some(a)), "missing at {a}");
    }
    let nc = net
        .blocks
        .iter()
        .find(|b| b.at.as_deref() == Some("%IX0.2"))
        .unwrap();
    assert!(!nc.negated.is_empty(), "NC contact lost negated");
    let coil = coils[0];
    assert!(coil.negated.is_empty());

    // positions survive (rung rows drive band clustering)
    let ys: Vec<u32> = net.blocks.iter().map(|b| b.pos.y).collect();
    assert!(ys.contains(&30) && ys.contains(&70));
}

#[test]
fn ld_wires_merge_branches_and_drop_rail_sources() {
    let rep = import(&ld_xml()).expect("import failed");
    let net = &rep.project.pous[0].networks[0];

    // rail-sourced connections (localId 1) must not appear as wires
    assert!(
        net.wires.iter().all(|w| w.src.block != 1),
        "rail source leaked into wires: {:?}",
        net.wires
    );

    // expected flow graph: 3->5, 4->5 (parallel merge at NC contact),
    // 5->6 (series to coil)
    let edges: Vec<(u64, Vec<u64>)> = net
        .wires
        .iter()
        .map(|w| (w.src.block, w.dsts.iter().map(|d| d.block).collect()))
        .collect();
    assert_eq!(edges.len(), 3, "wires: {edges:?}");
    for (src, dst) in [(3u64, 5u64), (4, 5), (5, 6)] {
        assert!(
            edges.iter().any(|(s, ds)| *s == src && ds.contains(&dst)),
            "missing edge {src}->{dst} in {edges:?}"
        );
    }
}

#[test]
fn ld_roundtrips_through_export() {
    let rep = import(&ld_xml()).expect("import failed");
    let (xml, losses) =
        plcopen_io::export(&rep.project, plcopen_io::ExportMode::Roundtrip);
    assert!(
        losses.is_empty(),
        "export losses: {:?}",
        losses.iter().map(|l| &l.what).collect::<Vec<_>>()
    );
    let rep2 = plcopen_io::import(&xml).expect("reimport failed");
    let n1 = &rep.project.pous[0].networks[0];
    let n2 = &rep2.project.pous[0].networks[0];
    assert_eq!(n1.kind, n2.kind);
    assert_eq!(n1.blocks.len(), n2.blocks.len());
    assert_eq!(n1.wires.len(), n2.wires.len());
    for (a, b) in n1.blocks.iter().zip(n2.blocks.iter()) {
        assert_eq!(a.at, b.at);
        assert_eq!(a.role, b.role);
        assert_eq!(a.negated.is_empty(), b.negated.is_empty());
    }
    let edges = |n: &fbd_model::Network| -> Vec<(u64, Vec<u64>)> {
        n.wires
            .iter()
            .map(|w| (w.src.block, w.dsts.iter().map(|d| d.block).collect()))
            .collect()
    };
    assert_eq!(edges(n1), edges(n2));
}

#[test]
fn ld_storage_edge_attrs_surface_notes() {
    let xml = ld_xml().replace(
        "storage=\"none\" edge=\"none\"",
        "storage=\"set\" edge=\"rising\"",
    );
    let rep = import(&xml).expect("import failed");
    assert!(
        rep.notes.iter().any(|n| n.what.contains("storage")),
        "expected storage note, got {:?}",
        rep.notes.iter().map(|n| &n.what).collect::<Vec<_>>()
    );
}
