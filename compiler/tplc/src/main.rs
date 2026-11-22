//! tplc — the unified TetherPLC entrypoint.
//!
//! One binary fronting the whole toolchain:
//!
//!   tplc dev [srcs…|pkg.tplc]     full local stack: compile → image →
//!                                 tether-plc → tplc-io → webserver+WebUI
//!   tplc run <srcs…|pkg>          compile-if-needed → image → tether-plc
//!   tplc sim <srcs…|pkg> --steps F  deterministic replay via tether-plc-sim
//!   tplc serve                   HTTP/WebSocket backend (was tplc-ctl-proxy)
//!                                 + static WebUI
//!   tplc ctl <op> …               native control channel (was tplc-ctl)
//!   tplc down|sessions            session lifecycle
//!   tplc doctor                   tool resolution check
//!   tplc cc <args>                tplcc passthrough (compiler daemon etc.)
//!   tplc image|bridge|io|pump …   runtime tool passthrough
//!   tplc check|build|info|…       tplcc subcommand passthrough

#[allow(dead_code)] // ABI mirror of include/tplc/tplc_ctl.h
mod ctl;
mod ctlcmd;
mod git;
mod project;
mod run;
mod serve;
mod session;
mod tools;

use tools::Tools;

const USAGE: &str = "\
tplc — unified TetherPLC toolchain entrypoint

USAGE:
  tplc dev [OPTIONS] [srcs…|pkg.tplc]   dev stack: runtime + tplc-io + web UI
  tplc run [OPTIONS] [srcs…|pkg.tplc]   compile → image → tether-plc (fg)
  tplc sim [OPTIONS] [srcs…|pkg.tplc]   replay via tether-plc-sim
  tplc serve [OPTIONS]                  HTTP backend + WebUI static files
  tplc ctl [--socket PATH|--session N] <op> [args]   runtime control
  tplc down [name]                      stop a session's runtime
  tplc sessions                         list sessions
  tplc new [DIR] [--name N]             create a project (.tplc/ + git)
  tplc init [DIR]                       adopt DIR as a project
  tplc project [show|path|set K=V]      project manifest
  tplc git <status|log|diff|commit|branch> [args]   project git
  tplc projects                         recently opened projects
  tplc fbd-schema                       print the fbd.v1 JSON Schema
  tplc pkg <dump|verify|gen|args…>      .tplc package inspect/fixture;
                                        other args → `tplcc pkg`
  tplc doctor                           check tool resolution
  tplc cc <args…>                       tplcc passthrough (e.g. `tplc cc serve`)
  tplc image|bridge|io|pump <args…>     runtime tool passthrough
  tplc check|build|info|vectors|sign|blocks|src <args…>   tplcc ops

OPTIONS (run/sim/dev):
  --name NAME        session/image/socket name (default: main, or the
                     project name inside a project)
  --project DIR      project root (default: discovered from cwd)
  --image PATH       shm image path (default: /tplc-<name>)
  --in N --out N     image q sizes (default 64)
  --ctl PATH         ctl socket (default: /tmp/tplc-<name>.ctl)
  --workdir DIR      build dir for compiled package
  --lib DIR          library dir for tplcc --lib (repeatable)
  -- <args>          extra args appended to tether-plc (run/dev)

sim only:
  --steps FILE       step vector file (required)
  --shadow FILE      shadow package (repeatable)
  --dump-q PATH      dump Q trace
  --settle-ms N --hold-ms N --scan

serve options:
  --port N --bind A --webui DIR --no-webui
  --tplcc PATH --tplccd SOCK --tplc-io HOST:PORT
  --libdir DIR (repeatable) --packages DIR --pkg FILE (repeatable) --io

ctl ops: see `tplc ctl` with no args, or tplc-ctl --help equivalent.
Env: TPLC_BINDIR, TPLC_CTL_SOCK, TPLC_STATE_DIR, TPLC_WEBUI.
";

fn main() {
    // Unix semantics: die silently on SIGPIPE instead of panicking in
    // println! when a consumer closes the pipe early (`tplc ctl … | head`).
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
    let args: Vec<String> = std::env::args().skip(1).collect();
    std::process::exit(dispatch(&args));
}

