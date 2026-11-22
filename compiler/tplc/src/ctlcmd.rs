//! `tplc ctl` — native port of `tools/tplc_ctl.c`. Same op names, same
//! wire payloads, same human-readable output.
//!
//!   tplc ctl <sock> <op> [args]        — tplc-ctl-compatible form
//!   tplc <op> [args] [--socket S]      — socket via flags/session/default

use crate::ctl::{op, Ctl, CtlError, Rd, Wr, F_ERROR};
use sha2::{Digest, Sha256};
use std::io::Write;

const PROG_NAME_MAX: usize = 64;
const PROG_STATUS_SIZE: usize = 336;
const TASK_STAT_SIZE: usize = 136; /* TplcCtlTaskStat: name[64]+…  */

/* ---------------- arg helpers ---------------- */

fn parse_u64(s: &str) -> u64 {
    s.parse().unwrap_or(0)
}
fn parse_u32(s: &str) -> u32 {
    s.parse().unwrap_or(0)
}

/// 32-hex → raw 16B uuid; otherwise u16 len + bytes (path|uuid form).
fn uuid_or_name(s: &str) -> Result<Vec<u8>, String> {
    if s.len() == 32 {
        let mut u = [0u8; 16];
        let mut ok = true;
        for i in 0..16 {
            match u8::from_str_radix(&s[i * 2..i * 2 + 2], 16) {
                Ok(v) => u[i] = v,
                Err(_) => {
                    ok = false;
                    break;
                }
            }
        }
        if ok {
            return Ok(u.to_vec());
        }
        return Err("bad uuid".into());
    }
    let mut w = Wr::new();
    w.u16(s.len() as u16).bytes(s.as_bytes());
    Ok(w.b)
}

/// shadow selector tail {u16 slen, sel}: 16B uuid or name bytes.
fn append_sel(pre: &mut Vec<u8>, sel: Option<&str>) {
    let Some(sel) = sel else { return };
    if sel.is_empty() {
        return;
    }
    let b = if sel.len() == 32 {
        let mut u = vec![0u8; 16];
        let mut ok = true;
        for i in 0..16 {
            match u8::from_str_radix(&sel[i * 2..i * 2 + 2], 16) {
                Ok(v) => u[i] = v,
                Err(_) => {
                    ok = false;
                    break;
                }
            }
        }
        if ok {
            u
        } else {
            sel.as_bytes().to_vec()
        }
    } else {
        sel.as_bytes().to_vec()
    };
    if b.len() > 256 {
        return;
    }
    let mut w = Wr::new();
    w.u16(b.len() as u16).bytes(&b);
    pre.extend_from_slice(&w.b);
}

fn parse_hex(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err("bad hex".into());
    }
    let mut out = Vec::with_capacity(s.len() / 2);
    let b = s.as_bytes();
    for i in 0..s.len() / 2 {
        let hi = (b[i * 2] as char).to_digit(16);
        let lo = (b[i * 2 + 1] as char).to_digit(16);
        match (hi, lo) {
            (Some(h), Some(l)) => out.push(((h << 4) | l) as u8),
            _ => return Err("bad hex".into()),
        }
    }
    Ok(out)
}

fn cstr_name(b: &[u8], max: usize) -> String {
    let n = b.iter().position(|&c| c == 0).unwrap_or(b.len().min(max));
    String::from_utf8_lossy(&b[..n.min(max)]).into_owned()
}

/* ---------------- reply printers (port of tplc_ctl.c) ------------- */

