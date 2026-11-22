//! ops — split from ctlcmd.rs.
use super::*;

/* ---------------- deploy (multi-frame) ---------------- */

pub(crate) fn sha256_file(path: &str) -> Result<(Vec<u8>, u64), String> {
    let data = std::fs::read(path).map_err(|e| format!("cannot read {path}: {e}"))?;
    let sz = data.len() as u64;
    let mut h = Sha256::new();
    h.update(&data);
    Ok((h.finalize().to_vec(), sz))
}

pub(crate) fn do_deploy(sock: &str, path: &str) -> i32 {
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

pub(crate) fn cmd_events(sock: &str, secs: u64) -> i32 {
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
