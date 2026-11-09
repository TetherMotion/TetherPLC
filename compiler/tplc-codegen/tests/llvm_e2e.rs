#![cfg(feature = "llvm")]
//! MIR → LLVM IR → JIT → ABI ctx → blob assertions.
//! The host-side __tplc_rt_TON stands in for tplc-rt bitcode.

use inkwell::context::Context;
use inkwell::targets::{InitializationConfig, Target};
use tplc_codegen::llvm_emit::{emit_bitcode, emit_object, emit_program};
use tplc_mir::interp::program_ctx;
use tplc_mir::ir::*;
use tplc_mir::layout::layout_program;

fn int_e(v: i64) -> Expr {
    Expr {
        ty: Type::Int {
            signed: true,
            bits: 32,
        },
        kind: ExprKind::Lit(Lit::Int(v)),
        prov: Prov::None,
    }
}
fn int_v(n: &str) -> Expr {
    Expr {
        ty: Type::Int {
            signed: true,
            bits: 32,
        },
        kind: ExprKind::Ref(Ref::Var(n.into())),
        prov: Prov::None,
    }
}
fn bool_v(n: &str) -> Expr {
    Expr {
        ty: Type::Bool,
        kind: ExprKind::Ref(Ref::Var(n.into())),
        prov: Prov::None,
    }
}
fn stmt(k: StmtKind) -> Stmt {
    Stmt {
        prov: Prov::None,
        kind: k,
    }
}
fn assign(lhs: Ref, rhs: Expr) -> Stmt {
    stmt(StmtKind::Assign { lhs, rhs })
}
fn bin(op: BinOp, a: Expr, b: Expr, ty: Type) -> Expr {
    Expr {
        ty,
        kind: ExprKind::Bin(op, Box::new(a), Box::new(b)),
        prov: Prov::None,
    }
}
fn vd(n: &str, ty: Type, init: Option<Expr>) -> VarDecl {
    VarDecl {
        name: n.into(),
        kind: VarKind::Local,
        ty,
        init,
        retain: false,
        at: None,
        probe: false,
        prov: Prov::None,
    }
}

fn demo_module() -> Module {
    let mut m = Module {
        pous: Vec::new(),
        gvls: Vec::new(),
        tasks: Vec::new(),
        checked_arith: false,
    };
    m.pous.push(Pou {
        kind: PouKind::Program,
        name: "P".into(),
        iface: vec![
            vd(
                "x",
                Type::Int {
                    signed: true,
                    bits: 32,
                },
                None,
            ),
            vd(
                "n",
                Type::Int {
                    signed: true,
                    bits: 32,
                },
                Some(int_e(10)),
            ),
            vd("y", Type::Bool, None),
            vd("q", Type::Bool, None),
            vd("ton1", Type::Fb { name: "TON".into() }, None),
        ],
        body: Body::Stmts(vec![
            assign(Ref::Var("x".into()), int_e(0)),
            assign(Ref::Var("n".into()), int_e(10)),
            stmt(StmtKind::While {
                cond: bin(BinOp::Gt, int_v("n"), int_e(0), Type::Bool),
                body: vec![
                    assign(
                        Ref::Var("x".into()),
                        bin(
                            BinOp::Add,
                            int_v("x"),
                            int_e(1),
                            Type::Int {
                                signed: true,
                                bits: 32,
                            },
                        ),
                    ),
                    assign(
                        Ref::Var("n".into()),
                        bin(
                            BinOp::Sub,
                            int_v("n"),
                            int_e(1),
                            Type::Int {
                                signed: true,
                                bits: 32,
                            },
                        ),
                    ),
                ],
            }),
            assign(
                Ref::Var("y".into()),
                bin(BinOp::Eq, int_v("x"), int_e(10), Type::Bool),
            ),
            stmt(StmtKind::Call {
                inst: Ref::Var("ton1".into()),
                args: vec![
                    ("IN".into(), bool_v("y")),
                    (
                        "PT".into(),
                        Expr {
                            ty: Type::Time,
                            kind: ExprKind::Lit(Lit::TimeNs(500_000_000)),
                            prov: Prov::None,
                        },
                    ),
                ],
            }),
            assign(
                Ref::Var("q".into()),
                Expr {
                    ty: Type::Bool,
                    kind: ExprKind::Ref(Ref::Field(Box::new(Ref::Var("ton1".into())), "Q".into())),
                    prov: Prov::None,
                },
            ),
        ]),
        ret_ty: None,
        prov: Prov::None,
    });
    m
}

// fake ABI ctx (tplc_abi.h field order)
#[repr(C)]
struct FakeCtx {
    ss: u32,
    cf: u32,
    blob: *mut u8,
    retain: *mut u8,
    iin: *mut u8,
    iout: *mut u8,
    mk: *mut u8,
    now: u64,
    cyc: u64,
    ovr: i64,
    svc: *const u8,
    hp: *mut u8,
    abort: u32,
    pad: u32,
    dl: u64,
}
#[repr(C)]
struct FakeTCtx {
    prog: *mut FakeCtx,
    idx: u32,
    fl: u32,
    tl: *mut u8,
    sn: u64,
    dl: u64,
    dt: u64,
}

extern "C" fn steady_probe(_h: *mut u8) -> u64 {
    u64::MAX
}

#[test]
fn llvm_lowering_executes_through_jit() {
    let m = demo_module();
    let l = layout_program(&m, &m.pous[0]);
    let m: &'static Module = Box::leak(Box::new(m));
    let l: &'static _ = Box::leak(Box::new(l));
    let program: &'static Pou = Box::leak(Box::new(m.pous[0].clone()));

    let lctx = Context::create();
    let io_binds = [IoBind {
        name: "q".into(),
        at: At {
            region: AtRegion::Q,
            byte: 4,
            bit: None,
            width: 'B',
        },
        ty: Type::Bool,
    }];
    let module = emit_program(
        &lctx, m, l, program, &io_binds, "demo", [0x51; 16], [0xAA; 32], None, 10_000_000,
    );
    std::fs::write("/tmp/gen.ll", module.print_to_string().to_string()).unwrap();
    module.verify().unwrap_or_else(|e| panic!("IR verify: {e}"));
    let ir = module.print_to_string().to_string();
    assert!(ir.contains("define i32 @tplc_task_run"));
    assert!(ir.contains("define i32 @__tplc_pou_P"));
    assert!(ir.contains("declare i32 @__tplc_rt_TON"));
    assert!(ir.contains("@__tplc_desc") && ir.contains("tplc_get_descriptor"));
    assert!(
        !ir.contains("inttoptr"),
        "no absolute pointers in program state"
    );

    Target::initialize_native(&InitializationConfig::default()).unwrap();
    let ee = module.create_execution_engine().unwrap();
    let ton = module.get_function("__tplc_rt_TON").unwrap();
    ee.add_global_mapping(&ton, tplc_rt::__tplc_rt_TON as *const () as usize);

    let scope = program_ctx(l);
    let mut blob = vec![0u8; l.total_size as usize];
    let mut img = vec![0u8; 64];
    let mut ctx = FakeCtx {
        ss: 0,
        cf: 0,
        blob: blob.as_mut_ptr(),
        retain: std::ptr::null_mut(),
        iin: img.as_mut_ptr(),
        iout: unsafe { img.as_mut_ptr().add(32) },
        mk: std::ptr::null_mut(),
        now: 0,
        cyc: 0,
        ovr: 0,
        svc: std::ptr::null(),
        hp: std::ptr::null_mut(),
        abort: 0,
        pad: 0,
        dl: 0,
    };
    let mut tctx = FakeTCtx {
        prog: &mut ctx,
        idx: 0,
        fl: 0,
        tl: std::ptr::null_mut(),
        sn: 0,
        dl: 0,
        dt: 100_000_000,
    };

    unsafe {
        let init: extern "C" fn(*mut FakeCtx) -> i32 =
            std::mem::transmute(ee.get_function_address("tplc_init").unwrap());
        let run: extern "C" fn(*mut FakeTCtx) -> i32 =
            std::mem::transmute(ee.get_function_address("tplc_task_run").unwrap());
        assert_eq!(init(&mut ctx), 0);
        let mut q_seen = false;
        for i in 0..10u64 {
            ctx.now = i * 100_000_000;
            let _ = ctx.now;
            assert_eq!(run(&mut tctx), 0, "cycle {i}");
            if blob[scope["q"].0 as usize] != 0 {
                q_seen = true;
            }
        }
        let x = (blob.as_ptr().add(scope["x"].0 as usize) as *const i32).read();
        assert_eq!(x, 10, "x after while loop");
        assert!(q_seen, "TON.Q fired within window");
        // descriptor sanity: blob size + task/io counts via JIT'd getter
        let getd: extern "C" fn() -> *const u8 =
            std::mem::transmute(ee.get_function_address("tplc_get_descriptor").unwrap());
        let d = getd();
        assert_eq!((d as *const u32).read(), 224, "desc struct_size");
        assert_eq!((d.add(4) as *const u32).read(), 1, "abi_version");
        let blob_sz = (d.add(128) as *const u64).read();
        assert_eq!(blob_sz, l.total_size);
        let task_cnt = (d.add(176) as *const u32).read();
        assert_eq!(task_cnt, 1);
        let io_cnt = (d.add(216) as *const u32).read();
        assert_eq!(io_cnt, 1, "one io binding");
        // out-of-range task index → fault rc
        tctx.idx = 7;
        assert_eq!(run(&mut tctx), -2);
        tctx.idx = 0;
        ctx.abort = 1;
        assert_eq!(run(&mut tctx), -22, "abort flag → TPLC_TASK_E_ABORTED");
        ctx.abort = 0;
        /* armed deadline → svc->steady_ns compare → -23 OVERRUN;
        FakeSvc.steady returns a huge value so any armed deadline
        is immediately exceeded                                  */
        let mut svc: [u64; 8] = [0; 8];
        svc[3] = steady_probe as usize as u64; /* steady_ns @byte24  */
        svc[7] = 0; /* host cookie @byte56*/
        ctx.svc = svc.as_ptr() as *const u8;
        ctx.dl = 1;
        assert_eq!(run(&mut tctx), -23, "deadline → TPLC_TASK_E_OVERRUN");
    }

    // P1-07: object + bitcode emission
    let dir = std::env::temp_dir().join("tplc_codegen_test");
    std::fs::create_dir_all(&dir).unwrap();
    let obj = dir.join("prog.o");
    emit_object(&module, &obj, "").unwrap();
    let hdr = std::fs::read(&obj).unwrap();
    assert_eq!(&hdr[..4], b"\x7fELF", "ELF object emitted");
    let bc = dir.join("prog.bc");
    emit_bitcode(&module, &bc).unwrap();
    let bh = std::fs::read(&bc).unwrap();
    assert_eq!(&bh[..2], b"BC", "bitcode emitted");

    // link into a real .so — proves the emitted object is a valid
    // link unit and the __tplc_rt_* extern seam resolves
    let shim = dir.join("rt_shim.c");
    std::fs::write(
        &shim,
        r#"
#include <stdint.h>
int32_t __tplc_rt_TON(uint8_t* t, uint8_t* c) {
    uint64_t now = *(uint64_t*)(c + 48);
    uint8_t in = t[0]; int64_t pt = *(int64_t*)(t + 8);
    if (in && !t[32]) *(uint64_t*)(t + 40) = now;
    int64_t st = *(int64_t*)(t + 40);
    int64_t el = in ? now - st : 0;
    t[16] = in && el >= pt; *(int64_t*)(t + 24) = in ? (el < pt ? el : pt) : 0;
    t[32] = in; t[48] = 0;
    return 0;
}"#,
    )
    .unwrap();
    let so = dir.join("prog.so");
    let out = std::process::Command::new("cc")
        .args(["-shared", "-o"])
        .arg(&so)
        .arg(&obj)
        .arg(&shim)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "cc: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let nm = std::process::Command::new("nm")
        .args(["-D", "--defined-only"])
        .arg(&so)
        .output()
        .unwrap();
    let syms = String::from_utf8_lossy(&nm.stdout);
    for want in [
        "tplc_abi_version",
        "tplc_get_descriptor",
        "tplc_init",
        "tplc_shutdown",
        "tplc_task_run",
    ] {
        assert!(syms.contains(want), ".so exports {want}");
    }
}

