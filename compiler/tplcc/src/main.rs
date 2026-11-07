//! `tplcc` — Tether PLC compiler driver (P1-08).
//!
//!   tplcc check <src.st>…            parse + sema, print diags
//!   tplcc build <src.st>… -o p.tplc  full pipeline → .tplc package
//!   tplcc info <p.tplc>              dump package header + sections

use inkwell::context::Context;
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use std::process::ExitCode;
use tplc_codegen::llvm_emit::{emit_bitcode, emit_object, emit_program};
use tplc_mir::desc;
use tplc_mir::ir::{AtRegion, IoBind, Pou, PouKind, Prov};
use tplc_mir::layout::layout_program;
use tplc_pkg::format::SectionType;
use tplc_pkg::writer::PackageWriter;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("usage: tplcc <check|build|info|vectors> …");
        return ExitCode::from(2);
    }
    match args[0].as_str() {
        "check" => cmd_check(&args[1..]),
        "build" => cmd_build(&args[1..]),
        "info" => cmd_info(&args[1..]),
        "vectors" => cmd_vectors(&args[1..]),
        c => {
            eprintln!("tplcc: unknown command {c}");
            ExitCode::from(2)
        }
    }
}

fn read_srcs(files: &[String]) -> Vec<(String, String)> {
    files
        .iter()
        .map(|f| {
            let s = std::fs::read_to_string(f)
                .unwrap_or_else(|e| panic!("{f}: {e}"));
            (f.clone(), s)
        })
        .collect()
}

fn print_diags(diags: &[tplc_st::Diag]) -> usize {
    let mut errs = 0;
    for d in diags {
        eprintln!("{:?} [{}] {}", d.severity, d.code, d.message);
        if d.severity == tplc_st::Severity::Error {
            errs += 1;
        }
    }
    errs
}

fn cmd_check(files: &[String]) -> ExitCode {
    let srcs = read_srcs(files);
    let refs: Vec<(&str, &str)> =
        srcs.iter().map(|(f, s)| (f.as_str(), s.as_str())).collect();
    let mut out = tplc_st::compile(&refs);
    let mut errs = print_diags(&out.diags);
    if errs == 0 {
        errs += print_diags(&tplc_st::sema::check(&mut out.module));
    }
    if errs > 0 {
        eprintln!("{errs} error(s)");
        ExitCode::FAILURE
    } else {
        println!("ok: {} pou(s)", out.module.pous.len());
        ExitCode::SUCCESS
    }
}

