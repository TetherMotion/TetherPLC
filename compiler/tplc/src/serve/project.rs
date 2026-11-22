use super::*;

/* ---------------- project & git meta-ops ----------------
   WS ops (JSON {"id":N,"op":"…"} → {"id":N,"ok":bool,"data"|"error"}):
     project_get / projects / project_open / project_save
     project_files / project_file_get / project_file_put / project_file_delete
     git_status / git_log / git_diff / git_branch / git_commit
   The active project root lives in cfg.project_root (Arc<Mutex>) so
   project_open re-roots every connection. */

pub(crate) fn proj_root(cfg: &ServeCfg) -> Option<PathBuf> {
    cfg.project_root.lock().ok().and_then(|g| g.clone())
}

pub(crate) fn pj_err(m: &str) -> serde_json::Value {
    serde_json::json!({"ok": false, "error": m})
}

pub(crate) fn pj_ok(v: serde_json::Value) -> serde_json::Value {
    serde_json::json!({"ok": true, "data": v})
}

pub(crate) fn project_manifest_json(p: &crate::project::Project) -> serde_json::Value {
    serde_json::json!({
        "root": p.root.display().to_string(),
        "name": p.display_name(),
        "manifest": p.manifest,
        "git": crate::git::is_repo(&p.root),
        "branch": crate::git::branch(&p.root).unwrap_or_default(),
    })
}

