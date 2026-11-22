//! Composite commands — the "use them all in unison" layer.
//!
//!   tplc run <srcs…|pkg.tplc>   compile-if-needed → image → tether-plc
//!   tplc sim <srcs…|pkg> --steps F   → tether-plc-sim replay
//!   tplc dev [srcs…|pkg]         runtime + tplc-io + webserver + cc daemon
//!   tplc down [name]             RT_SHUTDOWN a session's runtime
//!   tplc sessions                list live sessions

use crate::ctl::op;
use crate::session::{self, Session};
use crate::tools::Tools;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

pub struct RunOpts {
    pub name: String,
    pub image: Option<String>,
    pub in_size: u32,
    pub out_size: u32,
    pub ctl: Option<String>,
    pub workdir: Option<String>,
    pub libs: Vec<String>,
    pub tplcc_args: Vec<String>, // extra tplcc build flags (--task-period-ns…)
    pub plc_args: Vec<String>,   // verbatim tether-plc args after `--`
    pub steps: Option<String>,   // sim only
    pub shadow: Vec<String>,     // sim only
    pub dump_q: Option<String>,
    pub settle_ms: Option<u32>,
    pub hold_ms: Option<u32>,
    pub scan: bool,
    /// `--project DIR` — project root override; otherwise discovered
    /// by walking up from cwd for `.tplc/project.json`.
    pub project: Option<String>,
    /// True when the user passed `--name` explicitly (project name
    /// must not override it).
    pub name_explicit: bool,
    pub in_explicit: bool,
    pub out_explicit: bool,
}

fn parse_common(args: &[String]) -> (RunOpts, Vec<String>, Vec<String>) {
    // returns (opts, positional srcs, passthrough-after---)
    let mut o = RunOpts {
        name: "main".into(),
        image: None,
        in_size: 64,
        out_size: 64,
        ctl: None,
        workdir: None,
        libs: vec![],
        tplcc_args: vec![],
        plc_args: vec![],
        steps: None,
        shadow: vec![],
        dump_q: None,
        settle_ms: None,
        hold_ms: None,
        scan: false,
        project: None,
        name_explicit: false,
        in_explicit: false,
        out_explicit: false,
    };
    let mut pos = Vec::new();
    let mut i = 0;
    let mut passthrough = false;
    while i < args.len() {
        let a = &args[i];
        if passthrough {
            o.plc_args.push(a.clone());
            i += 1;
            continue;
        }
        let val = |i: &mut usize| -> String {
            *i += 1;
            args.get(*i).cloned().unwrap_or_default()
        };
        match a.as_str() {
            "--" => passthrough = true,
            "--name" => {
                o.name = val(&mut i);
                o.name_explicit = true;
            }
            "--project" => o.project = Some(val(&mut i)),
            "--image" => o.image = Some(val(&mut i)),
            "--in" => {
                o.in_size = val(&mut i).parse().unwrap_or(64);
                o.in_explicit = true;
            }
            "--out" => {
                o.out_size = val(&mut i).parse().unwrap_or(64);
                o.out_explicit = true;
            }
            "--ctl" => o.ctl = Some(val(&mut i)),
            "--workdir" => o.workdir = Some(val(&mut i)),
            "--lib" => o.libs.push(val(&mut i)),
            "--steps" => o.steps = Some(val(&mut i)),
            "--shadow" => o.shadow.push(val(&mut i)),
            "--dump-q" => o.dump_q = Some(val(&mut i)),
            "--settle-ms" => o.settle_ms = Some(val(&mut i).parse().unwrap_or(0)),
            "--hold-ms" => o.hold_ms = Some(val(&mut i).parse().unwrap_or(0)),
            "--scan" => o.scan = true,
            // tplcc build options forwarded verbatim
            "--task-period-ns" | "--checked-arith" | "--target" | "--pic-blob"
            | "--change-against" | "--library" => {
                o.tplcc_args.push(a.clone());
                if a != "--checked-arith" && a != "--library" && a != "--pic-blob" {
                    o.tplcc_args.push(val(&mut i));
                }
            }
            s if s.starts_with('-') => {
                eprintln!("tplc: unknown option {s}");
                std::process::exit(2);
            }
            _ => pos.push(a.clone()),
        }
        i += 1;
    }
    (o, pos, vec![])
}