fn dispatch(args: &[String]) -> i32 {
    let Some(cmd) = args.first() else {
        eprint!("{USAGE}");
        return 2;
    };
    let rest = &args[1..];
    let tools = Tools::default();
    match cmd.as_str() {
        "help" | "--help" | "-h" => {
            print!("{USAGE}");
            0
        }
        "version" | "--version" | "-V" => {
            println!("tplc {}", env!("CARGO_PKG_VERSION"));
            0
        }
        "doctor" => doctor(&tools),
        "ctl" => cmd_ctl(rest),
        "run" => run::cmd_run(&tools, rest),
        "sim" => run::cmd_sim(&tools, rest),
        "dev" => run::cmd_dev(&tools, rest),
        "serve" => cmd_serve(&tools, rest),
        "down" | "stop" => run::cmd_down(&tools, rest),
        "sessions" | "ps" | "ls" => run::cmd_sessions(&tools, rest),
        "new" | "init" => cmd_new(rest),
        "project" => cmd_project(rest),
        "git" => cmd_git(rest),
        "projects" | "recent" => cmd_projects(),
        "pkg" => cmd_pkg(rest, &tools),
        "fbd-schema" | "schema" => {
            println!("{}", fbd_model::schema_json());
            0
        }
        "image" => tools::run_tool(&tools, "tplc-image", rest),
        "bridge" => tools::run_tool(&tools, "tether-plc-bridge", rest),
        "io" => tools::run_tool(&tools, "tplc-io", rest),
        "pump" | "ec-pump" => tools::run_tool(&tools, "tplc-ec-pump", rest),
        "cc" => tools::run_tool(&tools, "tplcc", rest),
        // tplcc subcommand passthrough
        "check" | "build" | "info" | "vectors" | "sign" | "blocks" | "src" => {
            let mut a = vec![cmd.clone()];
            a.extend_from_slice(rest);
            tools::run_tool(&tools, "tplcc", &a)
        }
        other => {
            eprintln!("tplc: unknown command `{other}` — try `tplc help`");
            2
        }
    }
}

fn doctor(tools: &Tools) -> i32 {
    let names = [
        "tplcc",
        "tether-plc",
        "tether-plc-sim",
        "tether-plc-bridge",
        "tplc-image",
        "tplc-io",
        "tplc-ec-pump",
    ];
    let mut missing = 0;
    println!("tplc tool resolution:");
    for n in names {
        match tools.find(n) {
            Some(p) => println!("  {n:<20} {}", p.display()),
            None => {
                println!("  {n:<20} MISSING");
                missing += 1;
            }
        }
    }
    // webui hint
    if let Ok(d) = std::env::var("TPLC_WEBUI") {
        println!("  webui dir           {d}");
    }
    if missing == 0 {
        0
    } else {
        1
    }
}

/// Resolve which ctl socket `tplc ctl` should talk to.
fn ctl_socket(args: &[String]) -> (String, usize) {
    let mut sock = std::env::var("TPLC_CTL_SOCK").unwrap_or_default();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--socket" | "--ctl" | "--sock" => {
                if let Some(v) = args.get(i + 1) {
                    sock = v.clone();
                }
                i += 2;
            }
            "--session" | "--name" => {
                if let Some(v) = args.get(i + 1) {
                    match session::resolve(v) {
                        Ok(s) => sock = s.ctl,
                        Err(e) => {
                            eprintln!("tplc ctl: {e}");
                            std::process::exit(2);
                        }
                    }
                }
                i += 2;
            }
            // tplc-ctl-compatible form: `tplc ctl <sock-path> <op> …`
            s if s.contains('/') && !s.starts_with('-') => {
                sock = s.to_string();
                i += 1;
            }
            _ => break,
        }
    }
    if sock.is_empty() {
        // default: single live session, else the classic proxy socket
        if let Ok(s) = session::resolve("") {
            sock = s.ctl;
        } else {
            sock = "/tmp/tplc-image.ctl".into();
        }
    }
    (sock, i)
}

fn cmd_ctl(args: &[String]) -> i32 {
    if args.is_empty() {
        eprintln!("usage: tplc ctl [--socket PATH|--session NAME] <op> [args]");
        eprintln!("{}", ctlcmd::usage());
        return 2;
    }
    let (sock, off) = ctl_socket(args);
    ctlcmd::run(&sock, &args[off..])
}

