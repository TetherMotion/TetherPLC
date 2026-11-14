//! fbd-lower tests — the doc 06 §8 example end-to-end through the
//! mir interpreter, plus validation-code fixtures.

use fbd_lower::{codes, lower, LowerOutput};
use fbd_model::*;
use tplc_mir::interp::program_ctx;
use tplc_mir::interp::{Rt, Value};
use tplc_mir::ir::{Body, StmtKind};
use tplc_mir::layout::layout_program;

fn var_block(id: u64, role: VarRole, name: &str, at: Option<&str>, x: u32, y: u32) -> Block {
    Block {
        id,
        kind: BlockKind::Variable,
        type_ref: String::new(),
        instance: None,
        pos: Pos { x, y },
        params: Default::default(),
        pins: vec![],
        exec_order: 0,
        negated: vec![],
        comment: String::new(),
        role: Some(role),
        flow: None,
        name: Some(name.into()),
        at: at.map(|s| s.into()),
    }
}

fn neg_op_block(id: u64, op: &str, negated: &[u32], x: u32, y: u32) -> Block {
    let mut b = op_block(id, op, x, y);
    b.negated = negated.to_vec();
    b
}

fn op_block(id: u64, op: &str, x: u32, y: u32) -> Block {
    Block {
        id,
        kind: BlockKind::Operator,
        type_ref: op.into(),
        instance: None,
        pos: Pos { x, y },
        params: Default::default(),
        pins: vec![],
        exec_order: 0,
        negated: vec![],
        comment: String::new(),
        role: None,
        flow: None,
        name: None,
        at: None,
    }
}

fn fb_block(id: u64, ty: &str, inst: &str, params: &[(&str, &str)], x: u32, y: u32) -> Block {
    Block {
        id,
        kind: BlockKind::Fb,
        type_ref: ty.into(),
        instance: Some(inst.into()),
        pos: Pos { x, y },
        params: params
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect(),
        pins: vec![],
        exec_order: 0,
        negated: vec![],
        comment: String::new(),
        role: None,
        flow: None,
        name: None,
        at: None,
    }
}

fn wire(id: u64, src: (u64, PortSel), dsts: &[(u64, PortSel)], probe: bool) -> Wire {
    Wire {
        id,
        src: PortRef {
            block: src.0,
            port: src.1,
        },
        dsts: dsts
            .iter()
            .map(|(b, p)| PortRef {
                block: *b,
                port: p.clone(),
            })
            .collect(),
        probe,
    }
}

/// doc 06 §8: `%IX0.0 → TON(PT:=200ms).Q → AND(AND NOT %IX0.1) → %QX0.0`
fn debounce_project() -> FbdProject {
    FbdProject {
        schema: SCHEMA_ID.into(),
        project: "deb".into(),
        data_types: vec![],
        global_vars: vec![],
        pous: vec![Pou {
            id: "p1".into(),
            name: "DEB".into(),
            kind: PouKind::Program,
            ret_ty: None,
            en_eno: "relaxed".into(),
            interface: vec![],
            networks: vec![Network {
                id: "n1".into(),
                kind: NetworkKind::Fbd,
                label: String::new(),
                comments: vec![],
                assocs: vec![],
                labels: vec![],
                jumps: vec![],
                returns: vec![],
                blocks: vec![
                    var_block(13, VarRole::In, "StartBtn", Some("%IX0.0"), 1, 4),
                    fb_block(12, "TON", "T_Start", &[("PT", "T#200ms")], 8, 4),
                    var_block(99, VarRole::In, "StopBtn", Some("%IX0.1"), 1, 8),
                    neg_op_block(14, "AND", &[1], 20, 6),
                    var_block(15, VarRole::Out, "MotorRun", Some("%QX0.0"), 30, 6),
                ],
                wires: vec![
                    wire(
                        40,
                        (13, PortSel::Index(0)),
                        &[(12, PortSel::Index(0))],
                        false,
                    ),
                    wire(
                        41,
                        (12, PortSel::Name("Q".into())),
                        &[(14, PortSel::Index(0))],
                        true,
                    ),
                    wire(
                        42,
                        (99, PortSel::Index(0)),
                        &[(14, PortSel::Index(1))],
                        false,
                    ),
                    wire(
                        43,
                        (14, PortSel::Index(0)),
                        &[(15, PortSel::Index(0))],
                        false,
                    ),
                ],
            }],
        }],
        tasks: vec![Task {
            name: "MAIN".into(),
            period_ns: 10_000_000,
            priority: 0,
            programs: vec!["p1".into()],
        }],
    }
}

