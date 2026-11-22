//! `tplc serve` / `tplc dev` web backend — native port of
//! `tools/tplc_ctl_proxy.cpp` plus static WebUI file serving.
//!
//! Routes (all replies carry CORS headers like the C++ proxy):
//!   POST /ctl            verbatim ctl frame round-trip
//!   POST /compile[?build] src frame → tplcc check|build --json
//!   POST /pkgmap         {package:b64,map:text} → tplcc pkg add-section
//!   GET  /events         SSE stream of ctl F_EVENT frames
//!   GET  /blocks[?refresh] merged tplcc blocks --json over --libdir
//!   GET  /libsrc?lib&file   library source (.st direct, .tplc via tplcc src)
//!   GET  /tether-io      WebSocket → tplc-io TCP bridge (SLIP framing)
//!   GET  /health         ok iff ctl socket connects
//!   OPTIONS              CORS preflight
//!   GET  /*              static files from --webui dir (SPA fallback)

use crate::ctl::{op, Ctl};
use crate::tools::Tools;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct ServeCfg {
    /// Active project root for /project, /project/file(s) and /git/*
    /// endpoints.  `Arc<Mutex>` so POST /project/open re-roots every
    /// connection.  None = no project endpoints (404).
    pub project_root: Arc<Mutex<Option<PathBuf>>>,
    pub ctl_sock: String,
    pub bind: String,
    pub port: u16,
    pub tplcc: Option<PathBuf>,
    pub tplccd: Option<String>, // managed compile daemon socket; "" disables in caller
    pub tplc_io: Option<String>, // host:port upstream for /tether-io
    pub libdirs: Vec<String>,
    pub webui: Option<String>,
    pub packages: Option<String>,
    pub pkgs_for_io: Vec<String>, // --pkg args when tplc spawns tplc-io itself
    pub spawn_io: bool,
    pub io_port_hint: u16,
    /// tether-plc-sim binary for the `sim_run` meta-op.
    pub sim: Option<PathBuf>,
    /// tether-plc binary — sim_run passes it via --plc.
    pub plc: Option<PathBuf>,
    /// tplc-ec-pump binary for the `bus_scan` meta-op.
    pub ecpump: Option<PathBuf>,
    /// Tool resolver — meta-ops re-resolve lazily so binaries built
    /// after the session started (or a stale cached path) are picked
    /// up without restarting `tplc serve`/`dev`.
    pub tools: Tools,
    /// ecpump artifact dir (.fleet/.topology/.bus) — also the sim workdir.
    /// Default: <project>/.tplc/sim, else a temp dir.
    pub simdir: PathBuf,
}

static STOP: AtomicBool = AtomicBool::new(false);

extern "C" fn on_signal(_sig: i32) {
    STOP.store(true, Ordering::Relaxed);
}

fn install_stop_handler() {
    unsafe {
        libc::signal(libc::SIGINT, on_signal as extern "C" fn(i32) as usize);
        libc::signal(libc::SIGTERM, on_signal as extern "C" fn(i32) as usize);
    }
}

mod bus;
mod catalogs;
mod compile;
mod ctl;
mod helpers;
mod meta;
mod project;
mod staticf;
mod tetherio;
mod ws_conn;
mod ws_upgrade;

pub(crate) use bus::*;
pub(crate) use catalogs::*;
pub(crate) use compile::*;
pub(crate) use ctl::*;
pub(crate) use helpers::*;
pub(crate) use meta::*;
pub(crate) use project::*;
pub(crate) use staticf::*;
pub(crate) use tetherio::*;
pub(crate) use ws_conn::*;
pub(crate) use ws_upgrade::*;
/* ---------------- connection handling ---------------- */

