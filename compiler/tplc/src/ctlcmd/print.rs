//! print — split from ctlcmd.rs.
use super::*;

pub(crate) fn print_prog_status(pl: &[u8]) {
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

pub(crate) fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

pub(crate) fn print_reply(op_code: u16, pl: &[u8], csv_mode: bool) {
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
                let version = cstr_name(
                    &pl[40..(40 + PROG_NAME_MAX).min(pl.len())],
                    PROG_NAME_MAX,
                );
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

pub(crate) const SHADOW_STATUS_SIZE: usize = 128;

pub(crate) fn print_reply_fallback(op_code: u16, pl: &[u8], csv_mode: bool) {
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