fn errors(out: &LowerOutput) -> Vec<&Diag> {
    out.diags
        .iter()
        .filter(|d| d.severity == tplc_mir::diag::Severity::Error)
        .collect()
}

use tplc_mir::diag::Diag;

#[test]
fn debounce_lowers_and_runs() {
    let out = lower(&debounce_project());
    assert!(errors(&out).is_empty(), "{:?}", out.diags);

    let pou = &out.module.pous[0];
    // named located vars + FB instance + probe field
    let names: Vec<&str> = pou.iface.iter().map(|v| v.name.as_str()).collect();
    assert!(names.contains(&"StartBtn"), "{names:?}");
    assert!(names.contains(&"StopBtn"), "{names:?}");
    assert!(names.contains(&"MotorRun"), "{names:?}");
    assert!(names.contains(&"T_Start"), "{names:?}");
    assert!(names.iter().any(|n| n.starts_with("__probe_")), "{names:?}");
    // io bindings for all three located vars
    assert_eq!(out.io_bindings.len(), 3, "{:?}", out.io_bindings);

    // stmt shape: [Call T_Start] [probe store] [Assign MotorRun]
    let Body::Stmts(stmts) = &pou.body else {
        panic!()
    };
    assert!(matches!(stmts[0].kind, StmtKind::Call { .. }), "{stmts:?}");
    let last = stmts.last().unwrap();
    assert!(matches!(last.kind, StmtKind::Assign { .. }), "{stmts:?}");

    // run it: press start 200ms → Q up, motor on; release → off
    let layout = layout_program(&out.module, pou);
    let mut blob = vec![0u8; layout.total_size as usize + 64];
    let ctx = program_ctx(&layout);
    let Body::Stmts(stmts) = &pou.body else {
        panic!()
    };
    let mut rt = Rt::new(&mut blob, &layout, &out.module, 0);
    let start_field = "StartBtn";
    let run = |rt: &mut Rt, t: u64| {
        rt.now_ns = t;
        rt.exec_all(&ctx, stmts);
    };
    rt.write_field(start_field, &Value::Bool(true));
    for t in (0..250_000_000u64).step_by(10_000_000) {
        run(&mut rt, t);
    }
    assert_eq!(rt.read_field("MotorRun"), Value::Bool(true));
    // probe captured T_Start.Q high before AND
    let pname = names
        .iter()
        .find(|n| n.starts_with("__probe_"))
        .unwrap()
        .to_string();
    assert_eq!(rt.read_field(&pname), Value::Bool(true));
    // drop start → TON.Q falls → motor off
    rt.write_field(start_field, &Value::Bool(false));
    run(&mut rt, 300_000_000);
    assert_eq!(rt.read_field("MotorRun"), Value::Bool(false));
}

#[test]
fn algebraic_cycle_rejected() {
    // AND feeds OR feeds AND — no state element → FBD004
    let mut p = debounce_project();
    let net = &mut p.pous[0].networks[0];
    net.blocks = vec![
        op_block(1, "AND", 10, 0),
        op_block(2, "OR", 20, 0),
        var_block(3, VarRole::Out, "x", Some("%QX0.0"), 30, 0),
    ];
    net.wires = vec![
        wire(10, (1, PortSel::Index(0)), &[(2, PortSel::Index(0))], false),
        wire(11, (2, PortSel::Index(0)), &[(1, PortSel::Index(0))], false),
        wire(12, (2, PortSel::Index(0)), &[(3, PortSel::Index(0))], false),
    ];
    let out = lower(&p);
    let errs = errors(&out);
    assert!(
        errs.iter().any(|d| d.code == codes::ALG_CYCLE),
        "{:?}",
        out.diags
    );
}