/* ------------------------------------------------------------------ */
/* tplc_migrate: predecessor-hash gate + copy/widen ops                 */
/* ------------------------------------------------------------------ */

#[repr(C)]
struct FakeMCtx {
    new_prog: *mut FakeCtx,
    old_blob: *const u8,
    old_blob_size: u64,
    old_layout_hash: [u8; 32],
    old_abi_version: u32,
    _pad0: u32,
    migrate_desc: *const u8,
    migrate_desc_size: u64,
    out_status: u32,
    out_first_error: u32,
}

#[test]
fn migrate_jit() {
    use tplc_mir::migrate::{MigOp, MigrateEmit};
    let m = demo_module();
    let l = layout_program(&m, &m.pous[0]);
    let m: &'static Module = Box::leak(Box::new(m));
    let l: &'static _ = Box::leak(Box::new(l));
    let program: &'static Pou = Box::leak(Box::new(m.pous[0].clone()));
    let io_binds: Vec<IoBind> = vec![];
    let lctx = Context::create();
    let mut hash = [0u8; 32];
    hash[0] = 0xDE;
    hash[31] = 0xAD;
    let mig = MigrateEmit {
        predecessor_hash: hash,
        degraded: false,
        ops: vec![
            MigOp::Copy {
                old_off: 0,
                new_off: 8,
                size: 8,
            },
            MigOp::Widen {
                old_off: 16,
                new_off: 24,
                old_tag: 2, // i16
                new_tag: 3, // → i32
            },
        ],
    };
    let module = emit_program(
        &lctx,
        m,
        l,
        program,
        &io_binds,
        "mig",
        [0x51; 16],
        [0xAA; 32],
        Some(&mig),
        10_000_000,
    );
    module.verify().unwrap_or_else(|e| panic!("IR verify: {e}"));
    let ee = module.create_execution_engine().unwrap();
    let ton = module.get_function("__tplc_rt_TON").unwrap();
    ee.add_global_mapping(&ton, tplc_rt::__tplc_rt_TON as *const () as usize);

    let mut nblob = vec![0u8; l.total_size as usize];
    let mut nctx = FakeCtx {
        ss: 0,
        cf: 0,
        blob: nblob.as_mut_ptr(),
        retain: std::ptr::null_mut(),
        iin: std::ptr::null_mut(),
        iout: std::ptr::null_mut(),
        mk: std::ptr::null_mut(),
        now: 0,
        cyc: 0,
        ovr: 0,
        svc: std::ptr::null(),
        hp: std::ptr::null_mut(),
        abort: 0,
        pad: 0,
        dl: 0,
    };
    let mut oblob = vec![0u8; 128];
    oblob[0..8].copy_from_slice(&0x1122334455667788u64.to_le_bytes());
    oblob[16..18].copy_from_slice(&(-123i16).to_le_bytes());
    let mut mc = FakeMCtx {
        new_prog: &mut nctx,
        old_blob: oblob.as_ptr(),
        old_blob_size: 128,
        old_layout_hash: hash,
        old_abi_version: 1,
        _pad0: 0,
        migrate_desc: std::ptr::null(),
        migrate_desc_size: 0,
        out_status: 0xFF,
        out_first_error: 0xFF,
    };
    unsafe {
        let migfn: extern "C" fn(*mut FakeMCtx) -> i32 =
            std::mem::transmute(ee.get_function_address("tplc_migrate").unwrap());
        assert_eq!(migfn(&mut mc), 0);
        assert_eq!(mc.out_status, 0);
        assert_eq!(
            u64::from_le_bytes(nblob[8..16].try_into().unwrap()),
            0x1122334455667788
        );
        assert_eq!(i32::from_le_bytes(nblob[24..28].try_into().unwrap()), -123);
        // predecessor mismatch → DENY (-33), status 2
        mc.old_layout_hash[0] = 0x00;
        mc.out_status = 0xFF;
        assert_eq!(migfn(&mut mc), -33);
        assert_eq!(mc.out_status, 2);
        assert_eq!(mc.out_first_error, 1);
        // blob-range guard → -35, status 4
        mc.old_layout_hash = hash;
        mc.old_blob_size = 4;
        mc.out_status = 0xFF;
        assert_eq!(migfn(&mut mc), -35);
        assert_eq!(mc.out_status, 4);
    }
}

