//! Library autoresolution + precompiled-package ingestion for `tplcc`.
//!
//! `--libdir DIR` indexes every `.st` source and `.tplc` package in the
//! directory by the POU names it declares. `resolve` then pulls what
//! satisfies undeclared `FB_*`/`F_*` references — `.st` files merge
//! into the source list, `.tplc` packages join the `--lib` list —
//! iterating to a fixpoint because a pulled library may itself
//! reference other libraries. This is the CLI/serve equivalent of the
//! WebUI's automatic library attach: the user uses a block, the system
//! finds it.
//!
//! `pkg_meta`/`pkg_bitcode` feed the precompiled-link path in
//! `cmd_build`: a `kind:"library"` package carries LLVM bitcode whose
//! `__tplc_pou_*` bodies satisfy the extern declarations the emitter
//! leaves for library POUs, so program builds consume the artifact
//! instead of re-codegenning library sources.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use tplc_pkg::format::SectionType;
use tplc_pkg::reader::Package;

/* ---------------- source scanning ---------------------------------- */

/// POU-declaring keywords — mirror of the WebUI POU_DECL list; the
/// identifier right after one is a declared name.
const DECL_KW: &[&str] = &[
    "FUNCTION_BLOCK",
    "FUNCTION",
    "PROGRAM",
    "TYPE",
    "STRUCT",
    "VAR_GLOBAL",
    "VAR_EXTERNAL",
];

fn tokens(text: &str) -> impl Iterator<Item = &str> {
    text.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
        .filter(|t| {
            !t.is_empty() && (t.as_bytes()[0].is_ascii_alphabetic() || t.as_bytes()[0] == b'_')
        })
}

/// Names declared by POU keywords in `text` (uppercased). `.fbd.json`
/// sources are JSON — `fbd_decl_names` covers their `"name"` fields.
pub fn pou_decls(text: &str) -> HashSet<String> {
    let mut out = HashSet::new();
    let mut prev_decl = false;
    for t in tokens(text) {
        if prev_decl {
            out.insert(t.to_uppercase());
            prev_decl = false;
            continue;
        }
        prev_decl = DECL_KW.iter().any(|k| t.eq_ignore_ascii_case(k));
    }
    out
}

/// `FB_*`/`F_*` type references in `text` (uppercased) — the same
/// heuristic the WebUI's projectLibRefs uses: any identifier with the
/// library-block prefix counts, wherever it appears (decls, calls,
/// comments — over-inclusion is harmless, the declared set wins).
pub fn lib_refs(text: &str) -> HashSet<String> {
    tokens(text)
        .filter(|t| {
            t.len() > 3 && (t[..3].eq_ignore_ascii_case("FB_") || t[..2].eq_ignore_ascii_case("F_"))
        })
        .map(|t| t.to_uppercase())
        .collect()
}

/// POU names declared in an `.fbd.json` document: `"name":"<ident>"`
/// pairs. Block-instance names land here too — over-declaring only
/// suppresses a same-named lib pull, which is the safe direction
/// (project decls win, matching the WebUI rule).
pub fn fbd_decl_names(text: &str) -> HashSet<String> {
    let mut out = HashSet::new();
    let mut rest = text;
    while let Some(i) = rest.find("\"name\"") {
        rest = &rest[i + 6..];
        let rest_t = rest.trim_start();
        let Some(after_colon) = rest_t.strip_prefix(':') else {
            continue;
        };
        let after = after_colon.trim_start();
        let Some(inner) = after.strip_prefix('"') else {
            continue;
        };
        let Some(end) = inner.find('"') else {
            continue;
        };
        let name = &inner[..end];
        if !name.is_empty()
            && name
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '_')
            && name.as_bytes()[0].is_ascii_alphabetic()
        {
            out.insert(name.to_uppercase());
        }
        rest = &inner[end..];
    }
    out
}

/* ---------------- package introspection ---------------------------- */

/// Split the ST_SOURCES section text into `(file, body)` pairs — the
/// `==== <file>` wire format emitted by `tplcc build`/`build --library`.
pub fn split_st_sources(text: &str) -> Vec<(String, String)> {
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
    out
}

/// Package metadata the precompiled-link path needs: manifest `kind`,
/// `target_triple`, and the set of POU names the package declares.
pub struct PkgMeta {
    pub kind: String,
    pub triple: String,
    pub pous: HashSet<String>,
}