#[test]
fn feedback_through_fb_is_legal() {
    // FB.Q → NOT → FB.IN: cycle through state is legal
    let mut p = debounce_project();
    let net = &mut p.pous[0].networks[0];
    net.blocks = vec![
        var_block(1, VarRole::In, "en", Some("%IX0.0"), 0, 0),
        fb_block(2, "SR", "ff", &[], 10, 0),
        op_block(3, "NOT", 20, 4),
        var_block(4, VarRole::Out, "q", Some("%QX0.0"), 30, 0),
    ];
    net.wires = vec![
        wire(10, (1, PortSel::Index(0)), &[(2, PortSel::Index(0))], false), // en → S1
        wire(
            11,
            (2, PortSel::Name("Q1".into())),
            &[(3, PortSel::Index(0))],
            false,
        ),
        wire(12, (3, PortSel::Index(0)), &[(2, PortSel::Index(1))], false), // NOT.Q → R
        wire(
            13,
            (2, PortSel::Name("Q1".into())),
            &[(4, PortSel::Index(0))],
            false,
        ),
    ];
    let out = lower(&p);
    assert!(errors(&out).is_empty(), "{:?}", out.diags);

    // semantics: en pulse sets Q1; NOT(Q1) then clears next cycle
    // → the SR's own output feeds back through previous-cycle values.
    let pou = &out.module.pous[0];
    let layout = layout_program(&out.module, pou);
    let mut blob = vec![0u8; layout.total_size as usize + 64];
    let ctx = program_ctx(&layout);
    let Body::Stmts(stmts) = &pou.body else {
        panic!()
    };
    let mut rt = Rt::new(&mut blob, &layout, &out.module, 0);
    rt.write_field("en", &Value::Bool(true));
    rt.exec_all(&ctx, stmts);
    rt.write_field("en", &Value::Bool(false));
    assert_eq!(rt.read_field("q"), Value::Bool(true));
}

#[test]
fn sel_lowers_to_if() {
    let mut p = debounce_project();
    let net = &mut p.pous[0].networks[0];
    net.blocks = vec![
        var_block(1, VarRole::In, "g", Some("%IX0.0"), 0, 0),
        var_block(2, VarRole::In, "a", None, 0, 2),
        var_block(3, VarRole::In, "b", None, 0, 4),
        op_block(4, "SEL", 10, 2),
        var_block(5, VarRole::Out, "o", None, 20, 2),
    ];
    net.wires = vec![
        wire(10, (1, PortSel::Index(0)), &[(4, PortSel::Index(0))], false),
        wire(11, (2, PortSel::Index(0)), &[(4, PortSel::Index(1))], false),
        wire(12, (3, PortSel::Index(0)), &[(4, PortSel::Index(2))], false),
        wire(13, (4, PortSel::Index(0)), &[(5, PortSel::Index(0))], false),
    ];
    // a/b/o are plain iface vars
    p.pous[0].interface = vec![
        IfaceVar {
            name: "a".into(),
            kind: IfaceKind::Var,
            ty: "INT".into(),
            init: Some("3".into()),
            at: None,
            retain: false,
        },
        IfaceVar {
            name: "b".into(),
            kind: IfaceKind::Var,
            ty: "INT".into(),
            init: Some("7".into()),
            at: None,
            retain: false,
        },
        IfaceVar {
            name: "o".into(),
            kind: IfaceKind::Var,
            ty: "INT".into(),
            init: None,
            at: None,
            retain: false,
        },
    ];
    let out = lower(&p);
    assert!(errors(&out).is_empty(), "{:?}", out.diags);
    let pou = &out.module.pous[0];
    let layout = layout_program(&out.module, pou);
    let mut blob = vec![0u8; layout.total_size as usize + 64];
    let ctx = program_ctx(&layout);
    let Body::Stmts(stmts) = &pou.body else {
        panic!()
    };
    // init runs decl inits? no — apply literal init manually
    let mut rt = Rt::new(&mut blob, &layout, &out.module, 0);
    rt.write_field("a", &Value::I(3));
    rt.write_field("b", &Value::I(7));
    rt.write_field("g", &Value::Bool(false));
    rt.exec_all(&ctx, stmts);
    let o = rt.read_field("o").as_i64();
    assert_eq!(o, 3); // G=0 → IN0=a
}

#[test]
fn exec_order_collision_errors() {
    let mut p = debounce_project();
    p.pous[0].networks[0].blocks[0].exec_order = 5;
    p.pous[0].networks[0].blocks[1].exec_order = 5;
    let out = lower(&p);
    assert!(
        errors(&out)
            .iter()
            .any(|d| d.code == codes::ORDER_COLLISION),
        "{:?}",
        out.diags
    );
}

#[test]
fn undeclared_name_errors() {
    let mut p = debounce_project();
    p.pous[0].networks[0]
        .blocks
        .push(var_block(50, VarRole::In, "Ghost", None, 0, 20));
    let out = lower(&p);
    assert!(
        errors(&out).iter().any(|d| d.code == codes::UNDECLARED),
        "{:?}",
        out.diags
    );
}

#[test]
fn fan_in_errors() {
    let mut p = debounce_project();
    let net = &mut p.pous[0].networks[0];
    net.wires.push(wire(
        90,
        (13, PortSel::Index(0)),
        &[(12, PortSel::Index(0))],
        false,
    ));
    let out = lower(&p);
    assert!(
        errors(&out).iter().any(|d| d.code == codes::FAN_IN),
        "{:?}",
        out.diags
    );
}