fn print_prog_status(pl: &[u8]) {
    if pl.len() < PROG_STATUS_SIZE {
        println!("(short payload)");
        return;
    }
    let uuid = &pl[0..16];
    let name = cstr_name(&pl[16..16 + PROG_NAME_MAX], PROG_NAME_MAX);
    let state = pl[80];
    let task_count = u32::from_le_bytes(pl[84..88].try_into().unwrap());
    let cycles = u64::from_le_bytes(pl[88..96].try_into().unwrap());
    let last_fault = i32::from_le_bytes(pl[96..100].try_into().unwrap());
    let force_count = u32::from_le_bytes(pl[100..104].try_into().unwrap());
    let version = cstr_name(&pl[168..200], 32);
    let epoch = u64::from_le_bytes(pl[288..296].try_into().unwrap());
    let fault_task = u32::from_le_bytes(pl[200..204].try_into().unwrap());
    let fault_cycle = u64::from_le_bytes(pl[208..216].try_into().unwrap());
    let retain_restored = pl[216];
    let retain_dirty = u64::from_le_bytes(pl[224..232].try_into().unwrap());
    let retain_flush = u64::from_le_bytes(pl[232..240].try_into().unwrap());
    let deployed_by = u32::from_le_bytes(pl[248..252].try_into().unwrap());
    let swap_stage = pl[272];
    let quarantine = u32::from_le_bytes(pl[280..284].try_into().unwrap());
    let shadow_state = pl[296];
    let shadow_cycles = u64::from_le_bytes(pl[304..312].try_into().unwrap());
    let shadow_missed = u64::from_le_bytes(pl[312..320].try_into().unwrap());
    let shadow_diffs = u64::from_le_bytes(pl[320..328].try_into().unwrap());
    let shadow_last_diff = u64::from_le_bytes(pl[328..336].try_into().unwrap());
    let states = ["stopped", "running", "fault", "empty"];
    println!(
        "program    {}  state {}  uuid {}…",
        name,
        states[(state.min(2)) as usize],
        hex(&uuid[..4])
    );
    println!(
        "cycles     {}   last_fault {}   tasks {}  forces {}",
        cycles, last_fault, task_count, force_count
    );
    println!(
        "version    {}  epoch {}  pkg {}…  layout {}…",
        version,
        epoch,
        hex(&pl[104..108]),
        hex(&pl[136..140])
    );
    println!(
        "deployed   by uid {}  pred {}…  swap_stage {} quarantine {}",
        deployed_by,
        hex(&pl[256..260]),
        swap_stage,
        quarantine
    );
    if shadow_state != 0 {
        println!(
            "shadow     running  cycles {}  missed {}  diffs {}  last_diff c{}",
            shadow_cycles, shadow_missed, shadow_diffs, shadow_last_diff
        );
    }
    if last_fault != 0 {
        println!("fault      task {}  cycle {}", fault_task, fault_cycle);
    }
    println!(
        "retain     restored {}  dirty {} B  last_flush {} ns",
        retain_restored, retain_dirty, retain_flush
    );
    let mut n = task_count as usize;
    if PROG_STATUS_SIZE + n * TASK_STAT_SIZE > pl.len() {
        n = (pl.len() - PROG_STATUS_SIZE) / TASK_STAT_SIZE;
    }
    let tstates = ["idle", "sched", "susp"];
    for i in 0..n {
        let t = &pl[PROG_STATUS_SIZE + i * TASK_STAT_SIZE..];
        let tname = cstr_name(&t[0..64], 64);
        let kind = t[64];
        let tstate = t[65];
        let period = u64::from_le_bytes(t[72..80].try_into().unwrap());
        let runs = u64::from_le_bytes(t[80..88].try_into().unwrap());
        let late = u64::from_le_bytes(t[88..96].try_into().unwrap());
        let last_late = i32::from_le_bytes(t[108..112].try_into().unwrap());
        let faults = u64::from_le_bytes(t[96..104].try_into().unwrap());
        let overruns = u64::from_le_bytes(t[112..120].try_into().unwrap());
        let exec_max = u64::from_le_bytes(t[120..128].try_into().unwrap());
        let exec_avg = u64::from_le_bytes(t[128..136].try_into().unwrap());
        println!(
            "  task {:<8} kind {} state {:<5} period {} ns  runs {}  late {} (max {} ns)  faults {}",
            tname,
            kind,
            tstates[(tstate.min(2)) as usize],
            period,
            runs,
            late,
            last_late,
            faults
        );
        println!(
            "         overruns {}  exec max {} ns  avg {} ns",
            overruns, exec_max, exec_avg
        );
    }
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

fn print_reply(op_code: u16, pl: &[u8], csv_mode: bool) {
    match op_code {
        o if o == op::PROG_STATUS || o == op::TASK_STATS => print_prog_status(pl),
        o if o == op::RT_STATUS => {
            if pl.len() >= 88 {
                let mut r = Rd::new(pl);
                let proto = r.u32();
                let uptime = r.u32();
                let prog_count = r.u32();
                let running = r.u32();
                let master_st = r.u32();
                let plc_st = r.u32();
                let master_hb = r.u64();
                let plc_hb = r.u64();
                let version = cstr_name(&pl[32..96.min(pl.len())], PROG_NAME_MAX);
                println!("runtime    {}  proto {}  uptime {} s", version, proto, uptime);
                println!("programs   {} ({} running)", prog_count, running);
                println!("master_st  {}  hb {}", master_st, master_hb);
                println!("plc_st     {}  hb {}", plc_st, plc_hb);
            } else {
                println!("(short payload)");
            }
        }
        o if o == op::PROG_LAYOUT => {
            println!("uuid  {}", hex(&pl[..16.min(pl.len())]));
            if pl.len() >= 48 {
                println!("layout_hash  {}", hex(&pl[16..48]));
                println!("layd  {} B{}", pl.len() - 48, if pl.len() > 48 { "" } else { " (none stored)" });
            } else {
                println!("(short payload)");
            }
        }
        o if o == op::PROG_SWAP => {
            if pl.len() == 64 {
                println!("swap refused: SWAP_RACE\n  live    {}\n  pkg-pre {}", hex(&pl[..32]), hex(&pl[32..]));
            } else if pl.len() >= 8 {
                let st = u32::from_le_bytes(pl[0..4].try_into().unwrap());
                let fe = u32::from_le_bytes(pl[4..8].try_into().unwrap());
                println!("swap: out_status={} out_first_error={}", st, fe);
            }
        }
        o if o == op::SHADOW_LOAD || o == op::SHADOW_RESET => {
            if pl.len() == 64 {
                println!("shadow load refused: SWAP_RACE\n  live    {}\n  pkg-pre {}", hex(&pl[..32]), hex(&pl[32..]));
            } else if o == op::SHADOW_LOAD && pl.len() >= 17 && pl[0] != 0 {
                println!("shadow loaded  uuid {}", hex(&pl[1..17]));
            } else if pl.len() >= 8 {
                let st = u32::from_le_bytes(pl[0..4].try_into().unwrap());
                let fe = u32::from_le_bytes(pl[4..8].try_into().unwrap());
                println!("{}: out_status={} out_first_error={}", if o == op::SHADOW_LOAD { "shadow load" } else { "shadow reset" }, st, fe);
            }
        }
        o if o == op::SHADOW_LIST => {
            // TplcCtlShadowStatus (128 B): uuid@0 name@16 state@80
            // cycles@88 missed@96 diffs@104 last_diff@112 runs@120
            let mut r = Rd::new(pl);
            let ns = if pl.len() >= 4 { r.u32() } else { 0 };
            println!("shadows {}", ns);
            for i in 0..ns as usize {
                if r.left() < SHADOW_STATUS_SIZE { break; }
                let uuid = &pl[r.o..r.o + 16];
                let name = cstr_name(&pl[r.o + 16..r.o + 16 + PROG_NAME_MAX], PROG_NAME_MAX);
                let cycles = u64::from_le_bytes(pl[r.o + 88..r.o + 96].try_into().unwrap());
                let missed = u64::from_le_bytes(pl[r.o + 96..r.o + 104].try_into().unwrap());
                let diffs = u64::from_le_bytes(pl[r.o + 104..r.o + 112].try_into().unwrap());
                println!("  [{}] {:<20}  uuid {}…  cycles {}  missed {}  diffs {}", i, name, hex(&uuid[..4]), cycles, missed, diffs);
                r.o += SHADOW_STATUS_SIZE;
            }
        }
        _ => print_reply_fallback(op_code, pl, csv_mode),
    }
}

const SHADOW_STATUS_SIZE: usize = 128;

fn print_reply_fallback(op_code: u16, pl: &[u8], csv_mode: bool) {
    match op_code {
        o if o == op::SHADOW_STATUS => {
            if pl.len() == 1 && pl[0] == 0 {
                println!("shadow: none");
            } else if pl.len() >= SHADOW_STATUS_SIZE {
                let name = cstr_name(&pl[16..16 + PROG_NAME_MAX], PROG_NAME_MAX);
                let state = pl[80];
                let cycles = u64::from_le_bytes(pl[88..96].try_into().unwrap());
                let missed = u64::from_le_bytes(pl[96..104].try_into().unwrap());
                let diffs = u64::from_le_bytes(pl[104..112].try_into().unwrap());
                let last_diff = u64::from_le_bytes(pl[112..120].try_into().unwrap());
                let runs = u64::from_le_bytes(pl[120..128].try_into().unwrap());
                println!("shadow     {}  state {}  uuid {}…", name, state, hex(&pl[0..4]));
                println!("cycles     {}  missed {}  diffs {}  last_diff c{}  task_runs {}", cycles, missed, diffs, last_diff, runs);
            } else {
                println!("(short payload)");
            }
        }
        o if o == op::SHADOW_COMPARE => {
            let mut r = Rd::new(pl);
            let nd = if pl.len() >= 4 { r.u32() } else { 0 };
            println!("diffs {}", nd);
            // TplcCtlShadowDiff: cycle u64, img_off u64, len u32, active[32], shadow[32] = 88
            for _ in 0..nd {
                if r.left() < 88 { break; }
                let cycle = r.u64();
                let off = r.u64();
                let len = r.u32() as usize;
                let act = r.bytes(32);
                let shd = r.bytes(32);
                println!("  c{:<6} img+0x{:02x} len {}  act {}  shd {}", cycle, off, len, hex(&act[..len.min(32)]), hex(&shd[..len.min(32)]));
            }
        }
        o if o == op::SHADOW_PROMOTE => {
            if pl.len() >= 24 {
                let ep = u64::from_le_bytes(pl[16..24].try_into().unwrap());
                println!("promoted  uuid {}  epoch {}", hex(&pl[..16]), ep);
            }
        }
        o if o == op::SHADOW_IMAGE || o == op::DBG_READ => {
            for (i, b) in pl.iter().enumerate() {
                if i % 16 == 0 { print!("{:08x}  ", i); }
                print!("{}{}", format!("{b:02x}"), if i % 16 == 15 || i == pl.len() - 1 { "\n" } else { " " });
            }
        }
        o if o == op::IMAGE_STATS => {
            if pl.len() >= 72 {
                let epoch = u64::from_le_bytes(pl[0..8].try_into().unwrap());
                let in_seq = u64::from_le_bytes(pl[8..16].try_into().unwrap());
                let out_seq = u64::from_le_bytes(pl[16..24].try_into().unwrap());
                let in_w = u64::from_le_bytes(pl[24..32].try_into().unwrap());
                let out_w = u64::from_le_bytes(pl[32..40].try_into().unwrap());
                let in_r = u64::from_le_bytes(pl[40..48].try_into().unwrap());
                let out_r = u64::from_le_bytes(pl[48..56].try_into().unwrap());
                let in_sz = u64::from_le_bytes(pl[56..64].try_into().unwrap());
                let out_sz = u64::from_le_bytes(pl[64..72].try_into().unwrap());
                println!("epoch {}  in_seq {}  out_seq {}", epoch, in_seq, out_seq);
                println!("in  size {} writes {} retries {}", in_sz, in_w, in_r);
                println!("out size {} writes {} retries {}", out_sz, out_w, out_r);
            }
        }
        o if o == op::PROG_START || o == op::PROG_STOP || o == op::PROG_KILL
            || o == op::PROG_RESET || o == op::PROG_UNLOAD || o == op::PROG_PREPARE =>
        {
            if pl.len() >= 2 {
                println!("state {} -> {}", pl[0], pl[1]);
            }
        }
        o if o == op::PKG_LIST => {
            let mut r = Rd::new(pl);
            let n = if pl.len() >= 4 { r.u32() } else { 0 };
            println!("packages {}", n);
            for _ in 0..n {
                if r.left() < 24 { break; }
                let u = r.bytes(16);
                let sz = r.u64();
                println!("  uuid {}  {} B", hex(u), sz);
            }
        }
        o if o == op::PROG_LIST => {
            let mut r = Rd::new(pl);
            let n = if pl.len() >= 4 { r.u32() } else { 0 };
            for _ in 0..n {
                if r.left() < 16 { break; }
                let u0 = r.u8();
                r.skip(15);
                let nl = r.u16() as usize;
                let name = String::from_utf8_lossy(r.bytes(nl)).into_owned();
                if r.left() < 4 { break; }
                let state = r.bytes(4)[3];
                println!("uuid {:02x}… {} state {}", u0, name, state);
            }
        }
        o if o == op::DBG_RESOLVE => {
            if pl.len() < 40 {
                println!("(short payload)");
            } else {
                let sym = u32::from_le_bytes(pl[16..20].try_into().unwrap());
                let kind = pl[20];
                let flags = pl[21];
                let region = pl[22];
                let access = pl[23];
                let tr = u32::from_le_bytes(pl[24..28].try_into().unwrap());
                let off = u64::from_le_bytes(pl[28..36].try_into().unwrap());
                let sz = u32::from_le_bytes(pl[36..40].try_into().unwrap());
                println!("uuid  {}", hex(&pl[..16]));
                println!("sym {}  kind {} flags {:02x} region {} access {:02x}", sym, kind, flags, region, access);
                println!("type {}  off {}  size {}", tr, off, sz);
            }
        }
        o if o == op::DBG_STATE => {
            if pl.len() < 44 {
                println!("(short payload)");
            } else {
                let bs = u64::from_le_bytes(pl[16..24].try_into().unwrap());
                let ro = u64::from_le_bytes(pl[24..32].try_into().unwrap());
                let rl = u64::from_le_bytes(pl[32..40].try_into().unwrap());
                let sc = u32::from_le_bytes(pl[40..44].try_into().unwrap());
                println!("uuid  {}", hex(&pl[..16]));
                println!("blob {} B  retain off {} len {}  symbols {}", bs, ro, rl, sc);
            }
        }
        o if o == op::DBG_BREAK || o == op::DBG_CAPTURE || o == op::DBG_BREAK_EXPR || o == op::DBG_CAPTURE_EXPR
            || o == op::DBG_FORCE || o == op::DBG_WATCH_ADD =>
        {
            if pl.len() >= 4 {
                let h = u32::from_le_bytes(pl[0..4].try_into().unwrap());
                println!("armed  handle {}", h);
            }
        }
        o if o == op::DBG_BREAK_LIST => {
            let cmps = ["==", "!=", "<", "<=", ">", ">="];
            let mut r = Rd::new(pl);
            let n = if pl.len() >= 4 { r.u32() } else { 0 };
            println!("breaks {}", n);
            for _ in 0..n {
                if r.left() < 40 { break; }
                let h = r.u32();
                let tix = r.u32();
                let off = r.u64();
                let l = r.u32();
                let c = r.u8();
                let cap = r.u8();
                let stale = r.u8();
                r.skip(1);
                let val = r.u64();
                let hits = r.u64();
                let cap_s = if cap != 0 { " cap" } else { "" };
                let stale_s = if stale != 0 { " stale" } else { "" };
                if c == 0xFF {
                    println!("  h{}{}{} task {}  expr  hits {}", h, cap_s, stale_s, tix, hits);
                } else {
                    println!("  h{}{}{} task {}  blob[{}..{}] {} {}  hits {}", h, cap_s, stale_s, tix, off, off + l as u64, cmps[(c.min(5)) as usize], val, hits);
                }
            }
        }
        o if o == op::DBG_STATE_DIFF => {
            let kinds = ["copy", "xform", "init", "drop", "deny", "alias"];
            let sevs = ["info", "degraded", "deny"];
            let tids = ["", "intwiden", "uintwiden", "realwiden", "strtrunc", "arraygrow"];
            if pl.len() < 8 {
                println!("malformed");
            } else {
                let same = pl[0];
                let mst = pl[1];
                let n = u32::from_le_bytes(pl[4..8].try_into().unwrap());
                let mut q = Rd::new(&pl[8..]);
                println!("state-diff  {}  mig_status {}  rules {}", if same != 0 { "same-layout" } else { "migrated" }, mst, n);
                for _ in 0..n {
                    if q.left() < 60 { break; }
                    // wire: kind u8 sev u8 tid u16 aux u32 old_off u64
                    //       old_len u32 pad u32 new_off u64 new_len u32
                    //       pad u32 old_vl u8 new_vl u8 pad u16
                    //       old_v[8] new_v[8] → 60 B, then name strs
                    let kind = q.u8();
                    let sev = q.u8();
                    let tid = q.u16();
                    let _aux = q.u32();
                    let ooff = q.u64();
                    let olen = q.u32();
                    q.skip(4);
                    let noff = q.u64();
                    let nlen = q.u32();
                    q.skip(4);
                    let ovl = q.u8();
                    let nvl = q.u8();
                    q.skip(2);
                    let ov = q.bytes(8);
                    let nv = q.bytes(8);
                    let ol2 = q.u16() as usize;
                    if ol2 > 127 || q.left() < ol2 { break; }
                    let on = String::from_utf8_lossy(q.bytes(ol2)).into_owned();
                    let nl2 = q.u16() as usize;
                    if nl2 > 127 || q.left() < nl2 { break; }
                    let nn = String::from_utf8_lossy(q.bytes(nl2)).into_owned();
                    println!(
                        "  {:<6} sev={:<8} {:<9} {}[{}:{}] -> {}[{}:{}]",
                        kinds[(kind.min(5)) as usize],
                        sevs[(sev.min(2)) as usize],
                        if (tid as usize) <= 5 { tids[tid as usize] } else { "tid?" },
                        if ol2 > 0 { on.as_str() } else { "-" },
                        ooff,
                        olen,
                        if nl2 > 0 { nn.as_str() } else { "-" },
                        noff,
                        nlen
                    );
                    if ovl != 0 || nvl != 0 {
                        let ovl = ovl.min(8) as usize;
                        let nvl = nvl.min(8) as usize;
                        print!("  {} -> {}", hex(&ov[..ovl]), hex(&nv[..nvl]));
                        println!();
                    }
                }
            }
        }
        o if o == op::RETAIN_STATUS => {
            if pl.len() >= 44 {
                let rlen = u64::from_le_bytes(pl[4..12].try_into().unwrap());
                let rgen = u64::from_le_bytes(pl[12..20].try_into().unwrap());
                let rfl = u64::from_le_bytes(pl[20..28].try_into().unwrap());
                let rrs = u64::from_le_bytes(pl[28..36].try_into().unwrap());
                let rls = u64::from_le_bytes(pl[36..44].try_into().unwrap());
                println!("retain  attached {}  dirty {}  len {}  gen {}  flushes {}  restores {}  losses {}", pl[0], pl[1], rlen, rgen, rfl, rrs, rls);
            } else {
                println!("(short payload)");
            }
        }
        o if o == op::RETAIN_DUMP => {
            println!("retain region  {} B", pl.len());
            for i in (0..pl.len()).step_by(2) {
                if i + 1 < pl.len() {
                    print!("{:02x}{:02x}", pl[i], pl[i + 1]);
                } else {
                    print!("{:02x}", pl[i]);
                }
                if i % 32 == 30 || i + 2 >= pl.len() {
                    println!();
                } else {
                    print!(" ");
                }
            }
            if pl.len() % 2 == 1 || pl.len() % 32 != 0 {
                // already newline-terminated above for odd; ensure trailing
            }
        }
        o if o == op::OD_READ => {
            let tn = ["u8", "u16", "u32", "u64", "i32", "string", "bytes"];
            let an = ["ro", "rw", "wo"];
            if pl.len() < 4 {
                println!("(short payload)");
            } else {
                let ty = pl[0];
                let acc = pl[1];
                let l = u16::from_le_bytes([pl[2], pl[3]]) as usize;
                println!("type {}  access {}  len {}", if (ty as usize) < 7 { tn[ty as usize] } else { "?" }, if (acc as usize) < 3 { an[acc as usize] } else { "?" }, l);
                let l = l.min(pl.len() - 4);
                if ty == 5 {
                    println!("{}", String::from_utf8_lossy(&pl[4..4 + l]));
                } else if ty == 6 {
                    println!("{}", hex(&pl[4..4 + l]));
                } else {
                    let mut v = 0u64;
                    for i in 0..l.min(8) {
                        v |= (pl[4 + i] as u64) << (8 * i);
                    }
                    println!("{} (0x{:x})", v, v);
                }
            }
        }
        o if o == op::OD_LIST => {
            let an = ["ro", "rw", "wo"];
            if pl.len() < 4 {
                println!("(short payload)");
            } else {
                let mut r = Rd::new(pl);
                let n = r.u32();
                println!("od {} entries", n);
                for _ in 0..n {
                    if r.left() < 8 { break; }
                    let idx = r.u16();
                    let sub = r.u8();
                    let acc = r.u8();
                    let ol = r.u16() as usize;
                    let nl = r.u16() as usize;
                    if r.left() < nl { break; }
                    let name = String::from_utf8_lossy(r.bytes(nl)).into_owned();
                    println!("  0x{:04x}/{}  {:<2}  {:>5} B  {}", idx, sub, if (acc as usize) < 3 { an[acc as usize] } else { "?" }, ol, name);
                }
            }
        }
        o if o == op::OD_WRITE => {
            println!("{}", if pl.first().copied().unwrap_or(0) != 0 { "ok" } else { "rejected" });
        }
        o if o == op::DBG_FIELDS => {
            if pl.len() < 4 {
                println!("(short payload)");
            } else {
                let mut r = Rd::new(pl);
                let n = r.u32();
                println!("fields {}", n);
                for _ in 0..n {
                    if r.left() < 25 { break; }
                    let fo = r.u64();
                    let fs = r.u64();
                    let tr = r.u32();
                    let kind = r.u8();
                    let fl = r.u8();
                    let reg = r.u8();
                    let acc = r.u8();
                    let nl = r.u8() as usize;
                    if r.left() < nl { break; }
                    let name = String::from_utf8_lossy(r.bytes(nl)).into_owned();
                    println!("  [{:>4}] {:>4} B  t{:<4} k{} f{:02x} r{} a{:02x}  {}", fo, fs, tr, kind, fl, reg, acc, name);
                }
            }
        }
        o if o == op::DBG_FORCE_LIST => {
            let mut r = Rd::new(pl);
            let n = if pl.len() >= 4 { r.u32() } else { 0 };
            println!("forces {}", n);
            for _ in 0..n {
                if r.left() < 16 { break; }
                let h = r.u32();
                let off = r.u64();
                let l = r.u32() as usize;
                let data = r.bytes(l.min(r.left()));
                print!("  handle {}  off {}  {} B  {}", h, off, l, hex(&data[..data.len().min(16)]));
                // mask bytes follow when present
                if r.left() >= l {
                    let mask = r.bytes(l);
                    if mask.iter().any(|&m| m != 0xFF) {
                        print!("  mask {}", hex(&mask[..mask.len().min(16)]));
                    }
                }
                println!();
            }
        }
        o if o == op::TRACE_CONF => {
            if pl.len() >= 4 {
                let fl = u32::from_le_bytes(pl[0..4].try_into().unwrap());
                println!("trace armed  frame {} B", fl);
            }
        }
        o if o == op::TRACE_STATUS => {
            if pl.len() >= 24 {
                let nch = u32::from_le_bytes(pl[0..4].try_into().unwrap());
                let fl = u32::from_le_bytes(pl[4..8].try_into().unwrap());
                let wr = u64::from_le_bytes(pl[8..16].try_into().unwrap());
                let av = u64::from_le_bytes(pl[16..24].try_into().unwrap());
                let fl2 = if pl.len() >= 28 { u32::from_le_bytes(pl[24..28].try_into().unwrap()) } else { 0 };
                println!("trace  ch {}  frame {} B  written {}  avail {}{}", nch, fl, wr, av, if fl2 & 1 != 0 { "  FROZEN" } else { "" });
            }
        }
        o if o == op::TRACE_READ => {
            if pl.len() < 16 {
                println!("(short payload)");
            } else {
                let first = u64::from_le_bytes(pl[0..8].try_into().unwrap());
                let n = u32::from_le_bytes(pl[8..12].try_into().unwrap());
                let fl = u32::from_le_bytes(pl[12..16].try_into().unwrap()) as usize;
                let mut q = &pl[16..];
                if csv_mode {
                    print!("seq,ts_ns");
                    for c in 0..fl {
                        print!(",ch{}_b{}", c / 8, c % 8);
                    }
                    println!();
                } else {
                    println!("first seq {}  frames {}  frame {} B", first, n, fl);
                }
                for _ in 0..n {
                    if q.len() < 16 + fl { break; }
                    let s = u64::from_le_bytes(q[0..8].try_into().unwrap());
                    let ts = u64::from_le_bytes(q[8..16].try_into().unwrap());
                    q = &q[16..];
                    if csv_mode {
                        print!("{},{}", s, ts);
                        for b in &q[..fl] {
                            print!(",{:02x}", b);
                        }
                        println!();
                    } else {
                        print!("  #{} t={}  ", s, ts);
                        for b in &q[..fl] {
                            print!("{:02x}", b);
                        }
                        println!();
                    }
                    q = &q[fl..];
                }
            }
        }
        o if o == op::HIST_CONF => {
            if pl.len() >= 4 {
                let tot = u32::from_le_bytes(pl[0..4].try_into().unwrap());
                println!("hist armed: {} B frame", tot);
            }
        }
        o if o == op::HIST_LIST => {
            if pl.len() < 4 {
                println!("(short payload)");
            } else {
                let mut r = Rd::new(pl);
                let n = r.u32();
                println!("channels {}", n);
                for _ in 0..n {
                    if r.left() < 2 { break; }
                    let nl = r.u16() as usize;
                    if r.left() < nl + 12 { break; }
                    let name = String::from_utf8_lossy(r.bytes(nl)).into_owned();
                    let off = r.u64();
                    let len = r.u32();
                    println!("  {:<24} off={} len={}", name, off, len);
                }
            }
        }
        o if o == op::DBG_WATCH_GET => {
            if pl.len() >= 4 {
                let mut r = Rd::new(pl);
                let n = r.u32();
                println!("watch items {}", n);
                for i in 0..n {
                    if r.left() < 4 { break; }
                    let l = r.u32() as usize;
                    let data = r.bytes(l.min(r.left()));
                    print!("  [{}] {} B  {}", i, l, if l == 0 { "(stale)".to_string() } else { String::new() });
                    for b in data {
                        print!("{:02x}", b);
                    }
                    println!();
                }
            }
        }
        o if o == op::DBG_WATCH_LIST => {
            if pl.len() >= 4 {
                let mut r = Rd::new(pl);
                let n = r.u32();
                println!("watches {}", n);
                for _ in 0..n {
                    if r.left() < 16 { break; }
                    let h = r.u32();
                    let ni = r.u32();
                    let hz = r.u32();
                    let tl = r.u32();
                    let nst = if r.left() >= 4 { r.u32() } else { 0 };
                    println!("  h{}  items {}  {} B  stream {} Hz{}", h, ni, tl, hz, if nst != 0 { "  STALE" } else { "" });
                }
            }
        }
        o if o == op::MAP_GET => {
            if pl.is_empty() {
                println!("(no deploy map)");
            } else {
                print!("{}", String::from_utf8_lossy(pl));
            }
        }
        o if o == op::JOURNAL_TAIL => {
            if pl.len() < 4 {
                println!("(short payload)");
            } else {
                let mut r = Rd::new(pl);
                let n = r.u32();
                for _ in 0..n {
                    if r.left() < 2 { break; }
                    let sl = r.u16() as usize;
                    if r.left() < sl { break; }
                    println!("{}", String::from_utf8_lossy(r.bytes(sl)));
                }
            }
        }
        o if o == op::EVTLOG_LIST || o == op::EVTLOG_QUERY || o == op::EVTLOG_HISTORY || o == op::EVTLOG_STATS => {
            let sevnames = ["verbose", "info", "warning", "error", "critical"];
            let stnames = ["?", "raised", "cleared", "confirmed"];
            let mut r = Rd::new(pl);
            let mut base = 0u32;
            match o {
                x if x == op::EVTLOG_LIST => {
                    if pl.len() < 8 { println!("(short payload)"); return; }
                    let live = r.u32();
                    base = r.u32();
                    println!("live {}  shown {}", live, base);
                }
                x if x == op::EVTLOG_HISTORY => {
                    if pl.len() < 4 { println!("(short payload)"); return; }
                    base = r.u32();
                }
                x if x == op::EVTLOG_QUERY => {
                    base = 1;
                }
                x if x == op::EVTLOG_STATS => {
                    if pl.len() < 24 { println!("(short payload)"); return; }
                    let lv = r.u32();
                    let ra = r.u32();
                    let cl = r.u32();
                    let cf = r.u32();
                    let dr = r.u32();
                    println!("live {}  raised {}  cleared {}  confirmed {}  dropped {}", lv, ra, cl, cf, dr);
                    return;
                }
                _ => {}
            }
            for _ in 0..base {
                if o == op::EVTLOG_HISTORY {
                    if r.left() < 33 { break; }
                    let ts = r.u64();
                    let jid = r.u32();
                    let ec = r.u32();
                    let ei = r.u32();
                    let sv = r.u32();
                    let st = r.u32();
                    let tl = r.u8() as usize;
                    if r.left() < tl { break; }
                    let tb = String::from_utf8_lossy(r.bytes(tl)).into_owned();
                    println!("  #{}  {:<9} sev={:<8} id={} cls={} evt={}  {}", ts / 1_000_000_000, stnames.get(st as usize).copied().unwrap_or("?"), sevnames.get(sv as usize).copied().unwrap_or("?"), jid, ec, ei, tb);
                } else {
                    // wire (ev_write_info): u32 id + 5xu32 + 3xu64 +
                    // u8 len + text = 49+len
                    if r.left() < 49 { break; }
                    let jid = r.u32();
                    let ec = r.u32();
                    let ei = r.u32();
                    let sv = r.u32();
                    let st = r.u32();
                    let ar = r.u32();
                    let rn = r.u64();
                    let cn = r.u64();
                    let fn_ = r.u64();
                    let tl = r.u8() as usize;
                    if r.left() < tl { break; }
                    let tb = String::from_utf8_lossy(r.bytes(tl)).into_owned();
                    println!("  #{:<3} {:<9} sev={:<8} cls={} evt={} args={}  raised={} cleared={} confirmed={}  {}", jid, stnames.get(st as usize).copied().unwrap_or("?"), sevnames.get(sv as usize).copied().unwrap_or("?"), ec, ei, ar, rn / 1_000_000_000, cn / 1_000_000_000, fn_ / 1_000_000_000, tb);
                }
            }
        }
        o if o == op::MQTT_STATUS => {
            // {u32 n, {u32 handle,state,journal_count,port,transport,tls,mqtt_version, str host, str client_id}*}
            if pl.len() >= 4 {
                let mut r = Rd::new(pl);
                let n = r.u32();
                println!("mqtt {} connections", n);
                for _ in 0..n {
                    if r.left() < 28 { break; }
                    let handle = r.u32();
                    let state = r.u32();
                    let jcount = r.u32();
                    let port = r.u32();
                    let transport = r.u32();
                    let tls = r.u32();
                    let mv = r.u32();
                    let host = r.str();
                    let cid = r.str();
                    println!("  h{} state={} journal={} port={} transport={} tls={} mqttv{}  {} ({})", handle, state, jcount, port, transport, tls, mv, host, cid);
                }
            }
        }
        o if o == op::MQTT_JOURNAL => {
            // {i32 total, u32 n, {u64 ts_ms, u32 qos, u32 retain, u32 msgid, str topic, u32 plen, bytes payload}*}
            if pl.len() >= 8 {
                let total = i32::from_le_bytes(pl[0..4].try_into().unwrap());
                let n = u32::from_le_bytes(pl[4..8].try_into().unwrap());
                let mut r = Rd::new(&pl[8..]);
                println!("journal total={} shown={}", total, n);
                for _ in 0..n {
                    if r.left() < 24 { break; }
                    let ts = r.u64();
                    let qos = r.u32();
                    let retain = r.u32();
                    let msgid = r.u32();
                    let topic = r.str();
                    let plen = r.u32() as usize;
                    let payload = r.bytes(plen.min(r.left()));
                    println!("  t{} qos={} retain={} id={} {}  {}B {}", ts, qos, retain, msgid, topic, plen, String::from_utf8_lossy(payload));
                }
            }
        }
        o if o == op::PING && pl.len() >= 8 => {
            let ns = u64::from_le_bytes(pl[0..8].try_into().unwrap());
            println!("pong {} ns", ns);
        }
        _ => {
            // unknown/ack-only replies: hexdump small payloads
            if !pl.is_empty() {
                for (i, b) in pl.iter().enumerate().take(256) {
                    if i % 16 == 0 { print!("{:08x}  ", i); }
                    print!("{}{}", format!("{b:02x}"), if i % 16 == 15 { "\n" } else { " " });
                }
                if pl.len() > 16 { println!(); }
            }
        }
    }
}

/* ---------------- deploy (multi-frame) ---------------- */

fn sha256_file(path: &str) -> Result<(Vec<u8>, u64), String> {
    let data = std::fs::read(path).map_err(|e| format!("cannot read {path}: {e}"))?;
    let sz = data.len() as u64;
    let mut h = Sha256::new();
    h.update(&data);
    Ok((h.finalize().to_vec(), sz))
}

fn do_deploy(sock: &str, path: &str) -> i32 {
    let (sha, sz) = match sha256_file(path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{e}");
            return 1;
        }
    };
    let mut ctl = match Ctl::connect(sock, 3000) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("connect: {e}");
            return 1;
        }
    };
    if ctl.hello("tplc").is_err() {
        eprintln!("hello failed");
        return 1;
    }
    // DEPLOY_BEGIN {u64 size, sha256[32], u8 mode=0}
    let mut begin = Wr::new();
    begin.u64(sz).bytes(&sha).u8(0);
    let pl = match ctl.call(op::DEPLOY_BEGIN, &begin.b) {
        Ok((fl, pl)) => {
            if fl & F_ERROR != 0 {
                eprintln!("{}", crate::ctl::parse_ctl_error(&pl));
                return 1;
            }
            pl
        }
        Err(e) => {
            eprintln!("deploy begin: {e}");
            return 1;
        }
    };
    if pl.len() < 12 {
        eprintln!("deploy begin: short reply");
        return 1;
    }
    let upid = u64::from_le_bytes(pl[0..8].try_into().unwrap());
    let mut maxc = u32::from_le_bytes(pl[8..12].try_into().unwrap()) as usize;
    if maxc == 0 || maxc > 48 * 1024 {
        maxc = 48 * 1024;
    }
    let data = match std::fs::read(path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("cannot read {path}: {e}");
            return 1;
        }
    };
    let mut off = 0usize;
    while off < data.len() {
        let n = (data.len() - off).min(maxc);
        let mut frame = Vec::with_capacity(16 + n);
        frame.extend_from_slice(&upid.to_le_bytes());
        frame.extend_from_slice(&(off as u64).to_le_bytes());
        frame.extend_from_slice(&data[off..off + n]);
        match ctl.call(op::DEPLOY_CHUNK, &frame) {
            Ok((fl, pl)) => {
                if fl & F_ERROR != 0 {
                    eprintln!("{}", crate::ctl::parse_ctl_error(&pl));
                    return 1;
                }
            }
            Err(e) => {
                eprintln!("deploy chunk: {e}");
                return 1;
            }
        }
        off += n;
    }
    match ctl.call(op::DEPLOY_COMMIT, &upid.to_le_bytes()) {
        Ok((fl, pl)) => {
            if fl & F_ERROR != 0 {
                eprintln!("{}", crate::ctl::parse_ctl_error(&pl));
                return 1;
            }
            print!("deployed  uuid ");
            for i in 0..16 {
                if 1 + i < pl.len() {
                    print!("{:02x}", pl[1 + i]);
                }
            }
            println!("  ({} B)", sz);
            0
        }
        Err(e) => {
            eprintln!("deploy commit: {e}");
            1
        }
    }
}

