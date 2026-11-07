//! `tplcc check|build --json` contract — the ctl proxy /compile route
//! (and through it the web IDE) parses stdout as a single
//! `{"ok",…,"diags":[…]}` envelope with file/line/col/offset/len spans.

use serde_json::Value;
use std::io::Write;
use std::process::{Command, Stdio};

const TPLCC: &str = env!("CARGO_BIN_EXE_tplcc");

fn run(args: &[&str], src: &str, name: &str) -> (Value, bool) {
    let dir = std::env::temp_dir().join(format!("tplcc_json_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join(name);
    std::fs::File::create(&path)
        .unwrap()
        .write_all(src.as_bytes())
        .unwrap();
    let mut argv: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    argv.push(path.to_string_lossy().into());
    let out = Command::new(TPLCC)
        .args(&argv)
        .current_dir(&dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .unwrap();
    // stdout must be a single JSON object for the proxy wrapper
    let stdout = String::from_utf8(out.stdout).unwrap();
    let v: Value =
        serde_json::from_str(&stdout).unwrap_or_else(|_| panic!("stdout not JSON: {stdout:?}"));
    (v, out.status.success())
}

#[test]
fn check_json_reports_error_with_span() {
    let src = "PROGRAM P\nVAR x : INT; END_VAR\ny := x;\nEND_PROGRAM\n";
    let (v, ok) = run(&["check", "--json"], src, "bad.st");
    assert!(!ok);
    assert_eq!(v["ok"], false);
    assert_eq!(v["errors"], 1);
    let d = &v["diags"][0];
    assert_eq!(d["severity"], "error");
    // file echoes the path passed on argv (basename through the proxy)
    assert!(d["file"].as_str().unwrap().ends_with("bad.st"));
    // span resolves to the y reference on line 3
    assert_eq!(d["line"], 3);
    assert_eq!(d["offset"], 31);
    assert!(d["col"].as_u64().unwrap() >= 1);
    assert!(d["message"].as_str().unwrap().contains("y"));
}

#[test]
fn check_json_clean_program_is_ok() {
    let src = "PROGRAM P\nVAR x : INT; END_VAR\nx := 1;\nEND_PROGRAM\n";
    let (v, ok) = run(&["check", "--json"], src, "ok.st");
    assert!(ok);
    assert_eq!(v["ok"], true);
    assert_eq!(v["diags"].as_array().unwrap().len(), 0);
}

#[test]
fn build_json_emits_package_path_on_success() {
    let src = "PROGRAM P\nVAR x : INT; END_VAR\nx := 1;\nEND_PROGRAM\n";
    let (v, ok) = run(&["build", "--json", "-o", "out.tplc"], src, "p.st");
    assert!(ok);
    assert_eq!(v["ok"], true);
    let pkg = v["package"].as_str().unwrap();
    assert!(pkg.ends_with("out.tplc"), "package path: {pkg}");
}

#[test]
fn build_json_reports_errors_without_package() {
    let src = "PROGRAM P\nVAR x : INT; END_VAR\ny := x;\nEND_PROGRAM\n";
    let (v, ok) = run(&["build", "--json", "-o", "out.tplc"], src, "bad.st");
    assert!(!ok);
    assert_eq!(v["ok"], false);
    assert!(v.get("package").is_none());
    assert!(v["errors"].as_u64().unwrap() >= 1);
}
