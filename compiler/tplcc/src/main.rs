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
use tplc_codegen::llvm_emit::{
    emit_bitcode, emit_library_for_target, emit_object, emit_program_for_target, host_triple,
    set_target,
};
use tplc_mir::desc;
use tplc_mir::ir::{AtRegion, IoBind, Pou, PouKind, Prov, Type};
use tplc_mir::layout::layout_program;
use tplc_pkg::format::SectionType;
use tplc_pkg::writer::PackageWriter;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("usage: tplcc <check|blocks|build|info|serve|vectors|sign|src> …");
        return ExitCode::from(2);
    }
    match args[0].as_str() {
        "check" => cmd_check(&args[1..]),
        "blocks" => cmd_blocks(&args[1..]),
        "src" => cmd_src(&args[1..]),
        "build" => cmd_build(&args[1..]),
        "info" => cmd_info(&args[1..]),
        "serve" => cmd_serve(&args[1..]),
        "vectors" => cmd_vectors(&args[1..]),
        "sign" => cmd_sign(&args[1..]),
        "pkg" => cmd_pkg(&args[1..]),
        c => {
            eprintln!("tplcc: unknown command {c}");
            ExitCode::from(2)
        }
    }
}

/// `tplcc sign` — ed25519 package provenance (doc 15 §4.1).
///
///   tplcc sign keygen --dir <D> --name <N>   write N.key (sk) + <keyid>.pub
///   tplcc sign sign <in.tplc> --key <K.key> [-o <out.tplc>]
///   tplcc sign verify <pkg.tplc> [--trust-dir <D>]
///
/// Signature section payload: u32 ver(=1) | key_id[32] | pubkey[32] |
/// sig[64], where sig = ed25519.sk.sign(signing_hash) and
/// key_id = sha256(pubkey). signing_hash hashes non-signature TOC
/// entries (offset zeroed) + payloads, so it is stable when a section
/// is appended; signing an already-signed package replaces the
/// signature section.
/// `tplcc pkg` — package section plumbing (doc 15 §2 IO_MAP inject,
/// post-compile manifest edits).
///
///   tplcc pkg add-section <in.tplc> --type <u32|0xHEX> --file <F>
///       [-o <out.tplc>]
///
/// Copies every section of <in.tplc> then appends --file's bytes as a
/// new section of --type. Note this invalidates any signature —
/// re-sign afterwards.
fn cmd_pkg(args: &[String]) -> ExitCode {
    if args.first().map(|s| s.as_str()) != Some("add-section") {
        eprintln!("usage: tplcc pkg add-section <in.tplc> --type <T> --file <F> [-o out]");
        return ExitCode::from(2);
    }
    let mut input: Option<String> = None;
    let mut out: Option<String> = None;
    let mut stype: Option<u32> = None;
    let mut sfile: Option<String> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--type" if i + 1 < args.len() => {
                let t = &args[i + 1];
                stype = if let Some(h) = t.strip_prefix("0x") {
                    u32::from_str_radix(h, 16).ok()
                } else {
                    t.parse().ok()
                };
                i += 2;
            }
            "--file" if i + 1 < args.len() => {
                sfile = Some(args[i + 1].clone());
                i += 2;
            }
            "-o" if i + 1 < args.len() => {
                out = Some(args[i + 1].clone());
                i += 2;
            }
            a if !a.starts_with('-') && input.is_none() => {
                input = Some(args[i].clone());
                i += 1;
            }
            a => {
                eprintln!("pkg add-section: bad arg {a}");
                return ExitCode::from(2);
            }
        }
    }
    let (Some(input), Some(stype), Some(sfile)) = (input, stype, sfile) else {
        eprintln!("pkg add-section: needs <in.tplc> --type <T> --file <F>");
        return ExitCode::from(2);
    };
    let out = out.unwrap_or_else(|| input.clone());
    let bytes = std::fs::read(&input).unwrap_or_else(|e| {
        eprintln!("{input}: {e}");
        std::process::exit(1)
    });
    let payload = std::fs::read(&sfile).unwrap_or_else(|e| {
        eprintln!("{sfile}: {e}");
        std::process::exit(1)
    });
    let pkg = tplc_pkg::Package::parse(&bytes).unwrap_or_else(|e| {
        eprintln!("{input}: parse: {e:?}");
        std::process::exit(1)
    });
    let mut w = PackageWriter::new();
    w.uuid_raw(pkg.header.uuid);
    for s in pkg.sections() {
        let _ = w.add_section_raw(
            s.section_type,
            s.data.to_vec(),
            s.required(),
            s.section_version,
            s.aux,
        );
    }
    w.add_section_raw(stype, payload, false, 1, 0)
        .unwrap_or_else(|e| {
            eprintln!("add-section: {e:?}");
            std::process::exit(1)
        });
    let built = w.build().unwrap_or_else(|e| {
        eprintln!("rebuild: {e:?}");
        std::process::exit(1)
    });
    std::fs::write(&out, &built).unwrap_or_else(|e| {
        eprintln!("{out}: {e}");
        std::process::exit(1)
    });
    println!("add-section type 0x{stype:04x} → {out}");
    ExitCode::SUCCESS
}