/* ---------------- events subscription ---------------- */

fn cmd_events(sock: &str, secs: u64) -> i32 {
    let mut ctl = match Ctl::connect(sock, 3000) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("connect: {e}");
            return 1;
        }
    };
    if ctl.hello("tplc").is_err() {
        eprintln!("hello failed");
        return 1;
    }
    if ctl.notify(0xFFFF_FFFF).is_err() {
        eprintln!("notify failed");
        return 1;
    }
    let end = std::time::Instant::now() + std::time::Duration::from_secs(secs);
    let evnames = ["?", "TRANSITION", "FAULT", "DBG_BREAK", "DBG_WATCH", "USER"];
    loop {
        // poll with 500 ms slices via read timeout
        ctl.stream_mut()
            .set_read_timeout(Some(std::time::Duration::from_millis(500)))
            .ok();
        match ctl.next_frame() {
            Ok(f) => {
                let t = f.msg_type;
                let nm = if t >= op::EV_PROG_TRANSITION && t <= op::EV_USER {
                    evnames[(t - op::EV_PROG_TRANSITION + 1) as usize]
                } else {
                    "?"
                };
                print!("ev 0x{:04x} {:<11} len={}", t, nm, f.payload.len());
                if t == op::EV_USER && f.payload.len() >= 20 {
                    let jid = u32::from_le_bytes(f.payload[0..4].try_into().unwrap());
                    let st = u32::from_le_bytes(f.payload[4..8].try_into().unwrap());
                    let ec = u32::from_le_bytes(f.payload[8..12].try_into().unwrap());
                    let ei = u32::from_le_bytes(f.payload[12..16].try_into().unwrap());
                    let sv = u32::from_le_bytes(f.payload[16..20].try_into().unwrap());
                    print!(" id={} state={} cls={} evt={} sev={}", jid, st, ec, ei, sv);
                }
                if t == op::EV_PROG_TRANSITION && f.payload.len() >= 40 {
                    let from = f.payload[16];
                    let to = f.payload[17];
                    let reason = f.payload[18];
                    let epoch = u64::from_le_bytes(f.payload[32..40].try_into().unwrap());
                    print!(" {}->{} reason={} epoch={} uuid={}", from, to, reason, epoch, hex(&f.payload[0..2]));
                }
                println!();
                std::io::stdout().flush().ok();
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock || e.kind() == std::io::ErrorKind::TimedOut => {}
            Err(_) => break,
        }
        if std::time::Instant::now() >= end {
            break;
        }
    }
    0
}

