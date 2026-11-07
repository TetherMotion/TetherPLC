//! `tplc-pkg` — inspect, verify, and generate `.tplc` packages.
//!
//! Usage:
//!   tplc-pkg dump <file.tplc>       — header + TOC + per-section info
//!   tplc-pkg verify <file.tplc>     — parse + hash verification only
//!   tplc-pkg gen <out.tplc>         — write the deterministic conformance
//!                                     fixture (fixed uuid/content)

use std::process::ExitCode;
use tplc_pkg::*;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: {} <dump|verify|gen> <file.tplc>", args[0]);
        return ExitCode::from(2);
    }
    if args[1] == "gen" {
        return gen(&args[2]);
    }
    let data = match std::fs::read(&args[2]) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("{}: {}", args[2], e);
            return ExitCode::from(1);
        }
    };
    match Package::parse(&data) {
        Ok(pkg) => {
            if args[1] == "dump" {
                dump(&pkg);
                if let Ok(m) = pkg.manifest_json() {
                    println!("\nmanifest:\n{}", m);
                }
            } else {
                println!(
                    "{}: OK ({} bytes, {} sections)",
                    args[2],
                    data.len(),
                    pkg.toc().len()
                );
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{}: verify failed: {} ({})", args[2], e, e.code());
            ExitCode::from(1)
        }
    }
}

fn dump(pkg: &Package) {
    let h = &pkg.header;
    println!("package uuid : {}", h.uuid_str());
    println!("format       : v{}", h.format_version);
    println!("flags        : 0x{:08x}", h.flags);
    println!("total_size   : {} B", h.total_size);
    println!("header_size  : {} B", h.header_size);
    println!("toc_count    : {}", h.toc_count);
    println!("content_hash : {}", hex(&h.content_hash));
    println!("header_hash  : {}", hex(&h.header_hash));
    println!();
    for (i, s) in pkg.sections().enumerate() {
        let name = s
            .known_type()
            .map(|t| format!("{:?}", t))
            .unwrap_or_else(|| "unknown".into());
        println!(
            "sec {:>2}  {:<16} 0x{:04x}  size {:>6}  ver {}  aux {}  flags 0x{:x}",
            i,
            name,
            s.section_type,
            s.data.len(),
            s.section_version,
            s.aux,
            s.flags
        );
    }
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{:02x}", x)).collect()
}

/// Deterministic fixture identical in structure to `build_sample()` in the
/// test suite — fixed uuid and content so golden files diff cleanly.
fn fixture_bytes() -> Vec<u8> {
    let mut w = PackageWriter::new();
    w.uuid_str("018c2f7e-9a3b-7c4d-8e5f-1234567890ab").unwrap();
    w.add_section(
        SectionType::Manifest,
        br#"{"schema":"tplc.manifest.v1","name":"fixture_pkg","package_uuid":"018c2f7e-9a3b-7c4d-8e5f-1234567890ab","abi_version":1,"layout_hash":"aa"}"#
            .to_vec(),
        true,
        1,
        0,
    )
    .unwrap();
    w.add_section(
        SectionType::IrBitcode,
        b"FAKE-BC-PLACEHOLDER".to_vec(),
        true,
        1,
        0,
    )
    .unwrap();
    w.add_section(
        SectionType::TargetObj,
        vec![0x7f, b'E', b'L', b'F'],
        true,
        1,
        0,
    )
    .unwrap();
    w.add_section(SectionType::Symmap, b"SMAP-STUB".to_vec(), true, 1, 0)
        .unwrap();
    w.add_section(SectionType::TypeCatalog, b"TCAT-STUB".to_vec(), true, 1, 0)
        .unwrap();
    let layd = Layd {
        version: 1,
        total_size: 64,
        regions: vec![
            LaydRegion {
                kind: 0,
                align: 8,
                offset: 0,
                size: 48,
            },
            LaydRegion {
                kind: 3,
                align: 8,
                offset: 48,
                size: 16,
            },
        ],
        fields: vec![
            LaydField {
                sym_entry: u32::MAX,
                field_kind: 0,
                flags: 0,
                gen: 0,
                offset: 0,
                size: 8,
                type_ref: 4,
            },
            LaydField {
                sym_entry: u32::MAX,
                field_kind: 0,
                flags: 1,
                gen: 0,
                offset: 48,
                size: 8,
                type_ref: 4,
            },
        ],
    };
    w.add_section(SectionType::LayoutDesc, layd.encode(), true, 1, 0)
        .unwrap();
    let migd = Migd {
        predecessor_layout_hash: [0xA1; 32],
        predecessor_version: b"v1".to_vec(),
        rules: vec![MigdRule {
            kind: 0,
            severity: 0,
            transform_id: 0,
            old_field: 0,
            new_field: 0,
            aux: 0,
        }],
        predecessor_layd: Layd {
            version: 1,
            total_size: 8,
            regions: vec![LaydRegion {
                kind: 0,
                align: 8,
                offset: 0,
                size: 8,
            }],
            fields: vec![LaydField {
                sym_entry: u32::MAX,
                field_kind: 0,
                flags: 0,
                gen: 0,
                offset: 0,
                size: 8,
                type_ref: 4,
            }],
        },
    };
    w.add_section(SectionType::MigrateDesc, migd.encode(), false, 1, 0)
        .unwrap();
    w.add_section(
        SectionType::Toolchain,
        b"tplc-pkg 0.1 (fixture)".to_vec(),
        false,
        1,
        0,
    )
    .unwrap();
    w.build().unwrap()
}

fn gen(path: &str) -> ExitCode {
    let bytes = fixture_bytes();
    match std::fs::write(path, &bytes) {
        Ok(()) => {
            println!("{}: wrote {} bytes (8 sections)", path, bytes.len());
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{}: {}", path, e);
            ExitCode::from(1)
        }
    }
}