fn cmd_serve(tools: &Tools, args: &[String]) -> i32 {
    // project root: --project DIR wins; else discover from cwd
    let mut project_dir: Option<String> = None;
    let filtered: Vec<String> = {
        let mut v = Vec::new();
        let mut i = 0;
        while i < args.len() {
            if args[i] == "--project" {
                project_dir = args.get(i + 1).cloned();
                i += 2;
            } else {
                v.push(args[i].clone());
                i += 1;
            }
        }
        v
    };
    let args = &filtered[..];
    let project_root = std::sync::Arc::new(std::sync::Mutex::new(
        resolve_serve_project(project_dir.as_deref()),
    ));
    let mut cfg = serve::ServeCfg {
        project_root,
        ctl_sock: String::new(),
        bind: "127.0.0.1".into(),
        port: 8089,
        tplcc: tools.find("tplcc"),
        tplccd: None,
        tplc_io: None,
        libdirs: vec![],
        webui: None,
        packages: None,
        pkgs_for_io: vec![],
        spawn_io: false,
        io_port_hint: 0,
    };
    let mut no_webui = false;
    let mut ctl_set = false;
    let mut i = 0;
    while i < args.len() {
        let a = &args[i];
        let val = |i: &mut usize| -> String {
            *i += 1;
            args.get(*i).cloned().unwrap_or_default()
        };
        match a.as_str() {
            "--ctl" | "--socket" => {
                cfg.ctl_sock = val(&mut i);
                ctl_set = true;
            }
            "--port" => cfg.port = val(&mut i).parse().unwrap_or(8089),
            "--bind" => cfg.bind = val(&mut i),
            "--webui" => cfg.webui = Some(val(&mut i)),
            "--no-webui" => no_webui = true,
            "--tplcc" => cfg.tplcc = Some(std::path::PathBuf::from(val(&mut i))),
            "--tplccd" => cfg.tplccd = Some(val(&mut i)),
            "--tplc-io" => cfg.tplc_io = Some(val(&mut i)),
            "--libdir" => cfg.libdirs.push(val(&mut i)),
            "--packages" => cfg.packages = Some(val(&mut i)),
            "--pkg" => cfg.pkgs_for_io.push(val(&mut i)),
            "--io" => cfg.spawn_io = true,
            "--io-port" => cfg.io_port_hint = val(&mut i).parse().unwrap_or(0),
            "--help" | "-h" => {
                eprintln!("tplc serve [--ctl SOCK] [--port N] [--bind A] [--webui DIR] [--tplc-io HP] [--libdir D]… [--io]");
                return 0;
            }
            s => {
                eprintln!("tplc serve: unknown option {s}");
                return 2;
            }
        }
        i += 1;
    }
    if !ctl_set {
        cfg.ctl_sock = match session::resolve("") {
            Ok(s) => s.ctl,
            Err(_) => std::env::var("TPLC_CTL_SOCK")
                .ok()
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| "/tmp/tplc-image.ctl".into()),
        };
    }
    if cfg.webui.is_none() && !no_webui {
        cfg.webui = ensure_webui_dir();
    }
    if cfg.libdirs.is_empty() {
        cfg.libdirs = default_libdirs();
    }
    serve::serve(tools, &cfg)
}

/// Resolve the serve project root: explicit dir (adopted via
/// project::init when it has no manifest), else cwd discovery.
fn resolve_serve_project(dir: Option<&str>) -> Option<std::path::PathBuf> {
    match dir {
        Some(d) => {
            let p = std::path::PathBuf::from(d);
            match project::discover(&p) {
                Some(root) => project::load(&root).ok().map(|pr| pr.root),
                None => project::init(&p, None).ok().map(|pr| pr.root),
            }
        }
        None => project::open(None).ok().map(|p| p.root),
    }
}

/// Candidate TetherWebUI dist dirs: `<…>/TetherWebUI/dist` walked up
/// from cwd and the executable dir (same order as detection).
fn webui_candidates() -> Vec<std::path::PathBuf> {
    let mut starts = Vec::new();
    if let Ok(c) = std::env::current_dir() {
        starts.push(c);
    }
    if let Ok(e) = std::env::current_exe() {
        if let Some(d) = e.parent() {
            starts.push(d.to_path_buf());
        }
    }
    let mut out = Vec::new();
    for s in starts {
        let mut d = Some(s.as_path());
        for _ in 0..8 {
            let Some(dir) = d else { break };
            for rel in ["../TetherWebUI", "TetherWebUI", "../../TetherWebUI"] {
                out.push(dir.join(rel).join("dist"));
            }
            d = dir.parent();
        }
    }
    out
}

