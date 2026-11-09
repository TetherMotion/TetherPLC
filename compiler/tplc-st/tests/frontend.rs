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
fn string_semantics_are_typed() {
    // STRING =/<> is legal; an ordered compare is a type error (IEC
    // 61131-3) instead of a codegen surprise
    let ok = r#"
PROGRAM P
VAR a : STRING[16]; b : STRING[16]; eq : BOOL; END_VAR
    a := 'abc';
    b := a;
    eq := (a = b) OR (a <> 'x');
END_PROGRAM
"#;
    let out = compile(&[("ok.st", ok)]);
    let mut module = out.module;
    let diags = tplc_st::sema::check(&mut module);
    assert!(
        !diags.iter().any(|d| d.severity == tplc_st::Severity::Error),
        "sema errors: {diags:?}"
    );

    let bad = r#"
PROGRAM P
VAR a : STRING[16]; b : STRING[16]; lt : BOOL; END_VAR
    lt := (a < b);
END_PROGRAM
"#;
    let out = compile(&[("bad.st", bad)]);
    let mut module = out.module;
    let diags = tplc_st::sema::check(&mut module);
    assert!(
        diags
            .iter()
            .any(|d| d.code == tplc_st::codes::TYPE && d.message.contains("STRING comparison")),
        "expected ordered-string diag, got {diags:?}"
    );

    // WSTRING is its own type: wide literals assign, mixed narrow/wide
    // does not, and ordered compares are rejected the same way
    let wok = r#"
PROGRAM P
VAR w : WSTRING[16]; w2 : WSTRING[16]; eq : BOOL; END_VAR
    w := "abc";
    w2 := w;
    eq := (w = "abc");
END_PROGRAM
"#;
    let out = compile(&[("wok.st", wok)]);
    let mut module = out.module;
    let diags = tplc_st::sema::check(&mut module);
    assert!(
        !diags.iter().any(|d| d.severity == tplc_st::Severity::Error),
        "WSTRING sema errors: {diags:?}"
    );

    let wmix = r#"
PROGRAM P
VAR w : WSTRING[16]; s : STRING[16]; END_VAR
    s := "abc";
END_PROGRAM
"#;
    let out = compile(&[("wmix.st", wmix)]);
    let mut module = out.module;
    let diags = tplc_st::sema::check(&mut module);
    assert!(
        diags.iter().any(|d| d.code == tplc_st::codes::TYPE),
        "expected wide→narrow mismatch, got {diags:?}"
    );

    let wlt = r#"
PROGRAM P
VAR w : WSTRING[16]; lt : BOOL; END_VAR
    lt := (w < "b");
END_PROGRAM
"#;
    let out = compile(&[("wlt.st", wlt)]);
    let mut module = out.module;
    let diags = tplc_st::sema::check(&mut module);
    assert!(
        diags
            .iter()
            .any(|d| d.code == tplc_st::codes::TYPE && d.message.contains("STRING comparison")),
        "expected ordered-wide diag, got {diags:?}"
    );
}

#[test]
fn string_builtins_are_typed() {
    // LEN/CONCAT over both string kinds; misuse is a type error
    let ok = r#"
PROGRAM P
VAR s : STRING[16]; c : STRING[32]; w : WSTRING[16];
    wc : WSTRING[32]; n : INT; eq : BOOL; END_VAR
    s := 'abc';
    c := CONCAT(s, s);
    w := "aä";
    wc := CONCAT(w, w);
    n := LEN(c);
    eq := (n = 6);
END_PROGRAM
"#;
    let out = compile(&[("ok.st", ok)]);
    let mut module = out.module;
    let diags = tplc_st::sema::check(&mut module);
    assert!(
        !diags.iter().any(|d| d.severity == tplc_st::Severity::Error),
        "builtin sema errors: {diags:?}"
    );

    for (name, src) in [
        (
            "len_bad",
            "PROGRAM P\nVAR x : INT; n : INT; END_VAR\n n := LEN(x);\nEND_PROGRAM\n",
        ),
        (
            "concat_mixed",
            "PROGRAM P\nVAR s : STRING[16]; w : WSTRING[16];\n c : STRING[32]; END_VAR\n c := CONCAT(s, w);\nEND_PROGRAM\n",
        ),
    ] {
        let out = compile(&[(name, src)]);
        let mut module = out.module;
        let diags = tplc_st::sema::check(&mut module);
        assert!(
            diags.iter().any(|d| d.code == tplc_st::codes::TYPE),
            "{name}: expected TYPE diag, got {diags:?}"
        );
    }
}