/// Resolve the active project (explicit `--project DIR`, else walk up
/// from cwd for `.tplc/project.json`) and fold its manifest into the
/// run options: default sources when no positional args were given,
/// manifest libs, task period, image sizes, workdir, session name.
/// Returns the project when one is active.
fn apply_project(o: &mut RunOpts, pos: &mut Vec<String>) -> Option<crate::project::Project> {
    let proj = match &o.project {
        Some(d) => {
            let dir = PathBuf::from(d);
            // --project DIR adopts the directory: load its manifest or
            // create one (scaffold + git init) when absent.
            match crate::project::discover(&dir) {
                Some(root) => crate::project::load(&root).ok(),
                None => crate::project::init(&dir, None).ok(),
            }
        }
        None => crate::project::open(None).ok(),
    }?;
    let root = proj.root.clone();
    let m = proj.manifest.clone();

    if pos.is_empty() && !m.sources.is_empty() {
        let mut found = false;
        for s in &m.sources {
            let p = root.join(s);
            if p.is_file() {
                pos.push(p.display().to_string());
                found = true;
            }
        }
        if !found {
            eprintln!(
                "tplc: project `{}` lists sources but none exist on disk",
                proj.display_name()
            );
        }
    }
    for l in &m.libs {
        o.libs.push(root.join(l).display().to_string());
    }
    if let Some(ns) = m.task_period_ns {
        if !o.tplcc_args.iter().any(|a| a == "--task-period-ns") {
            o.tplcc_args.push("--task-period-ns".into());
            o.tplcc_args.push(ns.to_string());
        }
    }
    if o.workdir.is_none() {
        o.workdir = Some(proj.build_dir().display().to_string());
    }
    if !o.in_explicit && m.image_in != 0 {
        o.in_size = m.image_in;
    }
    if !o.out_explicit && m.image_out != 0 {
        o.out_size = m.image_out;
    }
    if !o.name_explicit {
        // sanitize for shm/ctl/session names
        o.name = proj
            .display_name()
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                    c
                } else {
                    '-'
                }
            })
            .collect();
    }
    crate::project::record_recent(&root);
    Some(proj)
}

/// Compile sources to a .tplc package, or pass a .tplc through.
/// Returns the package path.
fn ensure_package(tools: &Tools, o: &RunOpts, pos: &[String]) -> Result<PathBuf, String> {
    if pos.len() == 1 && pos[0].ends_with(".tplc") {
        return Ok(PathBuf::from(&pos[0]));
    }
    if pos.is_empty() {
        return Err("no source or package given".into());
    }
    let tplcc = tools.require("tplcc")?;
    let workdir = match &o.workdir {
        Some(d) => PathBuf::from(d),
        None => std::env::temp_dir().join(format!("tplc-{}", o.name)),
    };
    std::fs::create_dir_all(&workdir).map_err(|e| format!("workdir: {e}"))?;
    let pkg = workdir.join("app.tplc");
    let mut cmd = Command::new(&tplcc);
    cmd.arg("build").args(pos).arg("-o").arg(&pkg);
    for l in &o.libs {
        cmd.arg("--lib").arg(l);
    }
    // installed libraries auto-resolve: scan for undeclared FB_*/F_*
    // refs and pull what satisfies them — no manual attach needed
    for d in crate::default_libdirs() {
        cmd.arg("--libdir").arg(d);
    }
    cmd.args(&o.tplcc_args);
    let st = cmd.status().map_err(|e| format!("tplcc spawn: {e}"))?;
    if !st.success() || !pkg.exists() {
        return Err(format!("tplcc build failed ({st})"));
    }
    Ok(pkg)
}

fn default_ctl(name: &str) -> String {
    std::env::var("TPLC_CTL_SOCK").unwrap_or_else(|_| format!("/tmp/tplc-{name}.ctl"))
}

fn default_image(name: &str) -> String {
    format!("/tplc-{name}")
}

/// `tplc-image create` tolerating "already exists": probe with dump.
fn ensure_image(tools: &Tools, image: &str, in_sz: u32, out_sz: u32) -> Result<bool, String> {
    let cli = tools.require("tplc-image")?;
    let st = Command::new(&cli)
        .args(["create", image, &in_sz.to_string(), &out_sz.to_string()])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|e| format!("tplc-image: {e}"))?;
    if st.success() {
        return Ok(true); // we created it
    }
    // exists already? dump attaches read-only
    let probe = Command::new(&cli)
        .args(["dump", image])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|e| format!("tplc-image: {e}"))?;
    if probe.success() {
        return Ok(false);
    }
    Err(format!("cannot create or attach image {image}"))
}

