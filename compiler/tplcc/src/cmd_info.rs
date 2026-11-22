use std::process::ExitCode;
pub(crate) fn cmd_info(files: &[String]) -> ExitCode {
    for f in files {
        let bytes = std::fs::read(f).unwrap_or_else(|e| panic!("{f}: {e}"));
        let pkg = tplc_pkg::reader::Package::parse(&bytes).unwrap_or_else(|e| panic!("{f}: {e}"));
        println!("{f}: {} section(s)", pkg.toc().len());
        if let Ok(m) = pkg.manifest_json() {
            println!("manifest: {m}");
        }
        for (i, s) in pkg.sections().enumerate() {
            println!(
                "  #{i} type=0x{:04x} v{} {}B{}",
                s.section_type,
                s.section_version,
                s.data.len(),
                if s.required() { " required" } else { "" }
            );
        }
    }
    ExitCode::SUCCESS
}

