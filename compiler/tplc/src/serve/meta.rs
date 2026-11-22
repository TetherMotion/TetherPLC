use super::*;


/// JSON meta-op dispatch — runs on the meta worker thread.
pub(crate) fn meta_dispatch(
    cfg: &ServeCfg,
    cache: &Arc<Mutex<(String, bool)>>,
    req: &serde_json::Value,
) -> serde_json::Value {
    let op = req.get("op").and_then(|v| v.as_str()).unwrap_or("");
    let fail = |m: &str| serde_json::json!({"ok": false, "error": m});
    match op {
        "compile" => {
            let build = req.get("build").and_then(|v| v.as_bool()).unwrap_or(false);
            let mut files: Vec<(String, Vec<u8>)> = Vec::new();
            let mut bad = false;
            if let Some(arr) = req.get("files").and_then(|v| v.as_array()) {
                let mut total = 0usize;
                for f in arr.iter().take(65) {
                    let name = f.get("name").and_then(|v| v.as_str()).unwrap_or("");
                    let content = f.get("content").and_then(|v| v.as_str()).unwrap_or("");
                    let name = sanitize_name(name);
                    let bytes = content.as_bytes().to_vec();
                    total += bytes.len();
                    if total > 16 * 1024 * 1024 || bytes.len() > 4 * 1024 * 1024 {
                        bad = true;
                        break;
                    }
                    files.push((name, bytes));
                }
            } else {
                bad = true;
            }
            if bad || files.is_empty() {
                return fail("bad compile request");
            }
            serde_json::json!({"ok": true, "data": do_compile(cfg, &files, build)})
        }
        "blocks" => {
            let refresh = req.get("refresh").and_then(|v| v.as_bool()).unwrap_or(false);
            let text = do_blocks(cfg, refresh, cache);
            match serde_json::from_str::<serde_json::Value>(&text) {
                Ok(v) => serde_json::json!({"ok": true, "data": v}),
                Err(_) => serde_json::json!({"ok": true, "data": {"libs": []}}),
            }
        }
        "libsrc" => {
            let lib = req.get("lib").and_then(|v| v.as_str()).unwrap_or("");
            let file = req.get("file").and_then(|v| v.as_str()).unwrap_or("");
            match do_libsrc(cfg, lib, file) {
                Some(b) => serde_json::json!({
                    "ok": true,
                    "data": {"text": String::from_utf8_lossy(&b)}
                }),
                None => fail("not found"),
            }
        }
        "pkgmap" => {
            let pkg = req.get("package").and_then(|v| v.as_str()).unwrap_or("");
            let map = req.get("map").and_then(|v| v.as_str()).unwrap_or("");
            match do_pkgmap(cfg, pkg, map) {
                Ok(res) => serde_json::json!({
                    "ok": true,
                    "data": {"package": b64e(&res)}
                }),
                Err(e) => fail(&e),
            }
        }
        "bus_scan" => match do_bus_scan(cfg, req) {
            Ok(v) => serde_json::json!({"ok": true, "data": v}),
            Err(e) => fail(&e),
        },
        "iface_list" => serde_json::json!({"ok": true, "data": do_iface_list(cfg, req)}),
        "sim_run" => match do_sim_run(cfg, req) {
            Ok(v) => serde_json::json!({"ok": true, "data": v}),
            Err(e) => fail(&e),
        },
        _ => {
            if op.starts_with("project") || op.starts_with("git_") || op == "projects" {
                project_meta(cfg, op, req).unwrap_or_else(|| fail("unknown op"))
            } else {
                fail("unknown op")
            }
        }
    }
}
