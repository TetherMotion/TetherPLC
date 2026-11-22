//! builtin_fb_layout arms — bacnet/s7 domain.
use super::*;

        /* FB_Bacnet — tplc-bacnet service bridge (BACnet/IP client,
        TIMEOUT itself. */
pub(super) fn arm_fb_bacnet(off: u64, m: &mut Members) -> u64 {
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
            let byte = || Type::Int {
                signed: false,
                bits: 8,
            };
            push(&mut off, "HOST", Type::Str { cap: 15 });
            push(&mut off, "PORT", udint());
            push(&mut off, "FUNC", udint());
            push(&mut off, "OBJ_TYPE", udint());
            push(&mut off, "OBJ_INST", udint());
            push(&mut off, "PROP_ID", udint());
            push(&mut off, "HAS_IDX", Type::Bool);
            push(&mut off, "HAS_PRIO", Type::Bool);
            push(&mut off, "HAS_RANGE", Type::Bool);
            push(&mut off, "PROP_IDX", udint());
            push(&mut off, "PRIORITY", udint());
            push(&mut off, "RANGE_LO", udint());
            push(&mut off, "RANGE_HI", udint());
            push(&mut off, "VAL_TYPE", udint());
            push(&mut off, "VLEN", udint());
            push(&mut off, "TIMEOUT", Type::Time);
            push(
                &mut off,
                "VAL",
                Type::Array {
                    elem: Box::new(byte()),
                    lo: 0,
                    hi: 255,
                },
            );
            push(&mut off, "HANDLE", udint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "R_SERVICE", udint());
            push(&mut off, "R_APDU", udint());
            push(&mut off, "R_OBJ_TYPE", udint());
            push(&mut off, "R_OBJ_INST", udint());
            push(&mut off, "R_PROP_ID", udint());
            push(&mut off, "R_VTYPE", udint());
            push(&mut off, "R_VLEN", udint());
            push(
                &mut off,
                "R_VAL",
                Type::Array {
                    elem: Box::new(byte()),
                    lo: 0,
                    hi: 255,
                },
            );
            push(&mut off, "DEV_COUNT", udint());
            push(
                &mut off,
                "DEV_ID",
                Type::Array {
                    elem: Box::new(udint()),
                    lo: 0,
                    hi: 7,
                },
            );
            push(
                &mut off,
                "DEV_IP",
                Type::Array {
                    elem: Box::new(byte()),
                    lo: 0,
                    hi: 31,
                },
            );
            push(&mut off, "DEV_VENDOR", udint());
            push(&mut off, "DEV_MAX_APDU", udint());
            push(&mut off, "DEV_SEG", udint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "EXEC", Type::Bool);
            push(&mut off, "BUSY", Type::Bool);
            push(&mut off, "DONE", Type::Bool);
            push(&mut off, "__last_exec", Type::Bool);
    off
}

        /* FB_Pjlink — tplc-pjlink service bridge (JBMIA PJLink
        5=authfail. The service enforces TIMEOUT itself. */
pub(super) fn arm_fb_pjlink(off: u64, m: &mut Members) -> u64 {
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
            push(&mut off, "HOST", Type::Str { cap: 15 });
            push(&mut off, "PORT", udint());
            push(&mut off, "CLS", udint());
            push(&mut off, "CMD", Type::Str { cap: 4 });
            push(&mut off, "PARAM", Type::Str { cap: 128 });
            push(&mut off, "PASSWORD", Type::Str { cap: 32 });
            push(&mut off, "TIMEOUT", Type::Time);
            push(&mut off, "HANDLE", udint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "R_ERR", dint());
            push(&mut off, "R_CLS", udint());
            push(&mut off, "R_CMD", udint());
            push(&mut off, "R_VLEN", udint());
            push(&mut off, "R_VALUE", Type::Str { cap: 127 });
            push(&mut off, "AUTHED", Type::Bool);
            push(&mut off, "EXEC", Type::Bool);
            push(&mut off, "BUSY", Type::Bool);
            push(&mut off, "DONE", Type::Bool);
            push(&mut off, "__last_exec", Type::Bool);
    off
}

        /* FB_S7Conn — tplc-s7 service bridge (Siemens S7comm
        into the COTP called TSAP. */
