use std::path::PathBuf;
use std::process::ExitCode;
use tplc_mir::ir::{AtRegion, IoBind, Pou};
use tplc_mir::layout::layout_program;
use crate::helpers::*;
/* ------------------------------------------------------------------ */
/* vectors — behavior-vector runner (P1-10, doc 14 §3.1)               */
/* JSONL vectors drive the L1 interpreter; oracle-matiec produces      */
/* expected traces as DATA (quarantined subprocess, never linked).      */
/* ------------------------------------------------------------------ */

use tplc_mir::interp::{program_ctx, Flow, Rt};

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
        tplc_mir::ir::Type::Bool => Value::Bool(byte & (1 << b.at.bit.unwrap_or(0)) != 0),
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
        tplc_mir::ir::Type::Real { bits } => {
            let n = (*bits as usize) / 8;
            let mut buf = [0u8; 8];
            for i in 0..n.min(8) {
                buf[i] = *img.get(b.at.byte as usize + i).unwrap_or(&0);
            }
            Value::F(if *bits == 32 {
                f32::from_le_bytes(buf[..4].try_into().unwrap()) as f64
            } else {
                f64::from_le_bytes(buf[..8].try_into().unwrap())
            })
        }
        tplc_mir::ir::Type::Time => {
            let mut v: u64 = 0;
            for i in 0..8 {
                v |= (*img.get(b.at.byte as usize + i).unwrap_or(&0) as u64) << (8 * i);
            }
            Value::Time(v as i64)
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
        (tplc_mir::ir::Type::Real { bits }, Value::F(x)) => {
            let n = (*bits as usize) / 8;
            let bytes = if *bits == 32 {
                (*x as f32).to_le_bytes().to_vec()
            } else {
                x.to_le_bytes().to_vec()
            };
            for i in 0..n.min(8) {
                img[b.at.byte as usize + i] = bytes[i];
            }
        }
        (tplc_mir::ir::Type::Time, Value::Time(x)) => {
            for i in 0..8 {
                img[b.at.byte as usize + i] = (*x as u64 >> (8 * i)) as u8;
            }
        }
        _ => {}
    }
}

type VecRun = (
    tplc_st::FrontendOutput,
    Pou,
    tplc_mir::layout::Layout,
    tplc_mir::interp::Ctx,
    Vec<u8>,
);

