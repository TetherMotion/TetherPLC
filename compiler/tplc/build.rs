//! Ensure the C++ runtime tools `tplc` orchestrates are built.
//!
//! `cargo build` at the repo root now covers the whole stack: this
//! script configures `build/` via CMake when needed and incrementally
//! builds the tool targets. It is intentionally non-fatal — a missing
//! cmake or a failed optional target only produces a warning; `tplc
//! doctor` reports exactly which tools are absent. Set
//! `TPLC_SKIP_NATIVE_BUILD=1` to skip entirely (e.g. docs.rs, cross
//! builds, or when only hacking on the Rust side).

use std::path::{Path, PathBuf};
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

fn run(cmd: &mut Command) -> bool {
    match cmd.status() {
        Ok(st) => st.success(),
        Err(e) => {
            eprintln!("tplc build.rs: spawn failed: {e}");
            false
        }
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

    // 1. configure once
    if !build.join("CMakeCache.txt").is_file() {
        println!("cargo:warning=tplc: configuring CMake build dir {}", build.display());
        let ok = run(Command::new("cmake")
            .arg("-S").arg(&root)
            .arg("-B").arg(&build));
        if !ok {
            println!("cargo:warning=tplc: cmake configure failed — run `cmake -B build` manually; `tplc doctor` will list missing tools");
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
    if run(Command::new("cmake").args(&args)) {
        return;
    }

    // batch failed — build targets individually, report the misses
    let mut missing = Vec::new();
    for t in TOOLS {
        let ok = run(Command::new("cmake")
            .arg("--build").arg(&build)
            .arg("--target").arg(t)
            .arg("--parallel").arg("8"));
        if !ok {
            missing.push(*t);
        }
    }
    if !missing.is_empty() {
        println!(
            "cargo:warning=tplc: C++ tools not built: {} — see `cmake --build build` output; `tplc doctor` reports what is missing",
            missing.join(", ")
        );
    }
}
