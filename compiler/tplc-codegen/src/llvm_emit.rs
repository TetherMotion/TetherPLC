//! `tplc-mir` → LLVM IR lowering (doc 07 §4).
//!
//! Memory model: every access is `i8* + const-offset` into ctx fields
//! and the state blob — identical addressing to the interpreter
//! (interp.rs), so oracle/sim parity is structural, not aspirational.
//!
//! Ctx layout (tplc_abi.h): member offsets are computed per target
//! from the data layout (AbiOffsets) — state_blob@8 on both LP64 and
//! AAPCS32, but svc/abort/deadline shift on 32-bit targets.
//! TplcTaskCtx: prog@0, task_index@ptr_size.
//!
//! Generated pou fns: `__tplc_pou_<NAME>(u8* ctx, u8* this) -> i32`
//! rc<0 = abort/fault propagation. Program `this` = state_blob base;
//! FB `this` = instance sub-object; FUNCTION `this` = caller alloca.

use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::intrinsics::Intrinsic;
use inkwell::module::{Linkage, Module as LlvmModule};
use inkwell::types::BasicTypeEnum;
use inkwell::values::{BasicValue, BasicValueEnum, FunctionValue, IntValue, PointerValue};
use inkwell::{FloatPredicate, IntPredicate};
use std::collections::BTreeMap;
use tplc_mir::interp::{fb_ctx, pou_scope, program_ctx, Ctx};
use tplc_mir::ir::AtRegion;
use tplc_mir::ir::*;
use tplc_mir::layout::{builtin_fb_layout, Layout};
use tplc_mir::migrate::{MigOp, MigrateEmit};

/// Member offsets into the host-allocated ABI structs (tplc_abi.h),
/// computed from the target data layout so generated GEPs match the C
/// layout on 32- and 64-bit targets alike. Field order mirrors the
/// headers; only the members codegen touches are named.
mod abi;
mod access;
mod calls;
mod emit;
mod expr;
mod stmt;

use abi::*;

pub struct Gen<'ctx> {
    pub ctx: &'ctx Context,
    pub module: LlvmModule<'ctx>,
    pub b: Builder<'ctx>,
    layout: &'static Layout,
    mir: &'static Module,
    abort_bb: Option<BasicBlock<'ctx>>,
    /// RETURN statement target: normal exit rc=0 (IEC RETURN ends
    /// the invocation; it is not an abort)
    retok_bb: Option<BasicBlock<'ctx>>,
    /// name of the FUNCTION pou being emitted (RETURN <expr>
    /// writes the result member named after the pou)
    cur_fn: Option<String>,
    overrun_bb: Option<BasicBlock<'ctx>>,
    /// null-reference deref → -17, out-of-range deref → -18
    panic_bb: Option<BasicBlock<'ctx>>,
    bounds_bb: Option<BasicBlock<'ctx>>,
    /// checked-arith int div/mod by zero → -19 (saturating mode
    /// never branches here)
    divzero_bb: Option<BasicBlock<'ctx>>,
    /// checked-arith int overflow → -20, conversion range → -21
    overflow_bb: Option<BasicBlock<'ctx>>,
    range_bb: Option<BasicBlock<'ctx>>,
    loop_stack: Vec<(BasicBlock<'ctx>, BasicBlock<'ctx>)>, // (cond, exit)
    ctx_ptr: PointerValue<'ctx>,
    this_ptr: PointerValue<'ctx>,
    abi: AbiOffsets,
    /// VAR_IN_OUT names of the pou currently being emitted — their
    /// Ref-typed member slots hold caller-storage pointers and any
    /// access auto-derefs (see resolve Ref::Var).
    cur_inouts: std::collections::HashSet<String>,
}

/// Emit the module: abi_version, init, shutdown, task_run + per-POU fns.
/// Field offsets in TplcMigrateCtx (doc 02 ABI).

#[allow(clippy::too_many_arguments)]
pub fn emit_program<'ctx>(
    lctx: &'ctx Context,
    mir: &'static Module,
    layout: &'static Layout,
    program: &'static Pou,
    io_binds: &[IoBind],
    program_name: &str,
    uuid: [u8; 16],
    layout_hash: [u8; 32],
    migrate: Option<&MigrateEmit>,
    task_period_ns: u64,
) -> LlvmModule<'ctx> {
    emit_program_for_target(
        lctx,
        mir,
        layout,
        program,
        io_binds,
        program_name,
        uuid,
        layout_hash,
        migrate,
        task_period_ns,
        "",
        &Default::default(),
    )
    .expect("host target machine")
}

