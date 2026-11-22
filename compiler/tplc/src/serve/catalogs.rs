use super::*;


/// Merged `tplcc blocks --json` catalog over every --libdir; returns
/// the `{"libs":[…]}` JSON text (cached).
pub(crate) fn do_blocks(cfg: &ServeCfg, refresh: bool, cache: &Arc<Mutex<(String, bool)>>) -> String {
    let need = {
        let c = cache.lock().unwrap();
        refresh || c.1 || c.0.is_empty()
    };
    if need {
        let mut merged = String::from("{\"libs\":[");
        let mut first = true;
        if let Some(tplcc) = &cfg.tplcc {
            for dir in &cfg.libdirs {
                let files = collect_lib_files(dir);
                if files.is_empty() {
                    continue;
                }
                let mut cmd = Command::new(tplcc);
                cmd.arg("blocks").arg("--json");
                for f in &files {
                    cmd.arg(f);
                }
                let out = cmd.stderr(Stdio::null()).output().ok();
                let Some(o) = out else { continue };
                let text = String::from_utf8_lossy(&o.stdout);
                if !text.starts_with('{') {
                    continue;
                }
                let lib = dir.trim_end_matches('/').rsplit('/').next().unwrap_or(dir);
                if !first {
                    merged.push(',');
                }
                first = false;
                merged.push_str(&format!(
                    "{{\"name\":\"{}\",\"dir\":\"{}\",\"catalog\":{}}}",
                    json_escape(lib),
                    json_escape(dir),
                    text.trim()
                ));
            }
        }
        merged.push_str("]}");
        *cache.lock().unwrap() = (merged, false);
    }
    cache.lock().unwrap().0.clone()
}

/// Library source lookup: `lib` = libdir name or index, `file` =
/// basename (.st direct, .tplc via `tplcc src`). Returns the bytes.
pub(crate) fn do_libsrc(cfg: &ServeCfg, lib: &str, file: &str) -> Option<Vec<u8>> {
    let safe = !file.is_empty()
        && !file.contains('/')
        && file
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'));
    if lib.is_empty() || file.is_empty() || file.contains("..") || !safe {
        return None;
    }
    let dir = if let Ok(idx) = lib.parse::<usize>() {
        cfg.libdirs.get(idx).cloned()
    } else {
        cfg.libdirs.iter().find(|d| {
            d.trim_end_matches('/').rsplit('/').next().unwrap_or(d) == lib
        }).cloned()
    }?;
    let path = Path::new(&dir).join(file);
    let mut body = std::fs::read(&path).unwrap_or_default();
    if body.is_empty() {
        // try each .tplc package via `tplcc src <pkg> <name>`
        if let Some(tplcc) = &cfg.tplcc {
            for p in collect_lib_files(&dir) {
                if !p.to_string_lossy().ends_with(".tplc") {
                    continue;
                }
                if let Ok(o) = Command::new(tplcc)
                    .args(["src"])
                    .arg(&p)
                    .arg(file)
                    .stderr(Stdio::null())
                    .output()
                {
                    if o.status.success() && !o.stdout.is_empty() {
                        body = o.stdout;
                        break;
                    }
                }
            }
        }
    }
    if body.is_empty() {
        None
    } else {
        Some(body)
    }
}

