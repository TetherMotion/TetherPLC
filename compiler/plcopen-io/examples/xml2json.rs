//! xml2json — convert a PLCopen TC6 XML project to the WebUI
//! `.fbd.json` project format.
//!
//!   xml2json <in.xml> [out.fbd.json|-]
//!
//! `-` reads stdin / writes stdout. Import notes go to stderr.

use std::io::Read;

fn main() -> std::process::ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("usage: xml2json <in.xml> [out.fbd.json|-]");
        return std::process::ExitCode::from(2);
    }
    let mut src = String::new();
    if args[0] == "-" {
        if std::io::stdin().read_to_string(&mut src).is_err() {
            eprintln!("xml2json: stdin read failed");
            return std::process::ExitCode::FAILURE;
        }
    } else {
        match std::fs::read_to_string(&args[0]) {
            Ok(s) => src = s,
            Err(e) => {
                eprintln!("xml2json: {e}");
                return std::process::ExitCode::FAILURE;
            }
        }
    }
    match plcopen_io::import(&src) {
        Ok(rep) => {
            let json = match serde_json::to_string_pretty(&rep.project) {
                Ok(j) => j,
                Err(e) => {
                    eprintln!("xml2json: serialize failed: {e}");
                    return std::process::ExitCode::FAILURE;
                }
            };
            match args.get(1).map(|s| s.as_str()) {
                None | Some("-") => println!("{json}"),
                Some(path) => {
                    if let Err(e) = std::fs::write(path, format!("{json}\n")) {
                        eprintln!("xml2json: {e}");
                        return std::process::ExitCode::FAILURE;
                    }
                }
            }
            for n in &rep.notes {
                eprintln!("note: {}", n.what);
            }
            std::process::ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("xml2json: import failed: {e}");
            std::process::ExitCode::FAILURE
        }
    }
}