fn handle_conn(mut cfd: TcpStream, cfg: &ServeCfg, cache: &Arc<Mutex<(String, bool)>>) {
    cfd.set_read_timeout(Some(std::time::Duration::from_secs(10))).ok();
    let mut buf = Vec::with_capacity(8192);
    let mut tmp = [0u8; 8192];
    let hdr_end;
    loop {
        match cfd.read(&mut tmp) {
            Ok(0) | Err(_) => return,
            Ok(n) => {
                buf.extend_from_slice(&tmp[..n]);
                if let Some(p) = find_subslice(&buf, b"\r\n\r\n") {
                    hdr_end = p + 4;
                    break;
                }
                if buf.len() > 1 << 20 {
                    return;
                }
            }
        }
    }
    let head = String::from_utf8_lossy(&buf[..hdr_end]).into_owned();
    let mut parts = head.split_whitespace();
    let method = parts.next().unwrap_or("").to_string();
    let target = parts.next().unwrap_or("").to_string();
    let (path, _query) = match target.split_once('?') {
        Some((p, q)) => (p.to_string(), q.to_string()),
        None => (target.clone(), String::new()),
    };

    match (method.as_str(), path.as_str()) {
        ("OPTIONS", _) => reply_str(&mut cfd, 204, "No Content", "", "text/plain"),
        ("GET", "/health") => {
            // liveness probe — always 200; body reports ctl reachability
            let body = if Ctl::probe(&cfg.ctl_sock) { "ok" } else { "down" };
            reply_str(&mut cfd, 200, "OK", body, "text/plain");
        }
        ("GET", "/ws") => {
            if !wants_ws_upgrade(&head) {
                reply_str(&mut cfd, 400, "Bad Request", "websocket upgrade required", "text/plain");
            } else {
                handle_ws(cfd, head, cfg, cache);
            }
        }
        ("GET", "/tether-io") => {
            if cfg.tplc_io.is_none() {
                reply_str(&mut cfd, 404, "Not Found", "tether-io bridge not configured", "text/plain");
            } else if !wants_ws_upgrade(&head) {
                reply_str(&mut cfd, 400, "Bad Request", "websocket upgrade required", "text/plain");
            } else {
                handle_tetherio(cfd, head, cfg.tplc_io.clone().unwrap_or_default());
                return;
            }
        }
        ("GET", "/bus") => {
            /* ecpump artifact dump for the Discovery section:
               .topology (JSON per-slave), .fleet (canonical text —
               reusable as --fleet FILE), .bus (tx/rx pdo catalog).
               Missing files come back null — the section renders
               whatever the pump last published. */
            let rd = |name: &str| -> serde_json::Value {
                match std::fs::read_to_string(cfg.simdir.join(name)) {
                    Ok(s) if !s.is_empty() => serde_json::Value::String(s),
                    _ => serde_json::Value::Null,
                }
            };
            let body = serde_json::json!({
                "simdir": cfg.simdir.to_string_lossy(),
                "topology": rd(".topology"),
                "fleet": rd(".fleet"),
                "bus": rd(".bus"),
            });
            reply_str(&mut cfd, 200, "OK", &body.to_string(), "application/json");
        }
        ("GET", _) => {
            if let Some(w) = &cfg.webui {
                serve_static(&mut cfd, w, &path);
            } else if path == "/" || path.is_empty() {
                // API-only mode (no webui bundle): give browsers a
                // hint instead of a bare "not found".
                reply_str(&mut cfd, 200, "OK",
                          "tplc serve — api only (no WebUI bundle found).\n\
                           Clone https://github.com/TetherMotion/TetherWebUI\n\
                           next to tplc (it auto-builds), or set --webui\n\
                           <dir> / TPLC_WEBUI to a built dist/.\n\
                           Ops run over ws://<host>:<port>/ws\n",
                          "text/plain");
            } else {
                reply_str(&mut cfd, 404, "Not Found", "not found", "text/plain");
            }
        }
        _ => reply_str(&mut cfd, 404, "Not Found", "not found", "text/plain"),
    }
}

fn find_subslice(hay: &[u8], needle: &[u8]) -> Option<usize> {
    hay.windows(needle.len()).position(|w| w == needle)
}

/* ---------------- serve entry ---------------- */

