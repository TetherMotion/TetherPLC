//! Frontend path test — needs the `rusty` feature (LLVM-21 prefix).
//!   cargo test -p tplc-st --features rusty
#![cfg(feature = "rusty")]

use tplc_mir as mir;
use tplc_st::compile;

const SRC: &str = r#"
FUNCTION_BLOCK FB_TonWrap
VAR_INPUT  IN : BOOL; PT : TIME; END_VAR
VAR_OUTPUT Q  : BOOL; ET : TIME; END_VAR
VAR        tmr : TON; cnt : INT := 0; END_VAR
    tmr(IN := IN, PT := PT);
    Q := tmr.Q;
END_FUNCTION_BLOCK

PROGRAM MAIN
VAR
    ton1    : FB_TonWrap;
    running AT %QX0.0 : BOOL;
    sensor  AT %IW0.0 : INT;
    ticks   : UDINT;
END_VAR
    ton1(IN := sensor > 0, PT := T#100ms);
    running := ton1.Q;
    ticks := ticks + 1;
END_PROGRAM
"#;

#[test]
fn st_to_mir_to_blob_execution() {
    let out = compile(&[("main.st", SRC)]);
    assert!(!out.has_errors(), "diags: {:?}", out.diags);
    assert_eq!(out.module.pous.len(), 2);

    // P1-03 sema: names/types/conversions/call-shape on mutable module
    let mut module = out.module;
    let sdiags = tplc_st::sema::check(&mut module);
    assert!(
        !sdiags
            .iter()
            .any(|d| d.severity == tplc_st::Severity::Error),
        "sema errors: {sdiags:?}"
    );

    // io_bindings: sensor→%IW0.0, running→%QX0.0
    assert_eq!(out.io_bindings.len(), 2);
    let sensor = out
        .io_bindings
        .iter()
        .find(|b| b.name == "MAIN.sensor")
        .unwrap();
    assert_eq!(sensor.at.region, mir::AtRegion::I);
    assert_eq!(sensor.at.width, 'W');
    let running = out
        .io_bindings
        .iter()
        .find(|b| b.name == "MAIN.running")
        .unwrap();
    assert_eq!(running.at.region, mir::AtRegion::Q);

    // layout + execute 20 cycles @ 10ms: TON fires at 100ms
    let program = module.pous.iter().find(|p| p.name == "MAIN").unwrap();
    let layout = mir::layout_program(&module, program);
    let mut blob = vec![0u8; layout.total_size as usize];
    let mut rt = mir::Rt::new(&mut blob, &layout, &module, 0);
    let ctx = mir::interp::program_ctx(&layout);
    rt.write_field("sensor", &mir::Value::I(7));
    let stmts = match &program.body {
        mir::Body::Stmts(s) => s,
        _ => panic!(),
    };
    for c in 0..20u64 {
        rt.now_ns = c * 10_000_000;
        rt.exec_all(&ctx, stmts);
    }
    let ton_q = mir::Ref::Field(Box::new(mir::Ref::Var("ton1".into())), "Q".into());
    assert_eq!(rt.read_ref(&ctx, &ton_q), mir::Value::Bool(true));
    assert_eq!(rt.read_field("running"), mir::Value::Bool(true));
    assert_eq!(rt.read_field("ticks"), mir::Value::U(20));
}

#[test]
fn sema_catches_undeclared_and_bad_args() {
    let bad = r#"
PROGRAM P
VAR x : INT; END_VAR
    y := x + 1;
END_PROGRAM
"#;
    let out = compile(&[("bad.st", bad)]);
    let mut module = out.module;
    let diags = tplc_st::sema::check(&mut module);
    assert!(
        diags
            .iter()
            .any(|d| d.code == tplc_st::codes::UNKNOWN_NAME && d.message.contains('y')),
        "expected undeclared-name diag, got {diags:?}"
    );
}

#[test]
fn lints_fire_and_render() {
    let src = r#"
PROGRAM P
VAR
    unused1 : INT;
    big : DINT := 7;
    small : INT;
    dead : INT;
END_VAR
    small := big;
    CASE small OF
        1: dead := 1;
        1..5: dead := 2;
    END_CASE
    WHILE small > 0 DO
        EXIT;
        dead := 9;
    END_WHILE
    RETURN;
    dead := 4;
END_PROGRAM
"#;
    let out = compile(&[("lint.st", src)]);
    let mut module = out.module;
    let diags = tplc_st::sema::check(&mut module);
    for code in [
        tplc_st::wcodes::UNUSED_VAR,
        tplc_st::wcodes::IMPLICIT_NARROW,
        tplc_st::wcodes::UNREACHABLE_CASE,
        tplc_st::wcodes::NO_JUMPS,
        tplc_st::wcodes::DEAD_CODE,
    ] {
        assert!(
            diags.iter().any(|d| d.code == code),
            "missing {code}: {diags:?}"
        );
    }
    // every lint carries its source span into the renderer
    let rendered: String = diags.iter().map(|d| d.render("lint.st", src)).collect();
    assert!(rendered.contains("--> lint.st:"));
}