/* ---------------- WebUI auto-build ----------------------------------
   `cargo run -- dev` should just work: when a sibling TetherWebUI
   checkout is found, its dist/ is (re)built when missing or stale —
   any source newer than dist/index.html triggers `npm run build`
   (with `npm install` first when node_modules is absent).
   TPLC_NO_WEBUI_BUILD=1 opts out (CI/minimal environments). */

/// Newest mtime under `dir` (recursive, capped so pathological trees
/// like node_modules can't stall startup).
fn newest_mtime(dir: &std::path::Path) -> Option<std::time::SystemTime> {
    let mut newest: Option<std::time::SystemTime> = None;
    let mut stack = vec![dir.to_path_buf()];
    let mut seen = 0usize;
    while let Some(d) = stack.pop() {
        let Ok(rd) = std::fs::read_dir(&d) else { continue };
        for e in rd.flatten() {
            seen += 1;
            if seen > 50_000 {
                return newest;
            }
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if let Ok(m) = e.metadata() {
                if let Ok(t) = m.modified() {
                    if newest.is_none_or(|n| t > n) {
                        newest = Some(t);
                    }
                }
            }
        }
    }
    newest
}

/// True when dist/index.html is missing or older than any source the
/// build depends on (src/, public/, index.html, configs).
fn webui_stale(root: &std::path::Path, dist: &std::path::Path) -> bool {
    let Some(stamp) = dist
        .join("index.html")
        .metadata()
        .and_then(|m| m.modified())
        .ok()
    else {
        return true; // no dist at all
    };
    for sub in ["src", "public"] {
        if let Some(t) = newest_mtime(&root.join(sub)) {
            if t > stamp {
                return true;
            }
        }
    }
    for f in ["index.html", "package.json", "package-lock.json",
              "vite.config.ts", "vite.config.js", "tsconfig.json"] {
        if let Some(t) = root
            .join(f)
            .metadata()
            .and_then(|m| m.modified())
            .ok()
        {
            if t > stamp {
                return true;
            }
        }
    }
    false
}

/// Run `npm install` (when needed) + `npm run build` in `root`.
/// Inherits stdio so the user sees progress/errors.
fn build_webui(root: &std::path::Path) -> bool {
    use std::process::Command;
    if !root.join("node_modules").is_dir() {
        eprintln!("tplc: WebUI node_modules missing — npm install …");
        let ok = Command::new("npm")
            .arg("install")
            .current_dir(root)
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
        if !ok {
            eprintln!("tplc: WebUI npm install failed");
            return false;
        }
    }
    eprintln!("tplc: building WebUI (npm run build in {}) …", root.display());
    let ok = Command::new("npm")
        .args(["run", "build"])
        .current_dir(root)
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if !ok {
        eprintln!("tplc: WebUI build failed — serving whatever dist exists");
    }
    ok
}

/// Like detect_webui_dir() but builds a sibling TetherWebUI checkout
/// when dist/ is missing or stale.  Honors TPLC_WEBUI verbatim and
/// TPLC_NO_WEBUI_BUILD=1 as an opt-out.
pub(crate) fn ensure_webui_dir() -> Option<String> {
    if let Ok(d) = std::env::var("TPLC_WEBUI") {
        if !d.is_empty() && std::path::Path::new(&d).join("index.html").exists() {
            return Some(d);
        }
    }
    let autobuild = std::env::var("TPLC_NO_WEBUI_BUILD")
        .map(|v| v != "1")
        .unwrap_or(true);
    for dist in webui_candidates() {
        let Some(root) = dist.parent().map(|p| p.to_path_buf()) else {
            continue;
        };
        let buildable = root.join("package.json").is_file();
        let have_dist = dist.join("index.html").exists();
        if buildable && autobuild && (!have_dist || webui_stale(&root, &dist)) {
            build_webui(&root);
            if dist.join("index.html").exists() {
                return Some(dist.display().to_string());
            }
        }
        // Built or not, an existing dist is better than nothing.
        if dist.join("index.html").exists() {
            return Some(dist.display().to_string());
        }
    }
    None
}