fn wait_ctl(sock: &str, ms: u64) -> bool {
    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(ms);
    while std::time::Instant::now() < deadline {
        if crate::ctl::Ctl::probe(sock) {
            return true;
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    false
}

fn spawn_rt(tools: &Tools, image: &str, pkg: &Path, ctl: &str, extra: &[String]) -> Result<Child, String> {
    let plc = tools.require("tether-plc")?;
    let mut cmd = Command::new(&plc);
    cmd.arg("--image").arg(image)
        .arg("--package").arg(pkg)
        .arg("--ctl").arg(ctl);
    cmd.args(extra);
    cmd.spawn().map_err(|e| format!("tether-plc spawn: {e}"))
}

fn write_session(s: &Session) {
    if let Err(e) = session::write(s) {
        eprintln!("tplc: session write: {e}");
    }
}

/* ---------------- tplc run ---------------- */

pub fn cmd_run(tools: &Tools, args: &[String]) -> i32 {
    let (mut o, mut pos, _) = parse_common(args);
    apply_project(&mut o, &mut pos);
    let pkg = match ensure_package(tools, &o, &pos) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("tplc run: {e}");
            return 2;
        }
    };
    let image = o.image.clone().unwrap_or_else(|| default_image(&o.name));
    let ctl = o.ctl.clone().unwrap_or_else(|| default_ctl(&o.name));
    let _ = std::fs::remove_file(&ctl);
    match ensure_image(tools, &image, o.in_size, o.out_size) {
        Ok(created) => {
            if !created {
                eprintln!("tplc: reusing existing image {image}");
            }
        }
        Err(e) => {
            eprintln!("tplc run: {e}");
            return 1;
        }
    }
    let mut child = match spawn_rt(tools, &image, &pkg, &ctl, &o.plc_args) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("tplc run: {e}");
            return 1;
        }
    };
    if !wait_ctl(&ctl, 10_000) {
        eprintln!("tplc run: ctl socket {ctl} never appeared");
        let _ = child.kill();
        return 1;
    }
    write_session(&Session {
        name: o.name.clone(),
        pid: std::process::id(),
        runtime_pid: child.id(),
        ctl: ctl.clone(),
        image: image.clone(),
        pkg: pkg.display().to_string(),
        io_port: 0,
        http_port: 0,
        spawned: vec![],
        started: chrono_now(),
    });
    println!("tplc: running {}", pkg.display());
    println!("  image  {image}");
    println!("  ctl    {ctl}   (tplc ctl --session {} status · tplc down)", o.name);
    println!("  ctrl-c to stop");
    install_run_stop();
    let rc = loop {
        if RUN_STOP.load(std::sync::atomic::Ordering::Relaxed) {
            let _ = child.kill();
            break child.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1);
        }
        match child.try_wait() {
            Ok(Some(st)) => break st.code().unwrap_or(1),
            Ok(None) => std::thread::sleep(std::time::Duration::from_millis(100)),
            Err(_) => break 1,
        }
    };
    session::remove(&o.name);
    rc
}

static RUN_STOP: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

extern "C" fn run_on_signal(_sig: i32) {
    RUN_STOP.store(true, std::sync::atomic::Ordering::Relaxed);
}

fn install_run_stop() {
    unsafe {
        libc::signal(libc::SIGINT, run_on_signal as extern "C" fn(i32) as usize);
        libc::signal(libc::SIGTERM, run_on_signal as extern "C" fn(i32) as usize);
    }
}

fn chrono_now() -> String {
    // no chrono dep: seconds since epoch is enough for a session stamp
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs().to_string())
        .unwrap_or_default()
}

/* ---------------- tplc sim ---------------- */

