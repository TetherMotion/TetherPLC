//! builtin_fb_layout arms — tail/b domain.
use super::*;

        /* FB_XmlSrv — tplc-xmlsrv file-backed XML store bridge
        2=write (PATH/XPATH/VAL). */
pub(super) fn arm_fb_xml_srv(off: u64, m: &mut Members) -> u64 {
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
            push(&mut off, "VALLEN", udint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "PATH", Type::Str { cap: 127 });
            push(&mut off, "XPATH", Type::Str { cap: 127 });
            push(&mut off, "VAL", Type::Str { cap: 255 });
    off
}

        /* FB_Sym — tplc-sym by-name variable access
        8=read bool (-> BB), 9=find (-> TYPE/SIZE/FOUND). */
pub(super) fn arm_fb_sym(off: u64, m: &mut Members) -> u64 {
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
            push(&mut off, "VTYP", udint());
            push(&mut off, "SIZE", udint());
            push(&mut off, "VALLEN", udint());
            push(&mut off, "FOUND", Type::Bool);
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "IB", dint());
            push(&mut off, "FB", Type::Real { bits: 64 });
            push(&mut off, "BB", Type::Bool);
            push(&mut off, "NAME", Type::Str { cap: 63 });
            push(&mut off, "VAL", Type::Str { cap: 255 });
    off
}

        /* FB_Iof — I/O device/box enumeration (Tc2_IoFunctions
        4=device count, 5=device name (-> VAL). */
pub(super) fn arm_fb_iof(off: u64, m: &mut Members) -> u64 {
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
            push(&mut off, "IDX", udint());
            push(&mut off, "COUNT", udint());
            push(&mut off, "REGION", udint());
            push(&mut off, "ADDR", udint());
            push(&mut off, "UNIT", udint());
            push(&mut off, "FOUND", Type::Bool);
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "NAME", Type::Str { cap: 63 });
            push(&mut off, "VAL", Type::Str { cap: 63 });
    off
}

pub(super) fn arm_fb_mbus(off: u64, m: &mut Members) -> u64 {
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
            push(&mut off, "CONTROL", udint());
            push(&mut off, "ADDRESS", udint());
            push(&mut off, "CI", udint());
            push(&mut off, "DATA", Type::Str { cap: 255 });
            push(&mut off, "KIND", udint());
            push(&mut off, "RXPAYLOAD", Type::Str { cap: 255 });
            push(&mut off, "STATE", dint());
            push(&mut off, "TXTOT", udint());
            push(&mut off, "RXTOT", udint());
            push(&mut off, "LASTERR", dint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
    off
}
