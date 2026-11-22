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

/// Serialize cmake invocations on a build dir — `cargo build`'s build.rs
/// racing a manual `cmake --build` (or a second cargo) on the same dir
/// corrupts the dep graph / produces sporadic failures. O_EXCL lockfile
/// with stale-lock takeover; giving up waits builds anyway (the failure
/// is then reported like any other).
struct CmakeLock(PathBuf);
impl CmakeLock {
    fn acquire(dir: &std::path::Path) -> Option<CmakeLock> {
        let path = dir.join(".tplc-cmake.lock");
        let start = std::time::Instant::now();
        loop {
            match std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&path)
            {
                Ok(_) => return Some(CmakeLock(path)),
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                    let stale = std::fs::metadata(&path)
                        .and_then(|m| m.modified())
                        .map(|t| {
                            t.elapsed().unwrap_or_default()
                                > std::time::Duration::from_secs(30 * 60)
                        })
                        .unwrap_or(false);
                    if stale {
                        let _ = std::fs::remove_file(&path);
                        continue;
                    }
                    if start.elapsed() > std::time::Duration::from_secs(600) {
                        return None;
                    }
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
                Err(_) => return None,
            }
        }
    }
}
impl Drop for CmakeLock {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

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
    let _lock = CmakeLock::acquire(&build);

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
    if batch_ok {
        return;
    }

    // batch failed — build targets individually, report the misses with
    // the real error tail so the warning is actionable.
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