pub fn cmd_sim(tools: &Tools, args: &[String]) -> i32 {
    let (mut o, mut pos, _) = parse_common(args);
    apply_project(&mut o, &mut pos);
    let Some(steps) = &o.steps else {
        eprintln!("tplc sim: --steps FILE required");
        return 2;
    };
    let pkg = match ensure_package(tools, &o, &pos) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("tplc sim: {e}");
            return 2;
        }
    };
    let image = o.image.clone().unwrap_or_else(|| default_image(&format!("{}-sim", o.name)));
    let ctl = o.ctl.clone().unwrap_or_else(|| default_ctl(&format!("{}-sim", o.name)));
    let _ = std::fs::remove_file(&ctl);
    // sim creates the shm image itself — clear any stale one first
    if let Some(cli) = tools.find("tplc-image") {
        let _ = Command::new(&cli)
            .args(["unlink", &image])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
    let sim = match tools.require("tether-plc-sim") {
        Ok(p) => p,
        Err(e) => {
            eprintln!("tplc sim: {e}");
            return 127;
        }
    };
    let plc = match tools.require("tether-plc") {
        Ok(p) => p,
        Err(e) => {
            eprintln!("tplc sim: {e}");
            return 127;
        }
    };
    let mut cmd = Command::new(&sim);
    cmd.arg("--image").arg(&image)
        .arg("--package").arg(&pkg)
        .arg("--steps").arg(steps)
        .arg("--plc").arg(&plc)
        .arg("--ctl").arg(&ctl)
        .arg("--in").arg(o.in_size.to_string())
        .arg("--out").arg(o.out_size.to_string());
    for s in &o.shadow {
        cmd.arg("--shadow").arg(s);
    }
    if let Some(d) = &o.dump_q {
        cmd.arg("--dump-q").arg(d);
    }
    if let Some(ms) = o.settle_ms {
        cmd.arg("--settle-ms").arg(ms.to_string());
    }
    if let Some(ms) = o.hold_ms {
        cmd.arg("--hold-ms").arg(ms.to_string());
    }
    if o.scan {
        cmd.arg("--scan");
    }
    let st = match cmd.status() {
        Ok(s) => s.code().unwrap_or(1),
        Err(e) => {
            eprintln!("tplc sim: spawn: {e}");
            1
        }
    };
    let _ = Command::new(tools.require("tplc-image").unwrap_or_default())
        .args(["unlink", &image])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    st
}

/* ---------------- tplc dev ---------------- */

pub struct DevOpts {
    pub run: RunOpts,
    pub port: u16,
    pub bind: String,
    pub webui: Option<String>, // "" = none, path = dir
    pub no_webui: bool,
    pub io: bool,
    pub io_port: u16,
    pub tplc_io_upstream: Option<String>,
    pub packages: Option<String>,
    pub libdirs: Vec<String>,
    pub tplccd: Option<String>, // None = default managed; "" handled by caller
    pub rt_log: Option<String>,
}

fn parse_dev(args: &[String]) -> (DevOpts, Vec<String>) {
    // Strip dev-only flags first — parse_common exits on unknown options.
    let mut d = DevOpts {
        run: RunOpts::default_empty(),
        port: 8089,
        bind: "127.0.0.1".into(),
        webui: None,
        no_webui: false,
        io: true,
        io_port: 0,
        tplc_io_upstream: None,
        packages: None,
        libdirs: vec![],
        tplccd: None,
        rt_log: None,
    };
    let mut rest: Vec<String> = Vec::new();
    let mut i = 0;
    while i < args.len() {
        let a = &args[i];
        let val = |i: &mut usize| -> String {
            *i += 1;
            args.get(*i).cloned().unwrap_or_default()
        };
        match a.as_str() {
            "--" => {
                rest.extend_from_slice(&args[i..]);
                break;
            }
            "--port" => d.port = val(&mut i).parse().unwrap_or(8089),
            "--bind" => d.bind = val(&mut i),
            "--webui" => d.webui = Some(val(&mut i)),
            "--no-webui" => d.no_webui = true,
            "--no-io" => d.io = false,
            "--io-port" => d.io_port = val(&mut i).parse().unwrap_or(0),
            "--tplc-io" => d.tplc_io_upstream = Some(val(&mut i)),
            "--packages" => d.packages = Some(val(&mut i)),
            "--libdir" => d.libdirs.push(val(&mut i)),
            "--tplccd" => d.tplccd = Some(val(&mut i)),
            "--rt-log" => d.rt_log = Some(val(&mut i)),
            _ => rest.push(a.clone()),
        }
        i += 1;
    }
    let (run, pos, _) = parse_common(&rest);
    d.run = run;
    (d, pos)
}

impl RunOpts {
    fn default_empty() -> Self {
        RunOpts {
            name: "main".into(),
            image: None,
            in_size: 64,
            out_size: 64,
            ctl: None,
            workdir: None,
            libs: vec![],
            tplcc_args: vec![],
            plc_args: vec![],
            steps: None,
            shadow: vec![],
            dump_q: None,
            settle_ms: None,
            hold_ms: None,
            scan: false,
            project: None,
            name_explicit: false,
            in_explicit: false,
            out_explicit: false,
        }
    }
}

