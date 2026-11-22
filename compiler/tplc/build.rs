//! Ensure the C++ runtime tools `tplc` orchestrates are built.
//!
//! `cargo build` at the repo root now covers the whole stack: this
//! script configures `build/` via CMake when needed and incrementally
//! builds the tool targets. It is intentionally non-fatal — a missing
//! cmake or a failed optional target only produces a warning; `tplc
//! doctor` reports exactly which tools are absent. Set
//! `TPLC_SKIP_NATIVE_BUILD=1` to skip entirely (e.g. docs.rs, cross
//! builds, or when only hacking on the Rust side).

use std::path::PathBuf;
use std::process::Command;

/// CMake targets `tplc` shells out to (see tools.rs). Optional-service
/// targets (tplc-io*, tplc-ec-pump) only exist when their deps were
/// found at configure time — missing ones are skipped, not fatal.
const TOOLS: &[&str] = &[
    "tether-plc",
    "tether-plc-sim",
    "tether-plc-bridge",
    "tplc-image",
    "tplc-io",
    "tplc-io-probe",
    "tplc-io-cli",
    "tplc-ec-pump",
];

fn repo_root() -> Option<PathBuf> {
    // CARGO_MANIFEST_DIR = <root>/compiler/tplc
    let manifest = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").ok()?);
    manifest.parent()?.parent().map(|p| p.to_path_buf())
}

/// Run a cmake command, capturing stdout+stderr. Returns (ok, log).
/// Capturing (rather than inheriting stdio) keeps the cargo log clean
/// and lets us surface the real failure tail in the warning — an
/// inherited-stdio error scrolls past and the warning alone is
/// undecipherable.
fn run_capture(cmd: &mut Command) -> (bool, String) {
    match cmd.output() {
        Ok(o) => {
            let mut log = String::from_utf8_lossy(&o.stdout).into_owned();
            log.push_str(&String::from_utf8_lossy(&o.stderr));
            // A silent failure is undecipherable — name the exit status
            // (signal kills show as `signal: N`) so it is diagnosable.
            if !o.status.success() && log.trim().is_empty() {
                log = format!("(cmake exited {} with no output)", o.status);
            }
            (o.status.success(), log)
        }
        Err(e) => (false, format!("spawn failed: {e}")),
    }
}

/// Append a failed cmake invocation to build/.tplc-build.log — cargo
/// replays build-script warnings from cache, so the warning stream can
/// show a stale failure long after the run that produced it; this file
/// is the ground truth of what cmake actually did, when.
fn log_cmake_failure(build: &std::path::Path, what: &str, log: &str) {
    use std::io::Write;
    let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(build.join(".tplc-build.log"))
    else {
        return;
    };
    let _ = writeln!(
        f,
        "=== [{}] {} ===\n{}\n",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0),
        what,
        tail(log, 40)
    );
}

/// The scan path needs a pump whose arg surface includes --scan. The
/// probe is the binary's own --help usage: fresh exits 0 printing the
/// usage line with [--scan]; stale predates fleet autodiscovery and
/// rejects --help outright ("unknown arg" — which must not count as a
/// --scan match: "unknown arg --scan" contains the substring too).
/// Also false when the binary is missing/crashes.
fn pump_supports_scan(pump: &std::path::Path) -> bool {
    match Command::new(pump).arg("--help").output() {
        Ok(o) => {
            let mut s = String::from_utf8_lossy(&o.stdout).into_owned();
            s.push_str(&String::from_utf8_lossy(&o.stderr));
            o.status.success()
                && s.contains("--scan")
                && !s.contains("unknown arg")
        }
        Err(_) => false,
    }
}

/// Last meaningful lines of a build log for warnings.
fn tail(log: &str, n: usize) -> String {
    log.lines()
        .filter(|l| !l.trim().is_empty())
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .take(n)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .join("\n")
}

// Serialize cmake invocations on a build dir — shared implementation
// (`include!` because build scripts can't use crate modules); serve's
// ecpump auto-rebuild recovery uses the same lock file.
include!("src/cmake_lock.rs");