/// Dispatch for ops starting with `project`/`git_`/`projects`.
/// Returns Some(reply) for handled ops, None for unknown.
pub(crate) fn project_meta(cfg: &ServeCfg, op: &str, req: &serde_json::Value) -> Option<serde_json::Value> {
    let s = |k: &str| req.get(k).and_then(|v| v.as_str()).unwrap_or("");
    match op {
        "project_get" => {
            let root = proj_root(cfg)?;
            return Some(match crate::project::load(&root) {
                Ok(p) => pj_ok(project_manifest_json(&p)),
                Err(e) => pj_err(&e),
            });
        }
        "projects" => {
            let list: Vec<String> = crate::project::recent()
                .iter()
                .map(|p| p.display().to_string())
                .collect();
            return Some(pj_ok(serde_json::json!(list)));
        }
        "project_open" => {
            let path = s("path");
            if path.is_empty() {
                return Some(pj_err("missing path"));
            }
            let dir = PathBuf::from(path);
            if !dir.is_dir() {
                return Some(pj_err("not a directory"));
            }
            let proj = match crate::project::discover(&dir) {
                Some(root) => crate::project::load(&root),
                None => crate::project::init(&dir, None),
            };
            return Some(match proj {
                Ok(p) => {
                    if let Ok(mut g) = cfg.project_root.lock() {
                        *g = Some(p.root.clone());
                    }
                    crate::project::record_recent(&p.root);
                    pj_ok(project_manifest_json(&p))
                }
                Err(e) => pj_err(&e),
            });
        }
        "project_save" => {
            let Some(root) = proj_root(cfg) else {
                return Some(pj_err("no project open"));
            };
            // body is a full manifest; unknown keys round-trip via
            // `extra`.  Strip the envelope keys (id/op) first; accept
            // either {"manifest": {...}} or flat manifest fields.
            let mut clean = req.clone();
            if let Some(obj) = clean.as_object_mut() {
                obj.remove("id");
                obj.remove("op");
            }
            let parsed = match clean.get("manifest") {
                Some(inner) => serde_json::from_value::<crate::project::Manifest>(inner.clone()),
                None => serde_json::from_value::<crate::project::Manifest>(clean),
            };
            let manifest = match parsed {
                Ok(m) => m,
                Err(e) => return Some(pj_err(&format!("bad manifest: {e}"))),
            };
            let p = crate::project::Project {
                root: root.clone(),
                manifest,
            };
            return Some(match p.save() {
                Ok(()) => pj_ok(serde_json::json!({"saved": true})),
                Err(e) => pj_err(&e.to_string()),
            });
        }
        "project_files" => {
            let Some(root) = proj_root(cfg) else {
                return Some(pj_err("no project open"));
            };
            #[derive(serde::Serialize)]
            struct FEntry {
                path: String,
                size: u64,
            }
            let mut out: Vec<FEntry> = Vec::new();
            let mut stack = vec![root.clone()];
            while let Some(d) = stack.pop() {
                if out.len() >= 5000 {
                    break;
                }
                let Ok(rd) = std::fs::read_dir(&d) else { continue };
                for e in rd.flatten() {
                    let p = e.path();
                    let name = e.file_name().to_string_lossy().to_string();
                    if p.is_dir() {
                        if name == ".git" || name == "build" {
                            continue;
                        }
                        stack.push(p);
                    } else if p.is_file() {
                        let rel = p
                            .strip_prefix(&root)
                            .map(|r| r.display().to_string())
                            .unwrap_or_default();
                        out.push(FEntry {
                            path: rel,
                            size: e.metadata().map(|m| m.len()).unwrap_or(0),
                        });
                    }
                }
            }
            out.sort_by(|a, b| a.path.cmp(&b.path));
            return Some(pj_ok(serde_json::json!(out)));
        }
        "project_file_get" | "project_file_put" | "project_file_delete" => {
            let Some(root) = proj_root(cfg) else {
                return Some(pj_err("no project open"));
            };
            let rel = s("path");
            let Some(abs) = crate::project::safe_join(&root, &rel) else {
                return Some(pj_err("bad path"));
            };
            return Some(match op {
                "project_file_get" => match std::fs::metadata(&abs) {
                    Ok(m) if m.is_file() => {
                        if m.len() > 4 * 1024 * 1024 {
                            pj_err("file > 4 MiB")
                        } else {
                            match std::fs::read(&abs) {
                                Ok(data) => pj_ok(serde_json::json!({
                                    "path": rel,
                                    "content": String::from_utf8_lossy(&data),
                                    "size": m.len(),
                                })),
                                Err(e) => pj_err(&e.to_string()),
                            }
                        }
                    }
                    _ => pj_err("no such file"),
                },
                "project_file_put" => {
                    let content = s("content").to_string();
                    if let Some(d) = abs.parent() {
                        if let Err(e) = std::fs::create_dir_all(d) {
                            return Some(pj_err(&e.to_string()));
                        }
                    }
                    match std::fs::write(&abs, &content) {
                        Ok(()) => pj_ok(serde_json::json!({"written": rel})),
                        Err(e) => pj_err(&e.to_string()),
                    }
                }
                _ => match std::fs::remove_file(&abs) {
                    Ok(()) => pj_ok(serde_json::json!({"deleted": rel})),
                    Err(e) => pj_err(&e.to_string()),
                },
            });
        }
        "git_status" | "git_log" | "git_diff" | "git_branch" | "git_commit" => {
            let Some(root) = proj_root(cfg) else {
                return Some(pj_err("no project open"));
            };
            return Some(match op {
                "git_status" => match crate::git::status(&root) {
                    Ok(entries) => pj_ok(serde_json::json!(entries)),
                    Err(e) => pj_err(&e),
                },
                "git_log" => {
                    let n = req.get("n").and_then(|v| v.as_u64()).unwrap_or(50) as usize;
                    match crate::git::log(&root, n) {
                        Ok(commits) => pj_ok(serde_json::json!(commits)),
                        Err(e) => pj_err(&e),
                    }
                }
                "git_diff" => {
                    let p = req.get("path").and_then(|v| v.as_str());
                    match crate::git::diff(&root, p) {
                        Ok(Some(d)) => {
                            let d = if d.len() > 512 * 1024 {
                                format!("{}\n… [truncated]", &d[..512 * 1024])
                            } else {
                                d
                            };
                            pj_ok(serde_json::json!({"diff": d}))
                        }
                        Ok(None) => pj_ok(serde_json::json!({"diff": null})),
                        Err(e) => pj_err(&e),
                    }
                }
                "git_branch" => match crate::git::branch(&root) {
                    Ok(b) => pj_ok(serde_json::json!({"branch": b})),
                    Err(e) => pj_err(&e),
                },
                _ => {
                    let msg = s("message");
                    if msg.trim().is_empty() {
                        pj_err("empty commit message")
                    } else {
                        match crate::git::commit(&root, msg) {
                            Ok(oid) => pj_ok(serde_json::json!({
                                "oid": oid,
                                "short": &oid[..7.min(oid.len())],
                            })),
                            Err(e) => pj_err(&e),
                        }
                    }
                }
            });
        }
        _ => return None,
    }
}