fn free_port() -> u16 {
    std::net::TcpListener::bind("127.0.0.1:0")
        .ok()
        .and_then(|l| l.local_addr().ok())
        .map(|a| a.port())
        .unwrap_or(43770)
}

/// `tplc dev` — bring up the whole local stack:
///   compile (if srcs) → image → tether-plc → tplc-io → webserver
/// Reuses an already-running runtime when the ctl socket answers.
pub fn cmd_dev(tools: &Tools, args: &[String]) -> i32 {
    let (mut d, mut pos) = parse_dev(args);
    let proj = apply_project(&mut d.run, &mut pos);

    // No project discovered and none requested → open the cwd as the
    // workspace so the WebUI file/git panes have a root.  Manifest-only
    // adopt (no scaffold, no git writes, no auto-compiled program) —
    // an explicit `--project` that failed does not fall back to cwd.
    let proj = match (proj, d.run.project.is_none()) {
        (Some(p), _) => Some(p),
        (None, false) => None,
        (None, true) => {
            let cwd =
                std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
            match crate::project::adopt(&cwd) {
                Ok(p) => {
                    eprintln!(
                        "tplc dev: no project found — opened {} as workspace",
                        p.root.display()
                    );
                    Some(p)
                }
                Err(e) => {
                    eprintln!("tplc dev: workspace {}: {e}", cwd.display());
                    None
                }
            }
        }
    };
    let o = &d.run;

    // 1. package (compile if sources given)
    let pkg: Option<PathBuf> = if pos.is_empty() {
        None
    } else {
        match ensure_package(tools, o, &pos) {
            Ok(p) => Some(p),
            Err(e) => {
                eprintln!("tplc dev: {e}");
                return 2;
            }
        }
    };

    let ctl = o.ctl.clone().unwrap_or_else(|| default_ctl(&o.name));
    let image = o.image.clone().unwrap_or_else(|| default_image(&o.name));

    // 2. runtime: reuse if ctl already answers, else spawn — only
    // when there is a program to run. No package → serve-only dev
    // mode: the webserver still comes up (compile/catalog/project
    // meta-ops work; ctl frames get synthesized backend-down errors).
    let mut spawned_pids: Vec<u32> = vec![];
    let mut rt_child: Option<Child> = None;
    let mut rt_pid = 0u32;
    let mut runtime_up = false;
    if crate::ctl::Ctl::probe(&ctl) {
        eprintln!("tplc: reusing runtime on {ctl}");
        runtime_up = true;
    } else {
        let _ = std::fs::remove_file(&ctl);
        if let Some(pkg) = &pkg {
            match ensure_image(tools, &image, o.in_size, o.out_size) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("tplc dev: {e}");
                    return 1;
                }
            }
            match spawn_rt(tools, &image, pkg, &ctl, &o.plc_args) {
                Ok(mut c) => {
                    rt_pid = c.id();
                    spawned_pids.push(c.id());
                    if !wait_ctl(&ctl, 10_000) {
                        eprintln!("tplc dev: ctl socket {ctl} never appeared");
                        let _ = c.kill();
                        return 1;
                    }
                    rt_child = Some(c);
                    runtime_up = true;
                }
                Err(e) => {
                    eprintln!("tplc dev: {e}");
                    return 1;
                }
            }
        } else {
            eprintln!(
                "tplc dev: no program to run — serve-only mode \
                 (pass sources or a .tplc, or open a project)"
            );
        }
    }

    // 3. tplc-io provider
    let mut io_upstream = d.tplc_io_upstream.clone();
    let mut io_child: Option<Child> = None;
    if d.io && io_upstream.is_none() && runtime_up {
        if let Some(io_bin) = tools.find("tplc-io") {
            let port = if d.io_port != 0 { d.io_port } else { free_port() };
            let mut cmd = Command::new(&io_bin);
            cmd.arg("--ctl").arg(&ctl)
                .arg("--port").arg(port.to_string())
                .arg("--bind").arg("127.0.0.1");
            if let Some(pkgs) = &d.packages {
                cmd.arg("--packages").arg(pkgs);
            }
            if let Some(p) = &pkg {
                cmd.arg("--pkg").arg(p);
            }
            match cmd.spawn() {
                Ok(c) => {
                    io_child = Some(c);
                    spawned_pids.push(io_child.as_ref().unwrap().id());
                    io_upstream = Some(format!("127.0.0.1:{port}"));
                }
                Err(e) => eprintln!("tplc dev: tplc-io spawn: {e}"),
            }
        }
    }

    // 4. session record
    write_session(&Session {
        name: o.name.clone(),
        pid: std::process::id(),
        runtime_pid: rt_pid,
        ctl: ctl.clone(),
        image: image.clone(),
        pkg: pkg.as_ref().map(|p| p.display().to_string()).unwrap_or_default(),
        io_port: io_upstream
            .as_deref()
            .and_then(|s| s.rsplit(':').next()?.parse().ok())
            .unwrap_or(0),
        http_port: d.port,
        spawned: spawned_pids.clone(),
        started: chrono_now(),
    });

    // 5. webserver (foreground)
    let webui = if d.no_webui {
        None
    } else {
        d.webui.clone().or_else(crate::ensure_webui_dir)
    };
    println!("tplc dev: stack up");
    println!("  webui   {}", webui.as_deref().unwrap_or("(api only)"));
    println!("  http    http://{}:{}", d.bind, d.port);
    println!("  ctl     {ctl}");
    println!("  image   {image}");
    if let Some(p) = &pkg {
        println!("  pkg     {}", p.display());
    }
    if let Some(io) = &io_upstream {
        println!("  io      {io}");
    }
    println!("  tplc ctl --session {} status · tplc down --session {}", o.name, o.name);

    let simdir = proj
        .as_ref()
        .map(|p| p.root.join(".tplc").join("sim"))
        .unwrap_or_else(|| std::env::temp_dir().join("tplc-sim"));
    let rc = crate::serve::serve(tools, &crate::serve::ServeCfg {
        project_root: std::sync::Arc::new(std::sync::Mutex::new(
            proj.map(|p| p.root),
        )),
        ctl_sock: ctl.clone(),
        bind: d.bind.clone(),
        port: d.port,
        tplcc: tools.find("tplcc"),
        tplccd: d.tplccd.clone(),
        tplc_io: io_upstream.clone(),
        libdirs: if d.libdirs.is_empty() { crate::default_libdirs() } else { d.libdirs.clone() },
        webui,
        packages: d.packages.clone(),
        pkgs_for_io: pkg.iter().map(|p| p.display().to_string()).collect(),
        spawn_io: d.io && io_upstream.is_none() && runtime_up,
        io_port_hint: d.io_port,
        sim: tools.find("tether-plc-sim"),
        plc: tools.find("tether-plc"),
        ecpump: tools.find("tplc-ec-pump"),
        tools: tools.clone(),
        simdir,
    });

    // teardown children we spawned
    for pid in spawned_pids {
        unsafe {
            libc::kill(pid as i32, libc::SIGTERM);
        }
    }
    if let Some(mut c) = rt_child.take() {
        let _ = c.wait();
    }
    if let Some(mut c) = io_child.take() {
        let _ = c.wait();
    }
    session::remove(&o.name);
    rc
}