fn compile_run(f: &str, src_dir: &std::path::Path, id: &str) -> Result<VecRun, String> {
    let prog_file = src_dir.join(f);
    let src =
        std::fs::read_to_string(&prog_file).map_err(|e| format!("{id}: {prog_file:?}: {e}"))?;
    let (mut out, _eff) = compile_srcs(&[(prog_file.to_string_lossy().into(), src)]);
    let mut diags = std::mem::take(&mut out.diags);
    diags.extend(tplc_st::sema::check(&mut out.module));
    if diags.iter().any(|d| d.severity == tplc_st::Severity::Error) {
        return Err(format!("{id} {f}: frontend errors"));
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
    let blob = vec![0u8; layout.total_size as usize];
    Ok((out, program, layout, ctx, blob))
}

fn run_vector(v: &serde_json::Value, src_dir: &std::path::Path) -> Result<(), String> {
    let id = v["id"].as_str().unwrap_or("?");
    // "program": one file, or "programs": several sources run against the
    // same trace — their %Q images are compared cycle-for-cycle so an
    // .st and an .fbd.json model of the same logic must produce
    // bit-identical outputs (equivalence corpus).
    let files: Vec<String> = match &v["programs"] {
        serde_json::Value::Array(a) => a.iter().map(|p| p.as_str().unwrap().to_string()).collect(),
        _ => vec![v["program"].as_str().unwrap().to_string()],
    };
    // shadow candidates: "shadow" names one file, "shadows" an
    // array — up to 4 may be loaded, each sees identical %I, writes
    // to a private image, and seeds its state name-wise from the
    // active blob at the load boundary. "shadow_load"/"shadow_load@N"
    // loads candidate file N (true = 0); "shadow_promote[@N]" adopts
    // candidate N and drops every peer; "shadow_drop@N" removes one.
    // Candidate indices are load order, matching SHADOW_LIST.
    let shadow_file = v["shadow"].as_str().map(|s| s.to_string());
    let shadow_files: Vec<String> = match &v["shadows"] {
        serde_json::Value::Array(a) => a.iter().map(|p| p.as_str().unwrap().to_string()).collect(),
        _ => shadow_file.map(|f| vec![f]).unwrap_or_default(),
    };
    if !shadow_files.is_empty() && files.len() != 1 {
        return Err(format!("{id}: shadow pairs with a single program"));
    }
    let mut runs: Vec<VecRun> = Vec::new();
    for f in &files {
        runs.push(compile_run(f, src_dir, id)?);
    }
    // "checked_arith": compile every leg (active + shadow) in
    // checked-arithmetic mode: int div/mod by zero faults -19
    let checked = v["checked_arith"].as_bool().unwrap_or(false);
    for r in &mut runs {
        r.0.module.checked_arith = checked;
    }
    // (file index, run) pairs — the file index models the runtime's
    // duplicate-uuid rejection (same source = same program uuid)
    let mut shadows: Vec<(usize, VecRun)> = Vec::new();
    let mut img_in = vec![0u8; 64];
    let cycles = v["cycles"].as_array().ok_or("no cycles")?;
    for (ci, c) in cycles.iter().enumerate() {
        // apply image-in bytes
        if let Some(ih) = c["i"].as_str() {
            for (i, b) in hex_bytes(ih).iter().enumerate() {
                if i < img_in.len() {
                    img_in[i] = *b;
                }
            }
        }
        // "shadow_load": true → file 0, integer N → file N
        let sload = match &c["shadow_load"] {
            serde_json::Value::Bool(b) if *b => Some(0usize),
            serde_json::Value::Number(n) => n.as_u64().map(|x| x as usize),
            _ => None,
        };
        if let Some(fi) = sload {
            let f = shadow_files
                .get(fi)
                .ok_or(format!("{id} cycle {ci}: shadow_load@{fi} w/o shadow file"))?;
            if shadows.len() >= 4 {
                return Err(format!("{id} cycle {ci}: shadow slots full (4)"));
            }
            if shadows.iter().any(|(si, _)| *si == fi) {
                return Err(format!("{id} cycle {ci}: shadow already loaded"));
            }
            let mut sh = compile_run(f, src_dir, id)?;
            sh.0.module.checked_arith = checked;
            // declaration initializers run at load — the L1 model
            // of tplc_init on the candidate program
            {
                let (s_out, _, s_lay, s_ctx, s_blob) = &mut sh;
                let mut srt = Rt::new(s_blob, s_lay, &s_out.module, 0);
                srt.run_inits(s_ctx);
            }
            // seed: same-named fields carry the active blob's value —
            // the L1 model of the runtime's same-layout copy / MIGD
            {
                let (a_out, _, a_lay, _, a_blob) = &mut runs[0];
                let (s_out, _, s_lay, _, s_blob) = &mut sh;
                let art = Rt::new(a_blob, a_lay, &a_out.module, 0);
                let mut srt = Rt::new(s_blob, s_lay, &s_out.module, 0);
                for name in s_lay.fields.keys() {
                    if a_lay.fields.contains_key(name) {
                        let val = art.read_field(name);
                        srt.write_field(name, &val);
                    }
                }
            }
            shadows.push((fi, sh));
        }
        // "shadow_promote": true → candidate 0, integer N → N;
        // adoption drops every peer candidate
        let spr = match &c["shadow_promote"] {
            serde_json::Value::Bool(b) if *b => Some(0usize),
            serde_json::Value::Number(n) => n.as_u64().map(|x| x as usize),
            _ => None,
        };
        if let Some(pi) = spr {
            if pi >= shadows.len() {
                return Err(format!("{id} cycle {ci}: promote@{pi} w/o shadow"));
            }
            runs[0] = shadows.remove(pi).1;
            shadows.clear();
        }
        if let Some(di) = c["shadow_drop"].as_u64() {
            let di = di as usize;
            if di >= shadows.len() {
                return Err(format!("{id} cycle {ci}: drop@{di} w/o shadow"));
            }
            shadows.remove(di);
        }
        let mut q_imgs: Vec<Vec<u8>> = Vec::new();
        let mut aborts: Vec<Option<i32>> = Vec::new();
        for (pi, (out, program, layout, ctx, blob)) in runs.iter_mut().enumerate() {
            let tplc_mir::ir::Body::Stmts(stmts) = &program.body else {
                return Err("program body not stmts".into());
            };
            // state lives in blob; Rt is rebuilt per scan
            let mut rt = Rt::new(blob, layout, &out.module, 0);
            rt.now_ns = c["t_ns"].as_u64().unwrap_or(0);
            rt.cycle = ci as u64;
            /* declaration initializers run once at program start, as
            tplc_init does on the compiled path */
            if ci == 0 {
                rt.run_inits(ctx);
            }
            for b in &out.io_bindings {
                if b.at.region == AtRegion::I {
                    let val = img_get(&img_in, b);
                    let short = b.name.rsplit('.').next().unwrap_or(&b.name);
                    rt.write_field(short, &val);
                }
            }
            let flow = rt.exec_all(ctx, stmts);
            let aborted = matches!(flow, Flow::Abort(_));
            aborts.push(match flow {
                Flow::Abort(rc) => Some(rc),
                _ => None,
            });
            // check expect_fields (applied to every program — the corpus
            // uses identical field names across language variants);
            // a faulted scan leaves mid-statement state; only
            // expect_fault is meaningful there
            if !aborted {
                if let Some(ef) = c["expect_fields"].as_object() {
                    for (name, exp) in ef {
                        let want = parse_val(exp.as_str().unwrap());
                        let got = if name.contains('.') {
                            let (inst, mem) = name.split_once('.').unwrap();
                            rt.read_ref(
                                ctx,
                                &tplc_mir::ir::Ref::Field(
                                    Box::new(tplc_mir::ir::Ref::Var(inst.into())),
                                    mem.into(),
                                ),
                            )
                        } else {
                            rt.read_field(name)
                        };
                        let ok = match (&want, &got) {
                            (
                                tplc_mir::interp::Value::Bool(a),
                                tplc_mir::interp::Value::Bool(b),
                            ) => a == b,
                            (
                                tplc_mir::interp::Value::Time(a),
                                tplc_mir::interp::Value::Time(b),
                            ) => a == b,
                            _ => want.as_i64() == got.as_i64(),
                        };
                        if !ok {
                            return Err(format!(
                                "{id} cycle {ci} {}: {name} want {want:?} got {got:?}",
                                files[pi]
                            ));
                        }
                    }
                }
            }
            let mut img_out = vec![0u8; 64];
            for b in &out.io_bindings {
                if b.at.region == AtRegion::Q {
                    let short = b.name.rsplit('.').next().unwrap_or(&b.name);
                    let val = rt.read_field(short);
                    img_put(&mut img_out, b, &val);
                }
            }
            q_imgs.push(img_out);
        }
        // cross-program equivalence + optional expect_q, skipped on
        // a faulted scan (mid-statement state is not a trace)
        let any_abort = aborts.iter().any(|a| a.is_some());
        if !any_abort {
            for (pi, o) in q_imgs.iter().enumerate().skip(1) {
                if *o != q_imgs[0] {
                    return Err(format!(
                        "{id} cycle {ci}: {} diverges from {}: {} vs {}",
                        files[pi],
                        files[0],
                        hex_str(&o[..4]),
                        hex_str(&q_imgs[0][..4])
                    ));
                }
            }
            if let Some(qh) = c["expect_q"].as_str() {
                let want = hex_bytes(qh);
                if q_imgs[0][..want.len()] != want[..] {
                    return Err(format!(
                        "{id} cycle {ci}: %Q want {} got {}",
                        qh,
                        hex_str(&q_imgs[0][..want.len()])
                    ));
                }
            }
        }
        // task-fault expectation: "expect_fault": <rc> asserts every
        // program leg aborted with that TPLC_TASK_E_* code; absent,
        // any abort is an unexpected fault
        match c["expect_fault"].as_i64() {
            Some(code) => {
                for (pi, a) in aborts.iter().enumerate() {
                    match a {
                        Some(rc) if *rc == code as i32 => {}
                        Some(rc) => {
                            return Err(format!(
                                "{id} cycle {ci}: {} fault rc={rc} want {code}",
                                files[pi]
                            ))
                        }
                        None => {
                            return Err(format!(
                                "{id} cycle {ci}: {} completed, want fault {code}",
                                files[pi]
                            ))
                        }
                    }
                }
            }
            None => {
                for (pi, a) in aborts.iter().enumerate() {
                    if let Some(rc) = a {
                        return Err(format!(
                            "{id} cycle {ci}: {} unexpected task fault rc={rc}",
                            files[pi]
                        ));
                    }
                }
            }
        }
        // shadow %Q expectations: a string addresses candidate 0,
        // an object {"<idx>": "<hex>"} addresses candidates by
        // SHADOW_LIST (load-order) index
        let mut sh_expect: Vec<(usize, String)> = Vec::new();
        match &c["expect_shadow_q"] {
            serde_json::Value::String(s) => sh_expect.push((0, s.clone())),
            serde_json::Value::Object(o) => {
                for (k, val) in o {
                    let i = k
                        .parse::<usize>()
                        .map_err(|_| format!("{id} cycle {ci}: bad shadow idx {k}"))?;
                    sh_expect.push((
                        i,
                        val.as_str()
                            .ok_or(format!("{id} cycle {ci}: expect_shadow_q[{k}]"))?
                            .to_string(),
                    ));
                }
            }
            _ => {}
        }
        for (i, _) in &sh_expect {
            if *i >= shadows.len() {
                return Err(format!("{id} cycle {ci}: expect_shadow_q@{i} w/o shadow"));
            }
        }
        // shadow scans on the same inputs; each candidate's %Q
        // stays private. A faulting candidate drops out, matching
        // the runtime's per-shadow task-fault handling.
        let mut si = 0;
        while si < shadows.len() {
            let (aborted, sh_img) = {
                let (out, program, layout, ctx, blob) = &mut shadows[si].1;
                let tplc_mir::ir::Body::Stmts(stmts) = &program.body else {
                    return Err("shadow body not stmts".into());
                };
                let mut rt = Rt::new(blob, layout, &out.module, 0);
                rt.now_ns = c["t_ns"].as_u64().unwrap_or(0);
                rt.cycle = ci as u64;
                for b in &out.io_bindings {
                    if b.at.region == AtRegion::I {
                        let val = img_get(&img_in, b);
                        let short = b.name.rsplit('.').next().unwrap_or(&b.name);
                        rt.write_field(short, &val);
                    }
                }
                let flow = rt.exec_all(ctx, stmts);
                let mut sh_img = vec![0u8; 64];
                for b in &out.io_bindings {
                    if b.at.region == AtRegion::Q {
                        let short = b.name.rsplit('.').next().unwrap_or(&b.name);
                        let val = rt.read_field(short);
                        img_put(&mut sh_img, b, &val);
                    }
                }
                (matches!(flow, Flow::Abort(_)), sh_img)
            };
            if aborted {
                shadows.remove(si);
                continue;
            }
            for (ei, qh) in &sh_expect {
                if *ei != si {
                    continue;
                }
                let want = hex_bytes(qh);
                if sh_img[..want.len()] != want[..] {
                    return Err(format!(
                        "{id} cycle {ci}: shadow@{si} %Q want {} got {}",
                        qh,
                        hex_str(&sh_img[..want.len()])
                    ));
                }
            }
            si += 1;
        }
    }
    Ok(())
}

fn hex_str(bs: &[u8]) -> String {
    bs.iter().map(|b| format!("{b:02x}")).collect()
}

pub(crate) fn cmd_vectors(args: &[String]) -> ExitCode {
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

#[cfg(test)]
mod mixed_tests {
    use super::*;
    use tplc_mir::interp::{Rt, Value};

    /// st library side of the mixed fixture: an FB, a FUNCTION, and a
    /// PROGRAM that calls the FBD-defined FB `FbdFb`.
    const ST: &str = r#"
FUNCTION_BLOCK StFb
VAR_INPUT X : LREAL; END_VAR
VAR_OUTPUT Y : LREAL; END_VAR
Y := X * 2.0;
END_FUNCTION_BLOCK

FUNCTION StFn : LREAL
VAR_INPUT A : LREAL; END_VAR
StFn := A + 10.0;
END_FUNCTION

PROGRAM StProg
VAR f : FbdFb; r : BOOL; END_VAR
f(IN := FALSE);
r := f.OUT;
END_PROGRAM
"#;

    /// FBD side: Main instantiates ST's StFb + StFn + an inline st
    /// block; FbdFb is defined here and called from the ST program.
    fn fbd(extra_pou: &str) -> String {
        format!(
            r#"{{
  "schema": "fbd.v1",
  "project": "mixed",
  "pous": [
    {{
      "id": "p1", "name": "Main", "kind": "program",
      "interface": [
        {{"name": "pv", "kind": "var", "type": "LREAL", "init": "21.0"}},
        {{"name": "r1", "kind": "var", "type": "LREAL"}},
        {{"name": "r2", "kind": "var", "type": "LREAL"}},
        {{"name": "r3", "kind": "var", "type": "LREAL"}}
      ],
      "networks": [{{
        "id": "n1",
        "blocks": [
          {{"id": 1, "kind": "variable", "role": "in", "name": "pv"}},
          {{"id": 2, "kind": "fb", "type_ref": "StFb", "instance": "st1"}},
          {{"id": 3, "kind": "st",
             "pins": [
               {{"name": "A", "kind": "var_input", "type": "LREAL"}},
               {{"name": "Q", "kind": "var_output", "type": "LREAL"}}
             ],
             "params": {{"CODE": "Q := A * 3.0;"}}}},
          {{"id": 4, "kind": "function", "type_ref": "StFn"}},
          {{"id": 5, "kind": "variable", "role": "out", "name": "r1"}},
          {{"id": 6, "kind": "variable", "role": "out", "name": "r2"}},
          {{"id": 7, "kind": "variable", "role": "out", "name": "r3"}}
        ],
        "wires": [
          {{"id": 10, "src": {{"block": 1, "port": 0}}, "dsts": [{{"block": 2, "port": "X"}}, {{"block": 3, "port": "A"}}]}},
          {{"id": 11, "src": {{"block": 2, "port": "Y"}}, "dsts": [{{"block": 4, "port": 0}}, {{"block": 5, "port": 0}}]}},
          {{"id": 12, "src": {{"block": 4, "port": 0}}, "dsts": [{{"block": 6, "port": 0}}]}},
          {{"id": 14, "src": {{"block": 3, "port": "Q"}}, "dsts": [{{"block": 7, "port": 0}}]}}
        ]
      }}]
    }},
    {{
      "id": "p2", "name": "FbdFb", "kind": "function_block",
      "interface": [
        {{"name": "IN", "kind": "var_input", "type": "BOOL"}},
        {{"name": "OUT", "kind": "var_output", "type": "BOOL"}}
      ],
      "networks": [{{
        "id": "n1",
        "blocks": [
          {{"id": 60, "kind": "variable", "role": "in", "name": "IN"}},
          {{"id": 61, "kind": "operator", "type_ref": "NOT"}},
          {{"id": 62, "kind": "variable", "role": "out", "name": "OUT"}}
        ],
        "wires": [
          {{"id": 63, "src": {{"block": 60, "port": 0}}, "dsts": [{{"block": 61, "port": 0}}]}},
          {{"id": 64, "src": {{"block": 61, "port": 0}}, "dsts": [{{"block": 62, "port": 0}}]}}
        ]
      }}]
    }}{extra_pou}
  ],
  "tasks": [{{"name": "main", "period_ns": 10000000, "programs": ["Main", "StProg"]}}]
}}"#
        )
    }

    fn compile_all(srcs: &[(String, String)]) -> tplc_st::FrontendOutput {
        let (mut out, _eff) = compile_srcs(srcs);
        out.diags.extend(tplc_st::sema::check(&mut out.module));
        out
    }

    fn errs(out: &tplc_st::FrontendOutput) -> Vec<String> {
        out.diags
            .iter()
            .filter(|d| d.severity == tplc_st::Severity::Error)
            .map(|d| format!("{} {}", d.code, d.message))
            .collect()
    }

    /// Run every program in the module once against a *shared* state
    /// blob — the multi-program image layout the runtime uses.
    fn exec_all(out: &tplc_st::FrontendOutput) -> (tplc_mir::layout::Layout, Vec<u8>) {
        let layout = tplc_mir::layout::layout_module(&out.module);
        let mut blob = vec![0u8; layout.total_size as usize];
        for pou in out
            .module
            .pous
            .iter()
            .filter(|p| p.kind == tplc_mir::ir::PouKind::Program)
        {
            let ctx = tplc_mir::interp::pou_scope(&layout, &pou.name);
            let stmts = match &pou.body {
                tplc_mir::ir::Body::Stmts(s) => s.clone(),
                _ => panic!("body not stmts"),
            };
            let mut rt = Rt::new(&mut blob, &layout, &out.module, 0);
            rt.run_inits(&ctx);
            rt.exec_all(&ctx, &stmts);
        }
        (layout, blob)
    }

    /// Read a program var by qualified "PROG.name" or bare name.
    fn read<'a>(
        rt: &Rt<'a>,
        layout: &tplc_mir::layout::Layout,
        prog: &str,
        name: &str,
    ) -> Value {
        let key = if layout.fields.contains_key(&format!("{prog}.{name}")) {
            format!("{prog}.{name}")
        } else {
            name.to_string()
        };
        rt.read_field(&key)
    }

    #[test]
    fn mixed_bidirectional() {
        let srcs = vec![
            ("project.fbd.json".to_string(), fbd("")),
            ("lib.st".to_string(), ST.to_string()),
        ];
        let out = compile_all(&srcs);
        assert_eq!(errs(&out), Vec::<String>::new());

        // shared blob — one image, both programs' vars coexisting
        let (layout, mut blob) = exec_all(&out);
        let rt = Rt::new(&mut blob, &layout, &out.module, 0);
        // FBD program: r1 = 21*2 = 42 (StFb), r2 = 42+10 = 52 (StFn),
        // r3 = 21*3 = 63 (inline st block)
        assert_eq!(read(&rt, &layout, "Main", "r1"), Value::F(42.0));
        assert_eq!(read(&rt, &layout, "Main", "r2"), Value::F(52.0));
        assert_eq!(read(&rt, &layout, "Main", "r3"), Value::F(63.0));
        // ST program: r = NOT FALSE = TRUE via the FBD-defined FbdFb
        assert_eq!(read(&rt, &layout, "StProg", "r"), Value::Bool(true));
    }

    #[test]
    fn mixed_inout_st_block() {
        // an `st` block's var_in_out pin aliases the bound FBD variable;
        // writes inside the generated FB land in the caller's storage.
        // The ST program exercises the same semantics pure-ST (the FB
        // call's inout arg binds n2 by reference).
        let fbd = r#"{
  "schema": "fbd.v1",
  "project": "io",
  "pous": [
    {
      "id": "p1", "name": "Main", "kind": "program",
      "interface": [
        {"name": "n", "kind": "var", "type": "DINT", "init": "5"},
        {"name": "out", "kind": "var", "type": "DINT"}
      ],
      "networks": [{
        "id": "n1",
        "blocks": [
          {"id": 1, "kind": "variable", "role": "in", "name": "n"},
          {"id": 2, "kind": "st",
             "pins": [
               {"name": "BUF", "kind": "var_in_out", "type": "DINT"},
               {"name": "Q", "kind": "var_output", "type": "DINT"}
             ],
             "params": {"CODE": "BUF := BUF + 7; Q := BUF;"}},
          {"id": 3, "kind": "variable", "role": "out", "name": "out"}
        ],
        "wires": [
          {"id": 10, "src": {"block": 1, "port": 0}, "dsts": [{"block": 2, "port": "BUF"}]},
          {"id": 11, "src": {"block": 2, "port": "Q"}, "dsts": [{"block": 3, "port": 0}]}
        ]
      }]
    }
  ],
  "tasks": [{"name": "main", "period_ns": 10000000, "programs": ["Main", "StIo"]}]
}"#;
        let st = r#"
