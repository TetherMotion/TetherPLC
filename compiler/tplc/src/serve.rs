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

/* ---------------- small helpers ---------------- */

fn b64e(data: &[u8]) -> String {
    const T: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut o = String::with_capacity((data.len() + 2) / 3 * 4);
    for c in data.chunks(3) {
        let v = ((c[0] as u32) << 16)
            | ((*c.get(1).unwrap_or(&0) as u32) << 8)
            | (*c.get(2).unwrap_or(&0) as u32);
        o.push(T[(v >> 18) as usize & 63] as char);
        o.push(T[(v >> 12) as usize & 63] as char);
        o.push(if c.len() > 1 { T[(v >> 6) as usize & 63] as char } else { '=' });
        o.push(if c.len() > 2 { T[v as usize & 63] as char } else { '=' });
    }
    o
}

fn b64d(s: &str) -> Vec<u8> {
    fn val(c: u8) -> i32 {
        match c {
            b'A'..=b'Z' => (c - b'A') as i32,
            b'a'..=b'z' => (c - b'a' + 26) as i32,
            b'0'..=b'9' => (c - b'0' + 52) as i32,
            b'+' => 62,
            b'/' => 63,
            _ => -1,
        }
    }
    let mut out = Vec::new();
    let mut acc = 0u32;
    let mut nbits = 0;
    for &c in s.as_bytes() {
        if c == b'=' {
            break;
        }
        let v = val(c);
        if v < 0 {
            continue;
        }
        acc = (acc << 6) | v as u32;
        nbits += 6;
        if nbits >= 8 {
            nbits -= 8;
            out.push((acc >> nbits) as u8);
        }
    }
    out
}

fn json_escape(s: &str) -> String {
    let mut o = String::with_capacity(s.len() + 8);
    for c in s.chars() {
        match c {
            '"' | '\\' => {
                o.push('\\');
                o.push(c);
            }
            '\n' => o.push_str("\\n"),
            '\r' => {}
            c => o.push(c),
        }
    }
    o
}

fn sanitize_name(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        if c == '/' || c == '\\' {
            out.clear();
            continue;
        }
        if c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '-' {
            out.push(c);
        }
    }
    if out.is_empty() || out.starts_with('.') {
        out = format!("src{out}");
    }
    out
}

fn read_full(s: &mut impl Read, buf: &mut [u8]) -> bool {
    s.read_exact(buf).is_ok()
}

fn reply(fd: &mut TcpStream, code: u16, text: &str, body: &[u8], ctype: &str) {
    let hdr = format!(
        "HTTP/1.1 {} {}\r\nContent-Length: {}\r\nContent-Type: {}\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Headers: content-type\r\nAccess-Control-Allow-Methods: GET,POST,OPTIONS\r\nConnection: close\r\n\r\n",
        code, text, body.len(), ctype
    );
    let _ = fd.write_all(hdr.as_bytes());
    let _ = fd.write_all(body);
}

fn reply_str(fd: &mut TcpStream, code: u16, text: &str, body: &str, ctype: &str) {
    reply(fd, code, text, body.as_bytes(), ctype);
}

fn uds_connect(path: &str, timeout_ms: u64) -> Option<std::os::unix::net::UnixStream> {
    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(timeout_ms);
    loop {
        match std::os::unix::net::UnixStream::connect(path) {
            Ok(s) => {
                s.set_read_timeout(Some(std::time::Duration::from_secs(5))).ok();
                return Some(s);
            }
            Err(e)
                if (e.kind() == std::io::ErrorKind::NotFound
                    || e.raw_os_error() == Some(libc::ECONNREFUSED))
                    && std::time::Instant::now() < deadline =>
            {
                std::thread::sleep(std::time::Duration::from_millis(20));
            }
            Err(_) => return None,
        }
    }
}

/* ---------------- ctl UDS helpers ---------------- */

fn ctl_frame(msg_type: u16, flags: u16, req: u64, payload: &[u8]) -> Vec<u8> {
    let mut fr = Vec::with_capacity(16 + payload.len());
    fr.extend_from_slice(&msg_type.to_le_bytes());
    fr.extend_from_slice(&flags.to_le_bytes());
    fr.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    fr.extend_from_slice(&req.to_le_bytes());
    fr.extend_from_slice(payload);
    fr
}

/// HELLO handshake on an already-connected ctl socket.
fn ctl_hello(s: &mut std::os::unix::net::UnixStream, name: &str) -> bool {
    let mut pl = Vec::with_capacity(8 + 2 + name.len());
    pl.extend_from_slice(&3u16.to_le_bytes()); // proto ver
    pl.extend_from_slice(&0u16.to_le_bytes());
    pl.extend_from_slice(&0u32.to_le_bytes()); // caps
    pl.extend_from_slice(&(name.len() as u16).to_le_bytes());
    pl.extend_from_slice(name.as_bytes());
    if s.write_all(&ctl_frame(op::HELLO, 0, 1, &pl)).is_err() {
        return false;
    }
    let mut hdr = [0u8; 16];
    if !read_full(s, &mut hdr) {
        return false;
    }
    let rlen = u32::from_le_bytes(hdr[4..8].try_into().unwrap()) as usize;
    if rlen > (1 << 20) {
        return false;
    }
    let mut pl = vec![0u8; rlen];
    if rlen > 0 && !read_full(s, &mut pl) {
        return false;
    }
    u16::from_le_bytes([hdr[0], hdr[1]]) == op::HELLO_ACK && hdr[3] & 0x80 == 0
}