pub fn serve(tools: &Tools, cfg: &ServeCfg) -> i32 {
    // managed compile daemon: spawn `tplcc serve <sock>` if configured
    // and not already listening
    let mut cc_child: Option<Child> = None;
    if let (Some(tplcc), Some(ds)) = (&cfg.tplcc, &cfg.tplccd) {
        if !ds.is_empty() && !uds_connect(ds, 300).is_some() {
            if let Ok(c) = Command::new(tplcc)
                .arg("serve")
                .arg(ds)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
            {
                // wait briefly for bind
                for _ in 0..50 {
                    if uds_connect(ds, 100).is_some() {
                        break;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(50));
                }
                cc_child = Some(c);
            }
        }
    }
    let cc_child = Arc::new(Mutex::new(cc_child));
    let cache: Arc<Mutex<(String, bool)>> = Arc::new(Mutex::new((String::new(), true)));

    // optional tplc-io spawn (when dev asked for io but no upstream given)
    let mut io_child: Option<Child> = None;
    let mut upstream = cfg.tplc_io.clone();
    if cfg.spawn_io && upstream.is_none() {
        if let Some(io_bin) = tools.find("tplc-io") {
            let port = if cfg.io_port_hint != 0 { cfg.io_port_hint } else {
                std::net::TcpListener::bind("127.0.0.1:0")
                    .ok()
                    .and_then(|l| l.local_addr().ok())
                    .map(|a| a.port())
                    .unwrap_or(43770)
            };
            let mut cmd = Command::new(&io_bin);
            cmd.arg("--ctl").arg(&cfg.ctl_sock)
                .arg("--port").arg(port.to_string())
                .arg("--bind").arg("127.0.0.1");
            if let Some(p) = &cfg.packages {
                cmd.arg("--packages").arg(p);
            }
            for p in &cfg.pkgs_for_io {
                cmd.arg("--pkg").arg(p);
            }
            match cmd.spawn() {
                Ok(c) => {
                    io_child = Some(c);
                    upstream = Some(format!("127.0.0.1:{port}"));
                }
                Err(e) => eprintln!("tplc: tplc-io spawn: {e}"),
            }
        }
    }
    // patch cfg upstream for this run
    let mut cfg_owned = cfg.clone();
    if upstream.is_some() {
        cfg_owned.tplc_io = upstream;
    }
    let cfg = &cfg_owned;

    let l = match TcpListener::bind(format!("{}:{}", cfg.bind, cfg.port)) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("tplc serve: bind {}:{}: {e}", cfg.bind, cfg.port);
            return 1;
        }
    };
    eprintln!(
        "tplc serve {}:{} → ctl {}{}{}",
        cfg.bind,
        cfg.port,
        cfg.ctl_sock,
        cfg.webui.as_ref().map(|w| format!(" webui {w}")).unwrap_or_default(),
        cfg.tplc_io.as_ref().map(|u| format!(" io {u}")).unwrap_or_default(),
    );
    install_stop_handler();
    l.set_nonblocking(true).ok();
    loop {
        if STOP.load(Ordering::Relaxed) {
            break;
        }
        match l.accept() {
            Ok((s, _)) => {
                let cfg2 = ServeCfg {
                    project_root: cfg.project_root.clone(),
                    ctl_sock: cfg.ctl_sock.clone(),
                    bind: cfg.bind.clone(),
                    port: cfg.port,
                    tplcc: cfg.tplcc.clone(),
                    tplccd: cfg.tplccd.clone(),
                    tplc_io: cfg.tplc_io.clone(),
                    libdirs: cfg.libdirs.clone(),
                    webui: cfg.webui.clone(),
                    packages: cfg.packages.clone(),
                    pkgs_for_io: cfg.pkgs_for_io.clone(),
                    spawn_io: false,
                    io_port_hint: 0,
                    sim: cfg.sim.clone(),
                    plc: cfg.plc.clone(),
                    ecpump: cfg.ecpump.clone(),
                    tools: cfg.tools.clone(),
                    simdir: cfg.simdir.clone(),
                };
                let cache = cache.clone();
                std::thread::spawn(move || handle_conn(s, &cfg2, &cache));
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            Err(_) => break,
        }
    }
    STOP.store(true, Ordering::Relaxed);
    if let Some(mut c) = cc_child.lock().unwrap().take() {
        let _ = c.kill();
    }
    if let Some(mut c) = io_child {
        let _ = c.kill();
    }
    0
}