pub(crate) fn default_libdirs() -> Vec<String> {
    let mut out = Vec::new();
    // libraries/* + compat dirs under the repo root
    let Some(root) = Tools::default().repo_root() else {
        return out;
    };
    let lib = root.join("libraries");
    if let Ok(rd) = std::fs::read_dir(&lib) {
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() {
                out.push(p.display().to_string());
            }
        }
    }
    for rel in ["compat/tc2", "compat/tc3"] {
        let p = root.join(rel);
        if p.is_dir() {
            out.push(p.display().to_string());
        }
    }
    out
}

/* ---------------- projects & git ---------------- */

/// `tplc new [DIR] [--name N]` / `tplc init [DIR]` — create or adopt a
/// project directory: `.tplc/project.json` manifest, `.gitignore`,
/// scaffold `src/main.st`, and a git repo with an initial commit.
fn cmd_new(args: &[String]) -> i32 {
    let mut dir: Option<String> = None;
    let mut name: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--name" => {
                i += 1;
                name = args.get(i).cloned();
            }
            s if s.starts_with('-') => {
                eprintln!("tplc new: unknown option {s}");
                return 2;
            }
            s => dir = Some(s.to_string()),
        }
        i += 1;
    }
    let dir = dir.unwrap_or_else(|| ".".into());
    let path = std::path::PathBuf::from(&dir);
    match project::init(&path, name.as_deref()) {
        Ok(p) => {
            println!("tplc: project `{}` at {}", p.display_name(), p.root.display());
            println!("  manifest  {}", p.manifest_path().display());
            println!("  sources   {:?}", p.manifest.sources);
            println!("  git       {}", if git::is_repo(&p.root) { "initialized" } else { "unavailable" });
            println!("next: cd {} && tplc dev", p.root.display());
            0
        }
        Err(e) => {
            eprintln!("tplc new: {e}");
            1
        }
    }
}

/// `tplc project [show|path|set K=V …]` — inspect/edit the manifest.
fn cmd_project(args: &[String]) -> i32 {
    let proj = match project::open(None) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("tplc project: {e}");
            return 2;
        }
    };
    let sub = args.first().map(|s| s.as_str()).unwrap_or("show");
    match sub {
        "path" => {
            println!("{}", proj.root.display());
            0
        }
        "show" => {
            println!("{}", serde_json::to_string_pretty(&proj.manifest).unwrap_or_default());
            0
        }
        "set" => {
            let mut m = proj.manifest.clone();
            let mut changed = false;
            for kv in &args[1..] {
                let Some((k, v)) = kv.split_once('=') else {
                    eprintln!("tplc project set: expected K=V, got `{kv}`");
                    return 2;
                };
                changed |= set_manifest_field(&mut m, k, v);
            }
            if !changed {
                eprintln!("tplc project set: nothing changed (known keys: name description sources libs taskPeriodNs imageIn imageOut deployMap)");
                return 2;
            }
            let p2 = project::Project { root: proj.root.clone(), manifest: m };
            match p2.save() {
                Ok(()) => {
                    println!("saved {}", p2.manifest_path().display());
                    0
                }
                Err(e) => {
                    eprintln!("tplc project: {e}");
                    1
                }
            }
        }
        other => {
            eprintln!("tplc project: unknown subcommand `{other}` (show|path|set)");
            2
        }
    }
}

fn set_manifest_field(m: &mut project::Manifest, k: &str, v: &str) -> bool {
    match k {
        "name" => m.name = v.to_string(),
        "description" => m.description = Some(v.to_string()),
        "taskPeriodNs" => m.task_period_ns = v.parse().ok(),
        "imageIn" => m.image_in = v.parse().unwrap_or(64),
        "imageOut" => m.image_out = v.parse().unwrap_or(64),
        "deployMap" => m.deploy_map = Some(v.to_string()),
        "sources" => m.sources = v.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect(),
        "libs" => m.libs = v.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect(),
        _ => return false,
    }
    true
}