fn main() {
    if std::env::var_os("TPLC_SKIP_NATIVE_BUILD").is_some() {
        return;
    }
    let Some(root) = repo_root() else { return };
    if !root.join("CMakeLists.txt").is_file() {
        return; // not a source checkout (e.g. packaged crate)
    }
    let build = root.join("build");

    // Rerun policy — cargo's default "any package file changed" scan
    // watches compiler/tplc ONLY, but the C++ tool sources live outside
    // the package: editing runtime/ecpump/main.cpp never reran this
    // script, so cmake never ran and the binary stayed stale. That is
    // the original root cause of "unknown arg --scan". Track the C++
    // inputs explicitly. A missing CMakeCache.txt counts as changed,
    // so fresh checkouts always run the configure+build below.
    // NOTE: cargo ignores rerun-if-changed paths given absolute and
    // outside the package dir — everything here is `..`-relative to
    // compiler/tplc (verified: absolute paths did not trigger reruns).
    println!("cargo:rerun-if-changed=../../runtime");
    println!("cargo:rerun-if-changed=../../include");
    println!("cargo:rerun-if-changed=../../cmake");
    println!("cargo:rerun-if-changed=../../CMakeLists.txt");
    println!("cargo:rerun-if-changed=../../build/CMakeCache.txt");
    // …and the pump binary itself: deleted or replaced-with-stale
    // counts as changed → build.rs reruns → the freshness gate below
    // probes and force-relinks. Self-stabilizing (one extra no-op
    // cmake run after each relink, then steady state).
    println!("cargo:rerun-if-changed=../../build/tplc-ec-pump");
    println!("cargo:rerun-if-env-changed=TPLC_SKIP_NATIVE_BUILD");

    // Serialize with any other cmake invocations on this build dir.
    let _lock = CmakeLock::acquire(&build, std::time::Duration::from_secs(600));

    // 1. configure once
    if !build.join("CMakeCache.txt").is_file() {
        println!("cargo:warning=tplc: configuring CMake build dir {}", build.display());
        let (ok, log) = run_capture(Command::new("cmake")
            .arg("-S").arg(&root)
            .arg("-B").arg(&build));
        if !ok {
            println!(
                "cargo:warning=tplc: cmake configure failed — run `cmake -B build` manually; `tplc doctor` will list missing tools\n{}",
                tail(&log, 15)
            );
            return;
        }
    }

    // 2. incremental build of the tool targets (batch first; on failure
    //    fall back per-target so one missing optional target doesn't
    //    block the rest)
    let mut args: Vec<String> = vec![
        "--build".into(),
        build.display().to_string(),
        "--target".into(),
    ];
    args.extend(TOOLS.iter().map(|s| (*s).to_string()));
    args.push("--parallel".into());
    args.push("8".into());
    let (batch_ok, batch_log) = run_capture(Command::new("cmake").args(&args));
    if !batch_ok {
        log_cmake_failure(&build, "tool batch build", &batch_log);
        // batch failed — build targets individually, report the misses
        // with the real error tail so the warning is actionable.
        let mut missing = Vec::new();
        let mut last_log = batch_log;
        for t in TOOLS {
            let (ok, log) = run_capture(Command::new("cmake")
                .arg("--build").arg(&build)
                .arg("--target").arg(t)
                .arg("--parallel").arg("8"));
            if !ok {
                missing.push(*t);
                last_log = log;
            }
        }
        if !missing.is_empty() {
            log_cmake_failure(&build, "per-target builds", &last_log);
            println!(
                "cargo:warning=tplc: C++ tools not built: {} — `tplc doctor` reports what is missing; last cmake output:\n{}",
                missing.join(", "),
                tail(&last_log, 15)
            );
        }
    }

    // 3. Pump freshness gate — cmake "success" does not prove the
    //    binary got relinked: a file NEWER than its objects (restored
    //    backup, copied artifact, clock skew) makes make skip the
    //    relink and leaves a stale binary that lacks --scan. Probe the
    //    actual arg surface; if stale/missing, remove the file (forces
    //    a real relink) and build once more. A "No rule to make target"
    //    means the build dir predates the pump's configure-time deps
    //    (Tether libs) — reconfigure picks them up, then retry.
    //    This is the compile-time guarantee serve relies on — runtime
    //    never rebuilds.
    let pump = build.join("tplc-ec-pump");
    if !pump_supports_scan(&pump) {
        if pump.is_file() {
            println!(
                "cargo:warning=tplc: tplc-ec-pump binary is stale (no --scan) — forcing relink"
            );
            let _ = std::fs::remove_file(&pump);
        }
        let pump_build = |build: &std::path::Path| {
            run_capture(Command::new("cmake")
                .arg("--build").arg(build)
                .arg("--target").arg("tplc-ec-pump")
                .arg("--parallel").arg("8"))
        };
        let (mut ok, mut log) = pump_build(&build);
        if !ok {
            log_cmake_failure(&build, "tplc-ec-pump target build", &log);
            // A configure-drift failure (missing target, stale cache)
            // is recoverable: re-run configure, then build once more.
            let (cok, clog) = run_capture(Command::new("cmake")
                .arg("-S").arg(&root)
                .arg("-B").arg(&build));
            if !cok {
                log_cmake_failure(&build, "cmake reconfigure", &clog);
            } else {
                let r = pump_build(&build);
                ok = r.0;
                log = r.1;
                if !ok {
                    log_cmake_failure(
                        &build,
                        "tplc-ec-pump post-reconfigure build",
                        &log,
                    );
                }
            }
        }
        if !ok || !pump_supports_scan(&pump) {
            println!(
                "cargo:warning=tplc: tplc-ec-pump still stale/missing after forced rebuild — last cmake output:\n{}",
                tail(&log, 15)
            );
        }
    }
}