/// STRING semantics through the compiled path: literal materialization
/// (len byte + bytes + zero pad), cross-cap store, and =/<> compare.
/// Regression for the opaque-pointer era bug where a literal lowered
/// to a bare i8 zero and comparisons panicked on ArrayValue.
#[test]
fn string_assign_and_compare_through_jit() {
    fn str_lit(s: &str) -> Expr {
        Expr {
            ty: Type::Str {
                cap: s.len() as u16,
            },
            kind: ExprKind::Lit(Lit::Str(s.into())),
            prov: Prov::None,
        }
    }
    fn str_var(n: &str) -> Expr {
        Expr {
            ty: Type::Str { cap: 16 },
            kind: ExprKind::Ref(Ref::Var(n.into())),
            prov: Prov::None,
        }
    }
    let mut m = Module {
        pous: Vec::new(),
        gvls: Vec::new(),
        tasks: Vec::new(),
        checked_arith: false,
    };
    m.pous.push(Pou {
        kind: PouKind::Program,
        name: "S".into(),
        iface: vec![
            vd("a", Type::Str { cap: 16 }, None),
            vd("b", Type::Str { cap: 16 }, None),
            vd("eq1", Type::Bool, None),
            vd("eq2", Type::Bool, None),
            vd("eq3", Type::Bool, None),
        ],
        body: Body::Stmts(vec![
            assign(Ref::Var("a".into()), str_lit("abc")),
            assign(Ref::Var("b".into()), str_lit("abd")),
            assign(
                Ref::Var("eq1".into()),
                bin(BinOp::Eq, str_var("a"), str_var("b"), Type::Bool),
            ),
            assign(
                Ref::Var("eq2".into()),
                bin(BinOp::Eq, str_var("a"), str_lit("abc"), Type::Bool),
            ),
            assign(
                Ref::Var("eq3".into()),
                bin(BinOp::Ne, str_var("a"), str_var("b"), Type::Bool),
            ),
        ]),
        ret_ty: None,
        prov: Prov::None,
    });
    let l = layout_program(&m, &m.pous[0]);
    let m: &'static Module = Box::leak(Box::new(m));
    let l: &'static _ = Box::leak(Box::new(l));
    let program: &'static Pou = Box::leak(Box::new(m.pous[0].clone()));

    let lctx = Context::create();
    let module = emit_program(
        &lctx,
        m,
        l,
        program,
        &[],
        "str",
        [0x52; 16],
        [0xBB; 32],
        None,
        10_000_000,
    );
    module
        .verify()
        .unwrap_or_else(|e| panic!("IR verify: {e}\n{}", module.print_to_string()));
    Target::initialize_native(&InitializationConfig::default()).unwrap();
    let ee = module.create_execution_engine().unwrap();

    let scope = program_ctx(l);
    let mut blob = vec![0u8; l.total_size as usize];
    let mut img = vec![0u8; 64];
    let mut ctx = FakeCtx {
        ss: 0,
        cf: 0,
        blob: blob.as_mut_ptr(),
        retain: std::ptr::null_mut(),
        iin: img.as_mut_ptr(),
        iout: unsafe { img.as_mut_ptr().add(32) },
        mk: std::ptr::null_mut(),
        now: 0,
        cyc: 0,
        ovr: 0,
        svc: std::ptr::null(),
        hp: std::ptr::null_mut(),
        abort: 0,
        pad: 0,
        dl: 0,
    };
    let mut tctx = FakeTCtx {
        prog: &mut ctx,
        idx: 0,
        fl: 0,
        tl: std::ptr::null_mut(),
        sn: 0,
        dl: 0,
        dt: 10_000_000,
    };
    unsafe {
        let init: extern "C" fn(*mut FakeCtx) -> i32 =
            std::mem::transmute(ee.get_function_address("tplc_init").unwrap());
        let run: extern "C" fn(*mut FakeTCtx) -> i32 =
            std::mem::transmute(ee.get_function_address("tplc_task_run").unwrap());
        assert_eq!(init(&mut ctx), 0);
        assert_eq!(run(&mut tctx), 0);

        let a = scope["a"].0 as usize;
        assert_eq!(blob[a], 3, "len byte");
        assert_eq!(&blob[a + 1..a + 4], b"abc");
        assert_eq!(&blob[a + 4..a + 17], &[0u8; 13], "zero pad");
        let b = scope["b"].0 as usize;
        assert_eq!(blob[b], 3);
        assert_eq!(&blob[b + 1..b + 4], b"abd");

        assert_eq!(blob[scope["eq1"].0 as usize], 0, "a = b is false");
        assert_eq!(blob[scope["eq2"].0 as usize], 1, "a = 'abc' is true");
        assert_eq!(blob[scope["eq3"].0 as usize], 1, "a <> b is true");
    }
}

/// WSTRING is UTF-16: blob = u16 len + u16 code units. Verifies
/// encoding (incl. non-ASCII), cross-cap store and =/<> compare.
#[test]
fn wide_string_is_utf16_through_jit() {
    fn wlit(s: &str) -> Expr {
        Expr {
            ty: Type::WStr {
                cap: s.encode_utf16().count() as u16,
            },
            kind: ExprKind::Lit(Lit::WStr(s.into())),
            prov: Prov::None,
        }
    }
    fn wvar(n: &str) -> Expr {
        Expr {
            ty: Type::WStr { cap: 16 },
            kind: ExprKind::Ref(Ref::Var(n.into())),
            prov: Prov::None,
        }
    }
    let mut m = Module {
        pous: Vec::new(),
        gvls: Vec::new(),
        tasks: Vec::new(),
        checked_arith: false,
    };
    m.pous.push(Pou {
        kind: PouKind::Program,
        name: "W".into(),
        iface: vec![
            vd("w", Type::WStr { cap: 16 }, None),
            vd("w2", Type::WStr { cap: 16 }, None),
            vd("eq1", Type::Bool, None),
            vd("eq2", Type::Bool, None),
        ],
        body: Body::Stmts(vec![
            assign(Ref::Var("w".into()), wlit("aä€")),
            assign(Ref::Var("w2".into()), wlit("aäx")),
            assign(
                Ref::Var("eq1".into()),
                bin(BinOp::Eq, wvar("w"), wvar("w2"), Type::Bool),
            ),
            assign(
                Ref::Var("eq2".into()),
                bin(BinOp::Eq, wvar("w"), wlit("aä€"), Type::Bool),
            ),
        ]),
        ret_ty: None,
        prov: Prov::None,
    });
    let l = layout_program(&m, &m.pous[0]);
    let m: &'static Module = Box::leak(Box::new(m));
    let l: &'static _ = Box::leak(Box::new(l));
    let program: &'static Pou = Box::leak(Box::new(m.pous[0].clone()));

    let lctx = Context::create();
    let module = emit_program(
        &lctx,
        m,
        l,
        program,
        &[],
        "wstr",
        [0x57; 16],
        [0xCC; 32],
        None,
        10_000_000,
    );
    module
        .verify()
        .unwrap_or_else(|e| panic!("IR verify: {e}\n{}", module.print_to_string()));
    Target::initialize_native(&InitializationConfig::default()).unwrap();
    let ee = module.create_execution_engine().unwrap();

    let scope = program_ctx(l);
    let mut blob = vec![0u8; l.total_size as usize];
    let mut img = vec![0u8; 64];
    let mut ctx = FakeCtx {
        ss: 0,
        cf: 0,
        blob: blob.as_mut_ptr(),
        retain: std::ptr::null_mut(),
        iin: img.as_mut_ptr(),
        iout: unsafe { img.as_mut_ptr().add(32) },
        mk: std::ptr::null_mut(),
        now: 0,
        cyc: 0,
        ovr: 0,
        svc: std::ptr::null(),
        hp: std::ptr::null_mut(),
        abort: 0,
        pad: 0,
        dl: 0,
    };
    let mut tctx = FakeTCtx {
        prog: &mut ctx,
        idx: 0,
        fl: 0,
        tl: std::ptr::null_mut(),
        sn: 0,
        dl: 0,
        dt: 10_000_000,
    };
    unsafe {
        let init: extern "C" fn(*mut FakeCtx) -> i32 =
            std::mem::transmute(ee.get_function_address("tplc_init").unwrap());
        let run: extern "C" fn(*mut FakeTCtx) -> i32 =
            std::mem::transmute(ee.get_function_address("tplc_task_run").unwrap());
        assert_eq!(init(&mut ctx), 0);
        assert_eq!(run(&mut tctx), 0);

        let w = scope["w"].0 as usize;
        let units: Vec<u16> = (0..16)
            .map(|i| u16::from_le_bytes([blob[w + 2 + i * 2], blob[w + 3 + i * 2]]))
            .collect();
        assert_eq!(u16::from_le_bytes([blob[w], blob[w + 1]]), 3, "len units");
        assert_eq!(&units[..3], &[0x61, 0xE4, 0x20AC], "UTF-16 aä€");
        assert_eq!(&units[3..], &[0u16; 13], "zero pad");
        assert_eq!(blob[scope["eq1"].0 as usize], 0, "aä€ = aäx is false");
        assert_eq!(blob[scope["eq2"].0 as usize], 1, "aä€ = aä€ is true");
    }
}