/// `emit_program` for an explicit LLVM triple ("" = host). The ABI
/// context offsets are computed from that target's data layout, so
/// 32-bit targets get correct struct member offsets.
/// `extern_pous` names POUs whose bodies arrive later via bitcode
/// link (precompiled library packages) — they emit as declarations
/// only; call sites resolve them through the same extern symbol.
#[allow(clippy::too_many_arguments)]
pub fn emit_program_for_target<'ctx>(
    lctx: &'ctx Context,
    mir: &'static Module,
    layout: &'static Layout,
    program: &'static Pou,
    io_binds: &[IoBind],
    program_name: &str,
    uuid: [u8; 16],
    layout_hash: [u8; 32],
    migrate: Option<&MigrateEmit>,
    task_period_ns: u64,
    triple: &str,
    extern_pous: &std::collections::HashSet<String>,
) -> Result<LlvmModule<'ctx>, String> {
    let abi = abi_offsets_for_triple(triple)?;
    let module = lctx.create_module("tplc_program");
    let mut g = Gen {
        ctx: lctx,
        module,
        b: lctx.create_builder(),
        layout,
        mir,
        abort_bb: None,
        retok_bb: None,
        cur_fn: None,
        cur_inouts: std::collections::HashSet::new(),
        overrun_bb: None,
        panic_bb: None,
        bounds_bb: None,
        divzero_bb: None,
        overflow_bb: None,
        range_bb: None,
        loop_stack: Vec::new(),
        ctx_ptr: lctx.ptr_type(inkwell::AddressSpace::default()).const_zero(),
        this_ptr: lctx.ptr_type(inkwell::AddressSpace::default()).const_zero(),
        abi,
    };

    let i8p = g.i8p();
    // tplc_abi_version
    let ft = lctx.i32_type().fn_type(&[], false);
    let f = g
        .module
        .add_function("tplc_abi_version", ft, Some(Linkage::External));
    let bb = lctx.append_basic_block(f, "e");
    g.b.position_at_end(bb);
    g.b.build_return(Some(&lctx.i32_type().const_int(1, false)))
        .unwrap();

    // tplc_init(ctx): memset blob + zero-init (literal inits: P1-07)
    let ft = lctx.i32_type().fn_type(&[i8p.into()], false);
    let f = g
        .module
        .add_function("tplc_init", ft, Some(Linkage::External));
    let bb = lctx.append_basic_block(f, "e");
    g.b.position_at_end(bb);
    g.ctx_ptr = f.get_nth_param(0).unwrap().into_pointer_value();
    let blob = g
        .ctx_load(g.abi.ctx_state_blob, i8p.into(), "blob")
        .into_pointer_value();
    // zero the blob in two segments around the retain region; the
    // retain segment itself is zeroed only on a cold start — a warm
    // restore (CTXF_RETAIN_RESTORED = ctx_flags bit3) keeps host bytes
    let (roff, rsize) = (layout.retain_offset, layout.retain_size);
    if rsize == 0 {
        g.b.build_memset(
            blob,
            1,
            lctx.i8_type().const_zero(),
            lctx.i64_type().const_int(layout.total_size, false),
        )
        .unwrap();
    } else {
        g.b.build_memset(
            blob,
            1,
            lctx.i8_type().const_zero(),
            lctx.i64_type().const_int(roff, false),
        )
        .unwrap();
        if roff + rsize < layout.total_size {
            let tail = unsafe {
                g.b.build_gep(
                    lctx.i8_type(),
                    blob,
                    &[lctx.i64_type().const_int(roff + rsize, false)],
                    "tb",
                )
            }
            .unwrap();
            g.b.build_memset(
                tail,
                1,
                lctx.i8_type().const_zero(),
                lctx.i64_type()
                    .const_int(layout.total_size - roff - rsize, false),
            )
            .unwrap();
        }
    }
    // literal default inits; retain zeroing + retain literal inits are
    // skipped when the host signals a warm restore
    let progs: Vec<&Pou> = mir
        .pous
        .iter()
        .filter(|p| p.kind == PouKind::Program)
        .collect();
    let scopes: Vec<Ctx> = progs.iter().map(|p| pou_scope(layout, &p.name)).collect();
    g.this_ptr = blob;
    let cf = g
        .ctx_load(g.abi.ctx_flags, lctx.i32_type().into(), "cf")
        .into_int_value();
    let restored =
        g.b.build_and(cf, lctx.i32_type().const_int(8, false), "rr")
            .unwrap();
    let retain_bb = lctx.append_basic_block(f, "i.ret");
    let done_bb = lctx.append_basic_block(f, "i.done");
    let lit_inits = |retain: bool| -> Vec<(u64, Type, Expr)> {
        let mut out = Vec::new();
        for (p, sc) in progs.iter().zip(&scopes) {
            for v in &p.iface {
                if v.retain != retain
                    || !matches!(v.init.as_ref().map(|i| &i.kind), Some(ExprKind::Lit(_)))
                {
                    continue;
                }
                if let Some((off, ty)) = sc.get(v.name.as_str()) {
                    out.push((*off, ty.clone(), v.init.clone().unwrap()));
                }
            }
        }
        out
    };
    let plain = lit_inits(false);
    let retains = lit_inits(true);
    for (off, ty, init) in &plain {
        g.lit_store(blob, *off, ty, init);
    }
    if retains.is_empty() && rsize == 0 {
        g.b.build_unconditional_branch(done_bb).unwrap();
        g.b.position_at_end(retain_bb);
        g.b.build_unconditional_branch(done_bb).unwrap();
    } else {
        let nz =
            g.b.build_int_compare(
                IntPredicate::EQ,
                restored,
                lctx.i32_type().const_zero(),
                "z",
            )
            .unwrap();
        g.b.build_conditional_branch(nz, retain_bb, done_bb)
            .unwrap();
        g.b.position_at_end(retain_bb);
        if rsize > 0 {
            let rseg = unsafe {
                g.b.build_gep(
                    lctx.i8_type(),
                    blob,
                    &[lctx.i64_type().const_int(roff, false)],
                    "rb",
                )
            }
            .unwrap();
            g.b.build_memset(
                rseg,
                1,
                lctx.i8_type().const_zero(),
                lctx.i64_type().const_int(rsize, false),
            )
            .unwrap();
        }
        for (off, ty, init) in &retains {
            g.lit_store(blob, *off, ty, init);
        }
        g.b.build_unconditional_branch(done_bb).unwrap();
    }
    g.b.position_at_end(done_bb);
    g.b.build_return(Some(&lctx.i32_type().const_zero()))
        .unwrap();

    // tplc_shutdown(ctx)
    let ft = lctx.void_type().fn_type(&[i8p.into()], false);
    let f = g
        .module
        .add_function("tplc_shutdown", ft, Some(Linkage::External));
    let bb = lctx.append_basic_block(f, "e");
    g.b.position_at_end(bb);
    g.b.build_return(None).unwrap();

    // per-POU fns — each PROGRAM gets its own var scope
    for pou in &mir.pous {
        /* precompiled-library POUs emit as declarations only — the
        bodies arrive via link_in_module from the package's IrBitcode;
        codegenning them here would duplicate the definition at
        link time. Call sites still resolve the extern decl via
        pou_fn's lazy declaration. */
        if extern_pous.contains(&pou.name.to_uppercase()) {
            continue;
        }
        let sc: Ctx = match pou.kind {
            PouKind::Program => pou_scope(layout, &pou.name),
            PouKind::FunctionBlock => {
                let fbl = layout
                    .fb_types
                    .get(&pou.name)
                    .cloned()
                    .or_else(|| builtin_fb_layout(&pou.name))
                    .unwrap();
                fb_ctx(&fbl)
            }
            PouKind::Function => {
                let fbl = tplc_mir::layout::layout_members(&pou.iface, BTreeMap::new());
                fb_ctx(&fbl)
            }
        };
        g.emit_pou(pou, &sc);
    }

    // task table: mir.tasks, or a synthesized MAIN task running every
    // PROGRAM in decl order when no task config is declared
    let tasks: Vec<TaskDecl> = if mir.tasks.is_empty() {
        vec![TaskDecl {
            name: "MAIN".into(),
            period_ns: task_period_ns,
            wcet_hint_ns: 0,
            priority: 10,
            programs: progs.iter().map(|p| p.name.clone()).collect(),
            kind: TaskKind::Cyclic,
            trig: None,
        }]
    } else {
        mir.tasks.clone()
    };
    for t in &tasks {
        for pn in &t.programs {
            if !mir
                .pous
                .iter()
                .any(|p| p.kind == PouKind::Program && p.name.eq_ignore_ascii_case(pn))
            {
                return Err(format!("task {} binds unknown program {pn}", t.name));
            }
        }
    }

    // __tplc_task_<i>(tctx) -> i32 : run each bound program in order;
    // all programs share the state blob (their fields are disjoint).
    let ft = lctx.i32_type().fn_type(&[i8p.into()], false);
    for (ti, t) in tasks.iter().enumerate() {
        let f = g.module.add_function(
            &format!("__tplc_task_{ti}"),
            ft,
            Some(Linkage::Internal),
        );
        let bb = lctx.append_basic_block(f, "e");
        g.b.position_at_end(bb);
        let tctx = f.get_nth_param(0).unwrap().into_pointer_value();
        g.ctx_ptr = tctx;
        let prog = g
            .ctx_load(g.abi.tctx_prog, i8p.into(), "prog")
            .into_pointer_value();
        g.ctx_ptr = prog;
        let blob = g
            .ctx_load(g.abi.ctx_state_blob, i8p.into(), "blob")
            .into_pointer_value();
        let mut rc = lctx.i32_type().const_zero();
        for pn in &t.programs {
            let pou = mir
                .pous
                .iter()
                .find(|p| p.kind == PouKind::Program && p.name.eq_ignore_ascii_case(pn))
                .unwrap();
            let pf = g.pou_fn(&pou.name);
            let cs = g
                .b
                .build_call(pf, &[prog.into(), blob.into()], "run")
                .unwrap();
            let v = cs.try_as_basic_value().expect_basic("rc");
            rc = g
                .b
                .build_or(rc, v.into_int_value(), "rc")
                .unwrap();
        }
        g.b.build_return(Some(&rc)).unwrap();
    }

    if let Some(m) = migrate {
        g.emit_migrate(m);
    }
    g.emit_descriptor(
        program,
        io_binds,
        program_name,
        uuid,
        layout_hash,
        migrate.is_some(),
        &tasks,
    );
    Ok(g.module)
}