FUNCTION_BLOCK IoFb
VAR_IN_OUT BUF : DINT; END_VAR
BUF := BUF + 1;
END_FUNCTION_BLOCK
PROGRAM StIo
VAR a : IoFb; n2 : DINT; END_VAR
a(BUF := n2);
END_PROGRAM
"#;
        let srcs = vec![
            ("project.fbd.json".to_string(), fbd.to_string()),
            ("lib.st".to_string(), st.to_string()),
        ];
        let out = compile_all(&srcs);
        assert_eq!(errs(&out), Vec::<String>::new());
        let (layout, mut blob) = exec_all(&out);
        let rt = Rt::new(&mut blob, &layout, &out.module, 0);
        // inout write-through: n = 5+7, out mirrors it; n2 = 0+1
        assert_eq!(read(&rt, &layout, "Main", "n"), Value::I(12));
        assert_eq!(read(&rt, &layout, "Main", "out"), Value::I(12));
        assert_eq!(read(&rt, &layout, "StIo", "n2"), Value::I(1));
    }

    #[test]
    fn mixed_dup_impl_errors() {
        // FbdFb also implemented in ST → duplicate impl error
        let st = format!(
            "{}\nFUNCTION_BLOCK FbdFb\nVAR_INPUT IN : BOOL; END_VAR\nVAR_OUTPUT OUT : BOOL; END_VAR\nOUT := IN;\nEND_FUNCTION_BLOCK\n",
            ST.replace("PROGRAM StProg", "PROGRAM_UNUSED StProg")
        );
        let srcs = vec![
            ("project.fbd.json".to_string(), fbd("")),
            ("lib.st".to_string(), st),
        ];
        let out = compile_all(&srcs);
        assert!(errs(&out).iter().any(|e| e.contains("TPLC_E_MIX_DUP_POU")),
            "expected dup impl error, got {:?}", errs(&out));
    }

    #[test]
    fn mixed_decl_stub_merges() {
        // an interface decl for StFb inside the fbd project must merge
        // silently when it matches the ST impl
        let decl = r#",{
      "id": "d1", "name": "StFb", "kind": "function_block",
      "interface": [
        {"name": "X", "kind": "var_input", "type": "LREAL"},
        {"name": "Y", "kind": "var_output", "type": "LREAL"}
      ],
      "networks": []
    }"#;
        let srcs = vec![
            ("project.fbd.json".to_string(), fbd(decl)),
            ("lib.st".to_string(), ST.to_string()),
        ];
        let out = compile_all(&srcs);
        assert_eq!(errs(&out), Vec::<String>::new());
        let stfb = out.module.pous.iter().filter(|p| p.name == "StFb").count();
        assert_eq!(stfb, 1);
    }

    #[test]
    fn mixed_iface_mismatch_errors() {
        // decl with wrong pin type → iface mismatch error
        let decl = r#",{
      "id": "d1", "name": "StFb", "kind": "function_block",
      "interface": [
        {"name": "X", "kind": "var_input", "type": "INT"},
        {"name": "Y", "kind": "var_output", "type": "LREAL"}
      ],
      "networks": []
    }"#;
        let srcs = vec![
            ("project.fbd.json".to_string(), fbd(decl)),
            ("lib.st".to_string(), ST.to_string()),
        ];
        let out = compile_all(&srcs);
        assert!(errs(&out).iter().any(|e| e.contains("TPLC_E_MIX_IFACE")),
            "expected iface mismatch, got {:?}", errs(&out));
    }

    /// An FBD-authored `FUNCTION` POU (ret_ty + implicit return var)
    /// called from ST: the return extracts like an ST function's.
    #[test]
    fn mixed_fbd_function_from_st() {
        let fbd = r#"{
  "schema": "fbd.v1",
  "project": "fn",
  "pous": [
    {
      "id": "f1", "name": "FnX", "kind": "function", "ret_ty": "LREAL",
      "interface": [{"name": "IN", "kind": "var_input", "type": "LREAL"}],
      "networks": [{
        "id": "n1",
        "blocks": [
          {"id": 1, "kind": "variable", "role": "in", "name": "IN"},
          {"id": 2, "kind": "variable", "role": "out", "name": "FnX"}
        ],
        "wires": [
          {"id": 3, "src": {"block": 1, "port": 0}, "dsts": [{"block": 2, "port": 0}]}
        ]
      }]
    }
  ],
  "tasks": [{"name": "main", "period_ns": 10000000, "programs": ["FnProg"]}]
}"#;
        let st = r#"