/// LEN/CONCAT builtins: narrow and wide, cross-cap store, truncation
/// clamping the len element.
#[test]
fn string_builtins_through_jit() {
    fn sl(s: &str) -> Expr {
        Expr {
            ty: Type::Str {
                cap: s.len() as u16,
            },
            kind: ExprKind::Lit(Lit::Str(s.into())),
            prov: Prov::None,
        }
    }
    fn wl(s: &str) -> Expr {
        Expr {
            ty: Type::WStr {
                cap: s.encode_utf16().count() as u16,
            },
            kind: ExprKind::Lit(Lit::WStr(s.into())),
            prov: Prov::None,
        }
    }
    let mut m = Module {
        pous: Vec::new(),
        gvls: Vec::new(),
        tasks: Vec::new(),
        checked_arith: false,
    };
    m.pous.push(Pou {
        kind: PouKind::Program,
        name: "F".into(),
        iface: vec![
            vd("c", Type::Str { cap: 32 }, None),
            vd("wc", Type::WStr { cap: 32 }, None),
            vd("small", Type::Str { cap: 8 }, None),
            vd(
                "n1",
                Type::Int {
                    signed: true,
                    bits: 32,
                },
                None,
            ),
            vd(
                "n2",
                Type::Int {
                    signed: true,
                    bits: 32,
                },
                None,
            ),
            vd("eq", Type::Bool, None),
        ],
        body: Body::Stmts(vec![
            assign(
                Ref::Var("c".into()),
                Expr {
                    ty: Type::Str { cap: 32 },
                    kind: ExprKind::Call {
                        fun: "CONCAT".into(),
                        args: vec![(String::new(), sl("abc")), (String::new(), sl("xyz"))],
                    },
                    prov: Prov::None,
                },
            ),
            assign(
                Ref::Var("wc".into()),
                Expr {
                    ty: Type::WStr { cap: 32 },
                    kind: ExprKind::Call {
                        fun: "CONCAT".into(),
                        args: vec![(String::new(), wl("aä")), (String::new(), wl("aä"))],
                    },
                    prov: Prov::None,
                },
            ),
            // truncating store: len must clamp to the target cap
            assign(Ref::Var("small".into()), sl("abcdefghij")),
            assign(
                Ref::Var("n1".into()),
                Expr {
                    ty: Type::Int {
                        signed: true,
                        bits: 32,
                    },
                    kind: ExprKind::Call {
                        fun: "LEN".into(),
                        args: vec![(String::new(), sl("abcd"))],
                    },
                    prov: Prov::None,
                },
            ),
            assign(
                Ref::Var("n2".into()),
                Expr {
                    ty: Type::Int {
                        signed: true,
                        bits: 32,
                    },
                    kind: ExprKind::Call {
                        fun: "LEN".into(),
                        args: vec![(
                            String::new(),
                            Expr {
                                ty: Type::WStr { cap: 32 },
                                kind: ExprKind::Ref(Ref::Var("wc".into())),
                                prov: Prov::None,
                            },
                        )],
                    },
                    prov: Prov::None,
                },
            ),
            assign(
                Ref::Var("eq".into()),
                bin(
                    BinOp::Eq,
                    Expr {
                        ty: Type::Str { cap: 32 },
                        kind: ExprKind::Ref(Ref::Var("c".into())),
                        prov: Prov::None,
                    },
                    sl("abcxyz"),
                    Type::Bool,
                ),
            ),
        ]),
        ret_ty: None,
        prov: Prov::None,
    });
    let l = layout_program(&m, &m.pous[0]);
    let m: &'static Module = Box::leak(Box::new(m));
    let l: &'static _ = Box::leak(Box::new(l));
    let program: &'static Pou = Box::leak(Box::new(m.pous[0].clone()));

    let lctx = Context::create();
    let module = emit_program(
        &lctx,
        m,
        l,
        program,
        &[],
        "fns",
        [0x46; 16],
        [0xDD; 32],
        None,
        10_000_000,
    );
    module
        .verify()
        .unwrap_or_else(|e| panic!("IR verify: {e}\n{}", module.print_to_string()));
    Target::initialize_native(&InitializationConfig::default()).unwrap();
    let ee = module.create_execution_engine().unwrap();

    let scope = program_ctx(l);
    let mut blob = vec![0u8; l.total_size as usize];
    let mut img = vec![0u8; 64];
    let mut ctx = FakeCtx {
        ss: 0,
        cf: 0,
        blob: blob.as_mut_ptr(),
        retain: std::ptr::null_mut(),
        iin: img.as_mut_ptr(),
        iout: unsafe { img.as_mut_ptr().add(32) },
        mk: std::ptr::null_mut(),
        now: 0,
        cyc: 0,
        ovr: 0,
        svc: std::ptr::null(),
        hp: std::ptr::null_mut(),
        abort: 0,
        pad: 0,
        dl: 0,
    };
    let mut tctx = FakeTCtx {
        prog: &mut ctx,
        idx: 0,
        fl: 0,
        tl: std::ptr::null_mut(),
        sn: 0,
        dl: 0,
        dt: 10_000_000,
    };
    unsafe {
        let init: extern "C" fn(*mut FakeCtx) -> i32 =
            std::mem::transmute(ee.get_function_address("tplc_init").unwrap());
        let run: extern "C" fn(*mut FakeTCtx) -> i32 =
            std::mem::transmute(ee.get_function_address("tplc_task_run").unwrap());
        assert_eq!(init(&mut ctx), 0);
        assert_eq!(run(&mut tctx), 0);

        let c = scope["c"].0 as usize;
        assert_eq!(blob[c], 6);
        assert_eq!(&blob[c + 1..c + 7], b"abcxyz");
        let wc = scope["wc"].0 as usize;
        assert_eq!(u16::from_le_bytes([blob[wc], blob[wc + 1]]), 4);
        let u = |i: usize| u16::from_le_bytes([blob[wc + 2 + i * 2], blob[wc + 3 + i * 2]]);
        assert_eq!(&[u(0), u(1), u(2), u(3)], &[0x61, 0xE4, 0x61, 0xE4]);
        let small = scope["small"].0 as usize;
        assert_eq!(blob[small], 8, "truncating store clamps len");
        assert_eq!(&blob[small + 1..small + 9], b"abcdefgh");
        assert_eq!(
            (blob.as_ptr().add(scope["n1"].0 as usize) as *const i32).read(),
            4
        );
        assert_eq!(
            (blob.as_ptr().add(scope["n2"].0 as usize) as *const i32).read(),
            4
        );
        assert_eq!(blob[scope["eq"].0 as usize], 1, "c = 'abcxyz'");
    }
}

/// Scalar builtins: ABS/MIN/MAX/SEL/LIMIT on ints (and ABS on LREAL).
#[test]
fn scalar_builtins_through_jit() {
    fn d(v: i64) -> Expr {
        Expr {
            ty: Type::Int {
                signed: true,
                bits: 32,
            },
            kind: ExprKind::Lit(Lit::Int(v)),
            prov: Prov::None,
        }
    }
    fn call(fun: &str, args: Vec<Expr>, ty: Type) -> Expr {
        Expr {
            ty,
            kind: ExprKind::Call {
                fun: fun.into(),
                args: args.into_iter().map(|e| (String::new(), e)).collect(),
            },
            prov: Prov::None,
        }
    }
    let dt = Type::Int {
        signed: true,
        bits: 32,
    };
    let mut m = Module {
        pous: Vec::new(),
        gvls: Vec::new(),
        tasks: Vec::new(),
        checked_arith: false,
    };
    m.pous.push(Pou {
        kind: PouKind::Program,
        name: "S".into(),
        iface: vec![
            vd("a", dt.clone(), None),
            vd("b", dt.clone(), None),
            vd("c", dt.clone(), None),
            vd("d", dt.clone(), None),
            vd("e", dt.clone(), None),
        ],
        body: Body::Stmts(vec![
            assign(Ref::Var("a".into()), call("ABS", vec![d(-5)], dt.clone())),
            assign(
                Ref::Var("b".into()),
                call("MIN", vec![d(3), d(7)], dt.clone()),
            ),
            assign(
                Ref::Var("c".into()),
                call("MAX", vec![d(3), d(7)], dt.clone()),
            ),
            assign(
                Ref::Var("d".into()),
                call(
                    "SEL",
                    vec![
                        Expr {
                            ty: Type::Bool,
                            kind: ExprKind::Lit(Lit::Bool(false)),
                            prov: Prov::None,
                        },
                        d(11),
                        d(22),
                    ],
                    dt.clone(),
                ),
            ),
            assign(
                Ref::Var("e".into()),
                call("LIMIT", vec![d(10), d(42), d(20)], dt.clone()),
            ),
        ]),
        ret_ty: None,
        prov: Prov::None,
    });
    let l = layout_program(&m, &m.pous[0]);
    let m: &'static Module = Box::leak(Box::new(m));
    let l: &'static _ = Box::leak(Box::new(l));
    let program: &'static Pou = Box::leak(Box::new(m.pous[0].clone()));

    let lctx = Context::create();
    let module = emit_program(
        &lctx,
        m,
        l,
        program,
        &[],
        "sc",
        [0x53; 16],
        [0xEE; 32],
        None,
        10_000_000,
    );
    module
        .verify()
        .unwrap_or_else(|e| panic!("IR verify: {e}\n{}", module.print_to_string()));
    Target::initialize_native(&InitializationConfig::default()).unwrap();
    let ee = module.create_execution_engine().unwrap();

    let scope = program_ctx(l);
    let mut blob = vec![0u8; l.total_size as usize];
    let mut img = vec![0u8; 64];
    let mut ctx = FakeCtx {
        ss: 0,
        cf: 0,
        blob: blob.as_mut_ptr(),
        retain: std::ptr::null_mut(),
        iin: img.as_mut_ptr(),
        iout: unsafe { img.as_mut_ptr().add(32) },
        mk: std::ptr::null_mut(),
        now: 0,
        cyc: 0,
        ovr: 0,
        svc: std::ptr::null(),
        hp: std::ptr::null_mut(),
        abort: 0,
        pad: 0,
        dl: 0,
    };
    let mut tctx = FakeTCtx {
        prog: &mut ctx,
        idx: 0,
        fl: 0,
        tl: std::ptr::null_mut(),
        sn: 0,
        dl: 0,
        dt: 10_000_000,
    };
    unsafe {
        let init: extern "C" fn(*mut FakeCtx) -> i32 =
            std::mem::transmute(ee.get_function_address("tplc_init").unwrap());
        let run: extern "C" fn(*mut FakeTCtx) -> i32 =
            std::mem::transmute(ee.get_function_address("tplc_task_run").unwrap());
        assert_eq!(init(&mut ctx), 0);
        assert_eq!(run(&mut tctx), 0);
        let get = |n: &str| (blob.as_ptr().add(scope[n].0 as usize) as *const i32).read();
        assert_eq!(get("a"), 5, "ABS(-5)");
        assert_eq!(get("b"), 3, "MIN(3,7)");
        assert_eq!(get("c"), 7, "MAX(3,7)");
        assert_eq!(get("d"), 11, "SEL(FALSE,11,22)");
        assert_eq!(get("e"), 20, "LIMIT(10,42,20)");
    }
}