fn cmd_build(args: &[String]) -> ExitCode {
    let mut out_path = PathBuf::from("out.tplc");
    let mut files: Vec<String> = Vec::new();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-o" => {
                out_path = PathBuf::from(&args[i + 1]);
                i += 1;
            }
            f => files.push(f.into()),
        }
        i += 1;
    }
    if files.is_empty() {
        eprintln!("tplcc build: no sources");
        return ExitCode::from(2);
    }
    let srcs = read_srcs(&files);
    let refs: Vec<(&str, &str)> =
        srcs.iter().map(|(f, s)| (f.as_str(), s.as_str())).collect();
    let mut out = tplc_st::compile(&refs);
    let mut errs = print_diags(&out.diags);
    if errs == 0 {
        errs += print_diags(&tplc_st::sema::check(&mut out.module));
    }
    if errs > 0 {
        eprintln!("{errs} error(s)");
        return ExitCode::FAILURE;
    }

    let program: Pou = out
        .module
        .pous
        .iter()
        .find(|p| p.kind == PouKind::Program)
        .cloned()
        .unwrap_or_else(|| panic!("no PROGRAM found"));
    let prog_name = program.name.clone();

    let mut layout = layout_program(&out.module, &program);
    // io triples for descriptor channels
    let io_triples: Vec<(String, tplc_mir::ir::At, u64)> = out
        .io_bindings
        .iter()
        .map(|b| (b.name.clone(), b.at.clone(), b.at.byte))
        .collect();
    let prov_of = |name: &str| -> Option<Prov> {
        let short = name.rsplit('/').next().unwrap_or(name);
        program
            .iface
            .iter()
            .find(|v| v.name == short)
            .map(|v| v.prov.clone())
    };
    let descs = desc::emit(&mut layout, &program, &io_triples, &prov_of);
    let layout_hash: [u8; 32] = Sha256::digest(&descs.payload_for_hash).into();

    // uuid from source hash → deterministic per program content
    let mut h = Sha256::new();
    for (_, s) in &srcs {
        h.update(s.as_bytes());
    }
    let sh = h.finalize();
    let mut uuid = [0u8; 16];
    uuid.copy_from_slice(&sh[..16]);

    let io_binds: &'static [IoBind] =
        Box::leak(out.io_bindings.clone().into_boxed_slice());
    let module_ref: &'static _ = Box::leak(Box::new(out.module));
    let layout_ref: &'static _ = Box::leak(Box::new(layout));
    let prog_ref: &'static Pou = Box::leak(Box::new(program));

    let lctx = Context::create();
    let llmod = emit_program(&lctx, module_ref, layout_ref, prog_ref,
                             io_binds, &prog_name, uuid, layout_hash);
    if let Err(e) = llmod.verify() {
        eprintln!("tplcc: LLVM verify failed: {e}");
        return ExitCode::FAILURE;
    }

    let tmp = std::env::temp_dir().join("tplcc_build");
    std::fs::create_dir_all(&tmp).unwrap();
    let bc = tmp.join("prog.bc");
    let obj = tmp.join("prog.o");
    emit_bitcode(&llmod, &bc).unwrap();
    emit_object(&llmod, &obj).unwrap();
    let bc_bytes = std::fs::read(&bc).unwrap();

    // link .o + tplc-rt → deployable .so (doc 01 §5: shared object).
    // The rt source is embedded; rustc produces a lean no_std
    // staticlib (~10KB .so, only `tplc_*` exports + abort).
    // TPLC_RT_LIB overrides for prebuilt archives.
    let rt_lib = std::env::var("TPLC_RT_LIB").map(PathBuf::from).unwrap_or_else(|_| {
        const RT_SRC: &str = include_str!("../../tplc-rt/src/lib.rs");
        let src = tmp.join("tplc_rt.rs");
        std::fs::write(&src, RT_SRC).unwrap();
        let a = tmp.join("libtplc_rt.a");
        let rc = std::process::Command::new("rustc")
            .args([
                "--edition", "2021",
                "--crate-type", "staticlib",
                "-C", "panic=abort", "-C", "opt-level=2",
                "--cfg", "feature=\"nostd\"",
            ])
            .arg(&src)
            .arg("-o").arg(&a)
            .output()
            .expect("rustc for tplc-rt");
        if !rc.status.success() {
            eprintln!("tplcc: rt build: {}", String::from_utf8_lossy(&rc.stderr));
            std::process::exit(1);
        }
        a
    });
    let map = tmp.join("exports.map");
    std::fs::write(&map, "{ global: tplc_*; local: *; };
").unwrap();
    let so = tmp.join("prog.so");
    let cc = std::process::Command::new("cc")
        .args(["-shared", "-o"])
        .arg(&so)
        .arg(&obj)
        .arg(&rt_lib)
        .arg(format!("-Wl,--version-script={}", map.display()))
        .output()
        .unwrap();
    if !cc.status.success() {
        eprintln!("tplcc: link failed: {}", String::from_utf8_lossy(&cc.stderr));
        return ExitCode::FAILURE;
    }
    let obj_bytes = std::fs::read(&so).unwrap();

    // retain hash → hex for manifest
    let lh_hex: String = layout_hash.iter().map(|b| format!("{b:02x}")).collect();
    let uuid_str = format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        uuid[0], uuid[1], uuid[2], uuid[3], uuid[4], uuid[5], uuid[6],
        uuid[7], uuid[8], uuid[9], uuid[10], uuid[11], uuid[12], uuid[13],
        uuid[14], uuid[15]
    );
    let io_json: String = out
        .io_bindings
        .iter()
        .map(|b| {
            let r = match b.at.region {
                AtRegion::I => "I",
                AtRegion::Q => "Q",
                AtRegion::M => "M",
            };
            format!(
                "{{\"name\":\"{}\",\"region\":\"{}\",\"byte\":{},\"bit\":{}}}",
                b.name, r, b.at.byte,
                b.at.bit.map(|x| x.to_string()).unwrap_or("null".into())
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let manifest = format!(
        "{{\"schema\":\"tplc.manifest.v1\",\"name\":\"{prog_name}\",\"package_uuid\":\"{uuid_str}\",\"abi_version\":1,\"layout_hash\":\"{lh_hex}\"}}"
    );
    let src_bundle: String = srcs
        .iter()
        .map(|(f, s)| format!("==== {f}\n{s}\n"))
        .collect();

    let mut w = PackageWriter::new();
    w.uuid_str(&uuid_str).unwrap();
    w.add_section(SectionType::Manifest, manifest.into_bytes(), false, 1, 0)
        .unwrap();
    w.add_section(SectionType::IrBitcode, bc_bytes, true, 1, 0).unwrap();
    w.add_section(SectionType::TargetObj, obj_bytes, true, 1, 0).unwrap(); // aux=0 → targets[0]
    w.add_section(SectionType::Symmap, descs.symmap, true, 1, 0).unwrap();
    w.add_section(SectionType::TypeCatalog, descs.typecat, true, 1, 0).unwrap();
    w.add_section(SectionType::LayoutDesc, descs.layd.encode(), true, 1, 0).unwrap();
    w.add_section(SectionType::IoMap, format!("[{io_json}]").into_bytes(), false, 1, 0).unwrap();
    w.add_section(SectionType::StSources, src_bundle.into_bytes(), false, 1, 0).unwrap();
    let pkg = w.build().unwrap();
    std::fs::write(&out_path, pkg).unwrap();
    println!("wrote {}", out_path.display());
    ExitCode::SUCCESS
}

fn cmd_info(files: &[String]) -> ExitCode {
    for f in files {
        let bytes = std::fs::read(f).unwrap_or_else(|e| panic!("{f}: {e}"));
        let pkg = tplc_pkg::reader::Package::parse(&bytes)
            .unwrap_or_else(|e| panic!("{f}: {e}"));
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

/* ------------------------------------------------------------------ */
/* vectors — behavior-vector runner (P1-10, doc 14 §3.1)               */
/* JSONL vectors drive the L1 interpreter; oracle-matiec produces      */
/* expected traces as DATA (quarantined subprocess, never linked).      */
/* ------------------------------------------------------------------ */

use tplc_mir::interp::{program_ctx, Rt};

fn parse_val(s: &str) -> tplc_mir::interp::Value {
    use tplc_mir::interp::Value;
    let u = s.trim();
    match u {
        "TRUE" | "true" => return Value::Bool(true),
        "FALSE" | "false" => return Value::Bool(false),
        _ => {}
    }
    if let Some(t) = u.strip_prefix("T#") {
        let (num, mul) = if let Some(x) = t.strip_suffix("ms") {
            (x, 1_000_000i64)
        } else if let Some(x) = t.strip_suffix("us") {
            (x, 1_000i64)
        } else if let Some(x) = t.strip_suffix("ns") {
            (x, 1i64)
        } else if let Some(x) = t.strip_suffix("s") {
            (x, 1_000_000_000i64)
        } else {
            (t, 1i64)
        };
        let v: f64 = num.trim_start_matches('-').parse().unwrap();
        let neg = num.starts_with('-');
        return Value::Time(if neg { -1 } else { 1 } * (v * mul as f64) as i64);
    }
    if let Some(x) = u.strip_prefix("16#") {
        return Value::I(i64::from_str_radix(x, 16).unwrap());
    }
    if u.contains('.') || u.contains('e') || u.contains('E') {
        return Value::F(u.parse().unwrap());
    }
    if let Ok(v) = u.parse::<i64>() {
        return Value::I(v);
    }
    if let Ok(v) = u.parse::<u64>() {
        return Value::U(v);
    }
    panic!("unparseable vector value {s}");
}

fn hex_bytes(s: &str) -> Vec<u8> {
    let s = s.trim();
    (0..s.len() / 2)
        .map(|i| u8::from_str_radix(&s[2 * i..2 * i + 2], 16).unwrap())
        .collect()
}

/// image-region read for one IoBind at declared address
fn img_get(img: &[u8], b: &IoBind) -> tplc_mir::interp::Value {
    use tplc_mir::interp::Value;
    let byte = *img.get(b.at.byte as usize).unwrap_or(&0);
    match &b.ty {
        tplc_mir::ir::Type::Bool => {
            Value::Bool(byte & (1 << b.at.bit.unwrap_or(0)) != 0)
        }
        tplc_mir::ir::Type::Int { signed, bits } => {
            let n = (*bits as usize) / 8;
            let mut v: u64 = 0;
            for i in 0..n.min(8) {
                v |= (*img.get(b.at.byte as usize + i).unwrap_or(&0) as u64) << (8 * i);
            }
            if *signed {
                Value::I(v as i64)
            } else {
                Value::U(v)
            }
        }
        _ => Value::I(0),
    }
}

/// image-region expected bytes for one IoBind's shadow value
fn img_put(img: &mut [u8], b: &IoBind, v: &tplc_mir::interp::Value) {
    use tplc_mir::interp::Value;
    match (&b.ty, v) {
        (tplc_mir::ir::Type::Bool, Value::Bool(x)) => {
            let m = 1u8 << b.at.bit.unwrap_or(0);
            let e = &mut img[b.at.byte as usize];
            *e = if *x { *e | m } else { *e & !m };
        }
        (tplc_mir::ir::Type::Int { bits, .. }, Value::I(x)) => {
            let n = (*bits as usize) / 8;
            for i in 0..n.min(8) {
                img[b.at.byte as usize + i] = (*x as u64 >> (8 * i)) as u8;
            }
        }
        (tplc_mir::ir::Type::Int { bits, .. }, Value::U(x)) => {
            let n = (*bits as usize) / 8;
            for i in 0..n.min(8) {
                img[b.at.byte as usize + i] = (x >> (8 * i)) as u8;
            }
        }
        _ => {}
    }
}

fn run_vector(v: &serde_json::Value, src_dir: &std::path::Path) -> Result<(), String> {
    let id = v["id"].as_str().unwrap_or("?");
    let prog_file = src_dir.join(v["program"].as_str().unwrap());
    let src = std::fs::read_to_string(&prog_file)
        .map_err(|e| format!("{id}: {prog_file:?}: {e}"))?;
    let mut out = tplc_st::compile(&[("vec.st", &src)]);
    let mut diags = tplc_st::sema::check(&mut out.module);
    diags.extend(out.diags);
    if diags.iter().any(|d| d.severity == tplc_st::Severity::Error) {
        return Err(format!("{id}: frontend errors"));
    }
    let program = out
        .module
        .pous
        .iter()
        .find(|p| p.kind == tplc_mir::ir::PouKind::Program)
        .cloned()
        .ok_or("no PROGRAM")?;
    let layout = layout_program(&out.module, &program);
    let ctx = program_ctx(&layout);
    let mut blob = vec![0u8; layout.total_size as usize];
    let mut img_in = vec![0u8; 64];
    let mut img_out = vec![0u8; 64];

    let tplc_mir::ir::Body::Stmts(stmts) = &program.body else {
        return Err("program body not stmts".into());
    };
    let mut rt = Rt::new(&mut blob, &layout, &out.module, 0);
    let cycles = v["cycles"].as_array().ok_or("no cycles")?;
    for (ci, c) in cycles.iter().enumerate() {
        rt.now_ns = c["t_ns"].as_u64().unwrap_or(0);
        rt.cycle = ci as u64;
        // apply image-in bytes through io bindings
        if let Some(ih) = c["i"].as_str() {
            for (i, b) in hex_bytes(ih).iter().enumerate() {
                if i < img_in.len() {
                    img_in[i] = *b;
                }
            }
        }
        for b in &out.io_bindings {
            if b.at.region == AtRegion::I {
                let val = img_get(&img_in, b);
                let short = b.name.rsplit('.').next().unwrap_or(&b.name);
                rt.write_field(short, &val);
            }
        }
        rt.exec_all(&ctx, stmts);
        // check expect_fields
        if let Some(ef) = c["expect_fields"].as_object() {
            for (name, exp) in ef {
                let want = parse_val(exp.as_str().unwrap());
                let got = if name.contains('.') {
                    let (inst, mem) = name.split_once('.').unwrap();
                    rt.read_ref(&ctx, &tplc_mir::ir::Ref::Field(
                        Box::new(tplc_mir::ir::Ref::Var(inst.into())),
                        mem.into(),
                    ))
                } else {
                    rt.read_field(name)
                };
                let ok = match (&want, &got) {
                    (tplc_mir::interp::Value::Bool(a), tplc_mir::interp::Value::Bool(b)) => a == b,
                    (tplc_mir::interp::Value::Time(a), tplc_mir::interp::Value::Time(b)) => a == b,
                    _ => want.as_i64() == got.as_i64(),
                };
                if !ok {
                    return Err(format!(
                        "{id} cycle {ci}: {name} want {want:?} got {got:?}"
                    ));
                }
            }
        }
        // check expect_q: read Q-region shadows → image bytes
        if let Some(qh) = c["expect_q"].as_str() {
            let want = hex_bytes(qh);
            img_out.iter_mut().for_each(|b| *b = 0);
            for b in &out.io_bindings {
                if b.at.region == AtRegion::Q {
                    let short = b.name.rsplit('.').next().unwrap_or(&b.name);
                    let val = rt.read_field(short);
                    img_put(&mut img_out, b, &val);
                }
            }
            if img_out[..want.len()] != want[..] {
                return Err(format!(
                    "{id} cycle {ci}: %Q want {} got {}",
                    qh,
                    img_out[..want.len()]
                        .iter()
                        .map(|b| format!("{b:02x}"))
                        .collect::<String>()
                ));
            }
        }
    }
    Ok(())
}

fn cmd_vectors(args: &[String]) -> ExitCode {
    let mut files = Vec::new();
    let mut src_dir = PathBuf::from(".");
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "run" => {}
            "--src" => {
                src_dir = PathBuf::from(&args[i + 1]);
                i += 1;
            }
            f => files.push(PathBuf::from(f)),
        }
        i += 1;
    }
    let mut pass = 0;
    let mut fail = 0;
    for f in &files {
        let text = if f.is_dir() {
            let mut t = String::new();
            for e in std::fs::read_dir(f).unwrap() {
                let p = e.unwrap().path();
                if p.extension().map(|x| x == "jsonl").unwrap_or(false) {
                    t += &std::fs::read_to_string(&p).unwrap();
                    src_dir = p.parent().unwrap().to_path_buf();
                }
            }
            t
        } else {
            src_dir = f.parent().unwrap_or(&src_dir).to_path_buf();
            std::fs::read_to_string(f).unwrap()
        };
        for line in text.lines().filter(|l| !l.trim().is_empty()) {
            let v: serde_json::Value = serde_json::from_str(line).unwrap();
            match run_vector(&v, &src_dir) {
                Ok(()) => {
                    pass += 1;
                    println!("PASS {}", v["id"].as_str().unwrap_or("?"));
                }
                Err(e) => {
                    fail += 1;
                    println!("FAIL {e}");
                }
            }
        }
    }
    println!("{pass} pass, {fail} fail");
    if fail > 0 {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
