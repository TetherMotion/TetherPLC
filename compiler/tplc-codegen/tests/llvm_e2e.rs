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
    emit_object(&module, &obj).unwrap();
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
        &lctx, m, l, program, &[], "str", [0x52; 16], [0xBB; 32], None, 10_000_000,
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
