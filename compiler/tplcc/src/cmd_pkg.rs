use std::process::ExitCode;
use tplc_pkg::writer::PackageWriter;
/// `tplcc pkg` — package section plumbing (doc 15 §2 IO_MAP inject,
/// post-compile manifest edits).
///
///   tplcc pkg add-section <in.tplc> --type <u32|0xHEX> --file <F>
///       [-o <out.tplc>]
///
/// Copies every section of <in.tplc> then appends --file's bytes as a
/// new section of --type. Note this invalidates any signature —
/// re-sign afterwards.
pub(crate) fn cmd_pkg(args: &[String]) -> ExitCode {
    if args.first().map(|s| s.as_str()) != Some("add-section") {
        eprintln!("usage: tplcc pkg add-section <in.tplc> --type <T> --file <F> [-o out]");
        return ExitCode::from(2);
    }
    let mut input: Option<String> = None;
    let mut out: Option<String> = None;
    let mut stype: Option<u32> = None;
    let mut sfile: Option<String> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--type" if i + 1 < args.len() => {
                let t = &args[i + 1];
                stype = if let Some(h) = t.strip_prefix("0x") {
                    u32::from_str_radix(h, 16).ok()
                } else {
                    t.parse().ok()
                };
                i += 2;
            }
            "--file" if i + 1 < args.len() => {
                sfile = Some(args[i + 1].clone());
                i += 2;
            }
            "-o" if i + 1 < args.len() => {
                out = Some(args[i + 1].clone());
                i += 2;
            }
            a if !a.starts_with('-') && input.is_none() => {
                input = Some(args[i].clone());
                i += 1;
            }
            a => {
                eprintln!("pkg add-section: bad arg {a}");
                return ExitCode::from(2);
            }
        }
    }
    let (Some(input), Some(stype), Some(sfile)) = (input, stype, sfile) else {
        eprintln!("pkg add-section: needs <in.tplc> --type <T> --file <F>");
        return ExitCode::from(2);
    };
    let out = out.unwrap_or_else(|| input.clone());
    let bytes = std::fs::read(&input).unwrap_or_else(|e| {
        eprintln!("{input}: {e}");
        std::process::exit(1)
    });
    let payload = std::fs::read(&sfile).unwrap_or_else(|e| {
        eprintln!("{sfile}: {e}");
        std::process::exit(1)
    });
    let pkg = tplc_pkg::Package::parse(&bytes).unwrap_or_else(|e| {
        eprintln!("{input}: parse: {e:?}");
        std::process::exit(1)
    });
    let mut w = PackageWriter::new();
    w.uuid_raw(pkg.header.uuid);
    for s in pkg.sections() {
        let _ = w.add_section_raw(
            s.section_type,
            s.data.to_vec(),
            s.required(),
            s.section_version,
            s.aux,
        );
    }
    w.add_section_raw(stype, payload, false, 1, 0)
        .unwrap_or_else(|e| {
            eprintln!("add-section: {e:?}");
            std::process::exit(1)
        });
    let built = w.build().unwrap_or_else(|e| {
        eprintln!("rebuild: {e:?}");
        std::process::exit(1)
    });
    std::fs::write(&out, &built).unwrap_or_else(|e| {
        eprintln!("{out}: {e}");
        std::process::exit(1)
    });
    println!("add-section type 0x{stype:04x} → {out}");
    ExitCode::SUCCESS
}

