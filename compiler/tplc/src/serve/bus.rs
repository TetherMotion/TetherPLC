use super::*;

/* ---------------- bus scan + sim run (WS meta-ops) ---------------- */

/// `bus_scan` — spawn `tplc-ec-pump --scan` against a peer/iface; the
/// discovered fleet lands in <simdir>/.fleet for GET /bus to serve.
/// Needs a link target from the request — EtherCAT discovery is
/// environment-gated, so failures return the pump's own error text.
pub(crate) fn do_bus_scan(cfg: &ServeCfg, req: &serde_json::Value) -> Result<serde_json::Value, String> {
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
pub(crate) fn do_iface_list(cfg: &ServeCfg, _req: &serde_json::Value) -> serde_json::Value {
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
pub(crate) enum RawCapRoute {
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
pub(crate) fn raw_link_capable(cfg: &ServeCfg) -> bool {
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
pub(crate) fn raw_cap_route(
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
pub(crate) fn raw_cap_guidance(pump: &std::path::Path, iface: &str) -> String {
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
pub(crate) fn af_packet_probe() -> bool {
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
pub(crate) fn file_cap_net_raw(path: &std::path::Path) -> bool {
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
pub(crate) fn file_cap_net_raw(_path: &std::path::Path) -> bool {
    false
}
#[cfg(not(unix))]
pub(crate) fn raw_link_capable(_cfg: &ServeCfg) -> bool {
    false
}

/// `sim_run` — compile the posted sources to a package, then run
/// `tether-plc-sim --steps` against it. Returns the sim's combined
/// output (step results + trace) and exit code; the pass/fail verdict
/// is the sim's own "N pass, M fail" trailer.
pub(crate) fn do_sim_run(cfg: &ServeCfg, req: &serde_json::Value) -> Result<serde_json::Value, String> {
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

pub(crate) fn collect_lib_files(dir: &str) -> Vec<PathBuf> {
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