/// LEFT/RIGHT/MID/INSERT/DELETE/REPLACE/FIND on narrow strings,
/// including wide variants and clamping.
#[test]
fn string_functions_through_jit() {
    fn sl(s: &str) -> Expr {
        Expr {
            ty: Type::Str {
                cap: s.len() as u16,
            },
            kind: ExprKind::Lit(Lit::Str(s.into())),
            prov: Prov::None,
        }
    }
    fn sv(n: &str) -> Expr {
        Expr {
            ty: Type::Str { cap: 16 },
            kind: ExprKind::Ref(Ref::Var(n.into())),
            prov: Prov::None,
        }
    }
    fn wl(s: &str) -> Expr {
        Expr {
            ty: Type::WStr {
                cap: s.encode_utf16().count() as u16,
            },
            kind: ExprKind::Lit(Lit::WStr(s.into())),
            prov: Prov::None,
        }
    }
    fn d(v: i64) -> Expr {
        Expr {
            ty: Type::Int {
                signed: true,
                bits: 32,
            },
            kind: ExprKind::Lit(Lit::Int(v)),
            prov: Prov::None,
        }
    }
    fn call(fun: &str, args: Vec<Expr>, ty: Type) -> Expr {
        Expr {
            ty,
            kind: ExprKind::Call {
                fun: fun.into(),
                args: args.into_iter().map(|e| (String::new(), e)).collect(),
            },
            prov: Prov::None,
        }
    }
    let st = Type::Str { cap: 16 };
    let wt = Type::WStr { cap: 16 };
    let mut m = Module {
        pous: Vec::new(),
        gvls: Vec::new(),
        tasks: Vec::new(),
        checked_arith: false,
    };
    m.pous.push(Pou {
        kind: PouKind::Program,
        name: "F".into(),
        iface: vec![
            vd("s", st.clone(), None),
            vd("c1", st.clone(), None),
            vd("c2", st.clone(), None),
            vd("c3", st.clone(), None),
            vd("c4", st.clone(), None),
            vd("c5", st.clone(), None),
            vd("c6", st.clone(), None),
            vd("w", wt.clone(), None),
            vd("wc", wt.clone(), None),
            vd("n1", d(0).ty.clone(), None),
            vd("n2", d(0).ty.clone(), None),
        ],
        body: Body::Stmts(vec![
            assign(Ref::Var("s".into()), sl("abcdef")),
            assign(
                Ref::Var("c1".into()),
                call("LEFT", vec![sv("s"), d(3)], st.clone()),
            ),
            assign(
                Ref::Var("c2".into()),
                call("RIGHT", vec![sv("s"), d(3)], st.clone()),
            ),
            assign(
                Ref::Var("c3".into()),
                call("MID", vec![sv("s"), d(2), d(3)], st.clone()),
            ),
            assign(
                Ref::Var("c4".into()),
                call("INSERT", vec![sv("s"), sl("XY"), d(4)], st.clone()),
            ),
            assign(
                Ref::Var("c5".into()),
                call("DELETE", vec![sv("s"), d(2), d(3)], st.clone()),
            ),
            assign(
                Ref::Var("c6".into()),
                call("REPLACE", vec![sv("s"), sl("XY"), d(2), d(3)], st.clone()),
            ),
            assign(Ref::Var("w".into()), wl("aä€z")),
            assign(
                Ref::Var("wc".into()),
                call(
                    "MID",
                    vec![
                        Expr {
                            ty: wt.clone(),
                            kind: ExprKind::Ref(Ref::Var("w".into())),
                            prov: Prov::None,
                        },
                        d(2),
                        d(2),
                    ],
                    wt.clone(),
                ),
            ),
            assign(
                Ref::Var("n1".into()),
                call("FIND", vec![sv("s"), sl("cd")], d(0).ty.clone()),
            ),
            assign(
                Ref::Var("n2".into()),
                call("FIND", vec![sv("s"), sl("zz")], d(0).ty.clone()),
            ),
        ]),
        ret_ty: None,
        prov: Prov::None,
    });
    let l = layout_program(&m, &m.pous[0]);
    let m: &'static Module = Box::leak(Box::new(m));
    let l: &'static _ = Box::leak(Box::new(l));
    let program: &'static Pou = Box::leak(Box::new(m.pous[0].clone()));

    let lctx = Context::create();
    let module = emit_program(
        &lctx,
        m,
        l,
        program,
        &[],
        "sf",
        [0x53; 16],
        [0xEF; 32],
        None,
        10_000_000,
    );
    module
        .verify()
        .unwrap_or_else(|e| panic!("IR verify: {e}\n{}", module.print_to_string()));
    Target::initialize_native(&InitializationConfig::default()).unwrap();
    let ee = module.create_execution_engine().unwrap();

    let scope = program_ctx(l);
    let mut blob = vec![0u8; l.total_size as usize];
    let mut img = vec![0u8; 64];
    let mut ctx = FakeCtx {
        ss: 0,
        cf: 0,
        blob: blob.as_mut_ptr(),
        retain: std::ptr::null_mut(),
        iin: img.as_mut_ptr(),
        iout: unsafe { img.as_mut_ptr().add(32) },
        mk: std::ptr::null_mut(),
        now: 0,
        cyc: 0,
        ovr: 0,
        svc: std::ptr::null(),
        hp: std::ptr::null_mut(),
        abort: 0,
        pad: 0,
        dl: 0,
    };
    let mut tctx = FakeTCtx {
        prog: &mut ctx,
        idx: 0,
        fl: 0,
        tl: std::ptr::null_mut(),
        sn: 0,
        dl: 0,
        dt: 10_000_000,
    };
    unsafe {
        let init: extern "C" fn(*mut FakeCtx) -> i32 =
            std::mem::transmute(ee.get_function_address("tplc_init").unwrap());
        let run: extern "C" fn(*mut FakeTCtx) -> i32 =
            std::mem::transmute(ee.get_function_address("tplc_task_run").unwrap());
        assert_eq!(init(&mut ctx), 0);
        assert_eq!(run(&mut tctx), 0);
        let s_of = |n: &str| scope[n].0 as usize;
        let chk = |n: &str, want: &str| {
            let o = s_of(n);
            assert_eq!(blob[o] as usize, want.len(), "{n} len");
            assert_eq!(&blob[o + 1..o + 1 + want.len()], want.as_bytes(), "{n}");
        };
        chk("c1", "abc");
        chk("c2", "def");
        chk("c3", "cd");
        chk("c4", "abcXYdef");
        chk("c5", "abef");
        chk("c6", "abXYef");
        let w = s_of("wc");
        assert_eq!(u16::from_le_bytes([blob[w], blob[w + 1]]), 2, "wide len");
        let u = |i: usize| u16::from_le_bytes([blob[w + 2 + i * 2], blob[w + 3 + i * 2]]);
        assert_eq!(&[u(0), u(1)], &[0xE4, 0x20AC], "wide MID aä€z[2..4]");
        assert_eq!(
            (blob.as_ptr().add(s_of("n1")) as *const i32).read(),
            3,
            "FIND cd"
        );
        assert_eq!(
            (blob.as_ptr().add(s_of("n2")) as *const i32).read(),
            0,
            "FIND zz"
        );
    }
}

