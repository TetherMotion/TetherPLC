//! builtin_fb_layout arms — io/svc domain.
use super::*;

        /* FB_Socket — tplc-net service bridge (plan 21 C2). One
        (Beckhoff uses POINTER TO BYTE — no pointers in ST). */
pub(super) fn arm_fb_socket(off: u64, m: &mut Members) -> u64 {
    let mut push = |o: &mut u64, n: &str, t: Type| {
        let a = t.align();
        *o = (*o + a - 1) & !(a - 1);
        m.insert(n.to_string(), (*o, t.clone()));
        *o += t.size();
    };
    let mut off = off;
            let dint = || Type::Int {
                signed: true,
                bits: 32,
            };
            let udint = || Type::Int {
                signed: false,
                bits: 32,
            };
            push(&mut off, "OP", dint());
            push(&mut off, "HOST", Type::Str { cap: 15 });
            push(&mut off, "PORT", udint());
            push(&mut off, "TIMEOUT", Type::Time);
            push(&mut off, "DATA", Type::Str { cap: 256 });
            push(&mut off, "DLEN", udint());
            push(&mut off, "HANDLE", udint());
            push(&mut off, "AUX", udint());
            push(&mut off, "RESULT", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "EXEC", Type::Bool);
            push(&mut off, "BUSY", Type::Bool);
            push(&mut off, "DONE", Type::Bool);
            push(&mut off, "__last_exec", Type::Bool);
            push(&mut off, "__deadline", Type::Time);
            push(&mut off, "__sent", udint());
            push(&mut off, "__recvd", udint());
            push(&mut off, "__phase", dint());
    off
}

        /* FB_Serial — tplc-serial service bridge. Same op-multiplexed
        pointers in ST), DATA the bounded payload. */
pub(super) fn arm_fb_serial(off: u64, m: &mut Members) -> u64 {
    let mut push = |o: &mut u64, n: &str, t: Type| {
        let a = t.align();
        *o = (*o + a - 1) & !(a - 1);
        m.insert(n.to_string(), (*o, t.clone()));
        *o += t.size();
    };
    let mut off = off;
            let dint = || Type::Int {
                signed: true,
                bits: 32,
            };
            let udint = || Type::Int {
                signed: false,
                bits: 32,
            };
            push(&mut off, "OP", dint());
            push(&mut off, "DEV", Type::Str { cap: 63 });
            push(&mut off, "BAUD", udint());
            push(&mut off, "PARITY", udint());
            push(&mut off, "DBITS", udint());
            push(&mut off, "SBITS", udint());
            push(&mut off, "FLAGS", udint());
            push(&mut off, "AUX", udint());
            push(&mut off, "TIMEOUT", Type::Time);
            push(&mut off, "DATA", Type::Str { cap: 256 });
            push(&mut off, "DLEN", udint());
            push(&mut off, "HANDLE", udint());
            push(&mut off, "RESULT", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "EXEC", Type::Bool);
            push(&mut off, "BUSY", Type::Bool);
            push(&mut off, "DONE", Type::Bool);
            push(&mut off, "__last_exec", Type::Bool);
            push(&mut off, "__deadline", Type::Time);
            push(&mut off, "__sent", udint());
            push(&mut off, "__recvd", udint());
            push(&mut off, "__phase", dint());
    off
}

        /* FB_Ads — tplc-ads service bridge (AMS/TCP ADS client).
        write-control, RESULT carries the ADS-layer error code. */
pub(super) fn arm_fb_ads(off: u64, m: &mut Members) -> u64 {
    let mut push = |o: &mut u64, n: &str, t: Type| {
        let a = t.align();
        *o = (*o + a - 1) & !(a - 1);
        m.insert(n.to_string(), (*o, t.clone()));
        *o += t.size();
    };
    let mut off = off;
            let dint = || Type::Int {
                signed: true,
                bits: 32,
            };
            let udint = || Type::Int {
                signed: false,
                bits: 32,
            };
            push(&mut off, "OP", dint());
            push(&mut off, "HOST", Type::Str { cap: 15 });
            push(&mut off, "PORT", udint());
            push(&mut off, "IDXGRP", udint());
            push(&mut off, "IDXOFF", udint());
            push(&mut off, "STATE", dint());
            push(&mut off, "DEVSTATE", dint());
            push(&mut off, "AUX", udint());
            push(&mut off, "TIMEOUT", Type::Time);
            push(&mut off, "DATA", Type::Str { cap: 256 });
            push(&mut off, "DLEN", udint());
            push(&mut off, "HANDLE", udint());
            push(&mut off, "RESULT", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "EXEC", Type::Bool);
            push(&mut off, "BUSY", Type::Bool);
            push(&mut off, "DONE", Type::Bool);
            push(&mut off, "__last_exec", Type::Bool);
            push(&mut off, "__deadline", Type::Time);
            push(&mut off, "__sent", udint());
            push(&mut off, "__recvd", udint());
            push(&mut off, "__phase", dint());
    off
}

        /* FB_RtMon — tplc-rtmon service bridge (Tc3_RealtimeMonitoring).
        state. */
pub(super) fn arm_fb_rt_mon(off: u64, m: &mut Members) -> u64 {
    let mut push = |o: &mut u64, n: &str, t: Type| {
        let a = t.align();
        *o = (*o + a - 1) & !(a - 1);
        m.insert(n.to_string(), (*o, t.clone()));
        *o += t.size();
    };
    let mut off = off;
            let dint = || Type::Int {
                signed: true,
                bits: 32,
            };
            let udint = || Type::Int {
                signed: false,
                bits: 32,
            };
            push(&mut off, "OP", dint());
            push(&mut off, "CONTEXT", udint());
            push(&mut off, "GROUP", udint());
            push(&mut off, "EVENT", udint());
            push(&mut off, "MARKCTRL", udint());
            push(&mut off, "AUX", udint());
            push(&mut off, "RESULT", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "EXEC", Type::Bool);
            push(&mut off, "BUSY", Type::Bool);
            push(&mut off, "DONE", Type::Bool);
            push(&mut off, "__last_exec", Type::Bool);
    off
}