/* ---------------- dispatch ---------------- */

/// Run a ctl command line. `args` = everything after the socket when in
/// `tplc ctl <sock> …` form, or the bare op args in `tplc <op> …` form
/// (socket already resolved by caller into `sock`).
pub fn run(sock: &str, args: &[String]) -> i32 {
    if args.is_empty() {
        eprintln!("usage: tplc ctl <sock> <op> [args]  (see `tplc help ctl`)");
        return 2;
    }
    let cmd = args[0].as_str();
    let a = &args[1..];

    // deploy + events are multi-frame flows
    if cmd == "deploy" {
        if a.is_empty() {
            eprintln!("deploy <file.tplc>");
            return 2;
        }
        return do_deploy(sock, &a[0]);
    }
    if cmd == "events" {
        let secs = a.first().map(|s| parse_u64(s)).unwrap_or(5);
        return cmd_events(sock, secs);
    }

    // single op + optional prefix payload
    let mut pre: Vec<u8> = Vec::new();
    let op_code: u16;
    let mut csv_mode = false;
    let arg = a.first().map(|s| s.as_str());
    let arg2 = a.get(1).map(|s| s.as_str());
    let arg3 = a.get(2).map(|s| s.as_str());
    let arg4 = a.get(3).map(|s| s.as_str());
    let arg5 = a.get(4).map(|s| s.as_str());

    macro_rules! need {
        ($e:expr, $($m:tt)*) => {
            if !$e {
                eprintln!($($m)*);
                return 2;
            }
        };
    }

    match cmd {
        "ping" => op_code = op::PING,
        "status" => op_code = op::RT_STATUS,
        "prog" => op_code = op::PROG_STATUS,
        "list" => op_code = op::PROG_LIST,
        "start" => op_code = op::PROG_START,
        "stop" => op_code = op::PROG_STOP,
        "image" => op_code = op::IMAGE_STATS,
        "shutdown" => op_code = op::RT_SHUTDOWN,
        "kill" => op_code = op::PROG_KILL,
        "unload" => op_code = op::PROG_UNLOAD,
        "pkg" => op_code = op::PKG_LIST,
        "layout" => op_code = op::PROG_LAYOUT,
        "rollback" => op_code = op::PROG_ROLLBACK,
        "reset" => {
            op_code = op::PROG_RESET;
            pre.push(if arg == Some("cold") { 0 } else { 1 });
        }
        "prepare" => {
            need!(arg.is_some(), "prepare <path|uuid>");
            op_code = op::PROG_PREPARE;
            match uuid_or_name(arg.unwrap()) {
                Ok(v) => pre = v,
                Err(e) => {
                    eprintln!("{e}");
                    return 2;
                }
            }
        }
        "swap" | "shadow-load" => {
            need!(arg.is_some(), "{} <path|uuid>", cmd);
            op_code = if cmd == "swap" { op::PROG_SWAP } else { op::SHADOW_LOAD };
            match uuid_or_name(arg.unwrap()) {
                Ok(v) => pre = v,
                Err(e) => {
                    eprintln!("{e}");
                    return 2;
                }
            }
        }
        "shadow-status" => {
            op_code = op::SHADOW_STATUS;
            append_sel(&mut pre, arg);
        }
        "shadow-list" => op_code = op::SHADOW_LIST,
        "shadow-compare" => {
            op_code = op::SHADOW_COMPARE;
            let mx = arg.map(parse_u32).unwrap_or(32);
            pre.extend_from_slice(&mx.to_le_bytes());
            append_sel(&mut pre, arg2);
        }
        "shadow-image" => {
            need!(arg.is_some() && arg2.is_some(), "shadow-image <off> <len> [sel]");
            op_code = op::SHADOW_IMAGE;
            let off = parse_u64(arg.unwrap());
            let rl = parse_u32(arg2.unwrap());
            pre.extend_from_slice(&off.to_le_bytes());
            pre.extend_from_slice(&rl.to_le_bytes());
            append_sel(&mut pre, arg3);
        }
        "shadow-promote" => {
            op_code = op::SHADOW_PROMOTE;
            append_sel(&mut pre, arg);
        }
        "shadow-drop" => {
            op_code = op::SHADOW_DROP;
            append_sel(&mut pre, arg);
        }
        "shadow-reset" => {
            op_code = op::SHADOW_RESET;
            append_sel(&mut pre, arg);
        }
        "suspend" | "resume" => {
            need!(arg.is_some(), "{} <task_idx>", cmd);
            op_code = if cmd == "suspend" { op::TASK_SUSPEND } else { op::TASK_RESUME };
            pre.extend_from_slice(&parse_u32(arg.unwrap()).to_le_bytes());
        }
        "step" => {
            need!(arg.is_some(), "step <task_idx> [n]");
            op_code = op::TASK_STEP;
            let ti = parse_u32(arg.unwrap());
            let nn = arg2.map(parse_u32).unwrap_or(1);
            pre.extend_from_slice(&ti.to_le_bytes());
            pre.extend_from_slice(&nn.to_le_bytes());
        }
        "event" => {
            need!(arg.is_some(), "event <task_idx>");
            op_code = op::TASK_EVENT;
            pre.extend_from_slice(&parse_u32(arg.unwrap()).to_le_bytes());
        }
        "resolve" => {
            need!(arg.is_some(), "resolve <name>");
            op_code = op::DBG_RESOLVE;
            let nl = arg.unwrap().len() as u16;
            pre.extend_from_slice(&nl.to_le_bytes());
            pre.extend_from_slice(arg.unwrap().as_bytes());
        }
        "dbg-state" => op_code = op::DBG_STATE,
        "dbg-read" => {
            need!(arg.is_some() && arg2.is_some(), "dbg-read <off> <len>");
            op_code = op::DBG_READ;
            let off = parse_u64(arg.unwrap());
            let rl = parse_u32(arg2.unwrap());
            pre.extend_from_slice(&off.to_le_bytes());
            pre.extend_from_slice(&rl.to_le_bytes());
            if let Some(s) = arg3 {
                pre.push(parse_u32(s) as u8);
            }
        }
        "dbg-write" => {
            need!(arg.is_some() && arg2.is_some(), "dbg-write <off> <hex>");
            op_code = op::DBG_WRITE;
            let off = parse_u64(arg.unwrap());
            let bytes = match parse_hex(arg2.unwrap()) {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("{e}");
                    return 2;
                }
            };
            pre.extend_from_slice(&off.to_le_bytes());
            pre.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
            pre.extend_from_slice(&bytes);
        }
        "force" => {
            need!(arg.is_some() && arg2.is_some(), "force <off> <hex> [maskhex]");
            op_code = op::DBG_FORCE;
            let off = parse_u64(arg.unwrap());
            let bytes = match parse_hex(arg2.unwrap()) {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("{e}");
                    return 2;
                }
            };
            pre.extend_from_slice(&off.to_le_bytes());
            pre.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
            pre.extend_from_slice(&bytes);
            if let Some(mh) = arg3 {
                if mh.len() != bytes.len() * 2 {
                    eprintln!("mask must match hex length");
                    return 2;
                }
                match parse_hex(mh) {
                    Ok(mask) => pre.extend_from_slice(&mask),
                    Err(e) => {
                        eprintln!("bad mask hex: {e}");
                        return 2;
                    }
                }
            }
        }
        "force-list" => op_code = op::DBG_FORCE_LIST,
        "force-clear" => {
            op_code = op::DBG_FORCE_CLEAR;
            let h = match arg {
                None | Some("all") => 0xFFFF_FFFFu32,
                Some(s) => parse_u32(s),
            };
            pre.extend_from_slice(&h.to_le_bytes());
        }
        "break" | "capture" => {
            need!(a.len() >= 5, "break <task> <off> <len> <eq|ne|lt|le|gt|ge> <val>");
            let cmps = ["eq", "ne", "lt", "le", "gt", "ge"];
            let cmp = cmps.iter().position(|c| Some(*c) == arg4).map(|i| i as u8);
            let Some(cmp) = cmp else {
                eprintln!("bad cmp");
                return 2;
            };
            op_code = if cmd == "capture" { op::DBG_CAPTURE } else { op::DBG_BREAK };
            let ti = parse_u32(arg.unwrap());
            let off = parse_u64(arg2.unwrap());
            let bl = parse_u32(arg3.unwrap());
            let val = parse_u64(arg5.unwrap());
            pre.resize(28, 0);
            pre[0..4].copy_from_slice(&ti.to_le_bytes());
            pre[4..12].copy_from_slice(&off.to_le_bytes());
            pre[12..16].copy_from_slice(&bl.to_le_bytes());
            pre[16] = cmp;
            pre[20..28].copy_from_slice(&val.to_le_bytes());
        }
        "break-list" => op_code = op::DBG_BREAK_LIST,
        "break-clear" => {
            op_code = op::DBG_BREAK_CLEAR;
            let h = match arg {
                None | Some("all") => 0xFFFF_FFFFu32,
                Some(s) => parse_u32(s),
            };
            pre.extend_from_slice(&h.to_le_bytes());
        }
        "trace" => {
            // trace [nofreeze] OFF:LEN [OFF:LEN...]
            let mut base = 0usize;
            let mut flags = 0u32;
            if a.first().map(|s| s.as_str()) == Some("nofreeze") {
                flags = 1;
                base = 1;
            }
            let nch = a.len() - base;
            if nch > 8 {
                eprintln!("max 8 channels");
                return 2;
            }
            op_code = op::TRACE_CONF;
            pre.extend_from_slice(&(nch as u32).to_le_bytes());
            pre.extend_from_slice(&flags.to_le_bytes());
            for i in 0..nch {
                let spec = &a[base + i];
                let Some((os, ls)) = spec.split_once(':') else {
                    eprintln!("bad channel {} (want OFF:LEN, len 1..8)", spec);
                    return 2;
                };
                let off = parse_u64(os);
                let l: u32 = ls.parse().unwrap_or(0);
                if l == 0 || l > 8 {
                    eprintln!("bad channel {} (want OFF:LEN, len 1..8)", spec);
                    return 2;
                }
                pre.extend_from_slice(&off.to_le_bytes());
                pre.extend_from_slice(&l.to_le_bytes());
                pre.resize(pre.len() + 4, 0); // pad to 16B entry
            }
        }
        "trace-status" => op_code = op::TRACE_STATUS,
        "trace-read" => {
            op_code = op::TRACE_READ;
            let from = arg.map(parse_u64).unwrap_or(0);
            let mx = arg2.map(parse_u32).unwrap_or(512);
            pre.extend_from_slice(&from.to_le_bytes());
            pre.extend_from_slice(&mx.to_le_bytes());
        }
        "trace-clear" => op_code = op::TRACE_CLEAR,
        "trace-csv" => {
            op_code = op::TRACE_READ;
            csv_mode = true;
            let from = arg.map(parse_u64).unwrap_or(0);
            let mx = arg2.map(parse_u32).unwrap_or(512);
            pre.extend_from_slice(&from.to_le_bytes());
            pre.extend_from_slice(&mx.to_le_bytes());
        }
        "breakx" | "capturex" => {
            need!(arg.is_some() && arg2.is_some(), "{} <task> <expr>", cmd);
            op_code = if cmd == "capturex" { op::DBG_CAPTURE_EXPR } else { op::DBG_BREAK_EXPR };
            let ti = parse_u32(arg.unwrap());
            let ex = arg2.unwrap();
            if ex.len() > 256 {
                eprintln!("expr > 256 chars");
                return 2;
            }
            pre.extend_from_slice(&ti.to_le_bytes());
            pre.extend_from_slice(&(ex.len() as u16).to_le_bytes());
            pre.extend_from_slice(ex.as_bytes());
        }
        "state-diff" => op_code = op::DBG_STATE_DIFF,
        "fields" => {
            op_code = op::DBG_FIELDS;
            if let Some(s) = arg {
                pre.push(parse_u32(s) as u8);
            }
        }
        "od-read" => {
            need!(arg.is_some(), "od-read INDEX [SUB]");
            op_code = op::OD_READ;
            let idx = parse_u32(arg.unwrap()) as u16;
            let sub = arg2.map(|s| parse_u32(s) as u8).unwrap_or(0);
            pre.extend_from_slice(&idx.to_le_bytes());
            pre.push(sub);
            pre.push(0);
        }
        "od-list" => op_code = op::OD_LIST,
        "od-write" => {
            need!(a.len() >= 3, "od-write INDEX SUB HEX");
            op_code = op::OD_WRITE;
            let idx = parse_u32(&a[0]) as u16;
            let sub = parse_u32(&a[1]) as u8;
            let bytes = match parse_hex(&a[2]) {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("{e}");
                    return 2;
                }
            };
            if bytes.len() > 1024 {
                eprintln!("od-write: hex too long");
                return 2;
            }
            // {u16 index, u8 sub, u8 pad, u16 len, u16 pad, data@8}
            pre.extend_from_slice(&idx.to_le_bytes());
            pre.push(sub);
            pre.push(0);
            pre.extend_from_slice(&(bytes.len() as u16).to_le_bytes());
            pre.extend_from_slice(&0u16.to_le_bytes());
            pre.extend_from_slice(&bytes);
        }
        "retain-status" => op_code = op::RETAIN_STATUS,
        "retain-flush" => op_code = op::RETAIN_FLUSH,
        "retain-clear" => op_code = op::RETAIN_CLEAR,
        "retain-dump" => op_code = op::RETAIN_DUMP,
        "watch" => {
            need!(!a.is_empty() && a.len() <= 16, "watch needs 1..16 items");
            op_code = op::DBG_WATCH_ADD;
            pre.extend_from_slice(&(a.len() as u32).to_le_bytes());
            pre.extend_from_slice(&[0u8; 4]);
            for item in a {
                if let Some((os, ls)) = item.split_once(':') {
                    let off = parse_u64(os);
                    let l: u32 = ls.parse().unwrap_or(0);
                    if l == 0 || l > 64 {
                        eprintln!("bad item {} (want OFF:LEN, len 1..64, or a path)", item);
                        return 2;
                    }
                    pre.extend_from_slice(&off.to_le_bytes());
                    pre.extend_from_slice(&l.to_le_bytes());
                    pre.extend_from_slice(&0u16.to_le_bytes());
                } else {
                    if item.is_empty() || item.len() > 127 {
                        eprintln!("bad path {}", item);
                        return 2;
                    }
                    pre.extend_from_slice(&[0u8; 12]);
                    pre.extend_from_slice(&(item.len() as u16).to_le_bytes());
                    pre.extend_from_slice(item.as_bytes());
                }
            }
        }
        "watch-get" => {
            op_code = op::DBG_WATCH_GET;
            let h = arg.map(parse_u32).unwrap_or(0);
            pre.extend_from_slice(&h.to_le_bytes());
        }
        "watch-stream" => {
            op_code = op::DBG_WATCH_STREAM;
            let h = arg.map(parse_u32).unwrap_or(0);
            let hz = arg2.map(parse_u32).unwrap_or(10);
            pre.extend_from_slice(&h.to_le_bytes());
            pre.extend_from_slice(&hz.to_le_bytes());
        }
        "watch-rm" => {
            op_code = op::DBG_WATCH_REMOVE;
            let h = match arg {
                None | Some("all") => 0xFFFF_FFFFu32,
                Some(s) => parse_u32(s),
            };
            pre.extend_from_slice(&h.to_le_bytes());
        }
        "watch-list" => op_code = op::DBG_WATCH_LIST,
        "map-get" => op_code = op::MAP_GET,
        "journal-tail" => {
            op_code = op::JOURNAL_TAIL;
            let n = arg.map(parse_u32).unwrap_or(20);
            pre.extend_from_slice(&n.to_le_bytes());
        }
        "evlog-list" => {
            op_code = op::EVTLOG_LIST;
            let cap = arg.map(parse_u32).unwrap_or(128);
            pre.extend_from_slice(&cap.to_le_bytes());
        }
        "evlog-query" => {
            need!(arg.is_some(), "evlog-query ID");
            op_code = op::EVTLOG_QUERY;
            pre.extend_from_slice(&parse_u32(arg.unwrap()).to_le_bytes());
        }
        "evlog-stats" => op_code = op::EVTLOG_STATS,
        "evlog-history" => {
            op_code = op::EVTLOG_HISTORY;
            let cap = arg.map(parse_u32).unwrap_or(64);
            pre.extend_from_slice(&cap.to_le_bytes());
        }
        "evlog-confirm" => {
            need!(arg.is_some(), "evlog-confirm ID");
            op_code = op::EVTLOG_CONFIRM;
            pre.extend_from_slice(&parse_u32(arg.unwrap()).to_le_bytes());
        }
        "hist" => {
            need!(!a.is_empty() && a.len() <= 8, "hist needs 1..8 symbol names");
            op_code = op::HIST_CONF;
            pre.extend_from_slice(&(a.len() as u32).to_le_bytes());
            for nm in a {
                if nm.is_empty() || nm.len() > 127 {
                    eprintln!("bad name {}", nm);
                    return 2;
                }
                pre.extend_from_slice(&(nm.len() as u16).to_le_bytes());
                pre.extend_from_slice(nm.as_bytes());
                pre.extend_from_slice(&[0u8; 8]); // len=0 natural + pad
            }
        }
        "hist-list" => op_code = op::HIST_LIST,
        "mqtt-status" => op_code = op::MQTT_STATUS,
        "mqtt-journal" => {
            op_code = op::MQTT_JOURNAL;
            // {u32 handle, u32 idx, u32 max}
            let h = arg.map(parse_u32).unwrap_or(0);
            let idx = arg2.map(parse_u32).unwrap_or(0);
            let max = arg3.map(parse_u32).unwrap_or(64);
            pre.extend_from_slice(&h.to_le_bytes());
            pre.extend_from_slice(&idx.to_le_bytes());
            pre.extend_from_slice(&max.to_le_bytes());
        }
        _ => {
            eprintln!("unknown cmd {cmd}");
            return 2;
        }
    }

    let mut ctl = match Ctl::connect(sock, 3000) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("connect: {e}");
            return 1;
        }
    };
    if ctl.hello("tplc").is_err() {
        eprintln!("hello failed");
        return 1;
    }
    match ctl.call(op_code, &pre) {
        Ok((fl, pl)) => {
            if fl & F_ERROR != 0 {
                let e: CtlError = crate::ctl::parse_ctl_error(&pl);
                eprintln!("{e}");
                return 1;
            }
            print_reply(op_code, &pl, csv_mode);
            0
        }
        Err(e) => {
            eprintln!("recv failed: {e}");
            1;
            1
        }
    }
}