fn cmd_sign(args: &[String]) -> ExitCode {
    use ed25519_dalek::{Signer, Verifier};

    let hex32 = |b: &[u8]| -> String { b.iter().map(|x| format!("{x:02x}")).collect() };
    let key_id_of = |pk: &[u8; 32]| -> [u8; 32] { Sha256::digest(pk).into() };

    if args.is_empty() {
        eprintln!("usage: tplcc sign <keygen|sign|verify> …");
        return ExitCode::from(2);
    }
    match args[0].as_str() {
        "keygen" => {
            let mut dir = String::from(".");
            let mut name = String::from("key");
            let mut i = 1;
            while i < args.len() {
                match args[i].as_str() {
                    "--dir" => {
                        dir = args[i + 1].clone();
                        i += 2;
                    }
                    "--name" => {
                        name = args[i + 1].clone();
                        i += 2;
                    }
                    _ => {
                        eprintln!("sign keygen: bad arg {}", args[i]);
                        return ExitCode::from(2);
                    }
                }
            }
            if std::fs::create_dir_all(&dir).is_err() {
                eprintln!("sign keygen: mkdir {dir} failed");
                return ExitCode::from(1);
            }
            let mut sk_arr = [0u8; 32];
            rand::RngCore::fill_bytes(&mut rand::rngs::OsRng, &mut sk_arr);
            let sk = ed25519_dalek::SigningKey::from_bytes(&sk_arr);
            let pk = sk.verifying_key();
            let kid = key_id_of(pk.as_bytes());
            let kp = format!("{dir}/{name}.key");
            let pp = format!("{dir}/{}.pub", hex32(&kid));
            if std::fs::write(&kp, sk.to_bytes()).is_err()
                || std::fs::write(&pp, pk.to_bytes()).is_err()
            {
                eprintln!("sign keygen: write failed");
                return ExitCode::from(1);
            }
            println!("key_id {} → {kp} + {pp}", hex32(&kid));
            ExitCode::SUCCESS
        }
        "sign" => {
            let mut input: Option<String> = None;
            let mut key: Option<String> = None;
            let mut out: Option<String> = None;
            let mut i = 1;
            while i < args.len() {
                match args[i].as_str() {
                    "--key" => {
                        key = Some(args[i + 1].clone());
                        i += 2;
                    }
                    "-o" | "--out" => {
                        out = Some(args[i + 1].clone());
                        i += 2;
                    }
                    a if input.is_none() && !a.starts_with('-') => {
                        input = Some(a.to_string());
                        i += 1;
                    }
                    _ => {
                        eprintln!("sign sign: bad arg {}", args[i]);
                        return ExitCode::from(2);
                    }
                }
            }
            let (Some(input), Some(key)) = (input, key) else {
                eprintln!("usage: tplcc sign sign <in.tplc> --key <K.key> [-o out]");
                return ExitCode::from(2);
            };
            let out = out.unwrap_or_else(|| input.clone());
            let bytes = std::fs::read(&input).unwrap_or_else(|e| {
                eprintln!("{input}: {e}");
                std::process::exit(1)
            });
            let skb = std::fs::read(&key).unwrap_or_else(|e| {
                eprintln!("{key}: {e}");
                std::process::exit(1)
            });
            if skb.len() != 32 {
                eprintln!("{key}: expected 32-byte ed25519 secret");
                return ExitCode::from(1);
            }
            let sk_arr: [u8; 32] = skb[..32].try_into().unwrap();
            let sk = ed25519_dalek::SigningKey::from_bytes(&sk_arr);
            let pk = sk.verifying_key();
            let kid = key_id_of(pk.as_bytes());

            let pkg = tplc_pkg::Package::parse(&bytes).unwrap_or_else(|e| {
                eprintln!("{input}: parse: {e:?}");
                std::process::exit(1)
            });
            /* re-emit helper: copies every non-signature section */
            let emit = |sig_payload: Option<Vec<u8>>| -> Vec<u8> {
                let mut w = PackageWriter::new();
                w.uuid_raw(pkg.header.uuid);
                for s in pkg.sections() {
                    if s.section_type == SectionType::Signature as u32 {
                        continue; // replaced by the fresh signature
                    }
                    let _ = w.add_section_raw(
                        s.section_type,
                        s.data.to_vec(),
                        s.required(),
                        s.section_version,
                        s.aux,
                    );
                }
                if let Some(p) = sig_payload {
                    w.add_section(SectionType::Signature, p, false, 1, 0)
                        .unwrap_or_else(|e| {
                            eprintln!("sig section: {e:?}");
                            std::process::exit(1)
                        });
                }
                w.build().unwrap_or_else(|e| {
                    eprintln!("rebuild: {e:?}");
                    std::process::exit(1)
                })
            };
            let sig = sk.sign(&pkg.signing_hash());
            let mut payload = Vec::with_capacity(132);
            payload.extend_from_slice(&1u32.to_le_bytes());
            payload.extend_from_slice(&kid);
            payload.extend_from_slice(pk.as_bytes());
            payload.extend_from_slice(&sig.to_bytes());
            let signed = emit(Some(payload));
            std::fs::write(&out, &signed).unwrap_or_else(|e| {
                eprintln!("{out}: {e}");
                std::process::exit(1)
            });
            println!("signed {out} key_id {}", hex32(&kid));
            ExitCode::SUCCESS
        }
        "verify" => {
            let mut input: Option<String> = None;
            let mut trust_dir: Option<String> = None;
            let mut i = 1;
            while i < args.len() {
                match args[i].as_str() {
                    "--trust-dir" => {
                        trust_dir = Some(args[i + 1].clone());
                        i += 2;
                    }
                    a if input.is_none() && !a.starts_with('-') => {
                        input = Some(a.to_string());
                        i += 1;
                    }
                    _ => {
                        eprintln!("sign verify: bad arg {}", args[i]);
                        return ExitCode::from(2);
                    }
                }
            }
            let Some(input) = input else {
                eprintln!("usage: tplcc sign verify <pkg> [--trust-dir D]");
                return ExitCode::from(2);
            };
            let bytes = std::fs::read(&input).unwrap_or_else(|e| {
                eprintln!("{input}: {e}");
                std::process::exit(1)
            });
            let pkg = tplc_pkg::Package::parse(&bytes).unwrap_or_else(|e| {
                eprintln!("{input}: parse: {e:?}");
                std::process::exit(1)
            });
            let Some(sec) = pkg.section(SectionType::Signature) else {
                eprintln!("{input}: unsigned");
                return ExitCode::from(1);
            };
            let d = sec.data;
            if d.len() != 132 || u32::from_le_bytes(d[0..4].try_into().unwrap()) != 1 {
                eprintln!("{input}: bad signature section");
                return ExitCode::from(1);
            }
            let kid: &[u8; 32] = d[4..36].try_into().unwrap();
            let pk_bytes: &[u8; 32] = d[36..68].try_into().unwrap();
            if &key_id_of(pk_bytes) != kid {
                eprintln!("{input}: key_id/pubkey mismatch");
                return ExitCode::from(1);
            }
            let pk = ed25519_dalek::VerifyingKey::from_bytes(pk_bytes).unwrap();
            let sig = ed25519_dalek::Signature::from_bytes(d[68..132].try_into().unwrap());
            if pk.verify(&pkg.signing_hash(), &sig).is_err() {
                eprintln!("{input}: signature invalid");
                return ExitCode::from(1);
            }
            if let Some(td) = trust_dir {
                let f = format!("{td}/{}.pub", hex32(kid));
                match std::fs::read(&f) {
                    Ok(b) if b == pk_bytes => {}
                    Ok(_) => {
                        eprintln!("{input}: trust store key mismatch");
                        return ExitCode::from(1);
                    }
                    Err(e) => {
                        eprintln!("{input}: untrusted signer ({f}: {e})");
                        return ExitCode::from(1);
                    }
                }
            }
            println!("{input}: signature OK key_id {}", hex32(kid));
            ExitCode::SUCCESS
        }
        c => {
            eprintln!("tplcc sign: unknown subcommand {c}");
            ExitCode::from(2)
        }
    }
}

fn read_srcs(files: &[String]) -> Vec<(String, String)> {
    files
        .iter()
        .map(|f| {
            // a source file that is not UTF-8 (or unreadable) is a
            // diagnostic, not a crash — the fuzzer feeds raw bytes
            let bytes = std::fs::read(f).unwrap_or_else(|e| {
                eprintln!("error: {f}: {e}");
                std::process::exit(1);
            });
            let s = String::from_utf8(bytes).unwrap_or_else(|e| {
                eprintln!("error[TPLC_E_ST_PARSE] {f}: not valid UTF-8 ({e})");
                std::process::exit(1);
            });
            (f.clone(), s)
        })
        .collect()
}

/// Compile a mixed source set. `.fbd.json` files go through the FBD
/// frontend, everything else through the ST frontend; the two modules
/// merge into one namespace so POUs call across languages freely —
/// FBD blocks instantiate ST FBs/FUNCTIONs (extern signatures), ST
/// code declares/calls FBD-defined POUs (resolved post-merge in sema),
/// and `st` blocks expand into generated FUNCTION_BLOCKs compiled as
/// ordinary ST.
///
/// Returns the output plus the *effective* source list: st-block
/// expansion may append generated sources, and `Prov::St{file}` diag
/// indices refer into that list (`.fbd.json` entries are masked to
/// empty text for the ST frontend so indices stay aligned).
fn compile_srcs(srcs: &[(String, String)]) -> (tplc_st::FrontendOutput, Vec<(String, String)>) {
    let has_fbd = srcs.iter().any(|(n, _)| n.ends_with(".fbd.json"));
    if !has_fbd {
        let refs: Vec<(&str, &str)> =
            srcs.iter().map(|(f, s)| (f.as_str(), s.as_str())).collect();
        return (tplc_st::compile(&refs), srcs.to_vec());
    }

    // Parse each FBD project; expand st blocks into generated sources.
    let mut eff: Vec<(String, String)> = srcs.to_vec();
    let mut projects = Vec::new();
    let mut early_diags = Vec::new();
    for (fname, text) in srcs {
        if !fname.ends_with(".fbd.json") {
            continue;
        }
        match fbd_model::from_json(text) {
            Ok(mut project) => {
                eff.extend(fbd_lower::expand_st_blocks(&mut project));
                projects.push(project);
            }
            Err(e) => early_diags.push(tplc_mir::diag::Diag::error(
                "TPLC_E_MODEL_SCHEMA",
                format!("{fname}: {e}"),
                tplc_mir::ir::Prov::None,
            )),
        }
    }

    // ST frontend over the masked list — .fbd.json positions stay
    // occupied so Prov::St{file} matches `eff` indices exactly.
    let st_refs: Vec<(&str, &str)> = eff
        .iter()
        .map(|(n, c)| {
            (
                n.as_str(),
                if n.ends_with(".fbd.json") { "" } else { c.as_str() },
            )
        })
        .collect();
    let mut out = tplc_st::compile(&st_refs);

    // FBD frontend per project — ST pous visible as extern signatures.
    for project in &projects {
        let lo = fbd_lower::lower_mixed(project, &out.module.pous);
        merge_frontend(&mut out, lo.module, lo.io_bindings);
        out.diags.extend(lo.diags);
    }
    out.diags.extend(early_diags);
    (out, eff)
}

