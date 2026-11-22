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


mod ops;
mod parse;
mod print;

use ops::*;
use parse::*;
use print::*;

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