/// Emit a library module: `tplc_abi_version` plus one
/// `__tplc_pou_<NAME>(ctx, this) -> i32` per FUNCTION/FUNCTION_BLOCK.
/// No program entry points, init, task, descriptor, or migrate —
/// library packages are link-time artifacts, not loadable images.
pub fn emit_library_for_target<'ctx>(
    lctx: &'ctx Context,
    mir: &'static Module,
    layout: &'static Layout,
    triple: &str,
) -> Result<LlvmModule<'ctx>, String> {
    let abi = abi_offsets_for_triple(triple)?;
    let module = lctx.create_module("tplc_library");
    let mut g = Gen {
        ctx: lctx,
        module,
        b: lctx.create_builder(),
        layout,
        mir,
        abort_bb: None,
        retok_bb: None,
        cur_fn: None,
        cur_inouts: std::collections::HashSet::new(),
        overrun_bb: None,
        panic_bb: None,
        bounds_bb: None,
        divzero_bb: None,
        overflow_bb: None,
        range_bb: None,
        loop_stack: Vec::new(),
        ctx_ptr: lctx.ptr_type(inkwell::AddressSpace::default()).const_zero(),
        this_ptr: lctx.ptr_type(inkwell::AddressSpace::default()).const_zero(),
        abi,
    };

    let ft = lctx.i32_type().fn_type(&[], false);
    let f = g
        .module
        .add_function("tplc_abi_version", ft, Some(Linkage::External));
    let bb = lctx.append_basic_block(f, "e");
    g.b.position_at_end(bb);
    g.b.build_return(Some(&lctx.i32_type().const_int(1, false)))
        .unwrap();

    for pou in &mir.pous {
        if pou.kind == PouKind::Program {
            return Err(format!("library module contains PROGRAM {}", pou.name));
        }
        let sc: Ctx = match pou.kind {
            PouKind::Program => unreachable!(),
            PouKind::FunctionBlock => {
                let fbl = layout
                    .fb_types
                    .get(&pou.name)
                    .cloned()
                    .or_else(|| builtin_fb_layout(&pou.name))
                    .unwrap_or_else(|| panic!("no FB layout {}", pou.name));
                fb_ctx(&fbl)
            }
            PouKind::Function => {
                let fbl = tplc_mir::layout::layout_members(&pou.iface, BTreeMap::new());
                fb_ctx(&fbl)
            }
        };
        g.emit_pou(pou, &sc);
    }
    Ok(g.module)
}