PROGRAM FnProg
VAR r : LREAL; END_VAR
r := FnX(21.0) * 2.0;
END_PROGRAM
"#;
        let srcs = vec![
            ("project.fbd.json".to_string(), fbd.to_string()),
            ("lib.st".to_string(), st.to_string()),
        ];
        let out = compile_all(&srcs);
        assert_eq!(errs(&out), Vec::<String>::new());
        let (layout, mut blob) = exec_all(&out);
        let rt = Rt::new(&mut blob, &layout, &out.module, 0);
        assert_eq!(read(&rt, &layout, "FnProg", "r"), Value::F(42.0));
    }

    /// The TetherWebUI curve editor emits `tests/mixed/ui_profile.fbd.json`:
    /// an FB_AtcProfile instance, an inline `st` block that loads the SEG
    /// table member-wise through a var_in_out pin, and an FB_AtcPid wired
    /// SP/PV/OUT. This runs that exact document against the shipped
    /// library and checks the profile actually ramps and soaks.
    #[test]
    fn ui_curve_profile_runs() {
        let srcs = vec![
            (
                "project.fbd.json".to_string(),
                include_str!("../../../tests/mixed/ui_profile.fbd.json").to_string(),
            ),
            (
                "Atc_TempCtrl.st".to_string(),
                include_str!("../../../libraries/tempctrl/atc_tempctrl.st").to_string(),
            ),
        ];
        let out = compile_all(&srcs);
        assert_eq!(errs(&out), Vec::<String>::new());

        let layout = tplc_mir::layout::layout_module(&out.module);
        let mut blob = vec![0u8; layout.total_size as usize];
        let prog = out
            .module
            .pous
            .iter()
            .find(|p| p.name == "Main")
            .expect("Main program");
        let ctx = tplc_mir::interp::pou_scope(&layout, "Main");
        let stmts = match &prog.body {
            tplc_mir::ir::Body::Stmts(s) => s.clone(),
            _ => panic!("body not stmts"),
        };
        let mut rt = Rt::new(&mut blob, &layout, &out.module, 0);
        rt.run_inits(&ctx);
        // run enable + a PV that trails the setpoint (one program → the
        // layout keeps bare field names; qualify only when prefixed)
        let q = |n: &str| {
            if layout.fields.contains_key(&format!("Main.{n}")) {
                format!("Main.{n}")
            } else {
                n.to_string()
            }
        };
        rt.write_field(&q("Run"), &Value::Bool(true));
        rt.write_field(&q("Pv"), &Value::F(25.0));

        // cycle time 0.1 s: seg1 ramps 25→150 over 60 s (600 cycles),
        // seg2 soaks at 150 for 120 s. Sample at 30 s and 90 s.
        // The lowerer orders calls by data dependency, so the config
        // block (it writes `prof`) runs after the profile call — the
        // run request is held and starts on scan 2.
        let member = |rt: &mut Rt, inst: &str, name: &str| {
            rt.read_ref(
                &ctx,
                &tplc_mir::ir::Ref::Field(
                    Box::new(tplc_mir::ir::Ref::Var(inst.to_string())),
                    name.to_string(),
                ),
            )
        };
        let f = |v: Value| match v {
            Value::F(x) => x,
            v => panic!("not float: {v:?}"),
        };
        let mut sp30 = 0.0;
        let mut sp90 = 0.0;
        for i in 0..=900 {
            rt.exec_all(&ctx, &stmts);
            let t = (i + 1) as f64 * 0.1;
            if (t - 30.0).abs() < 1e-6 {
                sp30 = f(member(&mut rt, "prof", "SP"));
            }
            if (t - 90.0).abs() < 1e-6 {
                sp90 = f(member(&mut rt, "prof", "SP"));
            }
        }
        // halfway through the time-ramp: SP must sit near 25+(150-25)/2
        assert!((sp30 - 87.5).abs() < 5.0, "SP at 30 s = {sp30}, want ~87.5");
        // soak reached: SP pinned at the segment target
        assert!((sp90 - 150.0).abs() < 0.5, "SP at 90 s = {sp90}, want 150");
        // config block loaded the whole table: 4 segments, no END yet
        assert_eq!(member(&mut rt, "prof", "NUM_SEG"), Value::I(4));
        assert_eq!(member(&mut rt, "prof", "SegIdx"), Value::I(2));
        // controller runs off the profile SP (PID seeded Kp=2, Tn=60)
        let out_v = f(member(&mut rt, "ctl", "Out"));
        assert!(out_v > 0.0, "controller output {out_v} should be positive");
    }

    /// EK-terminal device blocks end-to-end: a DI channel enables the
    /// PID + PWM, an analog channel feeds the PV, and the PWM bit lands
    /// on a DO channel — the exact graphical wiring the web FBD editor
    /// produces (tests/device/ek_pid.fbd.json is editor-shaped JSON).
    #[test]
    fn ek_device_pid_pwm_chain() {
        let srcs = vec![
            (
                "ek_pid.fbd.json".to_string(),
                include_str!("../../../tests/device/ek_pid.fbd.json").to_string(),
            ),
            (
                "Atc_TempCtrl.st".to_string(),
                include_str!("../../../libraries/tempctrl/atc_tempctrl.st").to_string(),
            ),
        ];
        let out = compile_all(&srcs);
        assert_eq!(errs(&out), Vec::<String>::new());

        // every channel is a real process-image binding
        let binds: Vec<(String, String)> = out
            .io_bindings
            .iter()
            .map(|b| (b.name.clone(), format!("{:?}", b.at)))
            .collect();
        for want in [
            "EK_PID.__dev10_DI0",
            "EK_PID.__dev10_DI1",
            "EK_PID.__dev11_AI0",
            "EK_PID.__dev12_DO0",
        ] {
            assert!(binds.iter().any(|(n, _)| n == want), "missing io binding {want}");
        }

        let layout = tplc_mir::layout::layout_module(&out.module);
        let mut blob = vec![0u8; layout.total_size as usize];
        let prog = out
            .module
            .pous
            .iter()
            .find(|p| p.name == "EK_PID")
            .expect("EK_PID program");
        let ctx = tplc_mir::interp::pou_scope(&layout, "EK_PID");
        let stmts = match &prog.body {
            tplc_mir::ir::Body::Stmts(s) => s.clone(),
            _ => panic!("body not stmts"),
        };
        let mut rt = Rt::new(&mut blob, &layout, &out.module, 0);
        rt.run_inits(&ctx);
        let q = |n: &str| {
            if layout.fields.contains_key(&format!("EK_PID.{n}")) {
                format!("EK_PID.{n}")
            } else {
                n.to_string()
            }
        };
        // bus side: DI0 = run, AI0 = 20.0 °C process value
        rt.write_field(&q("__dev10_DI0"), &Value::Bool(true));
        rt.write_field(&q("__dev11_AI0"), &Value::F(20.0));

        let member = |rt: &mut Rt, inst: &str, name: &str| {
            rt.read_ref(
                &ctx,
                &tplc_mir::ir::Ref::Field(
                    Box::new(tplc_mir::ir::Ref::Var(inst.to_string())),
                    name.to_string(),
                ),
            )
        };
        let f = |v: Value| match v {
            Value::F(x) => x,
            v => panic!("not float: {v:?}"),
        };
        // P-only loop: Out = 0.5*(75-20) = 27.5 → Duty 27.5 % of a 10 s
        // period → Ton = 2.75 s → Q high for t < 2.75 s (≈ scan 275).
        let mut do_early = false;
        let mut do_late = true;
        let mut pid_out = 0.0;
        for i in 0..=400 {
            rt.exec_all(&ctx, &stmts);
            let t = (i + 1) as f64 * 0.01;
            if (t - 2.0).abs() < 1e-6 {
                do_early = matches!(
                    rt.read_field(&q("__dev12_DO0")),
                    Value::Bool(true)
                );
            }
            if (t - 4.0).abs() < 1e-6 {
                do_late = matches!(
                    rt.read_field(&q("__dev12_DO0")),
                    Value::Bool(true)
                );
                pid_out = f(member(&mut rt, "pid", "Out"));
            }
        }
        assert!(pid_out > 20.0, "PID out {pid_out} should be ~27.5");
        assert!(do_early, "DO0 should be high during the PWM on-window");
        assert!(!do_late, "DO0 should be low after Ton elapsed");
    }

    /// The TetherWebUI autotune editor emits
    /// tests/mixed/ui_tune_lqg.fbd.json: FB_AtcKalmanTune +
    /// FB_AtcLqrTune feeding an inline `st` handoff whose VAR_IN_OUT pins
    /// alias both tuners and the FB_AtcLqg controller. The handoff
    /// mirrors tuner outputs to live vars and copies the computed gains
    /// into the controller's input members on the APPLY strobe. Runs
    /// the exact document against the shipped library.
    #[test]
    fn ui_autotune_lqg_runs() {
        let srcs = vec![
            (
                "project.fbd.json".to_string(),
                include_str!("../../../tests/mixed/ui_tune_lqg.fbd.json").to_string(),
            ),
            (
                "Atc_TempCtrl.st".to_string(),
                include_str!("../../../libraries/tempctrl/atc_tempctrl.st").to_string(),
            ),
        ];
        let out = compile_all(&srcs);
        assert_eq!(errs(&out), Vec::<String>::new());

        let layout = tplc_mir::layout::layout_module(&out.module);
        let mut blob = vec![0u8; layout.total_size as usize];
        let prog = out
            .module
            .pous
            .iter()
            .find(|p| p.name == "Main")
            .expect("Main program");
        let ctx = tplc_mir::interp::pou_scope(&layout, "Main");
        let stmts = match &prog.body {
            tplc_mir::ir::Body::Stmts(s) => s.clone(),
            _ => panic!("body not stmts"),
        };
        let mut rt = Rt::new(&mut blob, &layout, &out.module, 0);
        rt.run_inits(&ctx);
        let q = |n: &str| {
            if layout.fields.contains_key(&format!("Main.{n}")) {
                format!("Main.{n}")
            } else {
                n.to_string()
            }
        };
        let member = |rt: &mut Rt, inst: &str, name: &str| {
            rt.read_ref(
                &ctx,
                &tplc_mir::ir::Ref::Field(
                    Box::new(tplc_mir::ir::Ref::Var(inst.to_string())),
                    name.to_string(),
                ),
            )
        };
        let f = |v: Value| match v {
            Value::F(x) => x,
            v => panic!("not float: {v:?}"),
        };
        let b = |v: Value| matches!(v, Value::Bool(true));

        // tuners run (Execute ← TuneRun), apply strobe held low
        rt.write_field(&q("TuneRun"), &Value::Bool(true));
        rt.write_field(&q("TuneApply"), &Value::Bool(false));
        rt.write_field(&q("TuneReset"), &Value::Bool(false));
        rt.write_field(&q("Pv"), &Value::F(50.0));
        rt.write_field(&q("Sp"), &Value::F(80.0));

        // DARE solves within a few scans (t06 converges by cycle 2)
        for _ in 0..8 {
            rt.exec_all(&ctx, &stmts);
        }
        // live mirrors track the tuner instances
        assert!(b(read(&rt, &layout, "Main", "lqr_done")), "lqr_done mirror");
        assert!(b(read(&rt, &layout, "Main", "kal_done")), "kal_done mirror");
        let k1 = f(read(&rt, &layout, "Main", "lqr_k1"));
        assert!(k1 > 0.0, "lqr_k1 mirror {k1} should be a computed gain");
        // gains not yet applied — controller inputs still zero
        assert_eq!(f(member(&mut rt, "pid", "K1")), 0.0);

        // APPLY strobe: handoff copies KAL.L*/LQR.K* into pid's inputs
        rt.write_field(&q("TuneApply"), &Value::Bool(true));
        for _ in 0..3 {
            rt.exec_all(&ctx, &stmts);
        }
        let eps = 1e-9;
        assert!(
            (f(member(&mut rt, "pid", "K1")) - k1).abs() < eps,
            "pid.K1 {} != tuned K1 {k1}",
            f(member(&mut rt, "pid", "K1"))
        );
        assert!(
            (f(member(&mut rt, "pid", "L1")) - f(read(&rt, &layout, "Main", "kal_l1"))).abs() < eps,
            "pid.L1 != kal_l1 mirror"
        );
        assert!(
            (f(member(&mut rt, "pid", "L2")) - f(read(&rt, &layout, "Main", "kal_l2"))).abs() < eps,
            "pid.L2 != kal_l2 mirror"
        );
        assert!(
            f(member(&mut rt, "pid", "K2")) != 0.0 || f(member(&mut rt, "pid", "Ki")) != 0.0,
            "pid K2/Ki should carry tuned gains"
        );
        // the wired controller runs: SP/PV error winds the loop and Out
        // leaves zero — only possible because the editor propagated the
        // plant A/B matrices + OutMin/OutMax onto the pid block
        for _ in 0..40 {
            rt.exec_all(&ctx, &stmts);
        }
        let out = f(read(&rt, &layout, "Main", "Out"));
        assert!(out != 0.0, "pid.Out should be driven by the tuned loop");
        // apply latch semantics (applied flag, RESET unlatch) are
        // covered pure-ST by tests/tempctrl/cases/t13_handoff.st
    }
}