/// `tplc git <status|log|diff|commit|branch> [args]` — project git.
fn cmd_git(args: &[String]) -> i32 {
    let proj = match project::open(None) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("tplc git: {e}");
            return 2;
        }
    };
    let root = &proj.root;
    let sub = args.first().map(|s| s.as_str()).unwrap_or("status");
    let rest = &args[1.min(args.len())..];
    match sub {
        "status" => match git::status(root) {
            Ok(entries) => {
                if entries.is_empty() {
                    println!("clean");
                } else {
                    for e in entries {
                        println!("{} {}", e.status, e.path);
                    }
                }
                0
            }
            Err(e) => {
                eprintln!("tplc git status: {e}");
                1
            }
        },
        "log" => {
            let n = rest.first().and_then(|s| s.parse().ok()).unwrap_or(20);
            match git::log(root, n) {
                Ok(commits) => {
                    for c in commits {
                        println!("{} {} {}", c.short, c.time, c.message);
                    }
                    0
                }
                Err(e) => {
                    eprintln!("tplc git log: {e}");
                    1
                }
            }
        }
        "diff" => {
            let path = rest.first().map(|s| s.as_str());
            match git::diff(root, path) {
                Ok(Some(d)) => {
                    print!("{d}");
                    0
                }
                Ok(None) => {
                    println!("(no commits yet — nothing to diff against)");
                    0
                }
                Err(e) => {
                    eprintln!("tplc git diff: {e}");
                    1
                }
            }
        }
        "commit" => {
            let msg = rest.join(" ");
            if msg.trim().is_empty() {
                eprintln!("usage: tplc git commit <message>");
                return 2;
            }
            match git::commit(root, &msg) {
                Ok(oid) => {
                    println!("{}", &oid[..7.min(oid.len())]);
                    0
                }
                Err(e) => {
                    eprintln!("tplc git commit: {e}");
                    1
                }
            }
        }
        "branch" => match git::branch(root) {
            Ok(b) => {
                println!("{b}");
                0
            }
            Err(e) => {
                eprintln!("tplc git branch: {e}");
                1
            }
        },
        other => {
            eprintln!("tplc git: unknown subcommand `{other}` (status|log|diff|commit|branch)");
            2
        }
    }
}

/// `tplc pkg <dump|verify|gen> …` — native .tplc package tools
/// (tplc-pkg lib); anything else forwards to `tplcc pkg` (add-section).
fn cmd_pkg(args: &[String], tools: &Tools) -> i32 {
    let sub = args.first().map(|s| s.as_str()).unwrap_or("");
    match sub {
        "dump" | "verify" | "gen" => {
            if args.len() < 2 {
                eprintln!("usage: tplc pkg {sub} <file.tplc>");
                return 2;
            }
            let path = &args[1];
            if sub == "gen" {
                return match tplc_pkg::tool::fixture_bytes() {
                    Ok(bytes) => match std::fs::write(path, &bytes) {
                        Ok(()) => {
                            println!("{}: wrote {} bytes (8 sections)", path, bytes.len());
                            0
                        }
                        Err(e) => {
                            eprintln!("{}: {e}", path);
                            1
                        }
                    },
                    Err(e) => {
                        eprintln!("tplc pkg gen: {e} ({})", e.code());
                        1
                    }
                };
            }
            let data = match std::fs::read(path) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("{}: {}", path, e);
                    return 1;
                }
            };
            match tplc_pkg::Package::parse(&data) {
                Ok(pkg) => {
                    if sub == "dump" {
                        tplc_pkg::tool::dump(&pkg);
                    } else {
                        println!(
                            "{}: OK ({} bytes, {} sections)",
                            path,
                            data.len(),
                            pkg.toc().len()
                        );
                    }
                    0
                }
                Err(e) => {
                    eprintln!("{}: verify failed: {} ({})", path, e, e.code());
                    1
                }
            }
        }
        _ => {
            let mut a = vec!["pkg".to_string()];
            a.extend_from_slice(args);
            tools::run_tool(tools, "tplcc", &a)
        }
    }
}

/// `tplc projects` — recently opened project dirs.
fn cmd_projects() -> i32 {
    let list = project::recent();
    if list.is_empty() {
        println!("no recent projects — `tplc new <dir>` to create one");
        return 0;
    }
    for p in list {
        println!("{}", p.display());
    }
    0
}