/// Read a `.tplc`'s Manifest + derive the declared-POU set. POU names
/// come from the manifest `pous` list when present (emitted by
/// `tplcc build --library`), else from scanning ST_SOURCES — the
/// fallback keeps packages built before the field existed usable.
/// Returns None for unreadable/invalid packages (callers fall back
/// to plain source inclusion).
pub fn pkg_meta(path: &Path) -> Option<PkgMeta> {
    let bytes = std::fs::read(path).ok()?;
    let pkg = Package::parse(&bytes).ok()?;
    let manifest = pkg.section(SectionType::Manifest)?;
    let mj: serde_json::Value =
        serde_json::from_slice(manifest.data).ok()?;
    let kind = mj["kind"].as_str().unwrap_or("program").to_string();
    let triple = mj["target_triple"].as_str().unwrap_or("").to_string();
    let mut pous: HashSet<String> = mj["pous"]
        .as_array()
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_uppercase())
                .collect()
        })
        .unwrap_or_default();
    if pous.is_empty() {
        if let Some(ss) = pkg.section(SectionType::StSources) {
            let text = String::from_utf8_lossy(&ss.data);
            for (_, body) in split_st_sources(&text) {
                pous.extend(pou_decls(&body));
            }
        }
    }
    Some(PkgMeta { kind, triple, pous })
}

/// IrBitcode bytes of a `.tplc`, if present.
pub fn pkg_bitcode(path: &Path) -> Option<Vec<u8>> {
    let bytes = std::fs::read(path).ok()?;
    let pkg = Package::parse(&bytes).ok()?;
    pkg.section(SectionType::IrBitcode)
        .map(|s| s.data.to_vec())
}

/* ---------------- libdir indexing + resolution --------------------- */

/// Where an indexed POU name resolves to.
pub enum LibHit {
    /// Plain `.st` source — merged into the source list.
    St(PathBuf),
    /// `.tplc` package — appended to the `--lib` list (precompiled
    /// link applies when it's a kind=library package).
    Pkg(PathBuf),
}

/// Build the `POU-name → source` index over all libdirs. First
/// directory wins on name collisions — `--libdir` order is precedence.
pub fn index_libdirs(dirs: &[String]) -> HashMap<String, LibHit> {
    let mut index: HashMap<String, LibHit> = HashMap::new();
    for dir in dirs {
        let Ok(rd) = std::fs::read_dir(dir) else {
            continue;
        };
        for e in rd.flatten() {
            let p = e.path();
            let n = p.file_name().and_then(|x| x.to_str()).unwrap_or("");
            if n.ends_with(".st") {
                if let Ok(text) = std::fs::read_to_string(&p) {
                    for name in pou_decls(&text) {
                        index.entry(name).or_insert_with(|| LibHit::St(p.clone()));
                    }
                }
            } else if n.ends_with(".tplc") {
                if let Some(meta) = pkg_meta(&p) {
                    /* only kind=library packages — a program package in
                    a libdir would index its PROGRAM and hand the
                    resolver a second main */
                    if meta.kind == "library" {
                        for name in meta.pous {
                            index.entry(name).or_insert_with(|| LibHit::Pkg(p.clone()));
                        }
                    }
                }
            }
        }
    }
    index
}

/// Outcome of `resolve` — for diagnostics/verbose output.
pub struct ResolveReport {
    /// sources/packages pulled in (display paths, for logging).
    pub pulled: Vec<String>,
    /// referenced names no library provides — left for sema's own
    /// "unknown type" diagnostic.
    pub misses: Vec<String>,
}

