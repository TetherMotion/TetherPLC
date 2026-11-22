//! builtin_fb_layout arms — tail/a domain.
use super::*;

        /* FB_Dmx — tplc-dmx service bridge (DMX512 output over an
        when the surface is used. */
pub(super) fn arm_fb_dmx(off: u64, m: &mut Members) -> u64 {
    let mut push = |o: &mut u64, n: &str, t: Type| {
        let a = t.align();
        *o = (*o + a - 1) & !(a - 1);
        m.insert(n.to_string(), (*o, t.clone()));
        *o += t.size();
    };
    let mut off = off;
            let udint = || Type::Int {
                signed: false,
                bits: 32,
            };
            let dint = || Type::Int {
                signed: true,
                bits: 32,
            };
            push(&mut off, "OP", udint());
            push(&mut off, "HANDLE", udint());
            push(&mut off, "PATH", Type::Str { cap: 127 });
            push(&mut off, "BAUD", udint());
            push(&mut off, "CHANNEL", udint());
            push(&mut off, "VALUE", udint());
            push(&mut off, "START", udint());
            push(&mut off, "COUNT", udint());
            push(&mut off, "VALUES", Type::Str { cap: 255 });
            push(&mut off, "STATE", dint());
            push(&mut off, "FRAMES", udint());
            push(&mut off, "LASTERR", dint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
    off
}

        /* FB_Mbus — tplc-mbus service bridge (wired M-Bus / EN 13757
        when the surface is used. */
        /* FB_EnOcean — tplc-enocean service bridge (EnOcean ESP3
        code, ERRID its positive form. */
pub(super) fn arm_fb_en_ocean(off: u64, m: &mut Members) -> u64 {
    let mut push = |o: &mut u64, n: &str, t: Type| {
        let a = t.align();
        *o = (*o + a - 1) & !(a - 1);
        m.insert(n.to_string(), (*o, t.clone()));
        *o += t.size();
    };
    let mut off = off;
            let udint = || Type::Int {
                signed: false,
                bits: 32,
            };
            let dint = || Type::Int {
                signed: true,
                bits: 32,
            };
            push(&mut off, "OP", udint());
            push(&mut off, "HANDLE", udint());
            push(&mut off, "PATH", Type::Str { cap: 127 });
            push(&mut off, "ORG", udint());
            push(&mut off, "SENDER", udint());
            push(&mut off, "DEST", udint());
            push(&mut off, "DATA", Type::Str { cap: 255 });
            push(&mut off, "PTYPE", udint());
            push(&mut off, "RORG", udint());
            push(&mut off, "RSENDER", udint());
            push(&mut off, "RXPAYLOAD", Type::Str { cap: 255 });
            push(&mut off, "STATE", dint());
            push(&mut off, "TXTOT", udint());
            push(&mut off, "RXTOT", udint());
            push(&mut off, "LASTERR", dint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "RXB0", udint());
    off
}

        /* FB_Genibus — tplc-genibus service bridge (GENIbus
        (DA/SA/APDU), 5=receive (-> RDA/RSA/RXAPDU/RXB0 or NOMSG). */
pub(super) fn arm_fb_genibus(off: u64, m: &mut Members) -> u64 {
    let mut push = |o: &mut u64, n: &str, t: Type| {
        let a = t.align();
        *o = (*o + a - 1) & !(a - 1);
        m.insert(n.to_string(), (*o, t.clone()));
        *o += t.size();
    };
    let mut off = off;
            let udint = || Type::Int {
                signed: false,
                bits: 32,
            };
            let dint = || Type::Int {
                signed: true,
                bits: 32,
            };
            push(&mut off, "OP", udint());
            push(&mut off, "HANDLE", udint());
            push(&mut off, "PATH", Type::Str { cap: 127 });
            push(&mut off, "DA", udint());
            push(&mut off, "SA", udint());
            push(&mut off, "APDU", Type::Str { cap: 250 });
            push(&mut off, "RDA", udint());
            push(&mut off, "RSA", udint());
            push(&mut off, "RXAPDU", Type::Str { cap: 250 });
            push(&mut off, "STATE", dint());
            push(&mut off, "TXTOT", udint());
            push(&mut off, "RXTOT", udint());
            push(&mut off, "LASTERR", dint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "RXB0", udint());
    off
}

        /* FB_Mpbus — tplc-mpbus service bridge (Belimo MP-Bus
        5=receive (same outs). */
pub(super) fn arm_fb_mpbus(off: u64, m: &mut Members) -> u64 {
    let mut push = |o: &mut u64, n: &str, t: Type| {
        let a = t.align();
        *o = (*o + a - 1) & !(a - 1);
        m.insert(n.to_string(), (*o, t.clone()));
        *o += t.size();
    };
    let mut off = off;
            let udint = || Type::Int {
                signed: false,
                bits: 32,
            };
            let dint = || Type::Int {
                signed: true,
                bits: 32,
            };
            push(&mut off, "OP", udint());
            push(&mut off, "HANDLE", udint());
            push(&mut off, "PATH", Type::Str { cap: 127 });
            push(&mut off, "DA", udint());
            push(&mut off, "CMD", udint());
            push(&mut off, "APDU", Type::Str { cap: 250 });
            push(&mut off, "RDA", udint());
            push(&mut off, "RCMD", udint());
            push(&mut off, "RXAPDU", Type::Str { cap: 250 });
            push(&mut off, "STATE", dint());
            push(&mut off, "TXTOT", udint());
            push(&mut off, "RXTOT", udint());
            push(&mut off, "LASTERR", dint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "RXB0", udint());
    off
}

        /* FB_DynMem — tplc-dynmem bounded-arena bridge
        6=stats -> LIVE/TOTAL/ARENA counters. */
pub(super) fn arm_fb_dyn_mem(off: u64, m: &mut Members) -> u64 {
    let mut push = |o: &mut u64, n: &str, t: Type| {
        let a = t.align();
        *o = (*o + a - 1) & !(a - 1);
        m.insert(n.to_string(), (*o, t.clone()));
        *o += t.size();
    };
    let mut off = off;
            let udint = || Type::Int {
                signed: false,
                bits: 32,
            };
            let dint = || Type::Int {
                signed: true,
                bits: 32,
            };
            push(&mut off, "OP", udint());
            push(&mut off, "ID", udint());
            push(&mut off, "SIZE", udint());
            push(&mut off, "RESET", udint());
            push(&mut off, "PRESERVE", udint());
            push(&mut off, "OFFSET", udint());
            push(&mut off, "BYTE", udint());
            push(&mut off, "LIVEBLKS", udint());
            push(&mut off, "LIVEBYTES", udint());
            push(&mut off, "TOTALALLOCS", udint());
            push(&mut off, "TOTALFREES", udint());
            push(&mut off, "ARENABYTES", udint());
            push(&mut off, "ARENAUSED", udint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
    off
}

        /* FB_EventLog — tplc-eventlog journal bridge
        STATE/SEVO/CLSO/EVTIDO/TXTO), 5=stats -> counters. */
pub(super) fn arm_fb_event_log(off: u64, m: &mut Members) -> u64 {
    let mut push = |o: &mut u64, n: &str, t: Type| {
        let a = t.align();
        *o = (*o + a - 1) & !(a - 1);
        m.insert(n.to_string(), (*o, t.clone()));
        *o += t.size();
    };
    let mut off = off;
            let udint = || Type::Int {
                signed: false,
                bits: 32,
            };
            let dint = || Type::Int {
                signed: true,
                bits: 32,
            };
            push(&mut off, "OP", udint());
            push(&mut off, "ID", udint());
            push(&mut off, "ECLASS", udint());
            push(&mut off, "EVTID", udint());
            push(&mut off, "SEV", udint());
            push(&mut off, "ARGS", udint());
            push(&mut off, "STATE", udint());
            push(&mut off, "SEVO", udint());
            push(&mut off, "CLSO", udint());
            push(&mut off, "EVTIDO", udint());
            push(&mut off, "LIVE", udint());
            push(&mut off, "RAISED", udint());
            push(&mut off, "CLEARED", udint());
            push(&mut off, "CONFIRMED", udint());
            push(&mut off, "DROPPED", udint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "TEXT", Type::Str { cap: 80 });
            push(&mut off, "TXTO", Type::Str { cap: 80 });
            /* transition timestamps (appended): query fills
               TSTAMP/CSTAMP/FSTAMP with raised/cleared/confirmed
               realtime ns (0 = not yet transitioned)              */
            push(&mut off, "TSTAMP", Type::Int {
                signed: false, bits: 64 });
            push(&mut off, "CSTAMP", Type::Int {
                signed: false, bits: 64 });
            push(&mut off, "FSTAMP", Type::Int {
                signed: false, bits: 64 });
    off
}

        /* FB_Db — tplc-db SQLite bridge (Tc2/Tc3_Database).
        3=query (SQL -> BUF/ROWS/COLS), 4=close, 5=error (-> ERR). */
pub(super) fn arm_fb_db(off: u64, m: &mut Members) -> u64 {
    let mut push = |o: &mut u64, n: &str, t: Type| {
        let a = t.align();
        *o = (*o + a - 1) & !(a - 1);
        m.insert(n.to_string(), (*o, t.clone()));
        *o += t.size();
    };
    let mut off = off;
            let udint = || Type::Int {
                signed: false,
                bits: 32,
            };
            let dint = || Type::Int {
                signed: true,
                bits: 32,
            };
            push(&mut off, "OP", udint());
            push(&mut off, "HANDLE", udint());
            push(&mut off, "CHANGES", udint());
            push(&mut off, "ROWS", udint());
            push(&mut off, "COLS", udint());
            push(&mut off, "ERRLEN", udint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "PATH", Type::Str { cap: 127 });
            push(&mut off, "SQL", Type::Str { cap: 255 });
            push(&mut off, "BUF", Type::Str { cap: 255 });
            push(&mut off, "ERR", Type::Str { cap: 127 });
    off
}