/// SET_NOTIFY with the all-events mask; drains the ack.
fn ctl_notify_all(s: &mut std::os::unix::net::UnixStream) -> bool {
    if s
        .write_all(&ctl_frame(op::SET_NOTIFY, 0, 2, &0xFFFF_FFFFu32.to_le_bytes()))
        .is_err()
    {
        return false;
    }
    let mut hdr = [0u8; 16];
    if !read_full(s, &mut hdr) {
        return false;
    }
    let rlen = u32::from_le_bytes(hdr[4..8].try_into().unwrap()) as usize;
    if rlen > (1 << 20) {
        return false;
    }
    let mut pl = vec![0u8; rlen];
    if rlen > 0 && !read_full(s, &mut pl) {
        return false;
    }
    hdr[3] & 0x80 == 0
}

/* ---------------- compile ---------------- */

/// Binary src frame `[u16 count][u16 nameLen,name,u32 len,content]×N`
/// — the tplccd daemon's wire format.
fn encode_src_frame(files: &[(String, Vec<u8>)]) -> Vec<u8> {
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
fn do_compile(cfg: &ServeCfg, files: &[(String, Vec<u8>)], build: bool) -> serde_json::Value {
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

fn unique() -> u64 {
    use std::sync::atomic::{AtomicU64, Ordering};
    static C: AtomicU64 = AtomicU64::new(0);
    C.fetch_add(1, Ordering::Relaxed)
}

/* ---------------- pkgmap ---------------- */

/// Inject a DEPLOY_MAP section into a .tplc package; returns the new
/// package bytes or an error string.
fn do_pkgmap(cfg: &ServeCfg, pkg_b64: &str, map_text: &str) -> Result<Vec<u8>, String> {
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

/* ---------------- bus scan + sim run (WS meta-ops) ---------------- */

/// `bus_scan` — spawn `tplc-ec-pump --scan` against a peer/iface; the
/// discovered fleet lands in <simdir>/.fleet for GET /bus to serve.
/// Needs a link target from the request — EtherCAT discovery is
/// environment-gated, so failures return the pump's own error text.
fn do_bus_scan(cfg: &ServeCfg, req: &serde_json::Value) -> Result<serde_json::Value, String> {
    // cfg.ecpump is resolved once at session start; re-resolve lazily so a
    // pump built (or fixed) mid-session is picked up without a restart.
    let mut pump = cfg
        .ecpump
        .clone()
        .or_else(|| cfg.tools.find("tplc-ec-pump"))
        .ok_or_else(|| "tplc-ec-pump binary not found".to_string())?;
    let peer = req.get("peer").and_then(|v| v.as_str()).unwrap_or("");
    let iface = req.get("iface").and_then(|v| v.as_str()).unwrap_or("");
    let port = req.get("port").and_then(|v| v.as_u64()).unwrap_or(34980);
    let hz = req.get("hz").and_then(|v| v.as_u64()).unwrap_or(500);
    if peer.is_empty() && iface.is_empty() {
        return Err(
            "bus scan needs a link: {peer,port} for a UDP slave chain, or {iface} for a raw NIC"
                .into(),
        );
    }
    let _ = std::fs::create_dir_all(&cfg.simdir);
    let fleet_out = cfg.simdir.join(".fleet");
    let mut args: Vec<String> = if !iface.is_empty() {
        vec!["--iface".into(), iface.into()]
    } else {
        vec!["--peer".into(), peer.into(), "--port".into(), port.to_string()]
    };
    args.extend([
        "--simdir".into(),
        cfg.simdir.to_string_lossy().into_owned(),
        "--hz".into(),
        hz.to_string(),
        "--scan".into(),
        "--fleet-out".into(),
        fleet_out.to_string_lossy().into_owned(),
    ]);
    // Raw NIC scans need CAP_NET_RAW on the spawned pump. Work out the
    // route up front — direct (serve privileged or file cap on the
    // binary), via runec, or impossible — and explain the options
    // instead of running a doomed scan.
    let runec = crate::tools::which_on_path("runec");
    let via_runec = if !iface.is_empty() {
        match raw_cap_route(&pump, runec.as_deref()) {
            RawCapRoute::Direct => false,
            RawCapRoute::Runec => true,
            RawCapRoute::Missing => return Err(raw_cap_guidance(&pump, iface)),
        }
    } else {
        false
    };
    let spawn = |pump: &std::path::Path, runec: bool| {
        if runec {
            Command::new("runec").arg(pump).args(&args).output()
        } else {
            Command::new(pump).args(&args).output()
        }
    };
    let mut out = spawn(&pump, via_runec);
    // A stale resolved binary (an old pump reached via PATH/bindir before
    // build/ existed) rejects newer args, and a rebuilt tree can leave the
    // cached path dangling — in both cases re-resolve once and retry.
    // Retry even when re-resolution returns the same path: the file at it
    // may have been rebuilt mid-scan, and "the resolved path itself holds
    // a stale binary" is exactly the case this recovers from.
    let stale = match &out {
        Err(_) => true,
        Ok(o) => {
            !o.status.success()
                && String::from_utf8_lossy(&o.stderr).contains("unknown arg")
        }
    };
    if stale {
        if let Some(fresh) = cfg.tools.find("tplc-ec-pump") {
            pump = fresh;
            out = spawn(&pump, via_runec);
        }
    }
    // After all retries a pump that still rejects --scan stays a stale
    // build — say so instead of surfacing a bare arg-parse error that
    // looks like a bug in the scan itself. Rebuilding is a compile-time
    // concern (build.rs verifies freshness after cmake), not something
    // to attempt inside the serialized meta worker.
    if let Ok(o) = &out {
        if !o.status.success()
            && String::from_utf8_lossy(&o.stderr).contains("unknown arg")
        {
            return Err(format!(
                "tplc-ec-pump at {} is a stale build that doesn't support \
                 --scan — rebuild it:\n  cmake --build build --target tplc-ec-pump --parallel 8",
                pump.display()
            ));
        }
    }
    // Permission retry: the route probe said "direct" but the run still
    // hit CAP_NET_RAW (e.g. the file cap was wiped by a relink) —
    // escalate via runec when it's installed.
    if !via_runec {
        let perm_denied = matches!(&out, Ok(o) if !o.status.success()
            && String::from_utf8_lossy(&o.stderr).contains("CAP_NET_RAW"));
        if perm_denied && runec.is_some() {
            out = spawn(&pump, true);
        }
    }
    let out = out.map_err(|e| format!("tplc-ec-pump spawn failed ({e})"))?;
    let fleet = std::fs::read_to_string(&fleet_out).unwrap_or_default();
    Ok(serde_json::json!({
        "exit": out.status.code().unwrap_or(-1),
        "stderr": String::from_utf8_lossy(&out.stderr),
        "pump": pump.to_string_lossy(),
        "fleet": fleet,
    }))
}

/// `iface_list` — enumerate host NICs from /sys/class/net so the WebUI can
/// offer a real dropdown for the raw-EtherCAT transport instead of a
/// free-text field. Mirrors EtherCAT::HAL::resolveInterface: `auto` is the
/// interface name iff exactly one physical Ethernet NIC exists (the
/// "auto-select the sole physical interface" rule the Tether examples use).
/// Non-Linux hosts get an empty list — the UI falls back to free text.
fn do_iface_list(cfg: &ServeCfg, _req: &serde_json::Value) -> serde_json::Value {
    let mut ifaces: Vec<serde_json::Value> = Vec::new();
    let base = std::path::Path::new("/sys/class/net");
    if let Ok(rd) = std::fs::read_dir(base) {
        for e in rd.flatten() {
            let name = e.file_name().to_string_lossy().into_owned();
            let dir = e.path();
            let read = |f: &str| -> String {
                std::fs::read_to_string(dir.join(f))
                    .map(|s| s.trim().to_string())
                    .unwrap_or_default()
            };
            let arphrd: u32 = read("type").parse().unwrap_or(0);
            let virt = std::fs::canonicalize(&dir)
                .map(|p| p.to_string_lossy().contains("/devices/virtual/"))
                .unwrap_or(false);
            let wireless =
                dir.join("wireless").exists() || dir.join("phy80211").exists();
            let kind = if arphrd == 772 {
                "loopback"
            } else if wireless {
                "wireless"
            } else if !virt && arphrd == 1 {
                "ethernet"
            } else {
                "virtual"
            };
            ifaces.push(serde_json::json!({
                "name": name,
                "kind": kind,
                "state": read("operstate"),
                "mac": read("address"),
                "mtu": read("mtu"),
            }));
        }
    }
    let rank = |v: &serde_json::Value| match v["kind"].as_str().unwrap_or("") {
        "ethernet" => 0,
        "wireless" => 1,
        "virtual" => 2,
        _ => 3,
    };
    ifaces.sort_by(|a, b| {
        rank(a)
            .cmp(&rank(b))
            .then_with(|| a["name"].as_str().unwrap_or("").cmp(b["name"].as_str().unwrap_or("")))
    });
    let phys: Vec<&str> = ifaces
        .iter()
        .filter(|v| v["kind"] == "ethernet")
        .filter_map(|v| v["name"].as_str())
        .collect();
    serde_json::json!({
        "ifaces": ifaces,
        "auto": if phys.len() == 1 { phys[0] } else { "" },
        "raw_capable": raw_link_capable(cfg),
    })
}

/// How a spawned pump can get CAP_NET_RAW for a raw-NIC scan.
#[derive(Clone, Copy, PartialEq)]
enum RawCapRoute {
    /// This process (or the pump's file cap) already provides it.
    Direct,
    /// runec is on PATH — wrap the spawn to inherit ambient caps.
    Runec,
    /// No route — explain options to the user.
    Missing,
}

/// Raw EtherCAT (AF_PACKET) needs CAP_NET_RAW. "Capable" means any of:
/// this process can open AF_PACKET (privileged serve), the resolved pump
/// binary carries a cap_net_raw file cap (setcap install), or runec is
/// installed (serve wraps the spawn — children of an unprivileged serve
/// get nothing from the parent). Non-Linux hosts report false.
#[cfg(unix)]
fn raw_link_capable(cfg: &ServeCfg) -> bool {
    if af_packet_probe() {
        return true;
    }
    let Some(pump) = cfg
        .ecpump
        .clone()
        .or_else(|| cfg.tools.find("tplc-ec-pump"))
    else {
        return false;
    };
    raw_cap_route(&pump, crate::tools::which_on_path("runec").as_deref())
        != RawCapRoute::Missing
}

#[cfg(unix)]
fn raw_cap_route(
    pump: &std::path::Path,
    runec: Option<&std::path::Path>,
) -> RawCapRoute {
    if af_packet_probe() || file_cap_net_raw(pump) {
        RawCapRoute::Direct
    } else if runec.is_some() {
        RawCapRoute::Runec
    } else {
        RawCapRoute::Missing
    }
}

/// User-facing explanation when no CAP_NET_RAW route exists — the two
/// supported fixes (install runec / run serve under sudo) plus the
/// file-cap alternative and the privilege-free UDP path.
#[cfg(unix)]
fn raw_cap_guidance(pump: &std::path::Path, iface: &str) -> String {
    format!(
        "raw EtherCAT scan on '{iface}' needs CAP_NET_RAW (AF_PACKET) and no \
privilege route is available on this host.\n\
Options:\n\
  (a) install runec (recommended — setuid helper that grants just \
CAP_NET_RAW/CAP_NET_ADMIN/CAP_SYS_NICE):\n\
      git clone https://github.com/ulikoehler/runec && cd runec\n\
      sudo apt install build-essential libcap-dev && make && sudo make install\n\
  (b) start 'tplc serve' via sudo so spawned masters inherit privileges\n\
  (c) sudo setcap cap_net_raw+ep {pump}\n\
or switch to EtherCAT/UDP — it needs no elevated privileges.",
        pump = pump.display()
    )
}

/// Try to open an AF_PACKET socket — succeeds iff this process has
/// CAP_NET_RAW. Errors other than EPERM/EACCES don't prove the cap is
/// missing, so they count as capable.
#[cfg(unix)]
fn af_packet_probe() -> bool {
    // ETH_P_ALL = 0x0003, AF_PACKET = 17, SOCK_RAW = 3, SOCK_CLOEXEC = 0o2000000
    let fd = unsafe { libc::socket(17, 3 | 0o2000000, (0x0003u16).to_be() as i32) };
    if fd >= 0 {
        unsafe { libc::close(fd) };
        true
    } else {
        let e = std::io::Error::last_os_error().raw_os_error().unwrap_or(0);
        e != 1 && e != 13
    }
}

/// security.capability xattr on @p path: check CAP_NET_RAW (13) in the
/// permitted set. vfs_cap_data v2/v3 layout: u32 magic, then two u32
/// {permitted, inheritable} pairs (v3 appends a u32 rootid we ignore).
#[cfg(target_os = "linux")]
fn file_cap_net_raw(path: &std::path::Path) -> bool {
    use std::os::unix::ffi::OsStrExt;
    let mut buf = [0u8; 24];
    let cpath = match std::ffi::CString::new(path.as_os_str().as_bytes()) {
        Ok(c) => c,
        Err(_) => return false,
    };
    let n = unsafe {
        libc::getxattr(
            cpath.as_ptr(),
            c"security.capability".as_ptr(),
            buf.as_mut_ptr().cast(),
            buf.len(),
        )
    };
    if n < 12 {
        return false;
    }
    let rev = u32::from_le_bytes(buf[0..4].try_into().unwrap()) & 0xFF00_0000;
    if rev != 0x0100_0000 && rev != 0x0200_0000 && rev != 0x0300_0000 {
        return false;
    }
    let permitted_lo = u32::from_le_bytes(buf[4..8].try_into().unwrap());
    permitted_lo & (1 << 13) != 0
}
#[cfg(all(unix, not(target_os = "linux")))]
fn file_cap_net_raw(_path: &std::path::Path) -> bool {
    false
}
#[cfg(not(unix))]
fn raw_link_capable(_cfg: &ServeCfg) -> bool {
    false
}

/// `sim_run` — compile the posted sources to a package, then run
/// `tether-plc-sim --steps` against it. Returns the sim's combined
/// output (step results + trace) and exit code; the pass/fail verdict
/// is the sim's own "N pass, M fail" trailer.
fn do_sim_run(cfg: &ServeCfg, req: &serde_json::Value) -> Result<serde_json::Value, String> {
    let Some(sim) = &cfg.sim else {
        return Err("tether-plc-sim binary not found".into());
    };
    let plc = cfg.plc.clone().unwrap_or_else(|| PathBuf::from("tether-plc"));
    let mut files: Vec<(String, Vec<u8>)> = Vec::new();
    for f in req
        .get("files")
        .and_then(|v| v.as_array())
        .into_iter()
        .flatten()
        .take(64)
    {
        files.push((
            sanitize_name(f.get("name").and_then(|v| v.as_str()).unwrap_or("")),
            f.get("content")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .as_bytes()
                .to_vec(),
        ));
    }
    files.retain(|(n, _)| !n.is_empty());
    if files.is_empty() {
        return Err("no sources to simulate".into());
    }
    let compiled = do_compile(cfg, &files, true);
    let pkg_b64 = compiled.get("package").and_then(|v| v.as_str()).unwrap_or("");
    if pkg_b64.is_empty() {
        return Ok(serde_json::json!({
            "exit": -1,
            "compile": compiled.get("compile").cloned().unwrap_or(serde_json::Value::Null),
            "output": "compile failed — see diagnostics",
        }));
    }
    let dir = std::env::temp_dir().join(format!("tplc_sim_{}_{}", std::process::id(), unique()));
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let pkg = dir.join("prog.tplc");
    let steps = dir.join("steps.txt");
    std::fs::write(&pkg, b64d(pkg_b64)).map_err(|e| e.to_string())?;
    std::fs::write(
        &steps,
        req.get("steps").and_then(|v| v.as_str()).unwrap_or(""),
    )
    .map_err(|e| e.to_string())?;
    let image = format!("/tplc_sim_{}_{}", std::process::id(), unique());
    let in_sz = req.get("in").and_then(|v| v.as_u64()).unwrap_or(64);
    let out_sz = req.get("out").and_then(|v| v.as_u64()).unwrap_or(64);
    let out = Command::new(sim)
        .arg("--image")
        .arg(&image)
        .arg("--package")
        .arg(&pkg)
        .arg("--steps")
        .arg(&steps)
        .arg("--plc")
        .arg(&plc)
        .arg("--in")
        .arg(in_sz.to_string())
        .arg("--out")
        .arg(out_sz.to_string())
        .output()
        .map_err(|e| format!("tether-plc-sim spawn failed: {e}"))?;
    /* sim creates a POSIX shm at /dev/shm/<name> — best-effort cleanup;
       the spawned tether-plc unlinks on exit when it gets there first */
    let _ =
        std::fs::remove_file(std::path::Path::new("/dev/shm").join(image.trim_start_matches('/')));
    let _ = std::fs::remove_dir_all(&dir);
    Ok(serde_json::json!({
        "exit": out.status.code().unwrap_or(-1),
        "output": format!("{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)),
    }))
}

fn collect_lib_files(dir: &str) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if let Ok(rd) = std::fs::read_dir(dir) {
        for e in rd.flatten() {
            let p = e.path();
            let n = p.file_name().and_then(|x| x.to_str()).unwrap_or("");
            if n.ends_with(".st") || n.ends_with(".tplc") {
                out.push(p);
            }
        }
    }
    out
}

/// Merged `tplcc blocks --json` catalog over every --libdir; returns
/// the `{"libs":[…]}` JSON text (cached).
fn do_blocks(cfg: &ServeCfg, refresh: bool, cache: &Arc<Mutex<(String, bool)>>) -> String {
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
fn do_libsrc(cfg: &ServeCfg, lib: &str, file: &str) -> Option<Vec<u8>> {
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

/* ---------------- /tether-io (WebSocket → TCP, SLIP) ---------------- */

fn sha1(data: &[u8]) -> [u8; 20] {
    // FIPS 180-1, ported from the C++ proxy
    let mut h: [u32; 5] = [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0];
    let bitlen = (data.len() as u64) * 8;
    let mut m = data.to_vec();
    m.push(0x80);
    while m.len() % 64 != 56 {
        m.push(0);
    }
    m.extend_from_slice(&bitlen.to_be_bytes());
    for chunk in m.chunks(64) {
        let mut w = [0u32; 80];
        for i in 0..16 {
            w[i] = u32::from_be_bytes(chunk[i * 4..i * 4 + 4].try_into().unwrap());
        }
        for i in 16..80 {
            let x = w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16];
            w[i] = x.rotate_left(1);
        }
        let (mut a, mut b, mut c, mut d, mut e) = (h[0], h[1], h[2], h[3], h[4]);
        for i in 0..80 {
            let (f, k) = match i {
                0..=19 => ((b & c) | (!b & d), 0x5A827999u32),
                20..=39 => (b ^ c ^ d, 0x6ED9EBA1),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1BBCDC),
                _ => (b ^ c ^ d, 0xCA62C1D6),
            };
            let t = a.rotate_left(5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(k)
                .wrapping_add(w[i]);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = t;
        }
        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
    }
    let mut out = [0u8; 20];
    for i in 0..5 {
        out[i * 4..i * 4 + 4].copy_from_slice(&h[i].to_be_bytes());
    }
    out
}

fn ws_read_frame(s: &mut TcpStream) -> Option<(u8, Vec<u8>, bool)> {
    let mut h = [0u8; 2];
    s.read_exact(&mut h).ok()?;
    let fin = h[0] & 0x80 != 0;
    let op = h[0] & 0x0F;
    let masked = h[1] & 0x80 != 0;
    let mut len = (h[1] & 0x7F) as u64;
    if len == 126 {
        let mut b = [0u8; 2];
        s.read_exact(&mut b).ok()?;
        len = u16::from_be_bytes(b) as u64;
    } else if len == 127 {
        let mut b = [0u8; 8];
        s.read_exact(&mut b).ok()?;
        len = u64::from_be_bytes(b);
    }
    if len > (64 << 20) {
        return None;
    }
    let mut mask = [0u8; 4];
    if masked {
        s.read_exact(&mut mask).ok()?;
    }
    let mut payload = vec![0u8; len as usize];
    s.read_exact(&mut payload).ok()?;
    if masked {
        for (i, b) in payload.iter_mut().enumerate() {
            *b ^= mask[i & 3];
        }
    }
    Some((op, payload, fin))
}

fn ws_send_frame(s: &mut TcpStream, op: u8, payload: &[u8]) -> bool {
    let n = payload.len();
    let mut hdr = vec![0x80 | op];
    if n < 126 {
        hdr.push(n as u8);
    } else if n <= 0xFFFF {
        hdr.push(126);
        hdr.extend_from_slice(&(n as u16).to_be_bytes());
    } else {
        hdr.push(127);
        hdr.extend_from_slice(&(n as u64).to_be_bytes());
    }
    s.write_all(&hdr).is_ok() && (n == 0 || s.write_all(payload).is_ok())
}

const SLIP_END: u8 = 0xC0;
const SLIP_ESC: u8 = 0xDB;
const SLIP_ESC_END: u8 = 0xDC;
const SLIP_ESC_ESC: u8 = 0xDD;
const SLIP_MAX: usize = 1 << 20;

fn slip_encode(p: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(p.len() + p.len() / 8 + 1);
    for &b in p {
        match b {
            SLIP_END => out.extend_from_slice(&[SLIP_ESC, SLIP_ESC_END]),
            SLIP_ESC => out.extend_from_slice(&[SLIP_ESC, SLIP_ESC_ESC]),
            _ => out.push(b),
        }
    }
    out.push(SLIP_END);
    out
}

struct SlipDec {
    msg: Vec<u8>,
    in_esc: bool,
    discard: bool,
}
impl SlipDec {
    fn new() -> Self {
        SlipDec { msg: vec![], in_esc: false, discard: false }
    }
    /// feed bytes; complete messages pushed to `out`
    fn feed(&mut self, p: &[u8], out: &mut Vec<Vec<u8>>) {
        for &b in p {
            if self.discard {
                if b == SLIP_END {
                    self.discard = false;
                    self.msg.clear();
                    self.in_esc = false;
                }
                continue;
            }
            if self.in_esc {
                self.in_esc = false;
                match b {
                    SLIP_ESC_END => self.msg.push(SLIP_END),
                    SLIP_ESC_ESC => self.msg.push(SLIP_ESC),
                    _ => {
                        self.msg.clear();
                        self.discard = true;
                    }
                }
                continue;
            }
            if b == SLIP_END {
                if !self.msg.is_empty() && self.msg.len() <= SLIP_MAX {
                    out.push(std::mem::take(&mut self.msg));
                } else {
                    self.msg.clear();
                }
            } else if b == SLIP_ESC {
                self.in_esc = true;
            } else {
                self.msg.push(b);
                if self.msg.len() > SLIP_MAX {
                    self.msg.clear();
                    self.discard = true;
                }
            }
        }
    }
}

fn tcp_connect(hostport: &str) -> Option<TcpStream> {
    use std::net::ToSocketAddrs;
    hostport.to_socket_addrs().ok()?.find_map(|a| TcpStream::connect(a).ok())
}

fn handle_tetherio(mut cfd: TcpStream, head: String, upstream: String) {
    std::thread::spawn(move || {
        // Sec-WebSocket-Key → accept digest
        let key = head
            .lines()
            .find(|l| l.to_lowercase().starts_with("sec-websocket-key:"))
            .and_then(|l| l.split_once(':').map(|x| x.1.trim().to_string()))
            .unwrap_or_default();
        if key.is_empty() {
            return;
        }
        let magic = format!("{key}258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
        let accept = b64e(&sha1(magic.as_bytes()));
        let resp = format!(
            "HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {accept}\r\n\r\n"
        );
        if cfd.write_all(resp.as_bytes()).is_err() {
            return;
        }
        let Some(tfd) = tcp_connect(&upstream) else {
            let pl = [0x03u8, 0xE8];
            let _ = ws_send_frame(&mut cfd, 0x8, &pl);
            return;
        };
        let tfd_w = match tfd.try_clone() {
            Ok(s) => s,
            Err(_) => return,
        };
        let mut cfd_r = match cfd.try_clone() {
            Ok(s) => s,
            Err(_) => return,
        };
        // reader thread: WS → SLIP → TCP
        let mut tfd_w2 = match tfd_w.try_clone() {
            Ok(s) => s,
            Err(_) => return,
        };
        let reader = std::thread::spawn(move || {
            let mut wsmsg: Vec<u8> = Vec::new();
            loop {
                let Some((op, payload, fin)) = ws_read_frame(&mut cfd_r) else { break };
                match op {
                    0x8 => {
                        let _ = ws_send_frame(&mut cfd_r, 0x8, &payload);
                        break;
                    }
                    0x9 => {
                        let _ = ws_send_frame(&mut cfd_r, 0xA, &payload);
                    }
                    0xA => {}
                    0x0 | 0x1 | 0x2 => {
                        if op != 0x0 {
                            wsmsg.clear();
                        }
                        wsmsg.extend_from_slice(&payload);
                        if !fin {
                            continue;
                        }
                        let pkt = slip_encode(&wsmsg);
                        wsmsg.clear();
                        if tfd_w2.write_all(&pkt).is_err() {
                            break;
                        }
                    }
                    _ => {}
                }
            }
            // unblock the main pump
            let _ = tfd_w.shutdown(std::net::Shutdown::Both);
        });
        // main: TCP → SLIP decode → WS binary frames
        let mut tfd_r = tfd;
        tfd_r.set_read_timeout(Some(std::time::Duration::from_millis(500))).ok();
        let mut slip = SlipDec::new();
        let mut buf = [0u8; 16384];
        let mut msgs: Vec<Vec<u8>> = Vec::new();
        loop {
            match tfd_r.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    msgs.clear();
                    slip.feed(&buf[..n], &mut msgs);
                    for m in msgs.drain(..) {
                        if !ws_send_frame(&mut cfd, 0x2, &m) {
                            let _ = reader.join();
                            return;
                        }
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock
                    || e.kind() == std::io::ErrorKind::TimedOut => {}
                Err(_) => break,
            }
            if STOP.load(Ordering::Relaxed) {
                break;
            }
        }
        let _ = cfd.shutdown(std::net::Shutdown::Both);
        let _ = reader.join();
    });
}

/* ---------------- /ws — the primary UI channel ----------------
 *
 * One WebSocket per client, backed by a dedicated ctl UDS connection
 * (HELLO + SET_NOTIFY all-events at connect).
 *
 *   binary message → verbatim ctl frame forwarded to the runtime;
 *                    every reply/event frame comes back as a binary
 *                    message (client demuxes by req_id / F_EVENT).
 *   text message   → JSON meta-op, replied as a text message:
 *     {"id":N,"op":"compile","build":bool,"files":[{name,content}]}
 *     {"id":N,"op":"blocks","refresh":bool}
 *     {"id":N,"op":"libsrc","lib":"x","file":"y.st"}
 *     {"id":N,"op":"pkgmap","package":"<b64>","map":"<text>"}
 *     → {"id":N,"ok":true,"data":…}  or  {"id":N,"ok":false,"error":"…"}
 */

/// `Upgrade:` header presence, case-insensitively — HTTP field names are
/// case-insensitive (RFC 9110 §5.1); browsers send `Upgrade:` but node's
/// undici WebSocket lowercases to `upgrade:`.
fn wants_ws_upgrade(head: &str) -> bool {
    head.lines().any(|l| l.to_lowercase().starts_with("upgrade:"))
}

fn ws_accept(head: &str) -> Option<String> {
    let key = head
        .lines()
        .find(|l| l.to_lowercase().starts_with("sec-websocket-key:"))
        .and_then(|l| l.split_once(':').map(|x| x.1.trim().to_string()))?;
    if key.is_empty() {
        return None;
    }
    let magic = format!("{key}258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
    Some(b64e(&sha1(magic.as_bytes())))
}

fn ws_handshake(cfd: &mut TcpStream, head: &str) -> bool {
    let Some(accept) = ws_accept(head) else { return false };
    let resp = format!(
        "HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {accept}\r\n\r\n"
    );
    cfd.write_all(resp.as_bytes()).is_ok()
}

/* ---------------- project & git meta-ops ----------------
   WS ops (JSON {"id":N,"op":"…"} → {"id":N,"ok":bool,"data"|"error"}):
     project_get / projects / project_open / project_save
     project_files / project_file_get / project_file_put / project_file_delete
     git_status / git_log / git_diff / git_branch / git_commit
   The active project root lives in cfg.project_root (Arc<Mutex>) so
   project_open re-roots every connection. */

fn proj_root(cfg: &ServeCfg) -> Option<PathBuf> {
    cfg.project_root.lock().ok().and_then(|g| g.clone())
}

fn pj_err(m: &str) -> serde_json::Value {
    serde_json::json!({"ok": false, "error": m})
}

fn pj_ok(v: serde_json::Value) -> serde_json::Value {
    serde_json::json!({"ok": true, "data": v})
}

fn project_manifest_json(p: &crate::project::Project) -> serde_json::Value {
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
fn project_meta(cfg: &ServeCfg, op: &str, req: &serde_json::Value) -> Option<serde_json::Value> {
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

/// JSON meta-op dispatch — runs on the meta worker thread.
fn meta_dispatch(
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

/// One ctl frame carrying a TplcCtlError payload (flags bit15).
fn ctl_error_frame(req_type: u16, req: u64, msg: &str) -> Vec<u8> {
    let mut p = Vec::with_capacity(10 + msg.len());
    p.extend_from_slice(&(-1i32).to_le_bytes()); // TPLC_E_IO
    p.extend_from_slice(&0u32.to_le_bytes()); // subcode
    p.extend_from_slice(&(msg.len() as u16).to_le_bytes());
    p.extend_from_slice(msg.as_bytes());
    ctl_frame(req_type, 0x8000, req, &p)
}

fn handle_ws(mut cfd: TcpStream, head: String, cfg: &ServeCfg, cache: &Arc<Mutex<(String, bool)>>) {
    // ctl backend is best-effort: meta-ops (compile/blocks/…) work
    // without a runtime; ctl frames get a synthesized F_ERROR reply.
    // Handshake + meta worker come up FIRST — uds_connect retries a
    // missing ctl socket for up to 2s and must not stall the ws upgrade
    // or the meta-ops that never needed the runtime (iface_list took a
    // 2s+ hit on every connect before this reorder).
    if !ws_handshake(&mut cfd, &head) {
        return;
    }
    cfd.set_read_timeout(None).ok();
    cfd.set_nodelay(true).ok();

    let cfg2 = cfg.clone();
    let cache2 = cache.clone();
    let cfd_w = match cfd.try_clone() {
        Ok(s) => Arc::new(Mutex::new(s)),
        Err(_) => return,
    };
    let (tx, rx) = std::sync::mpsc::channel::<serde_json::Value>();

    // meta-op worker: JSON text requests → JSON text replies
    {
        let w = cfd_w.clone();
        std::thread::spawn(move || {
            while let Ok(req) = rx.recv() {
                let id = req.get("id").cloned().unwrap_or(serde_json::Value::Null);
                let mut rep = meta_dispatch(&cfg2, &cache2, &req);
                rep["id"] = id;
                let text = rep.to_string();
                let mut g = w.lock().unwrap();
                if !ws_send_frame(&mut g, 0x1, text.as_bytes()) {
                    break;
                }
            }
        });
    }

    /* ctl UDS connects in the background — set OnceLock when the retry
       budget resolves either way. ctl-frame writers wait on it (same
       ≤2s semantics as before); meta-ops and the ws upgrade don't. */
    let uds_slot = Arc::new(std::sync::OnceLock::<Mutex<Option<UnixStream>>>::new());
    {
        let slot = uds_slot.clone();
        let sock = cfg.ctl_sock.clone();
        let w = cfd_w.clone();
        std::thread::spawn(move || {
            let mut uds = uds_connect(&sock, 2000);
            if let Some(s) = uds.as_mut() {
                s.set_read_timeout(None).ok();
                if !ctl_hello(s, "tplc-webui") || !ctl_notify_all(s) {
                    uds = None;
                }
            }
            // UDS → WS: every ctl frame (replies + F_EVENT pushes) →
            // binary msg — spawned once the stream exists.
            if let Some(uds0) = uds {
                if let Ok(mut uds_r) = uds0.try_clone() {
                    uds_r.set_read_timeout(None).ok();
                    let w2 = w.clone();
                    std::thread::spawn(move || {
                        let mut hdr = [0u8; 16];
                        loop {
                            if !read_full(&mut uds_r, &mut hdr) {
                                break;
                            }
                            let len =
                                u32::from_le_bytes(hdr[4..8].try_into().unwrap()) as usize;
                            if len > 8 * 1024 * 1024 {
                                break;
                            }
                            let mut fr = hdr.to_vec();
                            fr.resize(16 + len, 0);
                            if len > 0 && !read_full(&mut uds_r, &mut fr[16..]) {
                                break;
                            }
                            let mut g = w2.lock().unwrap();
                            if !ws_send_frame(&mut g, 0x2, &fr) {
                                break;
                            }
                        }
                        // ctl went away — drop the ws too so the client reconnects
                        let _ = uds_r.shutdown(std::net::Shutdown::Both);
                    });
                }
                let _ = slot.set(Mutex::new(Some(uds0)));
            } else {
                let _ = slot.set(Mutex::new(None));
            }
        });
    }

    // WS → UDS (this thread): binary → ctl frame, text → meta channel
    let mut wsmsg: Vec<u8> = Vec::new();
    let mut ws_is_text = false;
    loop {
        let Some((op, payload, fin)) = ws_read_frame(&mut cfd) else { break };
        match op {
            0x8 => {
                let _ = ws_send_frame(&mut cfd, 0x8, &payload);
                break;
            }
            0x9 => {
                let _ = ws_send_frame(&mut cfd, 0xA, &payload);
            }
            0xA => {}
            0x0 | 0x1 | 0x2 => {
                if op != 0x0 {
                    wsmsg.clear();
                    ws_is_text = op == 0x1;
                }
                wsmsg.extend_from_slice(&payload);
                if !fin {
                    continue;
                }
                let msg = std::mem::take(&mut wsmsg);
                if ws_is_text {
                    if let Ok(v) = serde_json::from_slice::<serde_json::Value>(&msg) {
                        if tx.send(v).is_err() {
                            break;
                        }
                    }
                } else if msg.len() >= 16 {
                    // wait (bounded by the connect budget) for the ctl
                    // backend — the connect thread sets the slot whether
                    // it landed a stream or exhausted retries.
                    let wrote = match uds_slot.wait().lock().unwrap().as_mut() {
                        Some(u) => u.write_all(&msg).is_ok(),
                        None => false,
                    };
                    if !wrote {
                        // no runtime: parse req id and error the call
                        let t = u16::from_le_bytes([msg[0], msg[1]]);
                        let req = u64::from_le_bytes(msg[8..16].try_into().unwrap());
                        let err = ctl_error_frame(t, req, "ctl backend down");
                        let mut g = cfd_w.lock().unwrap();
                        if !ws_send_frame(&mut g, 0x2, &err) {
                            break;
                        }
                    }
                }
            }
            _ => {}
        }
        if STOP.load(Ordering::Relaxed) {
            break;
        }
    }
    if let Some(mu) = uds_slot.get() {
        if let Some(u) = mu.lock().unwrap().take() {
            let _ = u.shutdown(std::net::Shutdown::Both);
        }
    }
}

/* ---------------- static files ---------------- */

fn mime_for(path: &Path) -> &'static str {
    match path.extension().and_then(|x| x.to_str()).unwrap_or("") {
        "html" => "text/html; charset=utf-8",
        "js" | "mjs" => "text/javascript; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "json" | "map" | "webmanifest" => "application/json",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "ico" => "image/x-icon",
        "wasm" => "application/wasm",
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        "ttf" => "font/ttf",
        "txt" | "md" | "st" => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}

fn pct_decode(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'%' && i + 2 < b.len() {
            if let Ok(h) = u8::from_str_radix(&s[i + 1..i + 3], 16) {
                out.push(h);
                i += 3;
                continue;
            }
        }
        out.push(if b[i] == b'+' { b' ' } else { b[i] });
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn serve_static(fd: &mut TcpStream, webui: &str, path: &str) {
    // sanitize: no .., decode %XX
    let rel = pct_decode(path.trim_start_matches('/'));
    if rel.contains("..") {
        reply_str(fd, 403, "Forbidden", "bad path", "text/plain");
        return;
    }
    let root = Path::new(webui);
    let mut fp = root.join(&rel);
    if fp.is_dir() {
        fp = fp.join("index.html");
    }
    let mut body = std::fs::read(&fp).ok();
    if body.is_none() {
        // SPA fallback: unknown non-file paths → index.html
        let last = rel.rsplit('/').next().unwrap_or("");
        if !last.contains('.') {
            body = std::fs::read(root.join("index.html")).ok();
        }
    }
    match body {
        Some(b) => reply(fd, 200, "OK", &b, mime_for(&fp)),
        None => reply_str(fd, 404, "Not Found", "not found", "text/plain"),
    }
}

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
