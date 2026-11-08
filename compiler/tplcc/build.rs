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
}
