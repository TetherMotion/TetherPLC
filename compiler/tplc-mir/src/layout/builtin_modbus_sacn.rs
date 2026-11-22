//! builtin_fb_layout arms — modbus/sacn domain.
use super::*;

        /* FB_Modbus — tplc-modbus service bridge (Modbus TCP client).
        TIMEOUT itself, so no deadline fields. */
pub(super) fn arm_fb_modbus(off: u64, m: &mut Members) -> u64 {
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
            let byte = || Type::Int {
                signed: false,
                bits: 8,
            };
            push(&mut off, "HOST", Type::Str { cap: 15 });
            push(&mut off, "PORT", udint());
            push(&mut off, "UNIT", udint());
            push(&mut off, "TIMEOUT", Type::Time);
            push(&mut off, "FUNC", udint());
            push(
                &mut off,
                "DATA",
                Type::Array {
                    elem: Box::new(byte()),
                    lo: 0,
                    hi: 255,
                },
            );
            push(&mut off, "RLEN", udint());
            push(&mut off, "DLEN", udint());
            push(&mut off, "HANDLE", udint());
            push(&mut off, "RESULT", dint());
            push(&mut off, "EXC", udint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "EXEC", Type::Bool);
            push(&mut off, "BUSY", Type::Bool);
            push(&mut off, "DONE", Type::Bool);
            push(&mut off, "__last_exec", Type::Bool);
            /* UDP selects the datagram transport (mb_begin_udp);
            appended last so earlier member offsets stay fixed */
            push(&mut off, "UDP", Type::Bool);
    off
}

        /* FB_ModbusRtu — tplc-modbus RTU transport (Modbus RTU
        mirror FB_Modbus. The service enforces TIMEOUT itself. */
pub(super) fn arm_fb_modbus_rtu(off: u64, m: &mut Members) -> u64 {
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
            let byte = || Type::Int {
                signed: false,
                bits: 8,
            };
            push(&mut off, "DEV", Type::Str { cap: 63 });
            push(&mut off, "BAUD", udint());
            push(&mut off, "PARITY", udint());
            push(&mut off, "DBITS", udint());
            push(&mut off, "SBITS", udint());
            push(&mut off, "UNIT", udint());
            push(&mut off, "TIMEOUT", Type::Time);
            push(&mut off, "FUNC", udint());
            push(
                &mut off,
                "DATA",
                Type::Array {
                    elem: Box::new(byte()),
                    lo: 0,
                    hi: 255,
                },
            );
            push(&mut off, "RLEN", udint());
            push(&mut off, "DLEN", udint());
            push(&mut off, "HANDLE", udint());
            push(&mut off, "RESULT", dint());
            push(&mut off, "EXC", udint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "EXEC", Type::Bool);
            push(&mut off, "BUSY", Type::Bool);
            push(&mut off, "DONE", Type::Bool);
            push(&mut off, "__last_exec", Type::Bool);
    off
}

        /* FB_SacnSource — tplc-sacn source role (ANSI E1.31
        LAST_ERR the last socket errno. */
pub(super) fn arm_fb_sacn_source(off: u64, m: &mut Members) -> u64 {
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
            let ulint = || Type::Int {
                signed: false,
                bits: 64,
            };
            let byte = || Type::Int {
                signed: false,
                bits: 8,
            };
            push(&mut off, "ENABLE", Type::Bool);
            push(&mut off, "UNIVERSE", udint());
            push(&mut off, "PRIORITY", udint());
            push(&mut off, "SYNC_ADDR", udint());
            push(&mut off, "INTERVAL", udint());
            push(&mut off, "TTL", udint());
            push(&mut off, "OPTIONS", byte());
            push(&mut off, "FLAGS", byte());
            push(&mut off, "START_CODE", byte());
            push(&mut off, "NAME", Type::Str { cap: 63 });
            push(
                &mut off,
                "CID",
                Type::Array {
                    elem: Box::new(byte()),
                    lo: 0,
                    hi: 15,
                },
            );
            push(&mut off, "DLEN", udint());
            push(
                &mut off,
                "DMX",
                Type::Array {
                    elem: Box::new(byte()),
                    lo: 0,
                    hi: 511,
                },
            );
            push(&mut off, "HANDLE", udint());
            push(&mut off, "SEQ", udint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "LAST_ERR", dint());
            push(&mut off, "SENT", ulint());
            push(&mut off, "AUX", ulint());
            push(&mut off, "BUSY", Type::Bool);
            push(&mut off, "__err_latch", Type::Bool);
    off
}

        /* FB_SacnReceiver — tplc-sacn receiver role. ENABLE rising
        count). Counters are u64. */
pub(super) fn arm_fb_sacn_receiver(off: u64, m: &mut Members) -> u64 {
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
            let ulint = || Type::Int {
                signed: false,
                bits: 64,
            };
            let u16t = || Type::Int {
                signed: false,
                bits: 16,
            };
            let byte = || Type::Int {
                signed: false,
                bits: 8,
            };
            push(&mut off, "ENABLE", Type::Bool);
            push(&mut off, "RCV_UNIVERSE", udint());
            push(&mut off, "SYNC_UNIVERSE", udint());
            push(&mut off, "PORT", udint());
            push(&mut off, "DATA_RX", ulint());
            push(&mut off, "DATA_DROP", ulint());
            push(&mut off, "SYNC_RX", ulint());
            push(&mut off, "DISC_RX", ulint());
            push(&mut off, "REMOTE_IP", udint());
            push(&mut off, "HANDLE", udint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "DLEN", udint());
            push(&mut off, "SYNC_ADDR", u16t());
            push(&mut off, "UNIVERSE", u16t());
            push(&mut off, "SYNC_ADDR_SEEN", u16t());
            push(&mut off, "UNI_COUNT", u16t());
            push(&mut off, "START_CODE", byte());
            push(&mut off, "SEQ", byte());
            push(&mut off, "PRIORITY", byte());
            push(&mut off, "SYNC_SEQ", byte());
            push(&mut off, "UNI_PAGE", byte());
            push(&mut off, "UNI_LAST", byte());
            push(&mut off, "HAVE_DATA", Type::Bool);
            push(&mut off, "PREVIEW", Type::Bool);
            push(&mut off, "STREAM_TERM", Type::Bool);
            push(&mut off, "FORCE_SYNC", Type::Bool);
            push(&mut off, "HAVE_SYNC", Type::Bool);
            push(&mut off, "HAVE_DISC", Type::Bool);
            push(&mut off, "BUSY", Type::Bool);
            push(&mut off, "SRC_NAME", Type::Str { cap: 63 });
            push(
                &mut off,
                "DMX",
                Type::Array {
                    elem: Box::new(byte()),
                    lo: 0,
                    hi: 511,
                },
            );
            push(
                &mut off,
                "UNIS",
                Type::Array {
                    elem: Box::new(u16t()),
                    lo: 0,
                    hi: 63,
                },
            );
            push(&mut off, "__err_latch", Type::Bool);
    off
}
