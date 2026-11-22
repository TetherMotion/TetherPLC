use tplc_mir::ir::Prov;
use tplc_pkg::format::SectionType;
use crate::libs;

pub(crate) fn read_srcs(files: &[String]) -> Vec<(String, String)> {
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
pub(crate) fn compile_srcs(srcs: &[(String, String)]) -> (tplc_st::FrontendOutput, Vec<(String, String)>) {
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

pub(crate) fn print_diags(diags: &[tplc_st::Diag], srcs: &[(String, String)]) -> usize {
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

pub(crate) fn diag_report(
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
pub(crate) fn stage_failure_json(stage: &str, message: &str) -> serde_json::Value {
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

/// Extract `(name, text)` sources from a package's ST_SOURCES section.
/// Wire format (emitted by `tplcc build`): repeated
/// `==== <file>\n<content>` blocks.
pub(crate) fn lib_sources(path: &str) -> Vec<(String, String)> {
    let bytes = std::fs::read(path).unwrap_or_else(|e| panic!("tplcc: --lib {path}: {e}"));
    let pkg = tplc_pkg::reader::Package::parse(&bytes)
        .unwrap_or_else(|e| panic!("tplcc: --lib {path}: {e}"));
    let sec = pkg
        .section(SectionType::StSources)
        .unwrap_or_else(|| panic!("tplcc: --lib {path}: no ST_SOURCES section"));
    let text = String::from_utf8_lossy(&sec.data);
    let out = libs::split_st_sources(&text);
    if out.is_empty() {
        panic!("tplcc: --lib {path}: empty ST_SOURCES section");
    }
    out
}