pub(super) fn arm_fb_s7_conn(off: u64, m: &mut Members) -> u64 {
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
            push(&mut off, "HOST", Type::Str { cap: 15 });
            push(&mut off, "PORT", udint());
            push(&mut off, "RACK", udint());
            push(&mut off, "SLOT", udint());
            push(&mut off, "TIMEOUT", Type::Time);
            push(&mut off, "CONN_HANDLE", udint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "CLOSE", Type::Bool);
            push(&mut off, "EXEC", Type::Bool);
            push(&mut off, "BUSY", Type::Bool);
            push(&mut off, "DONE", Type::Bool);
            push(&mut off, "__last_exec", Type::Bool);
            push(&mut off, "__last_close", Type::Bool);
    off
}

        /* FB_S7Io — tplc-s7 service bridge for one Read Var /
        error word. One outstanding transaction per connection. */
pub(super) fn arm_fb_s7_io(off: u64, m: &mut Members) -> u64 {
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
            let byte = || Type::Int {
                signed: false,
                bits: 8,
            };
            push(&mut off, "CONN_HANDLE", udint());
            push(&mut off, "MODE", udint());
            push(&mut off, "AREA", udint());
            push(&mut off, "DB", udint());
            push(&mut off, "BYTE_OFF", udint());
            push(&mut off, "BIT_ACCESS", Type::Bool);
            push(&mut off, "BIT_NO", udint());
            push(&mut off, "LEN", udint());
            push(&mut off, "TIMEOUT", Type::Time);
            push(&mut off, "HANDLE", udint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ITEM_RC", udint());
            push(&mut off, "DATA_LEN", udint());
            push(&mut off, "ERRID", udint());
            push(
                &mut off,
                "DATA",
                Type::Array {
                    elem: Box::new(byte()),
                    lo: 0,
                    hi: 239,
                },
            );
            push(
                &mut off,
                "R_DATA",
                Type::Array {
                    elem: Box::new(byte()),
                    lo: 0,
                    hi: 239,
                },
            );
            push(&mut off, "EXEC", Type::Bool);
            push(&mut off, "BUSY", Type::Bool);
            push(&mut off, "DONE", Type::Bool);
            push(&mut off, "__last_exec", Type::Bool);
    off
}

        /* FB_S5Conn — tplc-s5 service bridge (Siemens S5, RK512
        PORT 0 = 102, TIMEOUT bounds the connect. */
pub(super) fn arm_fb_s5_conn(off: u64, m: &mut Members) -> u64 {
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
            push(&mut off, "HOST", Type::Str { cap: 15 });
            push(&mut off, "PORT", udint());
            push(&mut off, "TIMEOUT", Type::Time);
            push(&mut off, "CONN_HANDLE", udint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "CLOSE", Type::Bool);
            push(&mut off, "EXEC", Type::Bool);
            push(&mut off, "BUSY", Type::Bool);
            push(&mut off, "DONE", Type::Bool);
            push(&mut off, "__last_exec", Type::Bool);
            push(&mut off, "__last_close", Type::Bool);
    off
}

        /* FB_S5Io — tplc-s5 service bridge for one RK512 FETCH
        connection. */
pub(super) fn arm_fb_s5_io(off: u64, m: &mut Members) -> u64 {
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
            let byte = || Type::Int {
                signed: false,
                bits: 8,
            };
            push(&mut off, "CONN_HANDLE", udint());
            push(&mut off, "MODE", udint());
            push(&mut off, "DB", udint());
            push(&mut off, "BYTE_OFF", udint());
            push(&mut off, "LEN", udint());
            push(&mut off, "TIMEOUT", Type::Time);
            push(&mut off, "HANDLE", udint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "DATA_LEN", udint());
            push(
                &mut off,
                "DATA",
                Type::Array {
                    elem: Box::new(byte()),
                    lo: 0,
                    hi: 509,
                },
            );
            push(
                &mut off,
                "R_DATA",
                Type::Array {
                    elem: Box::new(byte()),
                    lo: 0,
                    hi: 509,
                },
            );
            push(&mut off, "EXEC", Type::Bool);
            push(&mut off, "BUSY", Type::Bool);
            push(&mut off, "DONE", Type::Bool);
            push(&mut off, "__last_exec", Type::Bool);
    off
}
