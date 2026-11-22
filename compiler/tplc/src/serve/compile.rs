use super::*;

/* ---------------- compile ---------------- */

/// Binary src frame `[u16 count][u16 nameLen,name,u32 len,content]×N`
/// — the tplccd daemon's wire format.
pub(crate) fn encode_src_frame(files: &[(String, Vec<u8>)]) -> Vec<u8> {
    let total = 2 + files.iter().map(|(n, c)| 2 + n.len() + 4 + c.len()).sum::<usize>();
    let mut out = Vec::with_capacity(total);
    out.extend_from_slice(&(files.len() as u16).to_le_bytes());
    for (name, content) in files {
        out.extend_from_slice(&(name.len() as u16).to_le_bytes());
        out.extend_from_slice(name.as_bytes());
        out.extend_from_slice(&(content.len() as u32).to_le_bytes());
        out.extend_from_slice(content);
    }
    out
}

/// Compile `files` via `tplcc check|build --json`; returns the
/// `{compile, exit, package?}` envelope as a JSON value. `check`
/// prefers the tplccd daemon when configured.
pub(crate) fn do_compile(cfg: &ServeCfg, files: &[(String, Vec<u8>)], build: bool) -> serde_json::Value {
    // fast path: tplccd daemon for check — same binary src frame
    if !build {
        if let Some(ds) = &cfg.tplccd {
            if !ds.is_empty() {
                if let Some(mut u) = uds_connect(ds, 500) {
                    let frame = encode_src_frame(files);
                    if u.write_all(&frame).is_ok() {
                        let _ = u.shutdown(std::net::Shutdown::Write);
                        let mut out = Vec::new();
                        let mut b = [0u8; 8192];
                        loop {
                            match u.read(&mut b) {
                                Ok(0) | Err(_) => break,
                                Ok(n) => out.extend_from_slice(&b[..n]),
                            }
                        }
                        if !out.is_empty() {
                            if let Ok(v) = serde_json::from_slice::<serde_json::Value>(&out) {
                                return serde_json::json!({"compile": v, "exit": 0});
                            }
                        }
                    }
                }
            }
        }
    }
    let err_env = |msg: &str| {
        serde_json::json!({
            "compile": {"ok": false, "errors": 1, "diags": [
                {"code": "TPLC_E_DRIVER", "severity": "error", "message": msg}]},
            "exit": -1
        })
    };
    if files.is_empty() || files.len() > 64 {
        return err_env("bad src frame");
    }
    let Some(tplcc) = &cfg.tplcc else {
        return err_env("tplcc not found");
    };
    let dir = std::env::temp_dir().join(format!("tplc_compile_{}_{}", std::process::id(), unique()));
    if std::fs::create_dir_all(&dir).is_err() {
        return err_env("mkdtemp failed");
    }
    let mut args: Vec<String> = if build {
        vec!["build".into(), "--json".into(), "-o".into(), "out.tplc".into()]
    } else {
        vec!["check".into(), "--json".into()]
    };
    /* auto-resolve library refs from the configured libdirs — same
    automagic the WebUI's projectLibRefs does client-side */
    for d in &cfg.libdirs {
        args.push("--libdir".into());
        args.push(d.clone());
    }
    let mut ok = true;
    for (name, content) in files {
        let p = dir.join(name);
        if std::fs::write(&p, content).is_err() {
            ok = false;
            break;
        }
        args.push(name.clone());
    }
    let mut resp = String::new();
    let mut errlog = String::new();
    let mut exit_code: i64 = -1;
    if ok {
        let out = Command::new(tplcc)
            .args(&args)
            .current_dir(&dir)
            .stderr(Stdio::piped())
            .output();
        match out {
            Ok(o) => {
                exit_code = o.status.code().map(|c| c as i64).unwrap_or(-1);
                resp = String::from_utf8_lossy(&o.stdout).into_owned();
                errlog = String::from_utf8_lossy(&o.stderr).into_owned();
            }
            Err(_) => {}
        }
    }
    let mut okv = exit_code == 0;
    if exit_code < 0 && resp.contains("\"ok\":true") {
        okv = true;
        exit_code = 0;
    }
    let compile_json = if !resp.is_empty() && resp.starts_with('{') {
        resp.trim_end().to_string()
    } else {
        let msg = if errlog.is_empty() {
            "tplcc invocation failed".to_string()
        } else {
            errlog
        };
        format!(
            "{{\"ok\":false,\"errors\":1,\"diags\":[{{\"code\":\"TPLC_E_DRIVER\",\"severity\":\"error\",\"message\":\"{}\"}}]}}",
            json_escape(&msg)
        )
    };
    let mut env = serde_json::json!({});
    if let Ok(cj) = serde_json::from_str::<serde_json::Value>(&compile_json) {
        env["compile"] = cj;
    } else {
        env["compile"] = serde_json::json!({"ok": false, "errors": 1});
    }
    env["exit"] = serde_json::json!(exit_code);
    if build && okv {
        let pkg = dir.join("out.tplc");
        if let Ok(data) = std::fs::read(&pkg) {
            env["package"] = serde_json::json!(b64e(&data));
        }
    }
    let _ = std::fs::remove_dir_all(&dir);
    env
}

pub(crate) fn unique() -> u64 {
    use std::sync::atomic::{AtomicU64, Ordering};
    static C: AtomicU64 = AtomicU64::new(0);
    C.fetch_add(1, Ordering::Relaxed)
}

/* ---------------- pkgmap ---------------- */

/// Inject a DEPLOY_MAP section into a .tplc package; returns the new
/// package bytes or an error string.
pub(crate) fn do_pkgmap(cfg: &ServeCfg, pkg_b64: &str, map_text: &str) -> Result<Vec<u8>, String> {
    let pkg = b64d(pkg_b64);
    if pkg.is_empty() || pkg.len() > 64 * 1024 * 1024 {
        return Err("bad package payload".into());
    }
    let tplcc = cfg.tplcc.as_ref().ok_or("tplcc not found")?;
    let dir = std::env::temp_dir().join(format!("tplc_pkgmap_{}_{}", std::process::id(), unique()));
    if std::fs::create_dir_all(&dir).is_err() {
        return Err("mkdtemp failed".into());
    }
    let ok = std::fs::write(dir.join("in.tplc"), &pkg).is_ok()
        && std::fs::write(dir.join("map.txt"), map_text.as_bytes()).is_ok();
    if !ok {
        let _ = std::fs::remove_dir_all(&dir);
        return Err("pkgmap temp write failed".into());
    }
    let out = Command::new(tplcc)
        .args(["pkg", "add-section", "in.tplc", "--type", "0x31", "--file", "map.txt", "-o", "out.tplc"])
        .current_dir(&dir)
        .stderr(Stdio::piped())
        .output();
    let mut errlog = String::new();
    if let Ok(o) = out {
        errlog = String::from_utf8_lossy(&o.stderr).into_owned();
        if let Ok(res) = std::fs::read(dir.join("out.tplc")) {
            if !res.is_empty() {
                let _ = std::fs::remove_dir_all(&dir);
                return Ok(res);
            }
        }
    }
    let _ = std::fs::remove_dir_all(&dir);
    Err(if errlog.is_empty() {
        "tplcc pkg add-section failed".into()
    } else {
        errlog
    })
}

