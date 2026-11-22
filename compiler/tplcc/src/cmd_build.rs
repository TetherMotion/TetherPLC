use inkwell::context::Context;
use inkwell::module::Linkage;
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use std::process::ExitCode;
use tplc_codegen::llvm_emit::{
    emit_bitcode, emit_library_for_target, emit_object, emit_program_for_target, host_triple,
    set_target, target_machine,
};
use tplc_mir::desc;
use tplc_mir::ir::{AtRegion, IoBind, Pou, PouKind, Prov};
use tplc_pkg::format::SectionType;
use tplc_pkg::writer::PackageWriter;
use crate::helpers::*;
use crate::libs;
pub(crate) fn cmd_build(args: &[String]) -> ExitCode {
    let mut out_path = PathBuf::from("out.tplc");
    let mut change_against: Option<PathBuf> = None;
    let mut target: Option<String> = None;
    let mut allow_retain_loss = false;
    let mut json_mode = false;
    let mut task_period_ns: u64 = 10_000_000;
    let mut prog_version = String::from("0");
    let mut checked_arith = false;
    let mut pic_blob = false;
    let mut library_mode = false;
    let mut lib_name: Option<String> = None;
    let mut lib_pkgs: Vec<String> = Vec::new();
    let mut libdirs: Vec<String> = Vec::new();
    let mut renames: Vec<(String, String)> = Vec::new();
    let mut files: Vec<String> = Vec::new();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-o" => {
                out_path = PathBuf::from(&args[i + 1]);
                i += 1;
            }
            "--change-against" => {
                change_against = Some(PathBuf::from(&args[i + 1]));
                i += 1;
            }
            "--target" => {
                target = Some(args[i + 1].clone());
                i += 1;
            }
            "--allow-retain-loss" => allow_retain_loss = true,
            "--prog-version" => {
                prog_version = args[i + 1].clone();
                i += 1;
            }
            "--json" => json_mode = true,
            "--task-period-ns" => {
                task_period_ns = args[i + 1].parse().unwrap_or_else(|_| {
                    eprintln!("tplcc build: bad --task-period-ns value");
                    std::process::exit(2);
                });
                i += 1;
            }
            "--rename" => {
                if let Some((o, n)) = args[i + 1].split_once('=') {
                    renames.push((o.into(), n.into()));
                }
                i += 1;
            }
            "--checked-arith" => checked_arith = true,
            "--pic-blob" => pic_blob = true,
            "--library" => library_mode = true,
            "--name" => {
                lib_name = Some(args[i + 1].clone());
                i += 1;
            }
            "--lib" => {
                lib_pkgs.push(args[i + 1].clone());
                i += 1;
            }
            "--libdir" => {
                libdirs.push(args[i + 1].clone());
                i += 1;
            }
            f => files.push(f.into()),
        }
        i += 1;
    }
    if files.is_empty() && lib_pkgs.is_empty() {
        eprintln!("tplcc build: no sources");
        return ExitCode::from(2);
    }
    let mut srcs = read_srcs(&files);
    /* --libdir DIR: auto-resolve undeclared library-prefixed
    references from the libdir index — .st files merge, .tplc packages
    join lib_pkgs. The system determines what the project uses; the
    user never "attaches" a library. */
    if !libdirs.is_empty() {
        let rep = libs::resolve(&libdirs, &mut srcs, &mut lib_pkgs);
        for p in &rep.pulled {
            eprintln!("tplcc: lib: pulled {p}");
        }
    }
    /* --lib <pkg.tplc>: pull the package's ST_SOURCES section into the
    compilation — needed for sema/layout either way (source text is the
    header; bitcode is the body). kind="library" packages whose
    bitcode was built for THIS target additionally take the
    precompiled-link path: their POU bodies link from IrBitcode and
    unreferenced ones are dropped by globaldce. */
    let eff_triple = target.clone().unwrap_or_else(host_triple);
    let mut lib_bitcode: Vec<Vec<u8>> = Vec::new();
    let mut extern_pous: std::collections::HashSet<String> = std::collections::HashSet::new();
    for lp in &lib_pkgs {
        let mut ls = lib_sources(lp);
        if let Some(meta) = libs::pkg_meta(std::path::Path::new(lp)) {
            if meta.kind == "library" {
                if meta.triple == eff_triple {
                    if let Some(bc) = libs::pkg_bitcode(std::path::Path::new(lp)) {
                        extern_pous.extend(meta.pous);
                        lib_bitcode.push(bc);
                    }
                } else if !meta.triple.is_empty() {
                    eprintln!(
                        "tplcc: warning: {lp} is a library built for {} — falling back to source inclusion (rebuild for {eff_triple} to link precompiled)",
                        meta.triple
                    );
                }
            }
        }
        ls.extend(srcs);
        srcs = ls;
    }
    let (mut out, eff) = compile_srcs(&srcs);
    let srcs = eff;
    let mut diags = std::mem::take(&mut out.diags);
    if !diags.iter().any(|d| d.severity == tplc_st::Severity::Error) {
        diags.extend(tplc_st::sema::check(&mut out.module));
    }
    let errs = diags
        .iter()
        .filter(|d| d.severity == tplc_st::Severity::Error)
        .count();
    if !json_mode {
        print_diags(&diags, &srcs);
    }
    if errs > 0 {
        if json_mode {
            println!("{}", diag_report(&diags, &srcs, errs, None));
        }
        eprintln!("{errs} error(s)");
        return ExitCode::FAILURE;
    }
    if checked_arith {
        out.module.checked_arith = true;
    }

    if library_mode {
        return build_library_pkg(
            &out_path,
            &srcs,
            &diags,
            out.module,
            out.io_bindings,
            target,
            pic_blob,
            checked_arith,
            lib_name,
            &prog_version,
            json_mode,
        );
    }

    let progs: Vec<&Pou> = out
        .module
        .pous
        .iter()
        .filter(|p| p.kind == PouKind::Program)
        .collect();
    if progs.is_empty() {
        eprintln!("tplcc build: no PROGRAM found");
        return ExitCode::FAILURE;
    }
    for t in &out.module.tasks {
        for pn in &t.programs {
            if !progs.iter().any(|p| p.name.eq_ignore_ascii_case(pn)) {
                eprintln!("tplcc build: task {} binds unknown program {pn}", t.name);
                return ExitCode::FAILURE;
            }
        }
    }
    let program: Pou = progs[0].clone();
    let prog_name = program.name.clone();

    // multi-program image: every PROGRAM pou shares one state blob,
    // field names qualified PROGRAM.var when more than one exists
    let mut layout = tplc_mir::layout::layout_module(&out.module);
    // io triples for descriptor channels
    let io_triples: Vec<(String, tplc_mir::ir::At, u64)> = out
        .io_bindings
        .iter()
        .map(|b| (b.name.clone(), b.at.clone(), b.at.byte))
        .collect();
    let prov_of = |name: &str| -> Option<Prov> {
        let short = name.rsplit('/').next().unwrap_or(name);
        let short = short.rsplit('.').next().unwrap_or(short);
        progs
            .iter()
            .flat_map(|p| p.iface.iter())
            .find(|v| v.name == short)
            .map(|v| v.prov.clone())
            .or_else(|| {
                let (g, n) = name.split_once('.')?;
                out.module
                    .gvls
                    .iter()
                    .find(|gvl| gvl.name == g)
                    .and_then(|gvl| gvl.vars.iter().find(|v| v.name == n))
                    .map(|v| v.prov.clone())
            })
    };
    let descs = desc::emit(&mut layout, &program, &io_triples, &prov_of);
    let layout_hash: [u8; 32] = Sha256::digest(&descs.payload_for_hash).into();

    // online-change analysis (doc 08 §4): diff the predecessor's
    // layout against the new one — denies are compile errors, the
    // approved rule table becomes migrate_desc + tplc_migrate
    let mut migd: Option<tplc_pkg::sections::Migd> = None;
    let mut mig_emit: Option<tplc_mir::migrate::MigrateEmit> = None;
    if let Some(prev_path) = &change_against {
        use tplc_mir::migrate as mig;
        let prev_bytes =
            std::fs::read(prev_path).unwrap_or_else(|e| panic!("{}: {e}", prev_path.display()));
        let prev = tplc_pkg::reader::Package::parse(&prev_bytes)
            .unwrap_or_else(|e| panic!("prev pkg: {e}"));
        let sec = |t| {
            prev.section(t)
                .unwrap_or_else(|| panic!("prev pkg missing section"))
                .data
        };
        let old_layd =
            tplc_pkg::sections::Layd::decode(sec(tplc_pkg::format::SectionType::LayoutDesc))
                .expect("prev LAYD");
        let old_smap =
            mig::decode_smap(sec(tplc_pkg::format::SectionType::Symmap)).expect("prev SMAP");
        let old_typc =
            mig::decode_typc(sec(tplc_pkg::format::SectionType::TypeCatalog)).expect("prev TYPC");
        let new_smap = mig::decode_smap(&descs.symmap).expect("SMAP");
        let new_typc = mig::decode_typc(&descs.typecat).expect("TYPC");
        let retain_region = |l: &tplc_pkg::sections::Layd| {
            l.regions
                .iter()
                .find(|r| r.kind == 3)
                .map(|r| (r.offset, r.size))
                .unwrap_or((0, 0))
        };
        let old_side = mig::Side {
            leaves: mig::leaf_fields(&old_layd, &old_smap, &old_typc),
            retain_region: retain_region(&old_layd),
        };
        let new_side = mig::Side {
            leaves: mig::leaf_fields(&descs.layd, &new_smap, &new_typc),
            retain_region: retain_region(&descs.layd),
        };
        let plan = mig::diff(&old_side, &new_side, &renames, allow_retain_loss);
        // report every rule; denies are compile errors
        let mut deny_errs = 0;
        let mut ref_resets = 0usize;
        for r in &plan.rules {
            match r {
                mig::Rule::Deny { reason, detail, .. } => {
                    deny_errs += 1;
                    eprintln!("error[CHANGE-DENY{reason}]: {detail}");
                }
                mig::Rule::Dropped {
                    name,
                    retain,
                    ref_reset,
                    ..
                } => {
                    if *ref_reset {
                        ref_resets += 1;
                        eprintln!(
                            "warning: online change: reference field {name} will be reset to null — the referenced layout moved"
                        );
                    } else {
                        eprintln!(
                            "note: field {name} dropped{}",
                            if *retain { " (retain value lost)" } else { "" }
                        )
                    }
                }
                mig::Rule::Alias {
                    old_name, new_name, ..
                } => {
                    eprintln!("note: alias {old_name} → {new_name}")
                }
                _ => {}
            }
        }
        for (name, _) in &plan.refs_preserved {
            eprintln!(
                "note: online change: reference field {name} preserved \
                 (referenced layout unchanged)"
            );
        }
        if ref_resets > 0 {
            eprintln!(
                "warning: online change resets {ref_resets} reference \
                 field(s) to null — re-arm them in the program after \
                 the swap"
            );
        }
        if plan.deny || deny_errs > 0 {
            if json_mode {
                println!("{}", stage_failure_json("change", "online change denied"));
            }
            eprintln!(
                "online change denied: {deny_errs} rule(s) — deploy a                  cold package instead"
            );
            return ExitCode::FAILURE;
        }
        let mut emit = mig::plan_to_emit(&plan);
        emit.predecessor_hash = Sha256::digest(
            sec(tplc_pkg::format::SectionType::LayoutDesc)
                .iter()
                .chain(sec(tplc_pkg::format::SectionType::Symmap).iter())
                .chain(sec(tplc_pkg::format::SectionType::TypeCatalog).iter())
                .copied()
                .collect::<Vec<u8>>(),
        )
        .into();
        // rule table for migrate_desc — field indices as emitted
        let preserved_refs: std::collections::HashSet<u32> =
            plan.refs_preserved.iter().map(|(_, i)| *i).collect();
        let rules = plan
            .rules
            .iter()
            .map(|r| match r {
                mig::Rule::Copy {
                    old_field,
                    new_field,
                    ..
                } => tplc_pkg::sections::MigdRule {
                    kind: 0,
                    severity: 0,
                    transform_id: 0,
                    old_field: *old_field,
                    new_field: *new_field,
                    /* bit2: this copy carries a reference that the
                    runtime may keep (layout unchanged) */
                    aux: if preserved_refs.contains(new_field) {
                        4
                    } else {
                        0
                    },
                },
                mig::Rule::Transform {
                    id,
                    old_field,
                    new_field,
                    ..
                } => tplc_pkg::sections::MigdRule {
                    kind: 1,
                    severity: if matches!(
                        id,
                        mig::Transform::StringTrunc | mig::Transform::ArrayGrow
                    ) {
                        1
                    } else {
                        0
                    },
                    transform_id: *id as u16,
                    old_field: *old_field,
                    new_field: *new_field,
                    aux: 0,
                },
                mig::Rule::DefaultInit { new_field, .. } => tplc_pkg::sections::MigdRule {
                    kind: 2,
                    severity: 0,
                    transform_id: 0,
                    old_field: u32::MAX,
                    new_field: *new_field,
                    aux: 0,
                },
                mig::Rule::Dropped {
                    old_field,
                    retain,
                    ref_reset,
                    ..
                } => tplc_pkg::sections::MigdRule {
                    kind: 3,
                    severity: 1,
                    transform_id: 0,
                    old_field: *old_field,
                    new_field: u32::MAX,
                    /* bit0 retain-lost, bit1 reference reset */
                    aux: (*retain as u32) | ((*ref_reset as u32) << 1),
                },
                mig::Rule::Alias {
                    old_field,
                    new_field,
                    ..
                } => tplc_pkg::sections::MigdRule {
                    kind: 5,
                    severity: 0,
                    transform_id: 0,
                    old_field: *old_field,
                    new_field: *new_field,
                    aux: 0,
                },
                mig::Rule::Deny { reason, field, .. } => tplc_pkg::sections::MigdRule {
                    kind: 4,
                    severity: 2,
                    transform_id: 0,
                    old_field: *field,
                    new_field: u32::MAX,
                    aux: *reason,
                },
            })
            .collect();
        migd = Some(tplc_pkg::sections::Migd {
            predecessor_layout_hash: emit.predecessor_hash,
            predecessor_version: b"prev".to_vec(),
            rules,
            predecessor_layd: old_layd,
        });
        mig_emit = Some(emit);
    }

    // uuid from source hash → deterministic per program content
    let mut h = Sha256::new();
    for (_, s) in &srcs {
        h.update(s.as_bytes());
    }
    let sh = h.finalize();
    let mut uuid = [0u8; 16];
    uuid.copy_from_slice(&sh[..16]);

    let io_binds: &'static [IoBind] = Box::leak(out.io_bindings.clone().into_boxed_slice());
    let module_ref: &'static _ = Box::leak(Box::new(out.module));
    let layout_ref: &'static _ = Box::leak(Box::new(layout));
    let prog_ref: &'static Pou = Box::leak(Box::new(program));

    /* --target selects the backend triple; resolved before emission
    so the ABI context offsets match the target's data layout.
    The package's target_obj is a monolithic object for non-host
    targets (the .so link is native-only) */
    let triple = eff_triple.clone();
    if pic_blob && triple == host_triple() {
        eprintln!("tplcc: --pic-blob requires a foreign --target (host builds link a .so)");
        return ExitCode::FAILURE;
    }

    let lctx = Context::create();
    let llmod = match emit_program_for_target(
        &lctx,
        module_ref,
        layout_ref,
        prog_ref,
        io_binds,
        &prog_name,
        uuid,
        layout_hash,
        mig_emit.as_ref(),
        task_period_ns,
        &triple,
        &extern_pous,
    ) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("tplcc: bad target triple {triple}: {e}");
            return ExitCode::FAILURE;
        }
    };
    if let Err(e) = llmod.verify() {
        if json_mode {
            println!("{}", stage_failure_json("verify", e.to_string().as_str()));
        }
        eprintln!("tplcc: LLVM verify failed: {e}");
        return ExitCode::FAILURE;
    }

    if let Err(e) = set_target(&llmod, &triple) {
        eprintln!("tplcc: bad target triple {triple}: {e}");
        return ExitCode::FAILURE;
    }

    /* precompiled library link: each library package's IrBitcode
    module joins this one — its __tplc_pou_* bodies satisfy the extern
    declarations emit left for library POUs (emit_program_for_target
    skips their codegen). Unreferenced library POUs are then dropped
    by globaldce, so the .so carries only the POUs the program
    actually calls — compile libraries once, link only what's used. */
    if !lib_bitcode.is_empty() {
        for bc in &lib_bitcode {
            /* inkwell's memory-buffer ctors require a trailing NUL
            (one-past-end sentinel, not part of the buffer) */
            let mut bc_nul = bc.clone();
            bc_nul.push(0);
            let buf = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range_copy(
                &bc_nul, "tplc_lib",
            );
            let libm =
                match inkwell::module::Module::parse_bitcode_from_buffer(&buf, &lctx) {
                    Ok(m) => m,
                    Err(e) => {
                        eprintln!("tplcc: library bitcode parse: {e:?}");
                        return ExitCode::FAILURE;
                    }
                };
            /* Duplicate definitions (tplc_abi_version, or a POU the
            program/an earlier lib already defines — e.g. two library
            packages that each baked in a shared dep) would fail the
            link as multiply-defined. Privatize+rename the incoming
            copy: lib-internal call sites retarget it automatically,
            and a same-source POU is layout-compatible by construction
            so the private duplicate is code-identical. */
            let prog_defs: std::collections::HashSet<String> = llmod
                .get_functions()
                .filter(|f| f.get_first_basic_block().is_some())
                .filter_map(|f| {
                    f.get_name().to_str().ok().map(|s| s.to_string())
                })
                .collect();
            let lib_fns: Vec<_> = libm.get_functions().collect();
            for f in lib_fns {
                if f.get_first_basic_block().is_none() {
                    continue;
                }
                let Ok(n) = f.get_name().to_str() else {
                    continue;
                };
                if prog_defs.contains(n) {
                    f.set_linkage(Linkage::Private);
                    let mut uniq = format!("{n}.libdup");
                    while llmod.get_function(&uniq).is_some()
                        || libm.get_function(&uniq).is_some()
                    {
                        uniq.push('x');
                    }
                    /* FunctionValue lacks set_name in inkwell 0.9 —
                    FFI directly; renaming updates lib-internal use
                    sites automatically. */
                    unsafe {
                        inkwell::llvm_sys::core::LLVMSetValueName2(
                            inkwell::values::AsValueRef::as_value_ref(&f),
                            uniq.as_ptr().cast(),
                            uniq.len(),
                        );
                    }
                }
            }
            if let Err(e) = llmod.link_in_module(libm) {
                eprintln!("tplcc: library bitcode link: {e:?}");
                return ExitCode::FAILURE;
            }
        }
        /* Internalize every defined __tplc_* function so unreferenced
        library POUs die in globaldce — the exports map already hides
        them at .so level; this removes the code outright. */
        for f in llmod.get_functions() {
            let n = f.get_name().to_str().unwrap_or_default();
            if n.starts_with("__tplc_") && f.get_first_basic_block().is_some() {
                f.set_linkage(Linkage::Internal);
            }
        }
        let tm = match target_machine(&triple) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("tplcc: {e}");
                return ExitCode::FAILURE;
            }
        };
        if let Err(e) = llmod.run_passes(
            "globaldce",
            &tm,
            inkwell::passes::PassBuilderOptions::create(),
        ) {
            eprintln!("tplcc: globaldce: {e:?}");
            return ExitCode::FAILURE;
        }
    }
    let tmp = std::env::temp_dir().join("tplcc_build");
    std::fs::create_dir_all(&tmp).unwrap();
    let bc = tmp.join("prog.bc");
    let obj = tmp.join("prog.o");
    emit_bitcode(&llmod, &bc).unwrap();
    emit_object(&llmod, &obj, &triple).unwrap();
    let bc_bytes = std::fs::read(&bc).unwrap();

    /* link .o + tplc-rt → deployable .so (doc 01 §5: shared object) —
    native triple only; a non-host target_obj is the raw object */
    let obj_bytes = if triple == host_triple() {
        // The rt source is embedded; rustc produces a lean no_std
        // staticlib (~10KB .so, only `tplc_*` exports + abort).
        // TPLC_RT_LIB overrides for prebuilt archives.
        let rt_lib = std::env::var("TPLC_RT_LIB")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                // tplc-rt ships as modules; tplcc's build.rs flattens
                // them into one TU under OUT_DIR for the staticlib build.
                const RT_SRC: &[u8] =
                    include_bytes!(concat!(env!("OUT_DIR"), "/tplc_rt_flat.rs"));
                let src = tmp.join("tplc_rt.rs");
                std::fs::write(&src, RT_SRC).unwrap();
                let a = tmp.join("libtplc_rt.a");
                let rc = std::process::Command::new("rustc")
                    .args([
                        "--edition",
                        "2021",
                        "--crate-type",
                        "staticlib",
                        "-C",
                        "panic=abort",
                        "-C",
                        "opt-level=2",
                        "--cfg",
                        "feature=\"nostd\"",
                    ])
                    .arg(&src)
                    .arg("-o")
                    .arg(&a)
                    .output()
                    .expect("rustc for tplc-rt");
                if !rc.status.success() {
                    eprintln!("tplcc: rt build: {}", String::from_utf8_lossy(&rc.stderr));
                    std::process::exit(1);
                }
                a
            });
        let map = tmp.join("exports.map");
        std::fs::write(
            &map,
            "{ global: tplc_*; local: *; };
",
        )
        .unwrap();
        let so = tmp.join("prog.so");
        let cc = std::process::Command::new("cc")
            .args(["-shared", "-o"])
            .arg(&so)
            .arg(&obj)
            .arg(&rt_lib)
            .arg(format!("-Wl,--version-script={}", map.display()))
            .output()
            .unwrap();
        if !cc.status.success() {
            if json_mode {
                println!(
                    "{}",
                    stage_failure_json("link", &String::from_utf8_lossy(&cc.stderr))
                );
            }
            eprintln!(
                "tplcc: link failed: {}",
                String::from_utf8_lossy(&cc.stderr)
            );
            return ExitCode::FAILURE;
        }
        std::fs::read(&so).unwrap()
    } else {
        /* bare-metal/foreign target: target_obj is the raw object,
        not a linked .so — the MCU shim consumes it statically.
        --pic-blob converts the ET_REL to a TPLB loadable image:
        PC-relative relocs resolved at emit time, absolute/import
        refs carried as a runtime fixup table (no target linker) */
        let raw = std::fs::read(&obj).unwrap();
        if pic_blob {
            match tplc_codegen::elf_blob::build_blob(&raw) {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("tplcc: --pic-blob failed: {e}");
                    return ExitCode::FAILURE;
                }
            }
        } else {
            raw
        }
    };

    // retain hash → hex for manifest
    let lh_hex: String = layout_hash.iter().map(|b| format!("{b:02x}")).collect();
    let uuid_str = format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        uuid[0], uuid[1], uuid[2], uuid[3], uuid[4], uuid[5], uuid[6],
        uuid[7], uuid[8], uuid[9], uuid[10], uuid[11], uuid[12], uuid[13],
        uuid[14], uuid[15]
    );
    let io_json: String = out
        .io_bindings
        .iter()
        .map(|b| {
            let r = match b.at.region {
                AtRegion::I => "I",
                AtRegion::Q => "Q",
                AtRegion::M => "M",
            };
            /* canonical AT text — the deploy-map generator re-emits
               these verbatim in `bind` lines */
            let at = format!(
                "%{}{}{}{}",
                r,
                b.at.width,
                b.at.byte,
                b.at.bit.map(|x| format!(".{x}")).unwrap_or_default()
            );
            format!(
                "{{\"name\":\"{}\",\"at\":\"{}\",\"region\":\"{}\",\"byte\":{},\"bit\":{},\"width\":\"{}\",\"size\":{}}}",
                b.name,
                at,
                r,
                b.at.byte,
                b.at.bit.map(|x| x.to_string()).unwrap_or("null".into()),
                b.at.width,
                b.ty.size()
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    // manifest requires: runtime resource needs verified pre-load
    // (same extent formula the descriptor's image_*_bytes uses)
    let img_need = |r: AtRegion| -> u64 {
        io_binds
            .iter()
            .filter(|b| b.at.region == r)
            .map(|b| b.at.byte + b.ty.size().max(1))
            .max()
            .unwrap_or(0)
    };
    let checked_json = if checked_arith {
        ",\"checked_arith\":true"
    } else {
        ""
    };
    let objfmt_json = if pic_blob {
        ",\"target_obj_format\":\"tplb\""
    } else {
        ""
    };
    let manifest = format!(
        "{{\"schema\":\"tplc.manifest.v1\",\"name\":\"{prog_name}\",\"version\":\"{prog_version}\",\"package_uuid\":\"{uuid_str}\",\"abi_version\":1,\"layout_hash\":\"{lh_hex}\",\"target_triple\":\"{triple}\",\"requires\":{{\"image_in_bytes\":{},\"image_out_bytes\":{},\"marker_bytes\":{},\"retain_bytes\":{},\"state_blob_bytes\":{}}}{checked_json}{objfmt_json}}}",
        img_need(AtRegion::I),
        img_need(AtRegion::Q),
        img_need(AtRegion::M),
        layout_ref.retain_size,
        layout_ref.total_size
    );
    let src_bundle: String = srcs
        .iter()
        .map(|(f, s)| format!("==== {f}\n{s}\n"))
        .collect();

    let mut w = PackageWriter::new();
    w.uuid_str(&uuid_str).unwrap();
    w.add_section(SectionType::Manifest, manifest.into_bytes(), false, 1, 0)
        .unwrap();
    w.add_section(SectionType::IrBitcode, bc_bytes, true, 1, 0)
        .unwrap();
    w.add_section(SectionType::TargetObj, obj_bytes, true, 1, 0)
        .unwrap(); // aux=0 → targets[0]
    w.add_section(SectionType::Symmap, descs.symmap, true, 1, 0)
        .unwrap();
    w.add_section(SectionType::TypeCatalog, descs.typecat, true, 1, 0)
        .unwrap();
    w.add_section(SectionType::LayoutDesc, descs.layd.encode(), true, 1, 0)
        .unwrap();
    w.add_section(
        SectionType::IoMap,
        format!("[{io_json}]").into_bytes(),
        false,
        1,
        0,
    )
    .unwrap();
    if let Some(m) = migd {
        w.add_section(SectionType::MigrateDesc, m.encode(), true, 1, 0)
            .unwrap();
    }
    w.add_section(
        SectionType::Toolchain,
        format!(
            "{{\"tplcc\":\"{}\",\"rustc\":\"{}\",\"llvm\":\"{}\",\"host\":\"{}\"}}",
            env!("CARGO_PKG_VERSION"),
            env!("TPLC_RUSTC"),
            env!("TPLC_LLVM"),
            env!("TPLC_HOST"),
        )
        .into_bytes(),
        false,
        1,
        0,
    )
    .unwrap();
    w.add_section(SectionType::StSources, src_bundle.into_bytes(), false, 1, 0)
        .unwrap();
    let pkg = w.build().unwrap();
    std::fs::write(&out_path, pkg).unwrap();
    if json_mode {
        println!("{}", diag_report(&diags, &srcs, 0, Some(&out_path)));
    } else {
        println!("wrote {}", out_path.display());
    }
    ExitCode::SUCCESS
}

/// `tplcc build --library` — package a FUNCTION/FUNCTION_BLOCK
/// collection with no PROGRAM, GVLs, or tasks. The result is a
/// link-time artifact (`"kind":"library"` in the manifest), not a
/// loadable program image: sections are Manifest, IrBitcode,
/// TargetObj (the relocatable object, or TPLB with --pic-blob),
/// StSources, Toolchain. Consumed today via `--lib` source inclusion;
/// precompiled symbol linking is future work.
#[allow(clippy::too_many_arguments)]
fn build_library_pkg(
    out_path: &PathBuf,
    srcs: &[(String, String)],
    diags: &[tplc_st::Diag],
    module: tplc_mir::ir::Module,
    io_bindings: Vec<IoBind>,
    target: Option<String>,
    pic_blob: bool,
    checked_arith: bool,
    lib_name: Option<String>,
    prog_version: &str,
    json_mode: bool,
) -> ExitCode {
    if module.pous.iter().any(|p| p.kind == PouKind::Program) {
        eprintln!("tplcc build --library: PROGRAM found — libraries contain FUNCTION/FUNCTION_BLOCK POUs only");
        return ExitCode::FAILURE;
    }
    if !module.gvls.is_empty() {
        eprintln!(
            "tplcc build --library: VAR_GLOBAL/VAR_EXTERNAL not supported in library packages — consume via --lib source inclusion instead"
        );
        return ExitCode::FAILURE;
    }
    if !module.tasks.is_empty() {
        eprintln!("tplcc build --library: TASK declarations not supported in library packages");
        return ExitCode::FAILURE;
    }
    if !io_bindings.is_empty() && !json_mode {
        eprintln!(
            "warning: --library: {} io binding(s) ignored (no program image)",
            io_bindings.len()
        );
    }

    let fb_types = tplc_mir::layout::library_fb_types(&module);
    let layout: &'static tplc_mir::layout::Layout =
        Box::leak(Box::new(tplc_mir::layout::Layout::library(fb_types)));
    let module_ref: &'static tplc_mir::ir::Module = Box::leak(Box::new(module));

    let triple = target.unwrap_or_else(host_triple);
    if pic_blob && triple == host_triple() {
        eprintln!(
            "tplcc: --pic-blob requires a foreign --target (host builds emit the raw object)"
        );
        return ExitCode::FAILURE;
    }

    let lctx = Context::create();
    let llmod = match emit_library_for_target(&lctx, module_ref, layout, &triple) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("tplcc: {e}");
            return ExitCode::FAILURE;
        }
    };
    if let Err(e) = llmod.verify() {
        if json_mode {
            println!("{}", stage_failure_json("verify", &e.to_string()));
        }
        eprintln!("tplcc: LLVM verify failed: {e}");
        return ExitCode::FAILURE;
    }
    if let Err(e) = set_target(&llmod, &triple) {
        eprintln!("tplcc: bad target triple {triple}: {e}");
        return ExitCode::FAILURE;
    }
    let tmp = std::env::temp_dir().join("tplcc_build");
    std::fs::create_dir_all(&tmp).unwrap();
    let bc = tmp.join("lib.bc");
    let obj = tmp.join("lib.o");
    emit_bitcode(&llmod, &bc).unwrap();
    emit_object(&llmod, &obj, &triple).unwrap();
    let bc_bytes = std::fs::read(&bc).unwrap();

    /* library target_obj is always the relocatable object: POU symbols
    stay linkable for a future precompiled-consumption path, and no
    program entry points exist to justify a .so link */
    let raw = std::fs::read(&obj).unwrap();
    let obj_bytes = if pic_blob {
        match tplc_codegen::elf_blob::build_blob(&raw) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("tplcc: --pic-blob failed: {e}");
                return ExitCode::FAILURE;
            }
        }
    } else {
        raw
    };

    let mut h = Sha256::new();
    for (_, s) in srcs {
        h.update(s.as_bytes());
    }
    let sh = h.finalize();
    let mut uuid = [0u8; 16];
    uuid.copy_from_slice(&sh[..16]);
    let uuid_str = format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        uuid[0], uuid[1], uuid[2], uuid[3], uuid[4], uuid[5], uuid[6],
        uuid[7], uuid[8], uuid[9], uuid[10], uuid[11], uuid[12], uuid[13],
        uuid[14], uuid[15]
    );
    let name = lib_name.unwrap_or_else(|| {
        out_path
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| "library".into())
    });
    let checked_json = if checked_arith {
        ",\"checked_arith\":true"
    } else {
        ""
    };
    let objfmt_json = if pic_blob {
        ",\"target_obj_format\":\"tplb\""
    } else {
        ""
    };
    let pous_json = format!(
        ",\"pous\":[{}]",
        module_ref
            .pous
            .iter()
            .map(|p| format!("\"{}\"", p.name.to_uppercase()))
            .collect::<Vec<_>>()
            .join(",")
    );
    let manifest = format!(
        "{{\"schema\":\"tplc.manifest.v1\",\"name\":\"{name}\",\"version\":\"{prog_version}\",\"package_uuid\":\"{uuid_str}\",\"abi_version\":1,\"kind\":\"library\",\"target_triple\":\"{triple}\"{checked_json}{objfmt_json}{pous_json}}}"
    );
    let src_bundle: String = srcs
        .iter()
        .map(|(f, s)| format!("==== {f}\n{s}\n"))
        .collect();

    let mut w = PackageWriter::new();
    w.uuid_str(&uuid_str).unwrap();
    w.add_section(SectionType::Manifest, manifest.into_bytes(), false, 1, 0)
        .unwrap();
    w.add_section(SectionType::IrBitcode, bc_bytes, true, 1, 0)
        .unwrap();
    w.add_section(SectionType::TargetObj, obj_bytes, true, 1, 0)
        .unwrap();
    w.add_section(SectionType::StSources, src_bundle.into_bytes(), false, 1, 0)
        .unwrap();
    w.add_section(
        SectionType::Toolchain,
        format!(
            "{{\"tplcc\":\"{}\",\"rustc\":\"{}\",\"llvm\":\"{}\",\"host\":\"{}\"}}",
            env!("CARGO_PKG_VERSION"),
            env!("TPLC_RUSTC"),
            env!("TPLC_LLVM"),
            env!("TPLC_HOST"),
        )
        .into_bytes(),
        false,
        1,
        0,
    )
    .unwrap();
    let pkg = w.build().unwrap();
    std::fs::write(out_path, pkg).unwrap();
    if json_mode {
        println!("{}", diag_report(diags, srcs, 0, Some(out_path)));
    } else {
        println!("wrote {} (library)", out_path.display());
    }
    ExitCode::SUCCESS
}