pub fn usage() -> &'static str {
    "usage: tplc ctl <sock> <op> [args]   (or `tplc <op> [--socket S]`)
ops:
  ping status prog list start stop image shutdown kill unload pkg
  prepare X layout rollback swap X reset [warm|cold] deploy F
  suspend N resume N step N [M]
  resolve NAME dbg-state dbg-read OFF LEN [SEL] dbg-write OFF HEX
  force OFF HEX [MASK] force-list force-clear [H|all]
  break T OFF LEN EQ|NE|LT|LE|GT|GE V   capture T OFF LEN CMP V
  break-list break-clear [H|all]
  breakx T EXPR capturex T EXPR state-diff fields [SEL]
  trace [nofreeze] OFF:LEN.. trace-status trace-read [SEQ [N]]
  trace-clear trace-csv [FROM] [MAX]
  retain-status retain-flush retain-clear retain-dump
  watch OFF:LEN|PATH.. watch-get H watch-stream H [HZ]
  watch-rm [H|all] watch-list map-get journal-tail [N]
  evlog-list [CAP] evlog-query ID evlog-stats evlog-history [CAP]
  evlog-confirm ID hist NAME.. hist-list
  mqtt-status mqtt-journal HANDLE [IDX] [MAX]
  od-read IDX [SUB] od-list od-write IDX SUB HEX
  shadow-load X shadow-status [SEL] shadow-list shadow-compare [N] [SEL]
  shadow-image OFF LEN [SEL] shadow-promote [SEL] shadow-drop [SEL]
  shadow-reset [SEL] events [SEC]"
}
