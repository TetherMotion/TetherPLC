use std::process::Command;

fn probe(prog: &str, arg: &str) -> Option<String> {
    Command::new(prog)
        .arg(arg)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .filter(|s| !s.is_empty())
}

fn main() {
    // toolchain provenance baked into tplcc so every emitted package can
    // record which rustc/LLVM built the artifact (TOOLCHAIN section).
    let rustc = probe("rustc", "--version").unwrap_or_else(|| "unknown".into());
    println!("cargo:rustc-env=TPLC_RUSTC={rustc}");

    let cfg = std::env::var("LLVM_CONFIG_PATH").unwrap_or_else(|_| "llvm-config".into());
    let llvm = probe(&cfg, "--version")
        .or_else(|| probe("llvm-config-21", "--version"))
        .unwrap_or_else(|| "unknown".into());
    println!("cargo:rustc-env=TPLC_LLVM={llvm}");

    let target = std::env::var("TARGET").unwrap_or_else(|_| "unknown".into());
    println!("cargo:rustc-env=TPLC_HOST={target}");

    println!("cargo:rerun-if-env-changed=LLVM_CONFIG_PATH");

    // tplc-rt ships as modules (src/*.rs) but deployable artifacts embed
    // it as ONE translation unit — cmd_build.rs rustc's the flattened
    // source into a staticlib. Rebuild the flat file whenever any rt
    // source changes.
    let rt_dir = std::path::Path::new("../tplc-rt/src");
    let lib = std::fs::read_to_string(rt_dir.join("lib.rs"))
        .expect("tplc-rt/src/lib.rs");
    let mut flat = String::new();
    let mut mods = Vec::new();
    for line in lib.lines() {
        if let Some(rest) = line.strip_prefix("mod ") {
            if let Some(name) = rest.strip_suffix(';') {
                mods.push(name.trim().to_string());
                continue;
            }
        }
        if line.starts_with("pub use ") || line.starts_with("# [allow")
            || line.starts_with("#[allow(unused_imports)]") {
            continue;
        }
        flat.push_str(line);
        flat.push('\n');
    }
    for m in &mods {
        let p = rt_dir.join(format!("{m}.rs"));
        println!("cargo:rerun-if-changed={}", p.display());
        let body = std::fs::read_to_string(&p)
            .unwrap_or_else(|e| panic!("{}: {e}", p.display()));
        for line in body.lines() {
            // module-mode scaffolding is meaningless in the flat TU
            if line.starts_with("//!") || line.trim() == "use crate::*;" {
                continue;
            }
            flat.push_str(line);
            flat.push('\n');
        }
    }
    println!("cargo:rerun-if-changed={}", rt_dir.join("lib.rs").display());
    let out = std::path::Path::new(&std::env::var("OUT_DIR").unwrap())
        .join("tplc_rt_flat.rs");
    std::fs::write(&out, flat).expect("write tplc_rt_flat.rs");
}