/// Pull library sources/packages satisfying undeclared `FB_*`/`F_*`
/// references until fixpoint. `.st` hits append to `srcs`; `.tplc`
/// hits append to `lib_pkgs` (their sources still merge for sema in
/// the caller's existing `--lib` loop).
pub fn resolve(
    libdirs: &[String],
    srcs: &mut Vec<(String, String)>,
    lib_pkgs: &mut Vec<String>,
) -> ResolveReport {
    let index = index_libdirs(libdirs);
    let mut report = ResolveReport { pulled: Vec::new(), misses: Vec::new() };
    if index.is_empty() {
        return report;
    }
    // pou names already covered by user --lib packages (cache per path)
    let mut pkg_names: HashMap<PathBuf, HashSet<String>> = HashMap::new();
    for lp in lib_pkgs.iter() {
        let p = PathBuf::from(lp);
        if !pkg_names.contains_key(&p) {
            pkg_names.insert(p.clone(), pkg_meta(&p).map(|m| m.pous).unwrap_or_default());
        }
    }
    let mut reported_misses = HashSet::new();
    for _round in 0..32 {
        let mut declared = HashSet::new();
        let mut refs = HashSet::new();
        for (name, text) in srcs.iter() {
            declared.extend(pou_decls(text));
            if name.ends_with(".fbd.json") {
                declared.extend(fbd_decl_names(text));
            }
            refs.extend(lib_refs(text));
        }
        for lp in lib_pkgs.iter() {
            let p = PathBuf::from(lp);
            if let Some(names) = pkg_names.get(&p) {
                declared.extend(names.iter().cloned());
            }
        }
        let missing: Vec<String> = refs
            .difference(&declared)
            .filter(|n| !reported_misses.contains(*n))
            .cloned()
            .collect();
        if missing.is_empty() {
            break;
        }
        let mut progress = false;
        for name in missing {
            match index.get(&name) {
                Some(LibHit::St(p)) => {
                    match std::fs::read_to_string(p) {
                        Ok(text) => {
                            report.pulled.push(p.display().to_string());
                            srcs.push((p.display().to_string(), text));
                            progress = true;
                        }
                        Err(_) => {
                            reported_misses.insert(name.clone());
                            report.misses.push(name);
                        }
                    }
                }
                Some(LibHit::Pkg(p)) => {
                    let disp = p.display().to_string();
                    if !lib_pkgs.iter().any(|x| x == &disp) {
                        let names =
                            pkg_meta(p).map(|m| m.pous).unwrap_or_default();
                        pkg_names.insert(p.clone(), names);
                        lib_pkgs.push(disp.clone());
                        report.pulled.push(disp);
                    }
                    progress = true;
                }
                None => {
                    reported_misses.insert(name.clone());
                    report.misses.push(name);
                }
            }
        }
        if !progress {
            break;
        }
    }
    report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decls_and_refs_scan() {
        let st = "FUNCTION_BLOCK FB_Pid\nVAR_INPUT\n K : REAL;\nEND_VAR\nEND_FUNCTION_BLOCK\n\
                  PROGRAM main\nVAR\n pid : FB_Pid; sq : F_Square;\nEND_VAR\nEND_PROGRAM\n";
        let d = pou_decls(st);
        assert!(d.contains("FB_PID") && d.contains("MAIN"));
        let r = lib_refs(st);
        assert!(r.contains("FB_PID") && r.contains("F_SQUARE"));
    }

    #[test]
    fn fbd_decl_names_scan() {
        let j = r#"{"pous":[{"name":"prog1","type":"PROGRAM"}],"blocks":[{"name":"inst1"}]}"#;
        let d = fbd_decl_names(j);
        assert!(d.contains("PROG1") && d.contains("INST1"));
    }

    #[test]
    fn resolve_pulls_st_and_fixpoint() {
        let dir = std::env::temp_dir().join(format!("tplc_libtest_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        // lib A uses lib B — fixpoint must pull both.
        std::fs::write(
            dir.join("a.st"),
            "FUNCTION_BLOCK FB_A\nVAR_INPUT X:DINT; END_VAR\nVAR y:FB_B; END_VAR\nEND_FUNCTION_BLOCK\n",
        )
        .unwrap();
        std::fs::write(
            dir.join("b.st"),
            "FUNCTION_BLOCK FB_B\nVAR_INPUT X:DINT; END_VAR\nEND_FUNCTION_BLOCK\n",
        )
        .unwrap();
        let mut srcs = vec![(
            "prog.st".to_string(),
            "PROGRAM p\nVAR a:FB_A; END_VAR\nEND_PROGRAM\n".to_string(),
        )];
        let mut pkgs: Vec<String> = Vec::new();
        let rep = resolve(&[dir.display().to_string()], &mut srcs, &mut pkgs);
        assert_eq!(srcs.len(), 3, "a.st and b.st must both be pulled");
        assert!(rep.misses.is_empty(), "misses: {:?}", rep.misses);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn pkg_meta_and_bitcode_roundtrip() {
        use tplc_pkg::writer::PackageWriter;
        let dir = std::env::temp_dir().join(format!("tplc_libtest3_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let p = dir.join("m.tplc");
        let mut w = PackageWriter::new();
        w.uuid_str("018c2f7e-9a3b-7c4d-8e5f-1234567890ab").unwrap();
        w.add_section(
            SectionType::Manifest,
            br#"{"kind":"library","target_triple":"x86_64-pc-linux-gnu","pous":["FB_A","F_B"]}"#
                .to_vec(),
            false,
            1,
            0,
        )
        .unwrap();
        w.add_section(SectionType::IrBitcode, b"BC\xde\xad".to_vec(), false, 1, 0)
            .unwrap();
        std::fs::write(&p, w.build().unwrap()).unwrap();
        let m = pkg_meta(&p).unwrap();
        assert_eq!(m.kind, "library");
        assert_eq!(m.triple, "x86_64-pc-linux-gnu");
        assert!(m.pous.contains("FB_A") && m.pous.contains("F_B"));
        assert_eq!(pkg_bitcode(&p).unwrap(), b"BC\xde\xad");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn resolve_reports_misses() {
        let dir = std::env::temp_dir().join(format!("tplc_libtest2_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        // an unrelated lib file so the index isn't empty
        std::fs::write(
            dir.join("other.st"),
            "FUNCTION_BLOCK FB_Other\nEND_FUNCTION_BLOCK\n",
        )
        .unwrap();
        let mut srcs = vec![(
            "prog.st".to_string(),
            "PROGRAM p\nVAR a:FB_Nope; END_VAR\nEND_PROGRAM\n".to_string(),
        )];
        let mut pkgs: Vec<String> = Vec::new();
        let rep = resolve(&[dir.display().to_string()], &mut srcs, &mut pkgs);
        assert!(rep.misses.contains(&"FB_NOPE".to_string()));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