/* ---------------- tplc down / sessions ---------------- */

pub fn cmd_sessions(_tools: &Tools, _args: &[String]) -> i32 {
    let all = session::list();
    if all.is_empty() {
        println!("no sessions");
        return 0;
    }
    println!("{:<12} {:<7} {:<7} {:<22} {}", "name", "pid", "rtpid", "ctl", "http");
    for s in &all {
        println!(
            "{:<12} {:<7} {:<7} {:<22} {}",
            s.name,
            s.pid,
            s.runtime_pid,
            s.ctl,
            if s.http_port != 0 { s.http_port.to_string() } else { "-".into() }
        );
    }
    0
}

pub fn cmd_down(_tools: &Tools, args: &[String]) -> i32 {
    // tplc down [name]
    let name = args.iter().find(|a| !a.starts_with('-')).cloned().unwrap_or_default();
    let s = match session::resolve(&name) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("tplc down: {e}");
            return 2;
        }
    };
    // RT_SHUTDOWN over ctl, then TERM spawned children
    if let Ok(mut ctl) = crate::ctl::Ctl::connect(&s.ctl, 2000) {
        if ctl.hello("tplc").is_ok() {
            let _ = ctl.call_ok(op::RT_SHUTDOWN, &[]);
        }
    }
    for pid in &s.spawned {
        unsafe {
            libc::kill(*pid as i32, libc::SIGTERM);
        }
    }
    println!("tplc: session {} stopped", s.name);
    session::remove(&s.name);
    0
}
