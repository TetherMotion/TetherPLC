use std::process::ExitCode;
use tplc_mir::ir::{PouKind, Type};
use crate::helpers::*;

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
pub(crate) fn cmd_blocks(args: &[String]) -> ExitCode {
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
