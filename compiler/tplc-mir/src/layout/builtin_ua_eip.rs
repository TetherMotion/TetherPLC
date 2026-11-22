//! builtin_fb_layout arms — ua/eip domain.
use super::*;

        /* FB_UaClient — tplc-opcua service bridge (OPC UA client via
        nodes lose precision above 2^53 — an honest LREAL limit. */
pub(super) fn arm_fb_ua_client(off: u64, m: &mut Members) -> u64 {
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
            push(&mut off, "URL", Type::Str { cap: 127 });
            push(&mut off, "NODE", Type::Str { cap: 255 });
            push(&mut off, "SVAL", Type::Str { cap: 255 });
            push(&mut off, "VAL", Type::Real { bits: 64 });
            push(&mut off, "VARTYPE", udint());
            push(&mut off, "STATE", dint());
            push(&mut off, "IDX", udint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
    off
}

        /* FB_Eip — tplc-eip service bridge (EtherNet/IP explicit
        carrier here). */
pub(super) fn arm_fb_eip(off: u64, m: &mut Members) -> u64 {
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
            push(&mut off, "HOST", Type::Str { cap: 63 });
            push(&mut off, "PORT", udint());
            push(&mut off, "CLS", udint());
            push(&mut off, "INST", udint());
            push(&mut off, "ATTR", udint());
            push(&mut off, "DATALEN", udint());
            push(&mut off, "DATA", Type::Str { cap: 255 });
            push(&mut off, "STATE", dint());
            push(&mut off, "SESSION", udint());
            push(&mut off, "LASTCIP", udint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
    off
}
