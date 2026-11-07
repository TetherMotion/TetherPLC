//! Round-trip and adversarial tests for the `.tplc` container.

use tplc_pkg::*;

fn build_sample() -> Vec<u8> {
    let mut w = PackageWriter::new();
    w.uuid_str("018c2f7e-9a3b-7c4d-8e5f-1234567890ab").unwrap();
    // add out of canonical order on purpose — writer must sort
    w.add_section(SectionType::Toolchain, b"tplcc 0.1 rustc 1.99".to_vec(), false, 1, 0).unwrap();
    w.add_section(SectionType::Symmap, vec![0xAA; 100], true, 1, 0).unwrap();
    w.add_section(SectionType::Manifest,
                  br#"{"schema":"tplc.manifest.v1","name":"demo"}"#.to_vec(), true, 1, 0).unwrap();
    w.add_section(SectionType::IrBitcode, vec![0xBB; 64], true, 1, 0).unwrap();
    w.add_section(SectionType::TypeCatalog, vec![0xCC; 32], true, 1, 0).unwrap();
    w.add_section(SectionType::LayoutDesc,
                  Layd {
                      version: 1,
                      total_size: 64,
                      regions: vec![
                          LaydRegion { kind: 0, align: 8, offset: 0, size: 48 },
                          LaydRegion { kind: 3, align: 8, offset: 48, size: 16 },
                      ],
                      fields: vec![
                          LaydField { sym_entry: u32::MAX, field_kind: 0, flags: 0,
                                      gen: 0, offset: 0, size: 8, type_ref: 4 },
                      ],
                  }
                  .encode(),
                  true, 1, 0).unwrap();
    w.add_section(SectionType::TargetObj, vec![0x7f; 16], true, 1, 0).unwrap();
    w.build().unwrap()
}

#[test]
fn roundtrip() {
    let bytes = build_sample();
    let pkg = Package::parse(&bytes).unwrap();
    assert_eq!(pkg.toc().len(), 7);
    assert_eq!(pkg.header.uuid_str(), "018c2f7e-9a3b-7c4d-8e5f-1234567890ab");
    let m = pkg.manifest_json().unwrap();
    assert!(m.contains("tplc.manifest.v1"));
    assert_eq!(pkg.section(SectionType::Symmap).unwrap().data.len(), 100);
    // TOC sorted by type
    let types: Vec<u32> = pkg.toc().iter().map(|e| e.section_type).collect();
    let mut sorted = types.clone();
    sorted.sort();
    assert_eq!(types, sorted);
    // sections 16-aligned
    for e in pkg.toc() {
        assert_eq!(e.offset % 16, 0);
    }
    pkg.validate_for_load(false).unwrap();
}

#[test]
fn layd_inside_package() {
    let bytes = build_sample();
    let pkg = Package::parse(&bytes).unwrap();
    let layd = Layd::decode(pkg.section(SectionType::LayoutDesc).unwrap().data).unwrap();
    assert_eq!(layd.regions.len(), 2);
    assert_eq!(layd.regions[1].kind, 3 /* retain */);
    assert_eq!(layd.fields[0].offset, 0);
}

#[test]
fn tamper_payload_detected() {
    let mut bytes = build_sample();
    // flip a byte inside the symmap payload (find it via TOC)
    let pkg = Package::parse(&bytes).unwrap();
    let off = pkg.toc().iter().find(|e| e.section_type == 0x20).unwrap().offset as usize;
    bytes[off] ^= 0xFF;
    let err = Package::parse(&bytes).unwrap_err();
    assert_eq!(err.code(), PkgError::HashContent.code());
}

#[test]
fn tamper_header_detected() {
    let mut bytes = build_sample();
    bytes[20] ^= 1; // flags field — covered by header_hash
    assert_eq!(Package::parse(&bytes).unwrap_err().code(), PkgError::HashHeader.code());
}

#[test]
fn bad_magic() {
    let mut bytes = build_sample();
    bytes[0] = b'X';
    assert_eq!(Package::parse(&bytes).unwrap_err().code(), PkgError::Magic.code());
}

#[test]
fn unsorted_toc_rejected() {
    let mut bytes = build_sample();
    // Swap two TOC entries (types 0x0001 and 0x0002 entries) — breaks sort.
    // Recomputing content_hash is not needed: TOC corruption fails earlier.
    let toc0 = 160usize;
    let toc1 = 160 + 32;
    for i in 0..32 {
        bytes.swap(toc0 + i, toc1 + i);
    }
    let err = Package::parse(&bytes).unwrap_err();
    assert!(matches!(err, PkgError::Toc | PkgError::HashContent));
}

#[test]
fn unknown_required_section_rejected() {
    let mut w = PackageWriter::new();
    w.add_section(SectionType::Manifest, b"{}".to_vec(), true, 1, 0).unwrap();
    // unknown type id written legitimately — reader must reject it since
    // it's flagged required-for-load
    w.add_section_raw(0x7fff, b"x".to_vec(), true, 1, 0).unwrap();
    let bytes = w.build().unwrap();
    assert_eq!(Package::parse(&bytes).unwrap_err().code(),
               PkgError::UnknownSection.code());
}

#[test]
fn unknown_optional_section_passes() {
    let mut w = PackageWriter::new();
    w.add_section(SectionType::Manifest, b"{}".to_vec(), true, 1, 0).unwrap();
    w.add_section_raw(0x7ffe, b"future".to_vec(), false, 1, 0).unwrap();
    let bytes = w.build().unwrap();
    let pkg = Package::parse(&bytes).unwrap();
    assert_eq!(pkg.sections_of(0x7ffe).next().unwrap().data, b"future");
}

#[test]
fn toc_tamper_detected_by_content_hash() {
    // TOC bytes are covered by content_hash (D-IMPL-13): flipping an
    // otherwise structurally-innocuous field (e.g. aux) must fail.
    let mut bytes = build_sample();
    let toc1 = 160 + 32; // second entry
    bytes[toc1 + 28] ^= 1; // aux field
    assert_eq!(Package::parse(&bytes).unwrap_err().code(),
               PkgError::HashContent.code());
}

#[test]
fn migd_roundtrip() {
    let pred = Layd {
        version: 1,
        total_size: 96,
        regions: vec![LaydRegion { kind: 0, align: 8, offset: 0, size: 96 }],
        fields: vec![
            LaydField { sym_entry: u32::MAX, field_kind: 0, flags: 0, gen: 0,
                        offset: 0, size: 8, type_ref: 4 },
            LaydField { sym_entry: u32::MAX, field_kind: 0, flags: 1, gen: 0,
                        offset: 8, size: 8, type_ref: 4 },
        ],
    };
    let m = Migd {
        predecessor_layout_hash: [0xA1; 32],
        predecessor_version: b"v1".to_vec(),
        rules: vec![
            MigdRule { kind: 0, severity: 0, transform_id: 0,
                       old_field: 0, new_field: 0, aux: 0 },
            MigdRule { kind: 3, severity: 1, transform_id: 0,
                       old_field: 1, new_field: 0, aux: 0 },
        ],
        predecessor_layd: pred.clone(),
    };
    let bytes = m.encode();
    let back = Migd::decode(&bytes).unwrap();
    assert_eq!(back, m);
    assert_eq!(back.rules[1].kind, 3);
    assert_eq!(back.predecessor_layd.fields.len(), 2);
}
