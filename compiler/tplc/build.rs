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
            (o.status.success(), log)
        }
        Err(e) => (false, format!("spawn failed: {e}")),
    }
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
    //    a real relink) and build once more. This is the compile-time
    //    guarantee serve relies on — runtime never rebuilds.
    let pump = build.join("tplc-ec-pump");
    if !pump_supports_scan(&pump) {
        if pump.is_file() {
            println!(
                "cargo:warning=tplc: tplc-ec-pump binary is stale (no --scan) — forcing relink"
            );
            let _ = std::fs::remove_file(&pump);
        }
        let (ok, log) = run_capture(Command::new("cmake")
            .arg("--build").arg(&build)
            .arg("--target").arg("tplc-ec-pump")
            .arg("--parallel").arg("8"));
        if !ok || !pump_supports_scan(&pump) {
            println!(
                "cargo:warning=tplc: tplc-ec-pump still stale/missing after forced rebuild — last cmake output:\n{}",
                tail(&log, 15)
            );
        }
    }
}