/// TRUNC/ROUND and MUX through the compiled path, plus a REAL→LREAL
/// conversion (fpext) so a chained X_TO_Y assignment stores 8 bytes.
#[test]
fn conversions_and_mux_through_jit() {
    fn d(v: i64) -> Expr {
        Expr {
            ty: Type::Int {
                signed: true,
                bits: 32,
            },
            kind: ExprKind::Lit(Lit::Int(v)),
            prov: Prov::None,
        }
    }
    fn real(v: f64, bits: u8) -> Expr {
        Expr {
            ty: Type::Real { bits },
            kind: ExprKind::Lit(Lit::Real(v)),
            prov: Prov::None,
        }
    }
    fn call(fun: &str, args: Vec<Expr>, ty: Type) -> Expr {
        Expr {
            ty,
            kind: ExprKind::Call {
                fun: fun.into(),
                args: args.into_iter().map(|e| (String::new(), e)).collect(),
            },
            prov: Prov::None,
        }
    }
    let dt = Type::Int {
        signed: true,
        bits: 32,
    };
    let mut m = Module {
        pous: Vec::new(),
        gvls: Vec::new(),
        tasks: Vec::new(),
        checked_arith: false,
    };
    m.pous.push(Pou {
        kind: PouKind::Program,
        name: "C".into(),
        iface: vec![
            vd("t1", dt.clone(), None),
            vd("t2", dt.clone(), None),
            vd("ro", dt.clone(), None),
            vd("m1", dt.clone(), None),
            vd("m2", dt.clone(), None),
            vd("r", Type::Real { bits: 64 }, None),
        ],
        body: Body::Stmts(vec![
            assign(
                Ref::Var("t1".into()),
                call("TRUNC", vec![real(3.9, 64)], dt.clone()),
            ),
            assign(
                Ref::Var("t2".into()),
                call("TRUNC", vec![real(-3.9, 64)], dt.clone()),
            ),
            assign(
                Ref::Var("ro".into()),
                call("ROUND", vec![real(2.5, 64)], dt.clone()),
            ),
            assign(
                Ref::Var("m1".into()),
                call("MUX", vec![d(1), d(10), d(20), d(30)], dt.clone()),
            ),
            assign(
                Ref::Var("m2".into()),
                call("MUX", vec![d(2), d(10), d(20), d(30)], dt.clone()),
            ),
            // REAL → LREAL conversion as sema emits it: nested Convs
            assign(
                Ref::Var("r".into()),
                Expr {
                    ty: Type::Real { bits: 64 },
                    kind: ExprKind::Conv {
                        to: Type::Real { bits: 64 },
                        e: Box::new(Expr {
                            ty: Type::Real { bits: 32 },
                            kind: ExprKind::Lit(Lit::Real(42.0)),
                            prov: Prov::None,
                        }),
                    },
                    prov: Prov::None,
                },
            ),
        ]),
        ret_ty: None,
        prov: Prov::None,
    });
    let l = layout_program(&m, &m.pous[0]);
    let m: &'static Module = Box::leak(Box::new(m));
    let l: &'static _ = Box::leak(Box::new(l));
    let program: &'static Pou = Box::leak(Box::new(m.pous[0].clone()));

    let lctx = Context::create();
    let module = emit_program(
        &lctx,
        m,
        l,
        program,
        &[],
        "cv",
        [0x43; 16],
        [0x11; 32],
        None,
        10_000_000,
    );
    module
        .verify()
        .unwrap_or_else(|e| panic!("IR verify: {e}\n{}", module.print_to_string()));
    Target::initialize_native(&InitializationConfig::default()).unwrap();
    let ee = module.create_execution_engine().unwrap();

    let scope = program_ctx(l);
    let mut blob = vec![0u8; l.total_size as usize];
    let mut img = vec![0u8; 64];
    let mut ctx = FakeCtx {
        ss: 0,
        cf: 0,
        blob: blob.as_mut_ptr(),
        retain: std::ptr::null_mut(),
        iin: img.as_mut_ptr(),
        iout: unsafe { img.as_mut_ptr().add(32) },
        mk: std::ptr::null_mut(),
        now: 0,
        cyc: 0,
        ovr: 0,
        svc: std::ptr::null(),
        hp: std::ptr::null_mut(),
        abort: 0,
        pad: 0,
        dl: 0,
    };
    let mut tctx = FakeTCtx {
        prog: &mut ctx,
        idx: 0,
        fl: 0,
        tl: std::ptr::null_mut(),
        sn: 0,
        dl: 0,
        dt: 10_000_000,
    };
    unsafe {
        let init: extern "C" fn(*mut FakeCtx) -> i32 =
            std::mem::transmute(ee.get_function_address("tplc_init").unwrap());
        let run: extern "C" fn(*mut FakeTCtx) -> i32 =
            std::mem::transmute(ee.get_function_address("tplc_task_run").unwrap());
        assert_eq!(init(&mut ctx), 0);
        assert_eq!(run(&mut tctx), 0);
        let gi = |n: &str| (blob.as_ptr().add(scope[n].0 as usize) as *const i32).read();
        assert_eq!(gi("t1"), 3, "TRUNC(3.9)");
        assert_eq!(gi("t2"), -3, "TRUNC(-3.9)");
        assert_eq!(gi("ro"), 3, "ROUND(2.5)");
        assert_eq!(gi("m1"), 20, "MUX(1,..)");
        assert_eq!(gi("m2"), 30, "MUX(2,..)");
        let rf = (blob.as_ptr().add(scope["r"].0 as usize) as *const f64).read();
        assert_eq!(rf, 42.0, "REAL→LREAL widened to f64");
    }
}

/// REFERENCE TO: REF= binds a blob offset, auto-deref reads through,
/// ISVALIDREF reports null, and a null deref faults with -17.
#[test]
fn reference_semantics_through_jit() {
    fn d(v: i64) -> Expr {
        Expr {
            ty: Type::Int {
                signed: true,
                bits: 32,
            },
            kind: ExprKind::Lit(Lit::Int(v)),
            prov: Prov::None,
        }
    }
    let dt = Type::Int {
        signed: true,
        bits: 32,
    };
    let rt = Type::Ref {
        to: Box::new(dt.clone()),
    };
    let mut m = Module {
        pous: Vec::new(),
        gvls: Vec::new(),
        tasks: Vec::new(),
        checked_arith: false,
    };
    m.pous.push(Pou {
        kind: PouKind::Program,
        name: "R".into(),
        iface: vec![
            vd("x", dt.clone(), Some(d(42))),
            vd("r", rt.clone(), None),
            vd("n", rt.clone(), None),
            vd("out", dt.clone(), None),
            vd("ok", Type::Bool, None),
            vd("valid", Type::Bool, None),
            vd(
                "bind",
                Type::Bool,
                Some(Expr {
                    ty: Type::Bool,
                    kind: ExprKind::Lit(Lit::Bool(true)),
                    prov: Prov::None,
                }),
            ),
        ],
        body: Body::Stmts(vec![
            stmt(StmtKind::If {
                branches: vec![(
                    Expr {
                        ty: Type::Bool,
                        kind: ExprKind::Ref(Ref::Var("bind".into())),
                        prov: Prov::None,
                    },
                    vec![assign(
                        Ref::Var("r".into()),
                        Expr {
                            ty: rt.clone(),
                            kind: ExprKind::RefTo {
                                r: Ref::Var("x".into()),
                            },
                            prov: Prov::None,
                        },
                    )],
                )],
                else_: vec![],
            }),
            assign(
                Ref::Var("out".into()),
                Expr {
                    ty: dt.clone(),
                    kind: ExprKind::Ref(Ref::Deref(Box::new(Ref::Var("r".into())))),
                    prov: Prov::None,
                },
            ),
            assign(
                Ref::Var("ok".into()),
                bin(
                    BinOp::Eq,
                    Expr {
                        ty: dt.clone(),
                        kind: ExprKind::Ref(Ref::Deref(Box::new(Ref::Var("r".into())))),
                        prov: Prov::None,
                    },
                    d(42),
                    Type::Bool,
                ),
            ),
            assign(
                Ref::Var("valid".into()),
                Expr {
                    ty: Type::Bool,
                    kind: ExprKind::Call {
                        fun: "ISVALIDREF".into(),
                        args: vec![(
                            String::new(),
                            Expr {
                                ty: rt.clone(),
                                kind: ExprKind::Ref(Ref::Var("r".into())),
                                prov: Prov::None,
                            },
                        )],
                    },
                    prov: Prov::None,
                },
            ),
        ]),
        ret_ty: None,
        prov: Prov::None,
    });
    let l = layout_program(&m, &m.pous[0]);
    let m: &'static Module = Box::leak(Box::new(m));
    let l: &'static _ = Box::leak(Box::new(l));
    let program: &'static Pou = Box::leak(Box::new(m.pous[0].clone()));

    let lctx = Context::create();
    let module = emit_program(
        &lctx,
        m,
        l,
        program,
        &[],
        "rf",
        [0x52; 16],
        [0x77; 32],
        None,
        10_000_000,
    );
    module
        .verify()
        .unwrap_or_else(|e| panic!("IR verify: {e}\n{}", module.print_to_string()));
    Target::initialize_native(&InitializationConfig::default()).unwrap();
    let ee = module.create_execution_engine().unwrap();

    let scope = program_ctx(l);
    let mut blob = vec![0u8; l.total_size as usize];
    let mut img = vec![0u8; 64];
    let mut ctx = FakeCtx {
        ss: 0,
        cf: 0,
        blob: blob.as_mut_ptr(),
        retain: std::ptr::null_mut(),
        iin: img.as_mut_ptr(),
        iout: unsafe { img.as_mut_ptr().add(32) },
        mk: std::ptr::null_mut(),
        now: 0,
        cyc: 0,
        ovr: 0,
        svc: std::ptr::null(),
        hp: std::ptr::null_mut(),
        abort: 0,
        pad: 0,
        dl: 0,
    };
    let mut tctx = FakeTCtx {
        prog: &mut ctx,
        idx: 0,
        fl: 0,
        tl: std::ptr::null_mut(),
        sn: 0,
        dl: 0,
        dt: 10_000_000,
    };
    unsafe {
        let init: extern "C" fn(*mut FakeCtx) -> i32 =
            std::mem::transmute(ee.get_function_address("tplc_init").unwrap());
        let run: extern "C" fn(*mut FakeTCtx) -> i32 =
            std::mem::transmute(ee.get_function_address("tplc_task_run").unwrap());
        assert_eq!(init(&mut ctx), 0);
        assert_eq!(run(&mut tctx), 0);
        let gi = |n: &str| (blob.as_ptr().add(scope[n].0 as usize) as *const i32).read();
        assert_eq!(gi("out"), 42, "auto-deref read");
        assert_eq!(blob[scope["ok"].0 as usize], 1, "r^ = 42");
        assert_eq!(blob[scope["valid"].0 as usize], 1, "ISVALIDREF");
        let r_off = (blob.as_ptr().add(scope["r"].0 as usize) as *const u64).read();
        assert_eq!(r_off, scope["x"].0, "ref holds x's blob offset");

        /* a ref past the blob end faults BOUNDS (-18); null faults
        PANIC (-17) — both are checked before the load */
        blob[scope["bind"].0 as usize] = 0; /* stop re-binding */
        let rp = blob.as_mut_ptr().add(scope["r"].0 as usize) as *mut u64;
        rp.write(l.total_size);
        assert_eq!(run(&mut tctx), -18, "out-of-range deref → BOUNDS");
        rp.write(u64::MAX);
        assert_eq!(run(&mut tctx), -17, "null deref → PANIC");
    }
}