#[test]
fn scalar_builtins_are_typed() {
    let ok = r#"
PROGRAM P
VAR a : DINT; b : DINT; c : DINT; d : LREAL; e : BOOL; END_VAR
    a := ABS(-5);
    b := MIN(3, 7);
    c := LIMIT(10, 42, 20);
    d := MAX(1.5, 2);
    e := (SEL(TRUE, 1, 2) = 2);
END_PROGRAM
"#;
    let out = compile(&[("ok.st", ok)]);
    let mut module = out.module;
    let diags = tplc_st::sema::check(&mut module);
    assert!(
        !diags.iter().any(|d| d.severity == tplc_st::Severity::Error),
        "scalar builtin sema errors: {diags:?}"
    );

    let bad = r#"
PROGRAM P
VAR s : STRING[16]; a : DINT; END_VAR
    a := ABS(s);
END_PROGRAM
"#;
    let out = compile(&[("bad.st", bad)]);
    let mut module = out.module;
    let diags = tplc_st::sema::check(&mut module);
    assert!(
        diags.iter().any(|d| d.code == tplc_st::codes::TYPE),
        "expected ABS-on-string diag, got {diags:?}"
    );
}

#[test]
fn string_functions_are_typed() {
    let ok = r#"
PROGRAM P
VAR s : STRING[16]; c : STRING[16]; w : WSTRING[16];
    wc : WSTRING[16]; n : INT; END_VAR
    s := 'abcdef';
    c := LEFT(s, 3);
    c := MID(s, 2, 3);
    c := INSERT(s, 'XY', 4);
    c := DELETE(s, 2, 3);
    c := REPLACE(s, 'XY', 2, 3);
    w := "aä€z";
    wc := RIGHT(w, 2);
    n := FIND(s, 'cd');
END_PROGRAM
"#;
    let out = compile(&[("ok.st", ok)]);
    let mut module = out.module;
    let diags = tplc_st::sema::check(&mut module);
    assert!(
        !diags.iter().any(|d| d.severity == tplc_st::Severity::Error),
        "string fn sema errors: {diags:?}"
    );

    for (name, src) in [
        (
            "find_mixed",
            "PROGRAM P\nVAR s : STRING[16]; w : WSTRING[16]; n : INT; END_VAR\n n := FIND(s, w);\nEND_PROGRAM\n",
        ),
        (
            "left_nonstring",
            "PROGRAM P\nVAR x : INT; c : STRING[16]; END_VAR\n c := LEFT(x, 3);\nEND_PROGRAM\n",
        ),
    ] {
        let out = compile(&[(name, src)]);
        let mut module = out.module;
        let diags = tplc_st::sema::check(&mut module);
        assert!(
            diags.iter().any(|d| d.code == tplc_st::codes::TYPE),
            "{name}: expected TYPE diag, got {diags:?}"
        );
    }
}

#[test]
fn conversions_round_and_mux_are_typed() {
    let ok = r#"
PROGRAM P
VAR i : DINT; r : LREAL; t : DINT; u : UDINT; m : DINT; END_VAR
    r := DINT_TO_REAL(i);
    t := TRUNC(3.9);
    t := ROUND(2.5);
    m := MUX(1, 10, 20, 30);
    u := DINT_TO_UDINT(7);
END_PROGRAM
"#;
    let out = compile(&[("ok.st", ok)]);
    let mut module = out.module;
    let diags = tplc_st::sema::check(&mut module);
    assert!(
        !diags.iter().any(|d| d.severity == tplc_st::Severity::Error),
        "conversion sema errors: {diags:?}"
    );
    // the conversion is a Conv node, not a call: one conversion path
    let program = module.pous.iter().find(|p| p.name == "P").unwrap();
    let has_conv = match &program.body {
        tplc_mir::Body::Stmts(ss) => ss.iter().any(|s| {
            matches!(
                &s.kind,
                tplc_mir::StmtKind::Assign { rhs, .. }
                    if matches!(rhs.kind, tplc_mir::ExprKind::Conv { .. })
            )
        }),
        _ => false,
    };
    assert!(has_conv, "DINT_TO_REAL should lower to a Conv node");

    let bad = r#"
PROGRAM P
VAR s : STRING[16]; r : LREAL; END_VAR
    r := STRING_TO_REAL(s);
END_PROGRAM
"#;
    let out = compile(&[("bad.st", bad)]);
    let mut module = out.module;
    let diags = tplc_st::sema::check(&mut module);
    assert!(
        diags.iter().any(|d| d.code == tplc_st::codes::TYPE),
        "expected STRING→REAL rejection, got {diags:?}"
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
