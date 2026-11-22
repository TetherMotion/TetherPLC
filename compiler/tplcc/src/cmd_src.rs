use std::process::ExitCode;
use crate::helpers::*;

/// `tplcc src` — print the ST_SOURCES embedded in a `.tplc` package.
///
///   tplcc src <pkg.tplc> [name]     all sources, or one by name
///
/// Each source prints as `==== <name>` followed by its text (the same
/// framing `lib_sources` parses). Used by the ctl proxy's /libsrc
/// route so the web IDE can attach a library package's sources to a
/// project without unpacking the package itself.
pub(crate) fn cmd_src(args: &[String]) -> ExitCode {
    let mut want: Option<String> = None;
    let mut pkg: Option<String> = None;
    for a in args {
        if a.starts_with('-') {
            eprintln!("tplcc src: unknown flag {a}");
            return ExitCode::from(2);
        }
        if pkg.is_none() {
            pkg = Some(a.clone());
        } else {
            want = Some(a.clone());
        }
    }
    let Some(pkg) = pkg else {
        eprintln!("usage: tplcc src <pkg.tplc> [name]");
        return ExitCode::from(2);
    };
    let srcs = lib_sources(&pkg);
    let mut n = 0;
    for (name, text) in &srcs {
        if let Some(w) = &want {
            // embedded names carry the build-time path — match on the
            // basename so callers can ask for "atc_tempctrl.st"
            let base = name.rsplit('/').next().unwrap_or(name);
            if !name.eq_ignore_ascii_case(w) && !base.eq_ignore_ascii_case(w) {
                continue;
            }
        }
        if want.is_none() {
            println!("==== {name}");
        }
        print!("{text}");
        if !text.ends_with('\n') {
            println!();
        }
        n += 1;
    }
    if n == 0 {
        if want.is_some() {
            eprintln!("tplcc src: {pkg}: no source named {want:?}");
        } else {
            eprintln!("tplcc src: {pkg}: no ST_SOURCES");
        }
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