/// Checked-arithmetic mode (`Module::checked_arith`, tplcc
/// --checked-arith): integer div/mod by zero aborts the scan with
/// TPLC_TASK_E_DIVZERO (-19): the faulting statement's store never
/// lands, later statements don't run, and the next scan recovers.
/// Saturating mode (default) keeps the select→0 guard.
fn divmod_scenario(checked: bool) {
    let dt = Type::Int {
        signed: true,
        bits: 32,
    };
    let mut m = Module {
        pous: Vec::new(),
        gvls: Vec::new(),
        tasks: Vec::new(),
        checked_arith: checked,
    };
    m.pous.push(Pou {
        kind: PouKind::Program,
        name: "D".into(),
        iface: vec![
            vd("d", dt.clone(), None),
            vd("e", dt.clone(), None),
            vd("q", dt.clone(), None),
            vd("r", dt.clone(), None),
        ],
        body: Body::Stmts(vec![
            assign(
                Ref::Var("q".into()),
                bin(BinOp::Div, int_e(100), int_v("d"), dt.clone()),
            ),
            assign(
                Ref::Var("r".into()),
                bin(BinOp::Mod, int_e(103), int_v("e"), dt.clone()),
            ),
        ]),
        ret_ty: None,
        prov: Prov::None,
    });
    let l = layout_program(&m, &m.pous[0]);
    let m: &'static Module = Box::leak(Box::new(m));
    let l: &'static _ = Box::leak(Box::new(l));
    let program: &'static Pou = Box::leak(Box::new(m.pous[0].clone()));

    let lctx = Context::create();
    let module = emit_program(
        &lctx,
        m,
        l,
        program,
        &[],
        "dm",
        [0x44; 16],
        [0xDD; 32],
        None,
        10_000_000,
    );
    module
        .verify()
        .unwrap_or_else(|e| panic!("IR verify: {e}\n{}", module.print_to_string()));
    Target::initialize_native(&InitializationConfig::default()).unwrap();
    let ee = module.create_execution_engine().unwrap();

    let scope = program_ctx(l);
    let mut blob = vec![0u8; l.total_size as usize];
    let mut img = vec![0u8; 64];
    let mut ctx = FakeCtx {
        ss: 0,
        cf: 0,
        blob: blob.as_mut_ptr(),
        retain: std::ptr::null_mut(),
        iin: img.as_mut_ptr(),
        iout: unsafe { img.as_mut_ptr().add(32) },
        mk: std::ptr::null_mut(),
        now: 0,
        cyc: 0,
        ovr: 0,
        svc: std::ptr::null(),
        hp: std::ptr::null_mut(),
        abort: 0,
        pad: 0,
        dl: 0,
    };
    let mut tctx = FakeTCtx {
        prog: &mut ctx,
        idx: 0,
        fl: 0,
        tl: std::ptr::null_mut(),
        sn: 0,
        dl: 0,
        dt: 10_000_000,
    };
    unsafe {
        let init: extern "C" fn(*mut FakeCtx) -> i32 =
            std::mem::transmute(ee.get_function_address("tplc_init").unwrap());
        let run: extern "C" fn(*mut FakeTCtx) -> i32 =
            std::mem::transmute(ee.get_function_address("tplc_task_run").unwrap());
        assert_eq!(init(&mut ctx), 0);
        let bp = blob.as_mut_ptr();
        let poke = |n: &str, v: i32| (bp.add(scope[n].0 as usize) as *mut i32).write(v);
        let get = |n: &str| (bp.add(scope[n].0 as usize) as *const i32).read();

        poke("d", 2);
        poke("e", 3);
        assert_eq!(run(&mut tctx), 0);
        assert_eq!(get("q"), 50, "100/2");
        assert_eq!(get("r"), 1, "103%3");

        if checked {
            /* d=0 → the Div faults before its store: q keeps the
            previous value and the Mod statement never runs */
            poke("d", 0);
            assert_eq!(run(&mut tctx), -19, "div by zero → DIVZERO");
            assert_eq!(get("q"), 50, "faulted store did not land");
            assert_eq!(get("r"), 1, "later statement skipped");

            /* e=0 with a healthy divisor → the Mod faults after q
            was already updated this scan */
            poke("d", 2);
            poke("e", 0);
            assert_eq!(run(&mut tctx), -19, "mod by zero → DIVZERO");
            assert_eq!(get("q"), 50, "q updated before the fault");
            assert_eq!(get("r"), 1, "mod store did not land");

            /* the fault is per-scan, not sticky */
            poke("d", 4);
            poke("e", 5);
            assert_eq!(run(&mut tctx), 0, "recovered");
            assert_eq!(get("q"), 25, "100/4");
            assert_eq!(get("r"), 3, "103%5");
        } else {
            poke("d", 0);
            poke("e", 0);
            assert_eq!(run(&mut tctx), 0, "saturating mode never faults");
            assert_eq!(get("q"), 0, "div saturated to 0");
            assert_eq!(get("r"), 0, "mod saturated to 0");
        }
    }
}

#[test]
fn checked_arith_divzero_jit() {
    divmod_scenario(true);
}

#[test]
fn saturating_arith_divmod_jit() {
    divmod_scenario(false);
}

