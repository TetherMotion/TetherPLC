//! Session registry — `tplc run`/`tplc dev` record what they spawned so
//! `tplc status`, `tplc down`, `tplc sessions` and `tplc serve` can find
//! a running stack without memorizing socket paths.
//!
//! Files: `$TPLC_STATE_DIR` or `${XDG_RUNTIME_DIR:-/tmp}/tplc/<name>.json`

use serde_json::{json, Value};
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Session {
    pub name: String,
    pub pid: u32, // tplc process that owns the stack (0 = external)
    pub runtime_pid: u32,
    pub ctl: String,
    pub image: String,
    pub pkg: String,
    pub io_port: u16,
    pub http_port: u16,
    pub spawned: Vec<u32>, // child pids tplc is responsible for
    pub started: String,
}

pub fn state_dir() -> PathBuf {
    if let Ok(d) = std::env::var("TPLC_STATE_DIR") {
        if !d.is_empty() {
            return PathBuf::from(d);
        }
    }
    if let Ok(d) = std::env::var("XDG_RUNTIME_DIR") {
        if !d.is_empty() {
            return PathBuf::from(d).join("tplc");
        }
    }
    PathBuf::from("/tmp/tplc.d")
}

fn pid_alive(pid: u32) -> bool {
    pid != 0 && PathBuf::from(format!("/proc/{pid}")).exists()
}

impl Session {
    pub fn alive(&self) -> bool {
        pid_alive(self.runtime_pid) || pid_alive(self.pid)
    }
    pub fn ctl_alive(&self) -> bool {
        crate::ctl::Ctl::probe(&self.ctl)
    }
}

pub fn session_path(name: &str) -> PathBuf {
    state_dir().join(format!("{name}.json"))
}

pub fn write(s: &Session) -> std::io::Result<()> {
    let dir = state_dir();
    std::fs::create_dir_all(&dir)?;
    let v = json!({
        "name": s.name,
        "pid": s.pid,
        "runtime_pid": s.runtime_pid,
        "ctl": s.ctl,
        "image": s.image,
        "pkg": s.pkg,
        "io_port": s.io_port,
        "http_port": s.http_port,
        "spawned": s.spawned,
        "started": s.started,
    });
    std::fs::write(session_path(&s.name), serde_json::to_string_pretty(&v)?)
}

pub fn remove(name: &str) {
    let _ = std::fs::remove_file(session_path(name));
}

fn from_json(v: &Value) -> Option<Session> {
    Some(Session {
        name: v.get("name")?.as_str()?.to_string(),
        pid: v.get("pid").and_then(|x| x.as_u64()).unwrap_or(0) as u32,
        runtime_pid: v.get("runtime_pid").and_then(|x| x.as_u64()).unwrap_or(0) as u32,
        ctl: v.get("ctl").and_then(|x| x.as_str()).unwrap_or("").to_string(),
        image: v.get("image").and_then(|x| x.as_str()).unwrap_or("").to_string(),
        pkg: v.get("pkg").and_then(|x| x.as_str()).unwrap_or("").to_string(),
        io_port: v.get("io_port").and_then(|x| x.as_u64()).unwrap_or(0) as u16,
        http_port: v.get("http_port").and_then(|x| x.as_u64()).unwrap_or(0) as u16,
        spawned: v
            .get("spawned")
            .and_then(|x| x.as_array())
            .map(|a| a.iter().filter_map(|x| x.as_u64()).map(|x| x as u32).collect())
            .unwrap_or_default(),
        started: v.get("started").and_then(|x| x.as_str()).unwrap_or("").to_string(),
    })
}

pub fn list() -> Vec<Session> {
    let mut out = Vec::new();
    let dir = state_dir();
    if let Ok(rd) = std::fs::read_dir(&dir) {
        for e in rd.flatten() {
            let p = e.path();
            if p.extension().and_then(|x| x.to_str()) != Some("json") {
                continue;
            }
            if let Ok(txt) = std::fs::read_to_string(&p) {
                if let Ok(v) = serde_json::from_str::<Value>(&txt) {
                    if let Some(s) = from_json(&v) {
                        out.push(s);
                    }
                }
            }
        }
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}

/// Resolve a session by name; when `name` is empty, use the only live
/// session, or error listing candidates.
pub fn resolve(name: &str) -> Result<Session, String> {
    if !name.is_empty() {
        let s = from_json(
            &serde_json::from_str::<Value>(
                &std::fs::read_to_string(session_path(name))
                    .map_err(|_| format!("no session `{name}`"))?,
            )
            .map_err(|e| format!("bad session file: {e}"))?,
        )
        .ok_or_else(|| format!("bad session `{name}`"))?;
        return Ok(s);
    }
    let live: Vec<Session> = list().into_iter().filter(|s| s.alive()).collect();
    match live.len() {
        0 => Err("no live sessions — start one with `tplc dev` or `tplc run`".into()),
        1 => Ok(live.into_iter().next().unwrap()),
        _ => {
            let names: Vec<String> = live.iter().map(|s| s.name.clone()).collect();
            Err(format!("multiple live sessions: {} — pass --session/--name", names.join(", ")))
        }
    }
}