/// Merge an FBD-lowered module into the ST frontend output. Empty-body
/// POUs are declaration stubs (the UI ships interface decls for pin
/// info): a decl merges into the real impl if the visible interface
/// matches; two impls or mismatched interfaces are hard errors.
fn merge_frontend(
    dst: &mut tplc_st::FrontendOutput,
    module: tplc_mir::ir::Module,
    io: Vec<tplc_st::IoBind>,
) {
    use tplc_mir::ir::{Body, Pou, PouKind};
    let empty = |b: &Body| matches!(b, Body::Stmts(s) if s.is_empty());
    let visible = |p: &Pou| -> Vec<String> {
        let mut v: Vec<_> = p
            .iface
            .iter()
            .filter(|v| {
                matches!(
                    v.kind,
                    tplc_mir::ir::VarKind::Input
                        | tplc_mir::ir::VarKind::Output
                        | tplc_mir::ir::VarKind::InOut
                )
            })
            .map(|v| {
                format!(
                    "{}|{:?}|{:?}",
                    v.name.to_ascii_uppercase(),
                    v.kind,
                    v.ty
                )
                .to_ascii_uppercase()
            })
            .collect();
        v.sort();
        v
    };
    let mut errs: Vec<tplc_mir::diag::Diag> = Vec::new();
    for pou in module.pous {
        let dup = dst
            .module
            .pous
            .iter_mut()
            .find(|p| p.name.eq_ignore_ascii_case(&pou.name));
        let Some(ex) = dup else {
            dst.module.pous.push(pou);
            continue;
        };
        if ex.kind != pou.kind {
            errs.push(tplc_mir::diag::Diag::error(
                "TPLC_E_MIX_DUP_POU",
                format!(
                    "POU {} declared as {:?} and {:?}",
                    pou.name, ex.kind, pou.kind
                ),
                tplc_mir::ir::Prov::None,
            ));
            continue;
        }
        match (empty(&ex.body), empty(&pou.body)) {
            (false, false) => errs.push(tplc_mir::diag::Diag::error(
                "TPLC_E_MIX_DUP_POU",
                format!("POU {} has two implementations", pou.name),
                tplc_mir::ir::Prov::None,
            )),
            _ => {
                if ex.kind == PouKind::Function
                    && format!("{:?}", ex.ret_ty).to_ascii_uppercase()
                        != format!("{:?}", pou.ret_ty).to_ascii_uppercase()
                    || visible(ex) != visible(&pou)
                {
                    errs.push(tplc_mir::diag::Diag::error(
                        "TPLC_E_MIX_IFACE",
                        format!(
                            "declared interface of {} does not match its implementation",
                            pou.name
                        ),
                        tplc_mir::ir::Prov::None,
                    ));
                    continue;
                }
                if empty(&ex.body) {
                    *ex = pou; // decl stub → impl
                }
            }
        }
    }
    for gvl in module.gvls {
        let dup = dst
            .module
            .gvls
            .iter_mut()
            .find(|g| g.name.eq_ignore_ascii_case(&gvl.name));
        let Some(g) = dup else {
            dst.module.gvls.push(gvl);
            continue;
        };
        g.retain |= gvl.retain;
        for v in gvl.vars {
            if g.vars.iter().any(|x| x.name.eq_ignore_ascii_case(&v.name)) {
                errs.push(tplc_mir::diag::Diag::error(
                    "TPLC_E_MIX_DUP_GVL",
                    format!("global {}.{} declared in both ST and FBD", g.name, v.name),
                    tplc_mir::ir::Prov::None,
                ));
            } else {
                g.vars.push(v);
            }
        }
    }
    for t in module.tasks {
        if dst
            .module
            .tasks
            .iter()
            .any(|x| x.name.eq_ignore_ascii_case(&t.name))
        {
            errs.push(tplc_mir::diag::Diag::error(
                "TPLC_E_MIX_DUP_TASK",
                format!("task {} defined in both ST and FBD", t.name),
                tplc_mir::ir::Prov::None,
            ));
        } else {
            dst.module.tasks.push(t);
        }
    }
    dst.io_bindings.extend(io);
    dst.module.checked_arith |= module.checked_arith;
    dst.diags.extend(errs);
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
    let (mut out, eff) = compile_srcs(&srcs);
    let mut diags = std::mem::take(&mut out.diags);
    if !diags.iter().any(|d| d.severity == tplc_st::Severity::Error) {
        diags.extend(tplc_st::sema::check(&mut out.module));
    }
    let errs = diags
        .iter()
        .filter(|d| d.severity == tplc_st::Severity::Error)
        .count();
    serde_json::json!({
        "compile": diag_report(&diags, &eff, errs, None),
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
    let (mut out, eff) = compile_srcs(&srcs);
    let mut diags = std::mem::take(&mut out.diags);
    if !diags.iter().any(|d| d.severity == tplc_st::Severity::Error) {
        diags.extend(tplc_st::sema::check(&mut out.module));
    }
    let errs = diags
        .iter()
        .filter(|d| d.severity == tplc_st::Severity::Error)
        .count();
    if json_mode {
        println!("{}", diag_report(&diags, &eff, errs, None));
    } else {
        print_diags(&diags, &eff);
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

/// `tplcc blocks` — POU catalog for external tools (the web IDE's
/// block palette). Compiles the given sources exactly like `check`
/// (mixed ST + FBD + `.tplc` library packages), then emits every
/// FUNCTION/FUNCTION_BLOCK interface as JSON:
///
///   tplcc blocks [--json] <src.st|project.fbd.json|lib.tplc>…
///
/// ```json
/// {"pous":N,"errors":N,"blocks":[{"name","kind","ret_ty","file",
///   "pins":[{"name","dir","ty","init"}]}],
///  "enums":{"NAME":["A","B"]}}
/// ```
///
/// `file` is the source the POU was declared in (for .tplc inputs the
/// embedded ST_SOURCES name), so a client can fetch the implementation
/// source for auto-attach. Warnings/diags do not fail the command —
/// the catalog is best-effort.
fn cmd_blocks(args: &[String]) -> ExitCode {
    let mut json_mode = false;
    let mut files: Vec<String> = Vec::new();
    for a in args {
        match a.as_str() {
            "--json" => json_mode = true,
            f => files.push(f.into()),
        }
    }
    if files.is_empty() {
        eprintln!("tplcc blocks: no sources");
        return ExitCode::from(2);
    }
    let mut srcs: Vec<(String, String)> = Vec::new();
    for f in &files {
        if f.ends_with(".tplc") {
            srcs.extend(lib_sources(f));
        } else {
            let bytes = std::fs::read(f).unwrap_or_else(|e| {
                eprintln!("error: {f}: {e}");
                std::process::exit(1);
            });
            let s = String::from_utf8_lossy(&bytes).into_owned();
            srcs.push((f.clone(), s));
        }
    }
    let (mut out, eff) = compile_srcs(&srcs);
    let mut diags = std::mem::take(&mut out.diags);
    if !diags.iter().any(|d| d.severity == tplc_st::Severity::Error) {
        diags.extend(tplc_st::sema::check(&mut out.module));
    }
    let errs = diags
        .iter()
        .filter(|d| d.severity == tplc_st::Severity::Error)
        .count();

    use tplc_mir::ir::{Lit, Prov, VarKind};
    let lit_str = |l: &Lit| -> Option<String> {
        Some(match l {
            Lit::Bool(b) => {
                if *b {
                    "TRUE".into()
                } else {
                    "FALSE".into()
                }
            }
            Lit::Int(v) => v.to_string(),
            Lit::UInt(v) => v.to_string(),
            Lit::Real(v) => format!("{v:?}"),
            Lit::TimeNs(ns) => {
                const U: &[(&str, i64)] = &[
                    ("d", 86_400_000_000_000),
                    ("h", 3_600_000_000_000),
                    ("m", 60_000_000_000),
                    ("s", 1_000_000_000),
                    ("ms", 1_000_000),
                    ("us", 1_000),
                ];
                let mut rem = *ns;
                let mut parts = String::new();
                for (u, div) in U {
                    let v = rem / div;
                    if v > 0 {
                        parts += &format!("{v}{u}");
                        rem %= div;
                    }
                }
                if rem > 0 {
                    parts += &format!("{rem}ns");
                }
                if parts.is_empty() {
                    "T#0s".into()
                } else {
                    format!("T#{parts}")
                }
            }
            Lit::Str(s) => format!("'{}'", s.replace('\'', "''")),
            Lit::WStr(s) => format!("'{}'", s.replace('\'', "''")),
            Lit::EnumVal { val, .. } => val.clone(),
        })
    };
    let expr_lit = |e: &tplc_mir::ir::Expr| -> Option<String> {
        if let tplc_mir::ir::ExprKind::Lit(l) = &e.kind {
            lit_str(l)
        } else {
            None
        }
    };

    let mut blocks = Vec::new();
    let mut enums = serde_json::Map::new();
    for p in &out.module.pous {
        if p.kind == PouKind::Program {
            continue; // programs are tasks, not callable blocks
        }
        let kind = match p.kind {
            PouKind::FunctionBlock => "fb",
            PouKind::Function => "function",
            PouKind::Program => unreachable!(),
        };
        let file = match &p.prov {
            Prov::St { file, .. } => eff
                .get(*file as usize)
                .map(|(n, _)| n.clone())
                .unwrap_or_default(),
            _ => String::new(),
        };
        // collect enum value sets for param-editor dropdowns
        for v in &p.iface {
            if let Type::Enum { name, values } = &v.ty {
                enums
                    .entry(name.clone())
                    .or_insert_with(|| serde_json::json!(values));
            }
        }
        let pins: Vec<serde_json::Value> = p
            .iface
            .iter()
            .filter(|v| {
                matches!(
                    v.kind,
                    VarKind::Input | VarKind::Output | VarKind::InOut
                )
            })
            .map(|v| {
                let dir = match v.kind {
                    VarKind::Input => "in",
                    VarKind::Output => "out",
                    VarKind::InOut => "inout",
                    _ => unreachable!(),
                };
                let mut o = serde_json::json!({
                    "name": v.name,
                    "dir": dir,
                    "ty": tplc_mir::ty::ty_str(&v.ty),
                });
                if let Some(init) = v.init.as_ref().and_then(expr_lit) {
                    o["init"] = serde_json::json!(init);
                }
                o
            })
            .collect();
        blocks.push(serde_json::json!({
            "name": p.name,
            "kind": kind,
            "ret_ty": p.ret_ty.as_ref().map(tplc_mir::ty::ty_str),
            "file": file,
            "pins": pins,
        }));
    }
    let cat = serde_json::json!({
        "pous": out.module.pous.len(),
        "errors": errs,
        "blocks": blocks,
        "enums": enums,
    });
    if json_mode {
        println!("{cat}");
    } else {
        for b in cat["blocks"].as_array().unwrap() {
            let pins = b["pins"].as_array().map(|p| p.len()).unwrap_or(0);
            println!(
                "{} {} ({} pins){}",
                b["kind"].as_str().unwrap_or("?"),
                b["name"].as_str().unwrap_or("?"),
                pins,
                if b["file"].as_str().unwrap_or("").is_empty() {
                    String::new()
                } else {
                    format!("  [{}]", b["file"].as_str().unwrap())
                }
            );
        }
        eprintln!("{} pou(s), {errs} error(s)", cat["pous"]);
    }
    if errs > 0 {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

/// `tplcc src` — print the ST_SOURCES embedded in a `.tplc` package.
///
///   tplcc src <pkg.tplc> [name]     all sources, or one by name
///
/// Each source prints as `==== <name>` followed by its text (the same
/// framing `lib_sources` parses). Used by the ctl proxy's /libsrc
/// route so the web IDE can attach a library package's sources to a
/// project without unpacking the package itself.
fn cmd_src(args: &[String]) -> ExitCode {
    let mut want: Option<String> = None;
    let mut pkg: Option<String> = None;
    for a in args {
        if a.starts_with('-') {
            eprintln!("tplcc src: unknown flag {a}");
            return ExitCode::from(2);
        }
        if pkg.is_none() {
            pkg = Some(a.clone());
        } else {
            want = Some(a.clone());
        }
    }
    let Some(pkg) = pkg else {
        eprintln!("usage: tplcc src <pkg.tplc> [name]");
        return ExitCode::from(2);
    };
    let srcs = lib_sources(&pkg);
    let mut n = 0;
    for (name, text) in &srcs {
        if let Some(w) = &want {
            // embedded names carry the build-time path — match on the
            // basename so callers can ask for "atc_tempctrl.st"
            let base = name.rsplit('/').next().unwrap_or(name);
            if !name.eq_ignore_ascii_case(w) && !base.eq_ignore_ascii_case(w) {
                continue;
            }
        }
        if want.is_none() {
            println!("==== {name}");
        }
        print!("{text}");
        if !text.ends_with('\n') {
            println!();
        }
        n += 1;
    }
    if n == 0 {
        if want.is_some() {
            eprintln!("tplcc src: {pkg}: no source named {want:?}");
        } else {
            eprintln!("tplcc src: {pkg}: no ST_SOURCES");
        }
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn cmd_build(args: &[String]) -> ExitCode {
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
            f => files.push(f.into()),
        }
        i += 1;
    }
    if files.is_empty() && lib_pkgs.is_empty() {
        eprintln!("tplcc build: no sources");
        return ExitCode::from(2);
    }
    let mut srcs = read_srcs(&files);
    /* --lib <pkg.tplc>: pull the package's ST_SOURCES section into the
    compilation — the source-inclusion path for library packages
    (manifest kind "library"; program packages work too, their
    PROGRAM would just join the module) */
    for lp in &lib_pkgs {
        let mut ls = lib_sources(lp);
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
    let triple = target.unwrap_or_else(host_triple);
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
    ) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("tplcc: bad target triple {triple}: {e}");
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

/// Extract `(name, text)` sources from a package's ST_SOURCES section.
/// Wire format (emitted by `tplcc build`): repeated
/// `==== <file>\n<content>` blocks.
fn lib_sources(path: &str) -> Vec<(String, String)> {
    let bytes = std::fs::read(path).unwrap_or_else(|e| panic!("tplcc: --lib {path}: {e}"));
    let pkg = tplc_pkg::reader::Package::parse(&bytes)
        .unwrap_or_else(|e| panic!("tplcc: --lib {path}: {e}"));
    let sec = pkg
        .section(SectionType::StSources)
        .unwrap_or_else(|| panic!("tplcc: --lib {path}: no ST_SOURCES section"));
    let text = String::from_utf8_lossy(&sec.data);
    let mut out = Vec::new();
    let mut name: Option<String> = None;
    let mut cur = String::new();
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("==== ") {
            if let Some(n) = name.take() {
                out.push((n, std::mem::take(&mut cur)));
            }
            name = Some(rest.trim().to_string());
        } else {
            cur.push_str(line);
            cur.push('\n');
        }
    }
    if let Some(n) = name {
        out.push((n, cur));
    }
    if out.is_empty() {
        panic!("tplcc: --lib {path}: empty ST_SOURCES section");
    }
    out
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
    let manifest = format!(
        "{{\"schema\":\"tplc.manifest.v1\",\"name\":\"{name}\",\"version\":\"{prog_version}\",\"package_uuid\":\"{uuid_str}\",\"abi_version\":1,\"kind\":\"library\",\"target_triple\":\"{triple}\"{checked_json}{objfmt_json}}}"
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

use tplc_mir::interp::{program_ctx, Flow, Rt};

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
        tplc_mir::ir::Type::Real { bits } => {
            let n = (*bits as usize) / 8;
            let mut buf = [0u8; 8];
            for i in 0..n.min(8) {
                buf[i] = *img.get(b.at.byte as usize + i).unwrap_or(&0);
            }
            Value::F(if *bits == 32 {
                f32::from_le_bytes(buf[..4].try_into().unwrap()) as f64
            } else {
                f64::from_le_bytes(buf[..8].try_into().unwrap())
            })
        }
        tplc_mir::ir::Type::Time => {
            let mut v: u64 = 0;
            for i in 0..8 {
                v |= (*img.get(b.at.byte as usize + i).unwrap_or(&0) as u64) << (8 * i);
            }
            Value::Time(v as i64)
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
        (tplc_mir::ir::Type::Real { bits }, Value::F(x)) => {
            let n = (*bits as usize) / 8;
            let bytes = if *bits == 32 {
                (*x as f32).to_le_bytes().to_vec()
            } else {
                x.to_le_bytes().to_vec()
            };
            for i in 0..n.min(8) {
                img[b.at.byte as usize + i] = bytes[i];
            }
        }
        (tplc_mir::ir::Type::Time, Value::Time(x)) => {
            for i in 0..8 {
                img[b.at.byte as usize + i] = (*x as u64 >> (8 * i)) as u8;
            }
        }
        _ => {}
    }
}

type VecRun = (
    tplc_st::FrontendOutput,
    Pou,
    tplc_mir::layout::Layout,
    tplc_mir::interp::Ctx,
    Vec<u8>,
);

fn compile_run(f: &str, src_dir: &std::path::Path, id: &str) -> Result<VecRun, String> {
    let prog_file = src_dir.join(f);
    let src =
        std::fs::read_to_string(&prog_file).map_err(|e| format!("{id}: {prog_file:?}: {e}"))?;
    let (mut out, _eff) = compile_srcs(&[(prog_file.to_string_lossy().into(), src)]);
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
    Ok((out, program, layout, ctx, blob))
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
    // shadow candidates: "shadow" names one file, "shadows" an
    // array — up to 4 may be loaded, each sees identical %I, writes
    // to a private image, and seeds its state name-wise from the
    // active blob at the load boundary. "shadow_load"/"shadow_load@N"
    // loads candidate file N (true = 0); "shadow_promote[@N]" adopts
    // candidate N and drops every peer; "shadow_drop@N" removes one.
    // Candidate indices are load order, matching SHADOW_LIST.
    let shadow_file = v["shadow"].as_str().map(|s| s.to_string());
    let shadow_files: Vec<String> = match &v["shadows"] {
        serde_json::Value::Array(a) => a.iter().map(|p| p.as_str().unwrap().to_string()).collect(),
        _ => shadow_file.map(|f| vec![f]).unwrap_or_default(),
    };
    if !shadow_files.is_empty() && files.len() != 1 {
        return Err(format!("{id}: shadow pairs with a single program"));
    }
    let mut runs: Vec<VecRun> = Vec::new();
    for f in &files {
        runs.push(compile_run(f, src_dir, id)?);
    }
    // "checked_arith": compile every leg (active + shadow) in
    // checked-arithmetic mode: int div/mod by zero faults -19
    let checked = v["checked_arith"].as_bool().unwrap_or(false);
    for r in &mut runs {
        r.0.module.checked_arith = checked;
    }
    // (file index, run) pairs — the file index models the runtime's
    // duplicate-uuid rejection (same source = same program uuid)
    let mut shadows: Vec<(usize, VecRun)> = Vec::new();
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
        // "shadow_load": true → file 0, integer N → file N
        let sload = match &c["shadow_load"] {
            serde_json::Value::Bool(b) if *b => Some(0usize),
            serde_json::Value::Number(n) => n.as_u64().map(|x| x as usize),
            _ => None,
        };
        if let Some(fi) = sload {
            let f = shadow_files
                .get(fi)
                .ok_or(format!("{id} cycle {ci}: shadow_load@{fi} w/o shadow file"))?;
            if shadows.len() >= 4 {
                return Err(format!("{id} cycle {ci}: shadow slots full (4)"));
            }
            if shadows.iter().any(|(si, _)| *si == fi) {
                return Err(format!("{id} cycle {ci}: shadow already loaded"));
            }
            let mut sh = compile_run(f, src_dir, id)?;
            sh.0.module.checked_arith = checked;
            // declaration initializers run at load — the L1 model
            // of tplc_init on the candidate program
            {
                let (s_out, _, s_lay, s_ctx, s_blob) = &mut sh;
                let mut srt = Rt::new(s_blob, s_lay, &s_out.module, 0);
                srt.run_inits(s_ctx);
            }
            // seed: same-named fields carry the active blob's value —
            // the L1 model of the runtime's same-layout copy / MIGD
            {
                let (a_out, _, a_lay, _, a_blob) = &mut runs[0];
                let (s_out, _, s_lay, _, s_blob) = &mut sh;
                let art = Rt::new(a_blob, a_lay, &a_out.module, 0);
                let mut srt = Rt::new(s_blob, s_lay, &s_out.module, 0);
                for name in s_lay.fields.keys() {
                    if a_lay.fields.contains_key(name) {
                        let val = art.read_field(name);
                        srt.write_field(name, &val);
                    }
                }
            }
            shadows.push((fi, sh));
        }
        // "shadow_promote": true → candidate 0, integer N → N;
        // adoption drops every peer candidate
        let spr = match &c["shadow_promote"] {
            serde_json::Value::Bool(b) if *b => Some(0usize),
            serde_json::Value::Number(n) => n.as_u64().map(|x| x as usize),
            _ => None,
        };
        if let Some(pi) = spr {
            if pi >= shadows.len() {
                return Err(format!("{id} cycle {ci}: promote@{pi} w/o shadow"));
            }
            runs[0] = shadows.remove(pi).1;
            shadows.clear();
        }
        if let Some(di) = c["shadow_drop"].as_u64() {
            let di = di as usize;
            if di >= shadows.len() {
                return Err(format!("{id} cycle {ci}: drop@{di} w/o shadow"));
            }
            shadows.remove(di);
        }
        let mut q_imgs: Vec<Vec<u8>> = Vec::new();
        let mut aborts: Vec<Option<i32>> = Vec::new();
        for (pi, (out, program, layout, ctx, blob)) in runs.iter_mut().enumerate() {
            let tplc_mir::ir::Body::Stmts(stmts) = &program.body else {
                return Err("program body not stmts".into());
            };
            // state lives in blob; Rt is rebuilt per scan
            let mut rt = Rt::new(blob, layout, &out.module, 0);
            rt.now_ns = c["t_ns"].as_u64().unwrap_or(0);
            rt.cycle = ci as u64;
            /* declaration initializers run once at program start, as
            tplc_init does on the compiled path */
            if ci == 0 {
                rt.run_inits(ctx);
            }
            for b in &out.io_bindings {
                if b.at.region == AtRegion::I {
                    let val = img_get(&img_in, b);
                    let short = b.name.rsplit('.').next().unwrap_or(&b.name);
                    rt.write_field(short, &val);
                }
            }
            let flow = rt.exec_all(ctx, stmts);
            let aborted = matches!(flow, Flow::Abort(_));
            aborts.push(match flow {
                Flow::Abort(rc) => Some(rc),
                _ => None,
            });
            // check expect_fields (applied to every program — the corpus
            // uses identical field names across language variants);
            // a faulted scan leaves mid-statement state; only
            // expect_fault is meaningful there
            if !aborted {
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
                            (
                                tplc_mir::interp::Value::Bool(a),
                                tplc_mir::interp::Value::Bool(b),
                            ) => a == b,
                            (
                                tplc_mir::interp::Value::Time(a),
                                tplc_mir::interp::Value::Time(b),
                            ) => a == b,
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
        // cross-program equivalence + optional expect_q, skipped on
        // a faulted scan (mid-statement state is not a trace)
        let any_abort = aborts.iter().any(|a| a.is_some());
        if !any_abort {
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
        // task-fault expectation: "expect_fault": <rc> asserts every
        // program leg aborted with that TPLC_TASK_E_* code; absent,
        // any abort is an unexpected fault
        match c["expect_fault"].as_i64() {
            Some(code) => {
                for (pi, a) in aborts.iter().enumerate() {
                    match a {
                        Some(rc) if *rc == code as i32 => {}
                        Some(rc) => {
                            return Err(format!(
                                "{id} cycle {ci}: {} fault rc={rc} want {code}",
                                files[pi]
                            ))
                        }
                        None => {
                            return Err(format!(
                                "{id} cycle {ci}: {} completed, want fault {code}",
                                files[pi]
                            ))
                        }
                    }
                }
            }
            None => {
                for (pi, a) in aborts.iter().enumerate() {
                    if let Some(rc) = a {
                        return Err(format!(
                            "{id} cycle {ci}: {} unexpected task fault rc={rc}",
                            files[pi]
                        ));
                    }
                }
            }
        }
        // shadow %Q expectations: a string addresses candidate 0,
        // an object {"<idx>": "<hex>"} addresses candidates by
        // SHADOW_LIST (load-order) index
        let mut sh_expect: Vec<(usize, String)> = Vec::new();
        match &c["expect_shadow_q"] {
            serde_json::Value::String(s) => sh_expect.push((0, s.clone())),
            serde_json::Value::Object(o) => {
                for (k, val) in o {
                    let i = k
                        .parse::<usize>()
                        .map_err(|_| format!("{id} cycle {ci}: bad shadow idx {k}"))?;
                    sh_expect.push((
                        i,
                        val.as_str()
                            .ok_or(format!("{id} cycle {ci}: expect_shadow_q[{k}]"))?
                            .to_string(),
                    ));
                }
            }
            _ => {}
        }
        for (i, _) in &sh_expect {
            if *i >= shadows.len() {
                return Err(format!("{id} cycle {ci}: expect_shadow_q@{i} w/o shadow"));
            }
        }
        // shadow scans on the same inputs; each candidate's %Q
        // stays private. A faulting candidate drops out, matching
        // the runtime's per-shadow task-fault handling.
        let mut si = 0;
        while si < shadows.len() {
            let (aborted, sh_img) = {
                let (out, program, layout, ctx, blob) = &mut shadows[si].1;
                let tplc_mir::ir::Body::Stmts(stmts) = &program.body else {
                    return Err("shadow body not stmts".into());
                };
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
                let flow = rt.exec_all(ctx, stmts);
                let mut sh_img = vec![0u8; 64];
                for b in &out.io_bindings {
                    if b.at.region == AtRegion::Q {
                        let short = b.name.rsplit('.').next().unwrap_or(&b.name);
                        let val = rt.read_field(short);
                        img_put(&mut sh_img, b, &val);
                    }
                }
                (matches!(flow, Flow::Abort(_)), sh_img)
            };
            if aborted {
                shadows.remove(si);
                continue;
            }
            for (ei, qh) in &sh_expect {
                if *ei != si {
                    continue;
                }
                let want = hex_bytes(qh);
                if sh_img[..want.len()] != want[..] {
                    return Err(format!(
                        "{id} cycle {ci}: shadow@{si} %Q want {} got {}",
                        qh,
                        hex_str(&sh_img[..want.len()])
                    ));
                }
            }
            si += 1;
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

#[cfg(test)]
mod mixed_tests {
    use super::*;
    use tplc_mir::interp::{Rt, Value};

    /// st library side of the mixed fixture: an FB, a FUNCTION, and a
    /// PROGRAM that calls the FBD-defined FB `FbdFb`.
    const ST: &str = r#"
FUNCTION_BLOCK StFb
VAR_INPUT X : LREAL; END_VAR
VAR_OUTPUT Y : LREAL; END_VAR
Y := X * 2.0;
END_FUNCTION_BLOCK

FUNCTION StFn : LREAL
VAR_INPUT A : LREAL; END_VAR
StFn := A + 10.0;
END_FUNCTION

PROGRAM StProg
VAR f : FbdFb; r : BOOL; END_VAR
f(IN := FALSE);
r := f.OUT;
END_PROGRAM
"#;

    /// FBD side: Main instantiates ST's StFb + StFn + an inline st
    /// block; FbdFb is defined here and called from the ST program.
    fn fbd(extra_pou: &str) -> String {
        format!(
            r#"{{
  "schema": "fbd.v1",
  "project": "mixed",
  "pous": [
    {{
      "id": "p1", "name": "Main", "kind": "program",
      "interface": [
        {{"name": "pv", "kind": "var", "type": "LREAL", "init": "21.0"}},
        {{"name": "r1", "kind": "var", "type": "LREAL"}},
        {{"name": "r2", "kind": "var", "type": "LREAL"}},
        {{"name": "r3", "kind": "var", "type": "LREAL"}}
      ],
      "networks": [{{
        "id": "n1",
        "blocks": [
          {{"id": 1, "kind": "variable", "role": "in", "name": "pv"}},
          {{"id": 2, "kind": "fb", "type_ref": "StFb", "instance": "st1"}},
          {{"id": 3, "kind": "st",
             "pins": [
               {{"name": "A", "kind": "var_input", "type": "LREAL"}},
               {{"name": "Q", "kind": "var_output", "type": "LREAL"}}
             ],
             "params": {{"CODE": "Q := A * 3.0;"}}}},
          {{"id": 4, "kind": "function", "type_ref": "StFn"}},
          {{"id": 5, "kind": "variable", "role": "out", "name": "r1"}},
          {{"id": 6, "kind": "variable", "role": "out", "name": "r2"}},
          {{"id": 7, "kind": "variable", "role": "out", "name": "r3"}}
        ],
        "wires": [
          {{"id": 10, "src": {{"block": 1, "port": 0}}, "dsts": [{{"block": 2, "port": "X"}}, {{"block": 3, "port": "A"}}]}},
          {{"id": 11, "src": {{"block": 2, "port": "Y"}}, "dsts": [{{"block": 4, "port": 0}}, {{"block": 5, "port": 0}}]}},
          {{"id": 12, "src": {{"block": 4, "port": 0}}, "dsts": [{{"block": 6, "port": 0}}]}},
          {{"id": 14, "src": {{"block": 3, "port": "Q"}}, "dsts": [{{"block": 7, "port": 0}}]}}
        ]
      }}]
    }},
    {{
      "id": "p2", "name": "FbdFb", "kind": "function_block",
      "interface": [
        {{"name": "IN", "kind": "var_input", "type": "BOOL"}},
        {{"name": "OUT", "kind": "var_output", "type": "BOOL"}}
      ],
      "networks": [{{
        "id": "n1",
        "blocks": [
          {{"id": 60, "kind": "variable", "role": "in", "name": "IN"}},
          {{"id": 61, "kind": "operator", "type_ref": "NOT"}},
          {{"id": 62, "kind": "variable", "role": "out", "name": "OUT"}}
        ],
        "wires": [
          {{"id": 63, "src": {{"block": 60, "port": 0}}, "dsts": [{{"block": 61, "port": 0}}]}},
          {{"id": 64, "src": {{"block": 61, "port": 0}}, "dsts": [{{"block": 62, "port": 0}}]}}
        ]
      }}]
    }}{extra_pou}
  ],
  "tasks": [{{"name": "main", "period_ns": 10000000, "programs": ["Main", "StProg"]}}]
}}"#
        )
    }

    fn compile_all(srcs: &[(String, String)]) -> tplc_st::FrontendOutput {
        let (mut out, _eff) = compile_srcs(srcs);
        out.diags.extend(tplc_st::sema::check(&mut out.module));
        out
    }

    fn errs(out: &tplc_st::FrontendOutput) -> Vec<String> {
        out.diags
            .iter()
            .filter(|d| d.severity == tplc_st::Severity::Error)
            .map(|d| format!("{} {}", d.code, d.message))
            .collect()
    }

    /// Run every program in the module once against a *shared* state
    /// blob — the multi-program image layout the runtime uses.
    fn exec_all(out: &tplc_st::FrontendOutput) -> (tplc_mir::layout::Layout, Vec<u8>) {
        let layout = tplc_mir::layout::layout_module(&out.module);
        let mut blob = vec![0u8; layout.total_size as usize];
        for pou in out
            .module
            .pous
            .iter()
            .filter(|p| p.kind == tplc_mir::ir::PouKind::Program)
        {
            let ctx = tplc_mir::interp::pou_scope(&layout, &pou.name);
            let stmts = match &pou.body {
                tplc_mir::ir::Body::Stmts(s) => s.clone(),
                _ => panic!("body not stmts"),
            };
            let mut rt = Rt::new(&mut blob, &layout, &out.module, 0);
            rt.run_inits(&ctx);
            rt.exec_all(&ctx, &stmts);
        }
        (layout, blob)
    }

    /// Read a program var by qualified "PROG.name" or bare name.
    fn read<'a>(
        rt: &Rt<'a>,
        layout: &tplc_mir::layout::Layout,
        prog: &str,
        name: &str,
    ) -> Value {
        let key = if layout.fields.contains_key(&format!("{prog}.{name}")) {
            format!("{prog}.{name}")
        } else {
            name.to_string()
        };
        rt.read_field(&key)
    }

    #[test]
    fn mixed_bidirectional() {
        let srcs = vec![
            ("project.fbd.json".to_string(), fbd("")),
            ("lib.st".to_string(), ST.to_string()),
        ];
        let out = compile_all(&srcs);
        assert_eq!(errs(&out), Vec::<String>::new());

        // shared blob — one image, both programs' vars coexisting
        let (layout, mut blob) = exec_all(&out);
        let rt = Rt::new(&mut blob, &layout, &out.module, 0);
        // FBD program: r1 = 21*2 = 42 (StFb), r2 = 42+10 = 52 (StFn),
        // r3 = 21*3 = 63 (inline st block)
        assert_eq!(read(&rt, &layout, "Main", "r1"), Value::F(42.0));
        assert_eq!(read(&rt, &layout, "Main", "r2"), Value::F(52.0));
        assert_eq!(read(&rt, &layout, "Main", "r3"), Value::F(63.0));
        // ST program: r = NOT FALSE = TRUE via the FBD-defined FbdFb
        assert_eq!(read(&rt, &layout, "StProg", "r"), Value::Bool(true));
    }

    #[test]
    fn mixed_inout_st_block() {
        // an `st` block's var_in_out pin aliases the bound FBD variable;
        // writes inside the generated FB land in the caller's storage.
        // The ST program exercises the same semantics pure-ST (the FB
        // call's inout arg binds n2 by reference).
        let fbd = r#"{
  "schema": "fbd.v1",
  "project": "io",
  "pous": [
    {
      "id": "p1", "name": "Main", "kind": "program",
      "interface": [
        {"name": "n", "kind": "var", "type": "DINT", "init": "5"},
        {"name": "out", "kind": "var", "type": "DINT"}
      ],
      "networks": [{
        "id": "n1",
        "blocks": [
          {"id": 1, "kind": "variable", "role": "in", "name": "n"},
          {"id": 2, "kind": "st",
             "pins": [
               {"name": "BUF", "kind": "var_in_out", "type": "DINT"},
               {"name": "Q", "kind": "var_output", "type": "DINT"}
             ],
             "params": {"CODE": "BUF := BUF + 7; Q := BUF;"}},
          {"id": 3, "kind": "variable", "role": "out", "name": "out"}
        ],
        "wires": [
          {"id": 10, "src": {"block": 1, "port": 0}, "dsts": [{"block": 2, "port": "BUF"}]},
          {"id": 11, "src": {"block": 2, "port": "Q"}, "dsts": [{"block": 3, "port": 0}]}
        ]
      }]
    }
  ],
  "tasks": [{"name": "main", "period_ns": 10000000, "programs": ["Main", "StIo"]}]
}"#;
        let st = r#"
FUNCTION_BLOCK IoFb
VAR_IN_OUT BUF : DINT; END_VAR
BUF := BUF + 1;
END_FUNCTION_BLOCK
PROGRAM StIo
VAR a : IoFb; n2 : DINT; END_VAR
a(BUF := n2);
END_PROGRAM
"#;
        let srcs = vec![
            ("project.fbd.json".to_string(), fbd.to_string()),
            ("lib.st".to_string(), st.to_string()),
        ];
        let out = compile_all(&srcs);
        assert_eq!(errs(&out), Vec::<String>::new());
        let (layout, mut blob) = exec_all(&out);
        let rt = Rt::new(&mut blob, &layout, &out.module, 0);
        // inout write-through: n = 5+7, out mirrors it; n2 = 0+1
        assert_eq!(read(&rt, &layout, "Main", "n"), Value::I(12));
        assert_eq!(read(&rt, &layout, "Main", "out"), Value::I(12));
        assert_eq!(read(&rt, &layout, "StIo", "n2"), Value::I(1));
    }

    #[test]
    fn mixed_dup_impl_errors() {
        // FbdFb also implemented in ST → duplicate impl error
        let st = format!(
            "{}\nFUNCTION_BLOCK FbdFb\nVAR_INPUT IN : BOOL; END_VAR\nVAR_OUTPUT OUT : BOOL; END_VAR\nOUT := IN;\nEND_FUNCTION_BLOCK\n",
            ST.replace("PROGRAM StProg", "PROGRAM_UNUSED StProg")
        );
        let srcs = vec![
            ("project.fbd.json".to_string(), fbd("")),
            ("lib.st".to_string(), st),
        ];
        let out = compile_all(&srcs);
        assert!(errs(&out).iter().any(|e| e.contains("TPLC_E_MIX_DUP_POU")),
            "expected dup impl error, got {:?}", errs(&out));
    }

    #[test]
    fn mixed_decl_stub_merges() {
        // an interface decl for StFb inside the fbd project must merge
        // silently when it matches the ST impl
        let decl = r#",{
      "id": "d1", "name": "StFb", "kind": "function_block",
      "interface": [
        {"name": "X", "kind": "var_input", "type": "LREAL"},
        {"name": "Y", "kind": "var_output", "type": "LREAL"}
      ],
      "networks": []
    }"#;
        let srcs = vec![
            ("project.fbd.json".to_string(), fbd(decl)),
            ("lib.st".to_string(), ST.to_string()),
        ];
        let out = compile_all(&srcs);
        assert_eq!(errs(&out), Vec::<String>::new());
        let stfb = out.module.pous.iter().filter(|p| p.name == "StFb").count();
        assert_eq!(stfb, 1);
    }

    #[test]
    fn mixed_iface_mismatch_errors() {
        // decl with wrong pin type → iface mismatch error
        let decl = r#",{
      "id": "d1", "name": "StFb", "kind": "function_block",
      "interface": [
        {"name": "X", "kind": "var_input", "type": "INT"},
        {"name": "Y", "kind": "var_output", "type": "LREAL"}
      ],
      "networks": []
    }"#;
        let srcs = vec![
            ("project.fbd.json".to_string(), fbd(decl)),
            ("lib.st".to_string(), ST.to_string()),
        ];
        let out = compile_all(&srcs);
        assert!(errs(&out).iter().any(|e| e.contains("TPLC_E_MIX_IFACE")),
            "expected iface mismatch, got {:?}", errs(&out));
    }

    /// An FBD-authored `FUNCTION` POU (ret_ty + implicit return var)
    /// called from ST: the return extracts like an ST function's.
    #[test]
    fn mixed_fbd_function_from_st() {
        let fbd = r#"{
  "schema": "fbd.v1",
  "project": "fn",
  "pous": [
    {
      "id": "f1", "name": "FnX", "kind": "function", "ret_ty": "LREAL",
      "interface": [{"name": "IN", "kind": "var_input", "type": "LREAL"}],
      "networks": [{
        "id": "n1",
        "blocks": [
          {"id": 1, "kind": "variable", "role": "in", "name": "IN"},
          {"id": 2, "kind": "variable", "role": "out", "name": "FnX"}
        ],
        "wires": [
          {"id": 3, "src": {"block": 1, "port": 0}, "dsts": [{"block": 2, "port": 0}]}
        ]
      }]
    }
  ],
  "tasks": [{"name": "main", "period_ns": 10000000, "programs": ["FnProg"]}]
}"#;
        let st = r#"
PROGRAM FnProg
VAR r : LREAL; END_VAR
r := FnX(21.0) * 2.0;
END_PROGRAM
"#;
        let srcs = vec![
            ("project.fbd.json".to_string(), fbd.to_string()),
            ("lib.st".to_string(), st.to_string()),
        ];
        let out = compile_all(&srcs);
        assert_eq!(errs(&out), Vec::<String>::new());
        let (layout, mut blob) = exec_all(&out);
        let rt = Rt::new(&mut blob, &layout, &out.module, 0);
        assert_eq!(read(&rt, &layout, "FnProg", "r"), Value::F(42.0));
    }

    /// The TetherWebUI curve editor emits `tests/mixed/ui_profile.fbd.json`:
    /// an FB_AtcProfile instance, an inline `st` block that loads the SEG
    /// table member-wise through a var_in_out pin, and an FB_AtcPid wired
    /// SP/PV/OUT. This runs that exact document against the shipped
    /// library and checks the profile actually ramps and soaks.
    #[test]
    fn ui_curve_profile_runs() {
        let srcs = vec![
            (
                "project.fbd.json".to_string(),
                include_str!("../../../tests/mixed/ui_profile.fbd.json").to_string(),
            ),
            (
                "Atc_TempCtrl.st".to_string(),
                include_str!("../../../libraries/tempctrl/atc_tempctrl.st").to_string(),
            ),
        ];
        let out = compile_all(&srcs);
        assert_eq!(errs(&out), Vec::<String>::new());

        let layout = tplc_mir::layout::layout_module(&out.module);
        let mut blob = vec![0u8; layout.total_size as usize];
        let prog = out
            .module
            .pous
            .iter()
            .find(|p| p.name == "Main")
            .expect("Main program");
        let ctx = tplc_mir::interp::pou_scope(&layout, "Main");
        let stmts = match &prog.body {
            tplc_mir::ir::Body::Stmts(s) => s.clone(),
            _ => panic!("body not stmts"),
        };
        let mut rt = Rt::new(&mut blob, &layout, &out.module, 0);
        rt.run_inits(&ctx);
        // run enable + a PV that trails the setpoint (one program → the
        // layout keeps bare field names; qualify only when prefixed)
        let q = |n: &str| {
            if layout.fields.contains_key(&format!("Main.{n}")) {
                format!("Main.{n}")
            } else {
                n.to_string()
            }
        };
        rt.write_field(&q("Run"), &Value::Bool(true));
        rt.write_field(&q("Pv"), &Value::F(25.0));

        // cycle time 0.1 s: seg1 ramps 25→150 over 60 s (600 cycles),
        // seg2 soaks at 150 for 120 s. Sample at 30 s and 90 s.
        // The lowerer orders calls by data dependency, so the config
        // block (it writes `prof`) runs after the profile call — the
        // run request is held and starts on scan 2.
        let member = |rt: &mut Rt, inst: &str, name: &str| {
            rt.read_ref(
                &ctx,
                &tplc_mir::ir::Ref::Field(
                    Box::new(tplc_mir::ir::Ref::Var(inst.to_string())),
                    name.to_string(),
                ),
            )
        };
        let f = |v: Value| match v {
            Value::F(x) => x,
            v => panic!("not float: {v:?}"),
        };
        let mut sp30 = 0.0;
        let mut sp90 = 0.0;
        for i in 0..=900 {
            rt.exec_all(&ctx, &stmts);
            let t = (i + 1) as f64 * 0.1;
            if (t - 30.0).abs() < 1e-6 {
                sp30 = f(member(&mut rt, "prof", "SP"));
            }
            if (t - 90.0).abs() < 1e-6 {
                sp90 = f(member(&mut rt, "prof", "SP"));
            }
        }
        // halfway through the time-ramp: SP must sit near 25+(150-25)/2
        assert!((sp30 - 87.5).abs() < 5.0, "SP at 30 s = {sp30}, want ~87.5");
        // soak reached: SP pinned at the segment target
        assert!((sp90 - 150.0).abs() < 0.5, "SP at 90 s = {sp90}, want 150");
        // config block loaded the whole table: 4 segments, no END yet
        assert_eq!(member(&mut rt, "prof", "NUM_SEG"), Value::I(4));
        assert_eq!(member(&mut rt, "prof", "SegIdx"), Value::I(2));
        // controller runs off the profile SP (PID seeded Kp=2, Tn=60)
        let out_v = f(member(&mut rt, "ctl", "Out"));
        assert!(out_v > 0.0, "controller output {out_v} should be positive");
    }

    /// EK-terminal device blocks end-to-end: a DI channel enables the
    /// PID + PWM, an analog channel feeds the PV, and the PWM bit lands
    /// on a DO channel — the exact graphical wiring the web FBD editor
    /// produces (tests/device/ek_pid.fbd.json is editor-shaped JSON).
    #[test]
    fn ek_device_pid_pwm_chain() {
        let srcs = vec![
            (
                "ek_pid.fbd.json".to_string(),
                include_str!("../../../tests/device/ek_pid.fbd.json").to_string(),
            ),
            (
                "Atc_TempCtrl.st".to_string(),
                include_str!("../../../libraries/tempctrl/atc_tempctrl.st").to_string(),
            ),
        ];
        let out = compile_all(&srcs);
        assert_eq!(errs(&out), Vec::<String>::new());

        // every channel is a real process-image binding
        let binds: Vec<(String, String)> = out
            .io_bindings
            .iter()
            .map(|b| (b.name.clone(), format!("{:?}", b.at)))
            .collect();
        for want in [
            "EK_PID.__dev10_DI0",
            "EK_PID.__dev10_DI1",
            "EK_PID.__dev11_AI0",
            "EK_PID.__dev12_DO0",
        ] {
            assert!(binds.iter().any(|(n, _)| n == want), "missing io binding {want}");
        }

        let layout = tplc_mir::layout::layout_module(&out.module);
        let mut blob = vec![0u8; layout.total_size as usize];
        let prog = out
            .module
            .pous
            .iter()
            .find(|p| p.name == "EK_PID")
            .expect("EK_PID program");
        let ctx = tplc_mir::interp::pou_scope(&layout, "EK_PID");
        let stmts = match &prog.body {
            tplc_mir::ir::Body::Stmts(s) => s.clone(),
            _ => panic!("body not stmts"),
        };
        let mut rt = Rt::new(&mut blob, &layout, &out.module, 0);
        rt.run_inits(&ctx);
        let q = |n: &str| {
            if layout.fields.contains_key(&format!("EK_PID.{n}")) {
                format!("EK_PID.{n}")
            } else {
                n.to_string()
            }
        };
        // bus side: DI0 = run, AI0 = 20.0 °C process value
        rt.write_field(&q("__dev10_DI0"), &Value::Bool(true));
        rt.write_field(&q("__dev11_AI0"), &Value::F(20.0));

        let member = |rt: &mut Rt, inst: &str, name: &str| {
            rt.read_ref(
                &ctx,
                &tplc_mir::ir::Ref::Field(
                    Box::new(tplc_mir::ir::Ref::Var(inst.to_string())),
                    name.to_string(),
                ),
            )
        };
        let f = |v: Value| match v {
            Value::F(x) => x,
            v => panic!("not float: {v:?}"),
        };
        // P-only loop: Out = 0.5*(75-20) = 27.5 → Duty 27.5 % of a 10 s
        // period → Ton = 2.75 s → Q high for t < 2.75 s (≈ scan 275).
        let mut do_early = false;
        let mut do_late = true;
        let mut pid_out = 0.0;
        for i in 0..=400 {
            rt.exec_all(&ctx, &stmts);
            let t = (i + 1) as f64 * 0.01;
            if (t - 2.0).abs() < 1e-6 {
                do_early = matches!(
                    rt.read_field(&q("__dev12_DO0")),
                    Value::Bool(true)
                );
            }
            if (t - 4.0).abs() < 1e-6 {
                do_late = matches!(
                    rt.read_field(&q("__dev12_DO0")),
                    Value::Bool(true)
                );
                pid_out = f(member(&mut rt, "pid", "Out"));
            }
        }
        assert!(pid_out > 20.0, "PID out {pid_out} should be ~27.5");
        assert!(do_early, "DO0 should be high during the PWM on-window");
        assert!(!do_late, "DO0 should be low after Ton elapsed");
    }

    /// The TetherWebUI autotune editor emits
    /// tests/mixed/ui_tune_lqg.fbd.json: FB_AtcKalmanTune +
    /// FB_AtcLqrTune feeding an inline `st` handoff whose VAR_IN_OUT pins
    /// alias both tuners and the FB_AtcLqg controller. The handoff
    /// mirrors tuner outputs to live vars and copies the computed gains
    /// into the controller's input members on the APPLY strobe. Runs
    /// the exact document against the shipped library.
    #[test]
    fn ui_autotune_lqg_runs() {
        let srcs = vec![
            (
                "project.fbd.json".to_string(),
                include_str!("../../../tests/mixed/ui_tune_lqg.fbd.json").to_string(),
            ),
            (
                "Atc_TempCtrl.st".to_string(),
                include_str!("../../../libraries/tempctrl/atc_tempctrl.st").to_string(),
            ),
        ];
        let out = compile_all(&srcs);
        assert_eq!(errs(&out), Vec::<String>::new());

        let layout = tplc_mir::layout::layout_module(&out.module);
        let mut blob = vec![0u8; layout.total_size as usize];
        let prog = out
            .module
            .pous
            .iter()
            .find(|p| p.name == "Main")
            .expect("Main program");
        let ctx = tplc_mir::interp::pou_scope(&layout, "Main");
        let stmts = match &prog.body {
            tplc_mir::ir::Body::Stmts(s) => s.clone(),
            _ => panic!("body not stmts"),
        };
        let mut rt = Rt::new(&mut blob, &layout, &out.module, 0);
        rt.run_inits(&ctx);
        let q = |n: &str| {
            if layout.fields.contains_key(&format!("Main.{n}")) {
                format!("Main.{n}")
            } else {
                n.to_string()
            }
        };
        let member = |rt: &mut Rt, inst: &str, name: &str| {
            rt.read_ref(
                &ctx,
                &tplc_mir::ir::Ref::Field(
                    Box::new(tplc_mir::ir::Ref::Var(inst.to_string())),
                    name.to_string(),
                ),
            )
        };
        let f = |v: Value| match v {
            Value::F(x) => x,
            v => panic!("not float: {v:?}"),
        };
        let b = |v: Value| matches!(v, Value::Bool(true));

        // tuners run (Execute ← TuneRun), apply strobe held low
        rt.write_field(&q("TuneRun"), &Value::Bool(true));
        rt.write_field(&q("TuneApply"), &Value::Bool(false));
        rt.write_field(&q("TuneReset"), &Value::Bool(false));
        rt.write_field(&q("Pv"), &Value::F(50.0));
        rt.write_field(&q("Sp"), &Value::F(80.0));

        // DARE solves within a few scans (t06 converges by cycle 2)
        for _ in 0..8 {
            rt.exec_all(&ctx, &stmts);
        }
        // live mirrors track the tuner instances
        assert!(b(read(&rt, &layout, "Main", "lqr_done")), "lqr_done mirror");
        assert!(b(read(&rt, &layout, "Main", "kal_done")), "kal_done mirror");
        let k1 = f(read(&rt, &layout, "Main", "lqr_k1"));
        assert!(k1 > 0.0, "lqr_k1 mirror {k1} should be a computed gain");
        // gains not yet applied — controller inputs still zero
        assert_eq!(f(member(&mut rt, "pid", "K1")), 0.0);

        // APPLY strobe: handoff copies KAL.L*/LQR.K* into pid's inputs
        rt.write_field(&q("TuneApply"), &Value::Bool(true));
        for _ in 0..3 {
            rt.exec_all(&ctx, &stmts);
        }
        let eps = 1e-9;
        assert!(
            (f(member(&mut rt, "pid", "K1")) - k1).abs() < eps,
            "pid.K1 {} != tuned K1 {k1}",
            f(member(&mut rt, "pid", "K1"))
        );
        assert!(
            (f(member(&mut rt, "pid", "L1")) - f(read(&rt, &layout, "Main", "kal_l1"))).abs() < eps,
            "pid.L1 != kal_l1 mirror"
        );
        assert!(
            (f(member(&mut rt, "pid", "L2")) - f(read(&rt, &layout, "Main", "kal_l2"))).abs() < eps,
            "pid.L2 != kal_l2 mirror"
        );
        assert!(
            f(member(&mut rt, "pid", "K2")) != 0.0 || f(member(&mut rt, "pid", "Ki")) != 0.0,
            "pid K2/Ki should carry tuned gains"
        );
        // the wired controller runs: SP/PV error winds the loop and Out
        // leaves zero — only possible because the editor propagated the
        // plant A/B matrices + OutMin/OutMax onto the pid block
        for _ in 0..40 {
            rt.exec_all(&ctx, &stmts);
        }
        let out = f(read(&rt, &layout, "Main", "Out"));
        assert!(out != 0.0, "pid.Out should be driven by the tuned loop");
        // apply latch semantics (applied flag, RESET unlatch) are
        // covered pure-ST by tests/tempctrl/cases/t13_handoff.st
    }
}