/* ------------------------------------------------------------------ */
/* Object / bitcode emission (P1-07)                                   */
/* ------------------------------------------------------------------ */

use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine, TargetTriple,
};
use inkwell::OptimizationLevel;
use std::path::Path;

/// Create a TargetMachine for `triple` ("x86_64-unknown-linux-gnu",
/// "aarch64-unknown-linux-gnu", "armv6-unknown-linux-gnueabihf").
pub fn target_machine(triple: &str) -> Result<TargetMachine, String> {
    if triple.is_empty() {
        Target::initialize_native(&InitializationConfig::default()).map_err(|e| e.to_string())?;
    } else {
        Target::initialize_all(&InitializationConfig::default());
    }
    let triple_t = if triple.is_empty() {
        TargetMachine::get_default_triple()
    } else {
        TargetTriple::create(triple)
    };
    let target = Target::from_triple(&triple_t).map_err(|e| e.to_string())?;
    /* PIC for hosted triples; bare-metal targets reject PIC, so fall
    back to static reloc + small code model (Cortex-M objects) */
    target
        .create_target_machine(
            &triple_t,
            "generic",
            "",
            OptimizationLevel::Default,
            RelocMode::PIC,
            CodeModel::Default,
        )
        .or_else(|| {
            target.create_target_machine(
                &triple_t,
                "generic",
                "",
                OptimizationLevel::Default,
                RelocMode::Static,
                CodeModel::Small,
            )
        })
        .ok_or_else(|| format!("no target machine for {triple}"))
}

