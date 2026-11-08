//! `tplcc` — Tether PLC compiler driver (P1-08).
//!
//!   tplcc check <src.st>…            parse + sema, print diags
//!   tplcc build <src.st>… -o p.tplc  full pipeline → .tplc package
//!   tplcc info <p.tplc>              dump package header + sections
//!
//! `check` and `build` accept `--json`: diagnostics go to stdout as a
//! `{"ok",…,"diags":[…]}` report (build adds `"package"` on success)
//! instead of the human rendering. Used by the ctl proxy's /compile
//! route for the web IDE.

use inkwell::context::Context;
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use std::process::ExitCode;
use tplc_codegen::llvm_emit::{emit_bitcode, emit_object, emit_program};
use tplc_mir::desc;
use tplc_mir::ir::{AtRegion, IoBind, Pou, PouKind, Prov};
use tplc_mir::layout::layout_program;
use tplc_pkg::format::SectionType;
use tplc_pkg::writer::PackageWriter;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("usage: tplcc <check|build|info|serve|vectors> …");
        return ExitCode::from(2);
    }
    match args[0].as_str() {
        "check" => cmd_check(&args[1..]),
        "build" => cmd_build(&args[1..]),
        "info" => cmd_info(&args[1..]),
        "serve" => cmd_serve(&args[1..]),
        "vectors" => cmd_vectors(&args[1..]),
        c => {
            eprintln!("tplcc: unknown command {c}");
            ExitCode::from(2)
        }
    }
}

fn read_srcs(files: &[String]) -> Vec<(String, String)> {
    files
        .iter()
        .map(|f| {
            let s = std::fs::read_to_string(f).unwrap_or_else(|e| panic!("{f}: {e}"));
            (f.clone(), s)
        })
        .collect()
}

/// Route sources to a frontend: a lone `.fbd.json` project goes through
/// `fbd-model` + `fbd-lower`; everything else is ST via `tplc-st`.
fn compile_srcs(srcs: &[(String, String)]) -> tplc_st::FrontendOutput {
    if srcs.len() == 1 && srcs[0].0.ends_with(".fbd.json") {
        let (fname, text) = &srcs[0];
        return match fbd_model::from_json(text) {
            Ok(project) => {
                let out = fbd_lower::lower(&project);
                tplc_st::FrontendOutput {
                    module: out.module,
                    io_bindings: out.io_bindings,
                    diags: out.diags,
                }
            }
            Err(e) => tplc_st::FrontendOutput {
                module: tplc_mir::ir::Module {
                    pous: vec![],
                    gvls: vec![],
                    tasks: vec![],
                },
                io_bindings: vec![],
                diags: vec![tplc_mir::diag::Diag::error(
                    "TPLC_E_MODEL_SCHEMA",
                    format!("{fname}: {e}"),
                    tplc_mir::ir::Prov::None,
                )],
            },
        };
    }
    let refs: Vec<(&str, &str)> = srcs.iter().map(|(f, s)| (f.as_str(), s.as_str())).collect();
    tplc_st::compile(&refs)
}

fn print_diags(diags: &[tplc_st::Diag], srcs: &[(String, String)]) -> usize {
    let mut errs = 0;
    for d in diags {
        // resolve Prov::St{file} → (name,text); unknown file → first
        let (name, text) = match d.span {
            tplc_mir::ir::Prov::St { file, .. } => srcs
                .get(file as usize)
                .map(|(n, t)| (n.as_str(), t.as_str()))
                .unwrap_or(("<src>", "")),
            _ => ("<src>", ""),
        };
        eprintln!("{}", d.render(name, text));
        if d.severity == tplc_st::Severity::Error {
            errs += 1;
        }
    }
    errs
}

/// Machine-readable diagnostic report (`--json`): one line on stdout,
/// `{"ok":bool,"errors":N,"diags":[…]}`. St spans carry file/line/col/
/// offset/len so the web IDE can underline the offending range; Fbd
/// spans carry sheet/node/pin ids.
fn diag_json(d: &tplc_st::Diag, srcs: &[(String, String)]) -> serde_json::Value {
    let sev = match d.severity {
        tplc_st::Severity::Error => "error",
        tplc_st::Severity::Warning => "warning",
        tplc_st::Severity::Note => "note",
    };
    let mut o = serde_json::json!({
        "code": d.code,
        "severity": sev,
        "message": d.message,
        "notes": d.notes,
    });
    if let Some(h) = &d.help {
        o["help"] = serde_json::json!(h);
    }
    match d.span {
        Prov::St { file, offset, len } => {
            let (name, text) = srcs
                .get(file as usize)
                .map(|(n, t)| (n.as_str(), t.as_str()))
                .unwrap_or(("<src>", ""));
            o["file"] = serde_json::json!(name);
            o["offset"] = serde_json::json!(offset);
            o["len"] = serde_json::json!(len);
            if let Some((line, col)) = d.line_col(text) {
                o["line"] = serde_json::json!(line);
                o["col"] = serde_json::json!(col);
            }
        }
        Prov::Fbd { sheet, node, pin } => {
            o["fbd"] = serde_json::json!({"sheet": sheet, "node": node, "pin": pin});
        }
        _ => {}
    }
    o
}