/// FB POU with one VAR_IN_OUT member — host for FBD005/014 fixtures.
fn inout_host_project(dst: Block, extra_wire: Option<Wire>) -> FbdProject {
    let mut p = debounce_project();
    // user FB: Accum — VAR_IN_OUT total : DINT
    p.pous.push(Pou {
        id: "fb_acc".into(),
        name: "ACCUM".into(),
        kind: PouKind::FunctionBlock,
        ret_ty: None,
        en_eno: "relaxed".into(),
        interface: vec![IfaceVar {
            name: "total".into(),
            kind: IfaceKind::VarInOut,
            ty: "DINT".into(),
            init: None,
            at: None,
            retain: false,
        }],
        networks: vec![],
    });
    // declare a DINT var to serve as legal inout storage
    p.pous[0].interface.push(IfaceVar {
        name: "AccTotal".into(),
        kind: IfaceKind::Var,
        ty: "DINT".into(),
        init: None,
        at: None,
        retain: false,
    });
    let net = &mut p.pous[0].networks[0];
    net.blocks.retain(|b| b.id != 15);
    net.wires.retain(|w| w.id != 43);
    let mut blocks = vec![
        fb_block(60, "ACCUM", "Acc1", &[], 20, 10),
        var_block(61, VarRole::Out, "MotorRun", Some("%QX0.0"), 30, 6),
        dst,
    ];
    net.blocks.append(&mut blocks);
    net.wires.push(wire(
        62,
        (60, PortSel::Index(0)),
        &[(61, PortSel::Index(0))],
        false,
    ));
    if let Some(w) = extra_wire {
        net.wires.push(w);
    }
    p
}

#[test]
fn inout_literal_param_errors() {
    // inout pin fed by param literal → FBD005
    let mut p = inout_host_project(var_block(70, VarRole::In, "AccTotal", None, 1, 10), None);
    p.pous[0].networks[0]
        .blocks
        .iter_mut()
        .find(|b| b.id == 60)
        .unwrap()
        .params
        .insert("total".into(), "42".into());
    let out = lower(&p);
    assert!(
        errors(&out).iter().any(|d| d.code == codes::INOUT_STORAGE),
        "{:?}",
        out.diags
    );
}

#[test]
fn inout_expr_chain_errors() {
    // inout pin fed by an operator output → FBD014
    let mut add = op_block(70, "ADD", 10, 10);
    add.params.insert("IN0".into(), "1".into());
    add.params.insert("IN1".into(), "2".into());
    let p = inout_host_project(
        add,
        Some(wire(
            71,
            (70, PortSel::Index(0)),
            &[(60, PortSel::Index(0))],
            false,
        )),
    );
    let out = lower(&p);
    assert!(
        errors(&out).iter().any(|d| d.code == codes::INOUT_EXPR),
        "{:?}",
        out.diags
    );
}

#[test]
fn inout_storage_ref_ok() {
    // inout pin fed by a var block → no FBD005/014
    let p = inout_host_project(
        var_block(70, VarRole::In, "AccTotal", None, 1, 10),
        Some(wire(
            71,
            (70, PortSel::Index(0)),
            &[(60, PortSel::Index(0))],
            false,
        )),
    );
    let out = lower(&p);
    let errs = errors(&out);
    assert!(
        !errs
            .iter()
            .any(|d| d.code == codes::INOUT_STORAGE || d.code == codes::INOUT_EXPR),
        "{:?}",
        out.diags
    );
}

#[test]
fn bool_input_to_arith_errors() {
    // AND is fine; feeding a BOOL var to ADD triggers FBD009
    let mut p = debounce_project();
    let net = &mut p.pous[0].networks[0];
    net.blocks.retain(|b| b.id != 14 && b.id != 15);
    net.wires.retain(|w| w.id != 42 && w.id != 43);
    net.blocks.push(op_block(20, "ADD", 20, 6));
    net.blocks.push(var_block(
        21,
        VarRole::Out,
        "MotorRun",
        Some("%QX0.0"),
        30,
        6,
    ));
    net.wires.push(wire(
        44,
        (12, PortSel::Name("Q".into())),
        &[(20, PortSel::Index(0))],
        false,
    ));
    net.wires.push(wire(
        45,
        (20, PortSel::Index(0)),
        &[(21, PortSel::Index(0))],
        false,
    ));
    let out = lower(&p);
    assert!(
        errors(&out).iter().any(|d| d.code == codes::ANY_UNRESOLVED),
        "{:?}",
        out.diags
    );
}