/// Checked-arithmetic overflow (-20) and conversion range (-21):
/// int Add/Sub/Mul/Neg leaving the type's range, signed INT_MIN/-1
/// division, and narrowing or real→int conversions fault the scan.
/// Saturating mode wraps/truncates as before.
fn ovf_rng_scenario(checked: bool) {
    let i16 = Type::Int {
        signed: true,
        bits: 16,
    };
    let i32 = Type::Int {
        signed: true,
        bits: 32,
    };
    let f64t = Type::Real { bits: 64 };
    let ev = |n: &str, t: Type| Expr {
        ty: t.clone(),
        kind: ExprKind::Ref(Ref::Var(n.into())),
        prov: Prov::None,
    };
    let el = |v: i64, t: Type| Expr {
        ty: t,
        kind: ExprKind::Lit(Lit::Int(v)),
        prov: Prov::None,
    };
    let conv = |to: Type, inner: Expr| Expr {
        ty: to.clone(),
        kind: ExprKind::Conv {
            to,
            e: Box::new(inner),
        },
        prov: Prov::None,
    };
    let mut m = Module {
        pous: Vec::new(),
        gvls: Vec::new(),
        tasks: Vec::new(),
        checked_arith: checked,
    };
    m.pous.push(Pou {
        kind: PouKind::Program,
        name: "O".into(),
        iface: vec![
            vd("a", i32.clone(), None),
            vd("b", i32.clone(), None),
            vd("c", i32.clone(), None),
            vd("d", i32.clone(), None),
            vd("s", i16.clone(), None),
            vd("n", i16.clone(), None),
            vd("r", f64t.clone(), None),
            vd("qa", i32.clone(), None),
            vd("qb", i32.clone(), None),
            vd("qd", i16.clone(), None),
            vd("qe", i32.clone(), None),
            vd("qf", i32.clone(), None),
            vd("qg", i32.clone(), None),
        ],
        body: Body::Stmts(vec![
            assign(
                Ref::Var("qa".into()),
                bin(
                    BinOp::Add,
                    ev("a", i32.clone()),
                    ev("b", i32.clone()),
                    i32.clone(),
                ),
            ),
            assign(
                Ref::Var("qb".into()),
                conv(
                    i32.clone(),
                    bin(
                        BinOp::Mul,
                        ev("s", i16.clone()),
                        el(1000, i16.clone()),
                        i16.clone(),
                    ),
                ),
            ),
            assign(
                Ref::Var("qd".into()),
                Expr {
                    ty: i16.clone(),
                    kind: ExprKind::Un(UnOp::Neg, Box::new(ev("n", i16.clone()))),
                    prov: Prov::None,
                },
            ),
            assign(
                Ref::Var("qe".into()),
                conv(i32.clone(), conv(i16.clone(), ev("a", i32.clone()))),
            ),
            assign(
                Ref::Var("qf".into()),
                conv(i32.clone(), ev("r", f64t.clone())),
            ),
            assign(
                Ref::Var("qg".into()),
                bin(
                    BinOp::Div,
                    ev("c", i32.clone()),
                    ev("d", i32.clone()),
                    i32.clone(),
                ),
            ),
        ]),
        ret_ty: None,
        prov: Prov::None,
    });
    let l = layout_program(&m, &m.pous[0]);
    let m: &'static Module = Box::leak(Box::new(m));
    let l: &'static _ = Box::leak(Box::new(l));
    let program: &'static Pou = Box::leak(Box::new(m.pous[0].clone()));

    let lctx = Context::create();
    let module = emit_program(
        &lctx,
        m,
        l,
        program,
        &[],
        "ov",
        [0x44; 16],
        [0xDD; 32],
        None,
        10_000_000,
    );
    module
        .verify()
        .unwrap_or_else(|e| panic!("IR verify: {e}\n{}", module.print_to_string()));
    Target::initialize_native(&InitializationConfig::default()).unwrap();
    let ee = module.create_execution_engine().unwrap();

    let scope = program_ctx(l);
    let mut blob = vec![0u8; l.total_size as usize];
    let mut img = vec![0u8; 64];
    let mut ctx = FakeCtx {
        ss: 0,
        cf: 0,
        blob: blob.as_mut_ptr(),
        retain: std::ptr::null_mut(),
        iin: img.as_mut_ptr(),
        iout: unsafe { img.as_mut_ptr().add(32) },
        mk: std::ptr::null_mut(),
        now: 0,
        cyc: 0,
        ovr: 0,
        svc: std::ptr::null(),
        hp: std::ptr::null_mut(),
        abort: 0,
        pad: 0,
        dl: 0,
    };
    let mut tctx = FakeTCtx {
        prog: &mut ctx,
        idx: 0,
        fl: 0,
        tl: std::ptr::null_mut(),
        sn: 0,
        dl: 0,
        dt: 10_000_000,
    };
    unsafe {
        let init: extern "C" fn(*mut FakeCtx) -> i32 =
            std::mem::transmute(ee.get_function_address("tplc_init").unwrap());
        let run: extern "C" fn(*mut FakeTCtx) -> i32 =
            std::mem::transmute(ee.get_function_address("tplc_task_run").unwrap());
        assert_eq!(init(&mut ctx), 0);
        let bp = blob.as_mut_ptr();
        macro_rules! p32 {
            ($n:expr, $v:expr) => {
                (bp.add(scope[$n].0 as usize) as *mut i32).write($v)
            };
        }
        macro_rules! p16 {
            ($n:expr, $v:expr) => {
                (bp.add(scope[$n].0 as usize) as *mut i16).write($v)
            };
        }
        macro_rules! pf {
            ($n:expr, $v:expr) => {
                (bp.add(scope[$n].0 as usize) as *mut f64).write($v)
            };
        }
        macro_rules! g32 {
            ($n:expr) => {
                (bp.add(scope[$n].0 as usize) as *const i32).read()
            };
        }
        macro_rules! g16 {
            ($n:expr) => {
                (bp.add(scope[$n].0 as usize) as *const i16).read()
            };
        }

        p32!("a", 10);
        p32!("b", 3);
        p32!("c", 20);
        p32!("d", 4);
        p16!("s", 2);
        p16!("n", 5);
        pf!("r", 3.7);
        assert_eq!(run(&mut tctx), 0);
        assert_eq!(g32!("qa"), 13);
        assert_eq!(g32!("qb"), 2000);
        assert_eq!(g16!("qd"), -5);
        assert_eq!(g32!("qe"), 10);
        assert_eq!(g32!("qf"), 3);
        assert_eq!(g32!("qg"), 5);

        if checked {
            /* DINT add overflow → -20; the store never lands */
            p32!("a", 0x40000000);
            p32!("b", 0x40000000);
            assert_eq!(run(&mut tctx), -20, "i32 add overflow");
            assert_eq!(g32!("qa"), 13, "faulted store kept old value");

            /* INT mul overflow → -20 (earlier stmts still ran) */
            p32!("a", 10);
            p32!("b", 3);
            p16!("s", 100);
            assert_eq!(run(&mut tctx), -20, "i16 mul overflow");
            assert_eq!(g32!("qa"), 13);

            /* INT_MIN negation → -20 */
            p16!("s", 2);
            p16!("n", -32768);
            assert_eq!(run(&mut tctx), -20, "i16 neg overflow");

            /* DINT→INT out of range → -21 */
            p16!("n", 5);
            p32!("a", 70000);
            assert_eq!(run(&mut tctx), -21, "narrowing conv range");
            assert_eq!(g32!("qa"), 70003, "earlier stmts ran");

            /* LREAL→DINT out of range → -21 */
            p32!("a", 10);
            pf!("r", 1e20);
            assert_eq!(run(&mut tctx), -21, "real→int range");

            /* signed INT_MIN / -1 → -20 */
            pf!("r", 3.7);
            p32!("c", i32::MIN);
            p32!("d", -1);
            assert_eq!(run(&mut tctx), -20, "MIN/-1 div overflow");

            /* per-scan, not sticky */
            p32!("c", 20);
            p32!("d", 4);
            assert_eq!(run(&mut tctx), 0, "recovered");
            assert_eq!(g32!("qg"), 5);
        } else {
            /* saturating mode: wrap/truncate, never fault */
            p32!("a", 0x40000000);
            p32!("b", 0x40000000);
            p16!("s", 100);
            p16!("n", -32768);
            pf!("r", -5.9);
            p32!("c", i32::MIN);
            p32!("d", -1);
            assert_eq!(run(&mut tctx), 0, "saturating mode never faults");
            assert_eq!(g32!("qa"), i32::MIN, "add wrapped");
            assert_eq!(g32!("qb"), -31072, "mul wrapped to i16");
            assert_eq!(g16!("qd"), -32768, "neg wrapped");
            assert_eq!(g32!("qf"), -5, "real→int trunc");
            assert_eq!(g32!("qg"), i32::MIN, "MIN/-1 wrapped to MIN");
            /* narrowing conv truncates: 70000 → i16 4464 */
            p32!("a", 70000);
            p32!("b", 3);
            p32!("c", 20);
            p32!("d", 4);
            p16!("s", 2);
            p16!("n", 5);
            assert_eq!(run(&mut tctx), 0);
            assert_eq!(g32!("qe"), 4464, "conv truncated to i16");
        }
    }
}

#[test]
fn checked_arith_overflow_range_jit() {
    ovf_rng_scenario(true);
}

#[test]
fn saturating_arith_overflow_range_jit() {
    ovf_rng_scenario(false);
}