fn diag_report(
    diags: &[tplc_st::Diag],
    srcs: &[(String, String)],
    errs: usize,
    package: Option<&std::path::Path>,
) -> serde_json::Value {
    let mut r = serde_json::json!({
        "ok": errs == 0,
        "errors": errs,
        "diags": diags.iter().map(|d| diag_json(d, srcs)).collect::<Vec<_>>(),
    });
    if let Some(p) = package {
        r["package"] = serde_json::json!(p.display().to_string());
    }
    r
}

/// non-frontend failures (verify/link/change-deny) in --json mode:
/// synthesize a single error diag so the envelope stays uniform.
fn stage_failure_json(stage: &str, message: &str) -> serde_json::Value {
    serde_json::json!({
        "ok": false,
        "errors": 1,
        "diags": [{
            "code": "TPLC_E_DRIVER",
            "severity": "error",
            "message": format!("{stage}: {message}"),
            "notes": [],
        }],
    })
}

/// `tplcc serve <uds>` — persistent check-only compile service for the
/// ctl proxy's /compile route. Each connection sends one binary source
/// frame `[u16 count][u16 nameLen,name,u32 len,content]*N` (the proxy's
/// HTTP body verbatim) and gets back the proxy-shaped envelope
/// `{"compile":{…,"diags":[…]},"exit":N}` as JSON. The daemon does the
/// check path in-process so the IDE's diagnostics don't pay a process
/// spawn per keystroke-debounce; builds still go through `tplcc build`.
fn cmd_serve(args: &[String]) -> ExitCode {
    use std::io::Write;
    use std::os::unix::net::UnixListener;

    let sock = args
        .first()
        .map(String::as_str)
        .unwrap_or("/tmp/tplccd.sock");
    let _ = std::fs::remove_file(sock);
    let listener = match UnixListener::bind(sock) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("tplcc serve: bind {sock}: {e}");
            return ExitCode::FAILURE;
        }
    };
    eprintln!("tplcc serve: listening on {sock}");
    for conn in listener.incoming() {
        match conn {
            Ok(mut s) => {
                let reply = serve_check(&mut s);
                let _ = s.write_all(reply.as_bytes());
            }
            Err(e) => eprintln!("tplcc serve: accept: {e}"),
        }
    }
    ExitCode::SUCCESS
}

fn serve_check(s: &mut std::os::unix::net::UnixStream) -> String {
    use std::io::Read;
    let mut rd = |n: usize| -> Option<Vec<u8>> {
        let mut b = vec![0u8; n];
        s.read_exact(&mut b).ok()?;
        Some(b)
    };
    let u16be = |b: &[u8]| u16::from_le_bytes([b[0], b[1]]) as usize;
    let frame_err = |msg: &str| -> String {
        serde_json::json!({"compile": {"ok": false, "errors": 1,
            "diags": [{"code": "TPLC_E_DRIVER", "severity": "error",
                       "message": msg, "notes": []}]}, "exit": 2})
        .to_string()
    };
    let Some(hdr) = rd(2) else {
        return frame_err("short frame");
    };
    let count = u16be(&hdr);
    if count > 64 {
        return frame_err("too many files");
    }
    let mut srcs: Vec<(String, String)> = Vec::with_capacity(count);
    for _ in 0..count {
        let Some(nl) = rd(2) else {
            return frame_err("short name len");
        };
        let Some(nb) = rd(u16be(&nl)) else {
            return frame_err("short name");
        };
        let Some(cl) = rd(4) else {
            return frame_err("short content len");
        };
        let clen = u32::from_le_bytes([cl[0], cl[1], cl[2], cl[3]]) as usize;
        if clen > 4 * 1024 * 1024 {
            return frame_err("file too large");
        }
        let Some(cb) = rd(clen) else {
            return frame_err("short content");
        };
        let name = String::from_utf8_lossy(&nb).into_owned();
        let text = String::from_utf8_lossy(&cb).into_owned();
        srcs.push((name, text));
    }
    let mut out = compile_srcs(&srcs);
    let mut diags = std::mem::take(&mut out.diags);
    if !diags.iter().any(|d| d.severity == tplc_st::Severity::Error) {
        diags.extend(tplc_st::sema::check(&mut out.module));
    }
    let errs = diags
        .iter()
        .filter(|d| d.severity == tplc_st::Severity::Error)
        .count();
    serde_json::json!({
        "compile": diag_report(&diags, &srcs, errs, None),
        "exit": if errs > 0 { 1 } else { 0 },
    })
    .to_string()
}

