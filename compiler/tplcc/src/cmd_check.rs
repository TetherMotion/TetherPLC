use std::process::ExitCode;
use crate::helpers::*;
use crate::libs;
pub(crate) fn cmd_check(args: &[String]) -> ExitCode {
    let mut json_mode = false;
    let mut lib_pkgs: Vec<String> = Vec::new();
    let mut libdirs: Vec<String> = Vec::new();
    let mut files: Vec<String> = Vec::new();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--json" => json_mode = true,
            "--lib" => {
                lib_pkgs.push(args[i + 1].clone());
                i += 1;
            }
            "--libdir" => {
                libdirs.push(args[i + 1].clone());
                i += 1;
            }
            f => files.push(f.into()),
        }
        i += 1;
    }
    let mut srcs = read_srcs(&files);
    if !libdirs.is_empty() {
        libs::resolve(&libdirs, &mut srcs, &mut lib_pkgs);
    }
    for lp in &lib_pkgs {
        let mut ls = lib_sources(lp);
        ls.extend(srcs);
        srcs = ls;
    }
    let (mut out, eff) = compile_srcs(&srcs);
    let mut diags = std::mem::take(&mut out.diags);
    if !diags.iter().any(|d| d.severity == tplc_st::Severity::Error) {
        diags.extend(tplc_st::sema::check(&mut out.module));
    }
    let errs = diags
        .iter()
        .filter(|d| d.severity == tplc_st::Severity::Error)
        .count();
    if json_mode {
        println!("{}", diag_report(&diags, &eff, errs, None));
    } else {
        print_diags(&diags, &eff);
    }
    if errs > 0 {
        eprintln!("{errs} error(s)");
        ExitCode::FAILURE
    } else {
        if !json_mode {
            println!("ok: {} pou(s)", out.module.pous.len());
        }
        ExitCode::SUCCESS
    }
}
