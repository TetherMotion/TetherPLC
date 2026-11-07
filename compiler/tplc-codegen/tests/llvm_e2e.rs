#![cfg(feature = "llvm")]
//! MIR → LLVM IR → JIT → ABI ctx → blob assertions.
//! The host-side __tplc_rt_TON stands in for tplc-rt bitcode.

use inkwell::context::Context;
use inkwell::targets::{InitializationConfig, Target};
use tplc_codegen::llvm_emit::emit_program;
use tplc_mir::interp::program_ctx;
use tplc_mir::ir::*;
use tplc_mir::layout::layout_program;

fn int_e(v: i64) -> Expr {
    Expr {
        ty: Type::Int { signed: true, bits: 32 },
        kind: ExprKind::Lit(Lit::Int(v)),
        prov: Prov::None,
    }
}
fn int_v(n: &str) -> Expr {
    Expr {
        ty: Type::Int { signed: true, bits: 32 },
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
    Stmt { prov: Prov::None, kind: k }
}
fn assign(lhs: Ref, rhs: Expr) -> Stmt {
    stmt(StmtKind::Assign { lhs, rhs })
}
fn bin(op: BinOp, a: Expr, b: Expr, ty: Type) -> Expr {
    Expr { ty, kind: ExprKind::Bin(op, Box::new(a), Box::new(b)), prov: Prov::None }
}
fn vd(n: &str, ty: Type, init: Option<Expr>) -> VarDecl {
    VarDecl {
        name: n.into(), kind: VarKind::Local, ty, init,
        retain: false, at: None, probe: false, prov: Prov::None,
    }
}

fn demo_module() -> Module {
    let mut m = Module { pous: Vec::new(), gvls: Vec::new(), tasks: Vec::new() };
    m.pous.push(Pou {
        kind: PouKind::Program,
        name: "P".into(),
        iface: vec![
            vd("x", Type::Int { signed: true, bits: 32 }, None),
            vd("n", Type::Int { signed: true, bits: 32 }, Some(int_e(10))),
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
                    assign(Ref::Var("x".into()),
                        bin(BinOp::Add, int_v("x"), int_e(1),
                            Type::Int { signed: true, bits: 32 })),
                    assign(Ref::Var("n".into()),
                        bin(BinOp::Sub, int_v("n"), int_e(1),
                            Type::Int { signed: true, bits: 32 })),
                ],
            }),
            assign(Ref::Var("y".into()),
                bin(BinOp::Eq, int_v("x"), int_e(10), Type::Bool)),
            stmt(StmtKind::Call {
                inst: Ref::Var("ton1".into()),
                args: vec![
                    ("IN".into(), bool_v("y")),
                    ("PT".into(), Expr {
                        ty: Type::Time,
                        kind: ExprKind::Lit(Lit::TimeNs(500_000_000)),
                        prov: Prov::None,
                    }),
                ],
            }),
            assign(Ref::Var("q".into()), Expr {
                ty: Type::Bool,
                kind: ExprKind::Ref(Ref::Field(
                    Box::new(Ref::Var("ton1".into())), "Q".into())),
                prov: Prov::None,
            }),
        ]),
        ret_ty: None,
        prov: Prov::None,
    });
    m
}

// fake ABI ctx (tplc_abi.h field order)
#[repr(C)]
struct FakeCtx {
    ss: u32, cf: u32,
    blob: *mut u8, retain: *mut u8, iin: *mut u8, iout: *mut u8, mk: *mut u8,
    now: u64, cyc: u64, ovr: i64,
    svc: *const u8, hp: *mut u8,
    abort: u32, pad: u32, dl: u64,
}
#[repr(C)]
struct FakeTCtx {
    prog: *mut FakeCtx, idx: u32, fl: u32, tl: *mut u8,
    sn: u64, dl: u64, dt: u64,
}

/// host TON: IN@0 i8, PT@8 i64, Q@16 i8, ET@24 i64, run@32 i8, t0@40 i64
/// — must match builtin_fb_layout("TON") member offsets.
unsafe extern "C" fn host_ton(this: *mut u8, ctx: *mut u8) -> i32 {
    let now = (ctx.add(48) as *const u64).read();
    let inn = this.add(0).read();
    let pt = (this.add(8) as *const i64).read();
    let run = this.add(32);
    if inn != 0 {
        if run.read() == 0 {
            run.write(1);
            (this.add(40) as *mut u64).write(now);
        }
        let el = now.saturating_sub((this.add(40) as *const u64).read()) as i64;
        (this.add(24) as *mut i64).write(el.min(pt));
        this.add(16).write((el >= pt) as u8);
    } else {
        run.write(0);
        (this.add(24) as *mut i64).write(0);
        this.add(16).write(0);
    }
    0
}

#[test]
fn llvm_lowering_executes_through_jit() {
    let m = demo_module();
    let l = layout_program(&m, &m.pous[0]);
    let m: &'static Module = Box::leak(Box::new(m));
    let l: &'static _ = Box::leak(Box::new(l));
    let program: &'static Pou = Box::leak(Box::new(m.pous[0].clone()));

    let lctx = Context::create();
    let module = emit_program(&lctx, m, l, program);
    std::fs::write("/tmp/gen.ll", module.print_to_string().to_string()).unwrap();
    module.verify().unwrap_or_else(|e| panic!("IR verify: {e}"));
    let ir = module.print_to_string().to_string();
    assert!(ir.contains("define i32 @tplc_task_run"));
    assert!(ir.contains("define i32 @__tplc_pou_P"));
    assert!(ir.contains("declare i32 @__tplc_rt_TON"));
    assert!(!ir.contains("inttoptr"), "no absolute pointers in program state");

    Target::initialize_native(&InitializationConfig::default()).unwrap();
    let ee = module.create_execution_engine().unwrap();
    let ton = module.get_function("__tplc_rt_TON").unwrap();
    ee.add_global_mapping(&ton, host_ton as *const () as usize);

    let scope = program_ctx(l);
    let mut blob = vec![0u8; l.total_size as usize];
    let mut img = vec![0u8; 64];
    let mut ctx = FakeCtx {
        ss: 0, cf: 0,
        blob: blob.as_mut_ptr(), retain: std::ptr::null_mut(),
        iin: img.as_mut_ptr(), iout: unsafe { img.as_mut_ptr().add(32) },
        mk: std::ptr::null_mut(),
        now: 0, cyc: 0, ovr: 0,
        svc: std::ptr::null(), hp: std::ptr::null_mut(),
        abort: 0, pad: 0, dl: 0,
    };
    let mut tctx = FakeTCtx {
        prog: &mut ctx, idx: 0, fl: 0, tl: std::ptr::null_mut(),
        sn: 0, dl: 0, dt: 100_000_000,
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
        ctx.abort = 1;
        let _ = ctx.abort;
        assert_eq!(run(&mut tctx), -1, "cooperative abort propagates");
    }
}