fn cmd_check(args: &[String]) -> ExitCode {
    let mut json_mode = false;
    let mut files: Vec<String> = Vec::new();
    for a in args {
        match a.as_str() {
            "--json" => json_mode = true,
            f => files.push(f.into()),
        }
    }
    let srcs = read_srcs(&files);
    let mut out = compile_srcs(&srcs);
    let mut diags = std::mem::take(&mut out.diags);
    if !diags.iter().any(|d| d.severity == tplc_st::Severity::Error) {
        diags.extend(tplc_st::sema::check(&mut out.module));
    }
    let errs = diags
        .iter()
        .filter(|d| d.severity == tplc_st::Severity::Error)
        .count();
    if json_mode {
        println!("{}", diag_report(&diags, &srcs, errs, None));
    } else {
        print_diags(&diags, &srcs);
    }
    if errs > 0 {
        eprintln!("{errs} error(s)");
        ExitCode::FAILURE
    } else {
        if !json_mode {
            println!("ok: {} pou(s)", out.module.pous.len());
        }
        ExitCode::SUCCESS
    }
}

fn cmd_build(args: &[String]) -> ExitCode {
    let mut out_path = PathBuf::from("out.tplc");
    let mut change_against: Option<PathBuf> = None;
    let mut allow_retain_loss = false;
    let mut json_mode = false;
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
            "--allow-retain-loss" => allow_retain_loss = true,
            "--json" => json_mode = true,
            "--rename" => {
                if let Some((o, n)) = args[i + 1].split_once('=') {
                    renames.push((o.into(), n.into()));
                }
                i += 1;
            }
            f => files.push(f.into()),
        }
        i += 1;
    }
    if files.is_empty() {
        eprintln!("tplcc build: no sources");
        return ExitCode::from(2);
    }
    let srcs = read_srcs(&files);
    let mut out = compile_srcs(&srcs);
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

    let program: Pou = out
        .module
        .pous
        .iter()
        .find(|p| p.kind == PouKind::Program)
        .cloned()
        .unwrap_or_else(|| panic!("no PROGRAM found"));
    let prog_name = program.name.clone();

    let mut layout = layout_program(&out.module, &program);
    // io triples for descriptor channels
    let io_triples: Vec<(String, tplc_mir::ir::At, u64)> = out
        .io_bindings
        .iter()
        .map(|b| (b.name.clone(), b.at.clone(), b.at.byte))
        .collect();
    let prov_of = |name: &str| -> Option<Prov> {
        let short = name.rsplit('/').next().unwrap_or(name);
        program
            .iface
            .iter()
            .find(|v| v.name == short)
            .map(|v| v.prov.clone())
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
        for r in &plan.rules {
            match r {
                mig::Rule::Deny { reason, detail, .. } => {
                    deny_errs += 1;
                    eprintln!("error[CHANGE-DENY{reason}]: {detail}");
                }
                mig::Rule::Dropped { name, retain, .. } => eprintln!(
                    "note: field {name} dropped{}",
                    if *retain { " (retain value lost)" } else { "" }
                ),
                mig::Rule::Alias {
                    old_name, new_name, ..
                } => {
                    eprintln!("note: alias {old_name} → {new_name}")
                }
                _ => {}
            }
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
                    aux: 0,
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
                    old_field, retain, ..
                } => tplc_pkg::sections::MigdRule {
                    kind: 3,
                    severity: 1,
                    transform_id: 0,
                    old_field: *old_field,
                    new_field: u32::MAX,
                    aux: *retain as u32,
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

    let lctx = Context::create();
    let llmod = emit_program(
        &lctx,
        module_ref,
        layout_ref,
        prog_ref,
        io_binds,
        &prog_name,
        uuid,
        layout_hash,
        mig_emit.as_ref(),
    );
    if let Err(e) = llmod.verify() {
        if json_mode {
            println!("{}", stage_failure_json("verify", &e.to_string()));
        }
        eprintln!("tplcc: LLVM verify failed: {e}");
        return ExitCode::FAILURE;
    }

    let tmp = std::env::temp_dir().join("tplcc_build");
    std::fs::create_dir_all(&tmp).unwrap();
    let bc = tmp.join("prog.bc");
    let obj = tmp.join("prog.o");
    emit_bitcode(&llmod, &bc).unwrap();
    emit_object(&llmod, &obj).unwrap();
    let bc_bytes = std::fs::read(&bc).unwrap();

    // link .o + tplc-rt → deployable .so (doc 01 §5: shared object).
    // The rt source is embedded; rustc produces a lean no_std
    // staticlib (~10KB .so, only `tplc_*` exports + abort).
    // TPLC_RT_LIB overrides for prebuilt archives.
    let rt_lib = std::env::var("TPLC_RT_LIB")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            const RT_SRC: &str = include_str!("../../tplc-rt/src/lib.rs");
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
    let obj_bytes = std::fs::read(&so).unwrap();

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
            format!(
                "{{\"name\":\"{}\",\"region\":\"{}\",\"byte\":{},\"bit\":{}}}",
                b.name,
                r,
                b.at.byte,
                b.at.bit.map(|x| x.to_string()).unwrap_or("null".into())
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let manifest = format!(
        "{{\"schema\":\"tplc.manifest.v1\",\"name\":\"{prog_name}\",\"package_uuid\":\"{uuid_str}\",\"abi_version\":1,\"layout_hash\":\"{lh_hex}\"}}"
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

fn cmd_info(files: &[String]) -> ExitCode {
    for f in files {
        let bytes = std::fs::read(f).unwrap_or_else(|e| panic!("{f}: {e}"));
        let pkg = tplc_pkg::reader::Package::parse(&bytes).unwrap_or_else(|e| panic!("{f}: {e}"));
        println!("{f}: {} section(s)", pkg.toc().len());
        if let Ok(m) = pkg.manifest_json() {
            println!("manifest: {m}");
        }
        for (i, s) in pkg.sections().enumerate() {
            println!(
                "  #{i} type=0x{:04x} v{} {}B{}",
                s.section_type,
                s.section_version,
                s.data.len(),
                if s.required() { " required" } else { "" }
            );
        }
    }
    ExitCode::SUCCESS
}

/* ------------------------------------------------------------------ */
/* vectors — behavior-vector runner (P1-10, doc 14 §3.1)               */
/* JSONL vectors drive the L1 interpreter; oracle-matiec produces      */
/* expected traces as DATA (quarantined subprocess, never linked).      */
/* ------------------------------------------------------------------ */

use tplc_mir::interp::{program_ctx, Rt};

fn parse_val(s: &str) -> tplc_mir::interp::Value {
    use tplc_mir::interp::Value;
    let u = s.trim();
    match u {
        "TRUE" | "true" => return Value::Bool(true),
        "FALSE" | "false" => return Value::Bool(false),
        _ => {}
    }
    if let Some(t) = u.strip_prefix("T#") {
        let (num, mul) = if let Some(x) = t.strip_suffix("ms") {
            (x, 1_000_000i64)
        } else if let Some(x) = t.strip_suffix("us") {
            (x, 1_000i64)
        } else if let Some(x) = t.strip_suffix("ns") {
            (x, 1i64)
        } else if let Some(x) = t.strip_suffix("s") {
            (x, 1_000_000_000i64)
        } else {
            (t, 1i64)
        };
        let v: f64 = num.trim_start_matches('-').parse().unwrap();
        let neg = num.starts_with('-');
        return Value::Time(if neg { -1 } else { 1 } * (v * mul as f64) as i64);
    }
    if let Some(x) = u.strip_prefix("16#") {
        return Value::I(i64::from_str_radix(x, 16).unwrap());
    }
    if u.contains('.') || u.contains('e') || u.contains('E') {
        return Value::F(u.parse().unwrap());
    }
    if let Ok(v) = u.parse::<i64>() {
        return Value::I(v);
    }
    if let Ok(v) = u.parse::<u64>() {
        return Value::U(v);
    }
    panic!("unparseable vector value {s}");
}

fn hex_bytes(s: &str) -> Vec<u8> {
    let s = s.trim();
    (0..s.len() / 2)
        .map(|i| u8::from_str_radix(&s[2 * i..2 * i + 2], 16).unwrap())
        .collect()
}

/// image-region read for one IoBind at declared address
fn img_get(img: &[u8], b: &IoBind) -> tplc_mir::interp::Value {
    use tplc_mir::interp::Value;
    let byte = *img.get(b.at.byte as usize).unwrap_or(&0);
    match &b.ty {
        tplc_mir::ir::Type::Bool => Value::Bool(byte & (1 << b.at.bit.unwrap_or(0)) != 0),
        tplc_mir::ir::Type::Int { signed, bits } => {
            let n = (*bits as usize) / 8;
            let mut v: u64 = 0;
            for i in 0..n.min(8) {
                v |= (*img.get(b.at.byte as usize + i).unwrap_or(&0) as u64) << (8 * i);
            }
            if *signed {
                Value::I(v as i64)
            } else {
                Value::U(v)
            }
        }
        _ => Value::I(0),
    }
}

/// image-region expected bytes for one IoBind's shadow value
fn img_put(img: &mut [u8], b: &IoBind, v: &tplc_mir::interp::Value) {
    use tplc_mir::interp::Value;
    match (&b.ty, v) {
        (tplc_mir::ir::Type::Bool, Value::Bool(x)) => {
            let m = 1u8 << b.at.bit.unwrap_or(0);
            let e = &mut img[b.at.byte as usize];
            *e = if *x { *e | m } else { *e & !m };
        }
        (tplc_mir::ir::Type::Int { bits, .. }, Value::I(x)) => {
            let n = (*bits as usize) / 8;
            for i in 0..n.min(8) {
                img[b.at.byte as usize + i] = (*x as u64 >> (8 * i)) as u8;
            }
        }
        (tplc_mir::ir::Type::Int { bits, .. }, Value::U(x)) => {
            let n = (*bits as usize) / 8;
            for i in 0..n.min(8) {
                img[b.at.byte as usize + i] = (x >> (8 * i)) as u8;
            }
        }
        _ => {}
    }
}

fn run_vector(v: &serde_json::Value, src_dir: &std::path::Path) -> Result<(), String> {
    let id = v["id"].as_str().unwrap_or("?");
    // "program": one file, or "programs": several sources run against the
    // same trace — their %Q images are compared cycle-for-cycle so an
    // .st and an .fbd.json model of the same logic must produce
    // bit-identical outputs (equivalence corpus).
    let files: Vec<String> = match &v["programs"] {
        serde_json::Value::Array(a) => a.iter().map(|p| p.as_str().unwrap().to_string()).collect(),
        _ => vec![v["program"].as_str().unwrap().to_string()],
    };
    let mut runs = Vec::new();
    for f in &files {
        let prog_file = src_dir.join(f);
        let src =
            std::fs::read_to_string(&prog_file).map_err(|e| format!("{id}: {prog_file:?}: {e}"))?;
        let mut out = compile_srcs(&[(prog_file.to_string_lossy().into(), src)]);
        let mut diags = std::mem::take(&mut out.diags);
        diags.extend(tplc_st::sema::check(&mut out.module));
        if diags.iter().any(|d| d.severity == tplc_st::Severity::Error) {
            return Err(format!("{id} {f}: frontend errors"));
        }
        let program = out
            .module
            .pous
            .iter()
            .find(|p| p.kind == tplc_mir::ir::PouKind::Program)
            .cloned()
            .ok_or("no PROGRAM")?;
        let layout = layout_program(&out.module, &program);
        let ctx = program_ctx(&layout);
        let blob = vec![0u8; layout.total_size as usize];
        runs.push((out, program, layout, ctx, blob));
    }
    let mut img_in = vec![0u8; 64];
    let cycles = v["cycles"].as_array().ok_or("no cycles")?;
    for (ci, c) in cycles.iter().enumerate() {
        // apply image-in bytes
        if let Some(ih) = c["i"].as_str() {
            for (i, b) in hex_bytes(ih).iter().enumerate() {
                if i < img_in.len() {
                    img_in[i] = *b;
                }
            }
        }
        let mut q_imgs: Vec<Vec<u8>> = Vec::new();
        for (pi, (out, program, layout, ctx, blob)) in runs.iter_mut().enumerate() {
            let tplc_mir::ir::Body::Stmts(stmts) = &program.body else {
                return Err("program body not stmts".into());
            };
            // state lives in blob; Rt is rebuilt per scan
            let mut rt = Rt::new(blob, layout, &out.module, 0);
            rt.now_ns = c["t_ns"].as_u64().unwrap_or(0);
            rt.cycle = ci as u64;
            for b in &out.io_bindings {
                if b.at.region == AtRegion::I {
                    let val = img_get(&img_in, b);
                    let short = b.name.rsplit('.').next().unwrap_or(&b.name);
                    rt.write_field(short, &val);
                }
            }
            rt.exec_all(ctx, stmts);
            // check expect_fields (applied to every program — the corpus
            // uses identical field names across language variants)
            if let Some(ef) = c["expect_fields"].as_object() {
                for (name, exp) in ef {
                    let want = parse_val(exp.as_str().unwrap());
                    let got = if name.contains('.') {
                        let (inst, mem) = name.split_once('.').unwrap();
                        rt.read_ref(
                            ctx,
                            &tplc_mir::ir::Ref::Field(
                                Box::new(tplc_mir::ir::Ref::Var(inst.into())),
                                mem.into(),
                            ),
                        )
                    } else {
                        rt.read_field(name)
                    };
                    let ok = match (&want, &got) {
                        (tplc_mir::interp::Value::Bool(a), tplc_mir::interp::Value::Bool(b)) => {
                            a == b
                        }
                        (tplc_mir::interp::Value::Time(a), tplc_mir::interp::Value::Time(b)) => {
                            a == b
                        }
                        _ => want.as_i64() == got.as_i64(),
                    };
                    if !ok {
                        return Err(format!(
                            "{id} cycle {ci} {}: {name} want {want:?} got {got:?}",
                            files[pi]
                        ));
                    }
                }
            }
            let mut img_out = vec![0u8; 64];
            for b in &out.io_bindings {
                if b.at.region == AtRegion::Q {
                    let short = b.name.rsplit('.').next().unwrap_or(&b.name);
                    let val = rt.read_field(short);
                    img_put(&mut img_out, b, &val);
                }
            }
            q_imgs.push(img_out);
        }
        // cross-program equivalence + optional expect_q
        for (pi, o) in q_imgs.iter().enumerate().skip(1) {
            if *o != q_imgs[0] {
                return Err(format!(
                    "{id} cycle {ci}: {} diverges from {}: {} vs {}",
                    files[pi],
                    files[0],
                    hex_str(&o[..4]),
                    hex_str(&q_imgs[0][..4])
                ));
            }
        }
        if let Some(qh) = c["expect_q"].as_str() {
            let want = hex_bytes(qh);
            if q_imgs[0][..want.len()] != want[..] {
                return Err(format!(
                    "{id} cycle {ci}: %Q want {} got {}",
                    qh,
                    hex_str(&q_imgs[0][..want.len()])
                ));
            }
        }
    }
    Ok(())
}

fn hex_str(bs: &[u8]) -> String {
    bs.iter().map(|b| format!("{b:02x}")).collect()
}

fn cmd_vectors(args: &[String]) -> ExitCode {
    let mut files = Vec::new();
    let mut src_dir = PathBuf::from(".");
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "run" => {}
            "--src" => {
                src_dir = PathBuf::from(&args[i + 1]);
                i += 1;
            }
            f => files.push(PathBuf::from(f)),
        }
        i += 1;
    }
    let mut pass = 0;
    let mut fail = 0;
    for f in &files {
        let text = if f.is_dir() {
            let mut t = String::new();
            for e in std::fs::read_dir(f).unwrap() {
                let p = e.unwrap().path();
                if p.extension().map(|x| x == "jsonl").unwrap_or(false) {
                    t += &std::fs::read_to_string(&p).unwrap();
                    src_dir = p.parent().unwrap().to_path_buf();
                }
            }
            t
        } else {
            src_dir = f.parent().unwrap_or(&src_dir).to_path_buf();
            std::fs::read_to_string(f).unwrap()
        };
        for line in text.lines().filter(|l| !l.trim().is_empty()) {
            let v: serde_json::Value = serde_json::from_str(line).unwrap();
            match run_vector(&v, &src_dir) {
                Ok(()) => {
                    pass += 1;
                    println!("PASS {}", v["id"].as_str().unwrap_or("?"));
                }
                Err(e) => {
                    fail += 1;
                    println!("FAIL {e}");
                }
            }
        }
    }
    println!("{pass} pass, {fail} fail");
    if fail > 0 {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
