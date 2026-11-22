//! `tplcc` — Tether PLC compiler driver (P1-08).
//!
//!   tplcc check <src.st>…            parse + sema, print diags
//!   tplcc build <src.st>… -o p.tplc  full pipeline → .tplc package
//!   tplcc info <p.tplc>              dump package header + sections
//!
//! `check` and `build` accept `--json`: diagnostics go to stdout as a
//! `{"ok",…,"diags":[…]}` report (build adds `"package"` on success)
//! instead of the human rendering. Used by the ctl proxy's /compile
//! route for the web IDE.
//!
//! Subcommand implementations live in `cmd_*.rs`; shared pipeline
//! helpers in `helpers.rs`; library/source helpers in `libs.rs`.

mod cmd_blocks;
mod cmd_build;
mod cmd_check;
mod cmd_info;
mod cmd_pkg;
mod cmd_serve;
mod cmd_sign;
mod cmd_src;
mod cmd_vectors;
mod helpers;
mod libs;

use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("usage: tplcc <check|blocks|build|info|serve|vectors|sign|src> …");
        return ExitCode::from(2);
    }
    match args[0].as_str() {
        "check" => cmd_check::cmd_check(&args[1..]),
        "blocks" => cmd_blocks::cmd_blocks(&args[1..]),
        "src" => cmd_src::cmd_src(&args[1..]),
        "build" => cmd_build::cmd_build(&args[1..]),
        "info" => cmd_info::cmd_info(&args[1..]),
        "serve" => cmd_serve::cmd_serve(&args[1..]),
        "vectors" => cmd_vectors::cmd_vectors(&args[1..]),
        "sign" => cmd_sign::cmd_sign(&args[1..]),
        "pkg" => cmd_pkg::cmd_pkg(&args[1..]),
        c => {
            eprintln!("tplcc: unknown command {c}");
            ExitCode::from(2)
        }
    }
}