/// The host target triple ("" triple in target_machine means this).
pub fn host_triple() -> String {
    TargetMachine::get_default_triple()
        .as_str()
        .to_str()
        .unwrap_or("")
        .to_string()
}

/// Stamp the module with a target triple + data layout so the backend
/// emits for that target (the `tplcc --target` path).
pub fn set_target(module: &LlvmModule<'_>, triple: &str) -> Result<(), String> {
    let tm = target_machine(triple)?;
    module.set_triple(&tm.get_triple());
    module.set_data_layout(&tm.get_target_data().get_data_layout());
    Ok(())
}

/// Emit an ELF object file for `triple` ("" = host).
pub fn emit_object(module: &LlvmModule<'_>, path: &Path, triple: &str) -> Result<(), String> {
    let tm = target_machine(triple)?;
    tm.write_to_file(module, FileType::Object, path)
        .map_err(|e| e.to_string())
}

/// Emit LLVM bitcode (the canonical .tplc retained form, doc 01 §4).
pub fn emit_bitcode(module: &LlvmModule<'_>, path: &Path) -> Result<(), String> {
    module.write_bitcode_to_path(path);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /* ABI offsets must match the C headers on both pointer widths —
    verified against offsetof() compiled by gcc (LP64) and
    arm-none-eabi-gcc (AAPCS32). */
    #[test]
    fn abi_offsets_lp64() {
        let a = abi_offsets(8, 8);
        assert_eq!(a.ctx_state_blob, 8);
        assert_eq!(a.ctx_flags, 4);
        assert_eq!(a.ctx_svc, 72);
        assert_eq!(a.ctx_abort, 88);
        assert_eq!(a.ctx_deadline, 96);
        assert_eq!(a.tctx_prog, 0);
        assert_eq!(a.tctx_index, 8);
        assert_eq!(a.svc_steady_ns, 24);
        assert_eq!(a.svc_host, 56);
        assert_eq!(a.mctx_new_prog, 0);
        assert_eq!(a.mctx_old_blob, 8);
        assert_eq!(a.mctx_old_blob_size, 16);
        assert_eq!(a.mctx_old_layout_hash, 24);
        assert_eq!(a.mctx_out_status, 80);
        assert_eq!(a.mctx_out_first_err, 84);
    }

    #[test]
    fn abi_offsets_aapcs32() {
        let a = abi_offsets(4, 8);
        assert_eq!(a.ctx_state_blob, 8);
        assert_eq!(a.ctx_flags, 4);
        assert_eq!(a.ctx_svc, 56);
        assert_eq!(a.ctx_abort, 64);
        assert_eq!(a.ctx_deadline, 72);
        assert_eq!(a.tctx_prog, 0);
        assert_eq!(a.tctx_index, 4);
        assert_eq!(a.svc_steady_ns, 16);
        assert_eq!(a.svc_host, 32);
        assert_eq!(a.mctx_new_prog, 0);
        assert_eq!(a.mctx_old_blob, 4);
        assert_eq!(a.mctx_old_blob_size, 8);
        assert_eq!(a.mctx_old_layout_hash, 16);
        assert_eq!(a.mctx_out_status, 72);
        assert_eq!(a.mctx_out_first_err, 76);
    }

    #[test]
    fn layout_geom_parses_ptr_and_i64() {
        assert_eq!(
            layout_geom(
                "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
            ),
            (8, 8)
        );
        assert_eq!(
            layout_geom("e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64"),
            (4, 8)
        );
        assert_eq!(layout_geom(""), (8, 8));
    }
}
