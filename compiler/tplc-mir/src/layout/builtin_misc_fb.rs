//! builtin_fb_layout arms — misc/fb domain.
use super::*;

        /* FB_CanOpen — tplc-canopen service bridge (CANopen/CoE CiA 301
        ERRID its positive form. DATA is a 256-byte payload buffer. */
pub(super) fn arm_fb_can_open(off: u64, m: &mut Members) -> u64 {
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
            push(&mut off, "OP", udint());
            push(&mut off, "SLAVE", udint());
            push(&mut off, "INDEX", udint());
            push(&mut off, "SUB", udint());
            push(&mut off, "VERIFY", udint());
            push(&mut off, "DLEN", udint());
            push(&mut off, "ABORT", udint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "DATA", Type::Array {
                elem: Box::new(byte()),
                lo: 0,
                hi: 255,
            });
    off
}

        /* FB_IoLink — tplc-iolink service bridge (IO-Link master
        form. */
pub(super) fn arm_fb_io_link(off: u64, m: &mut Members) -> u64 {
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
            let arr = |hi: i64| Type::Array {
                elem: Box::new(byte()),
                lo: 0,
                hi,
            };
            push(&mut off, "OP", udint());
            push(&mut off, "PORT", udint());
            push(&mut off, "MSEQ", udint());
            push(&mut off, "ADDR", udint());
            push(&mut off, "DLEN", udint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "CKSFLAGS", udint());
            push(&mut off, "IDENT", arr(15));
            push(&mut off, "PDIN", arr(39));
            push(&mut off, "PDOUT", arr(31));
            push(&mut off, "OD", arr(7));
            push(&mut off, "ISDUERR", udint());
            push(&mut off, "ISDU", arr(239));
            push(&mut off, "DIAG", arr(18)); /* event memory 19 octets */
            push(&mut off, "ALPDO", udint());
            push(&mut off, "ALPDI", udint());
    off
}

        /* FB_Profinet — tplc-profinet service bridge (PROFINET DCP
        STATUS carries the service return code, ERRID its magnitude. */
pub(super) fn arm_fb_profinet(off: u64, m: &mut Members) -> u64 {
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
            let arr = |hi: i64| Type::Array {
                elem: Box::new(byte()),
                lo: 0,
                hi,
            };
            push(&mut off, "OP", udint());
            push(&mut off, "PORT", udint());
            push(&mut off, "XID", udint());
            push(&mut off, "OPTION", udint());
            push(&mut off, "SUB", udint());
            push(&mut off, "DLEN", udint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "ROLE", udint());
            push(&mut off, "VENDOR", udint());
            push(&mut off, "DEVID", udint());
            push(&mut off, "DEVINST", udint());
            push(&mut off, "OEM", udint());
            push(&mut off, "NAME", Type::Str { cap: 31 });
            push(&mut off, "OD", arr(63));
            push(&mut off, "IOOUT", arr(63));
            push(&mut off, "IOIN", arr(63));
            push(&mut off, "PNCYC", udint());  /* OP8: device cycle counter */
            push(&mut off, "PNSTAT", udint()); /* OP8: DataStatus byte */
            /* PNIO-CM AR session ops (OP 9..14): UUIDs for Connect,
               ARBLK carries encoded PNIO blocks in and response blocks
               out (the service copies input before writing output, so
               the buffer may alias), ARSTATE/ARKEY/ARPSTAT receive the
               AR state, session key and NDR pnio_status. */
            push(&mut off, "DEVOBJ", arr(15));
            push(&mut off, "ACTUUID", arr(15));
            push(&mut off, "ARUUID", arr(15));
            push(&mut off, "ARBLK", arr(511));
            push(&mut off, "ARSTATE", udint());
            push(&mut off, "ARKEY", udint());
            push(&mut off, "ARPSTAT", udint());
    off
}

        /* FB_Soe — tplc-soe service bridge (EtherCAT SoE mailbox over
        code, ERRID its positive form. */
pub(super) fn arm_fb_soe(off: u64, m: &mut Members) -> u64 {
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
            let lreal = || Type::Real { bits: 64 };
            push(&mut off, "OP", udint());
            push(&mut off, "SLAVE", udint());
            push(&mut off, "VALUE", dint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "STATUSW", udint());
            push(&mut off, "CMDSTATE", udint());
            push(&mut off, "PASSWD", udint());
            push(&mut off, "DIAGNUM", udint());
            push(&mut off, "CLASSX", udint());
            push(&mut off, "DIAGCOUNT", udint());
            push(&mut off, "DIAGLIST", Type::Array {
                elem: Box::new(udint()),
                lo: 0,
                hi: 7,
            });
            push(&mut off, "AMPTEMP", lreal());
            push(&mut off, "MTEMPT", lreal());
            push(&mut off, "DCVOLT", lreal());
            push(&mut off, "DCCURR", lreal());
            push(&mut off, "AXMAINS", lreal());
            push(&mut off, "AXCTRL", udint());
            push(&mut off, "AXPARK", udint());
            push(&mut off, "AXFWREV", udint());
            push(&mut off, "AXFWBUILD", udint());
            push(&mut off, "CMDCTRL", udint());
            push(&mut off, "DIAGMSG", Type::Str { cap: 63 });
    off
}

        /* FB_Ocpp — tplc-ocpp service bridge (OCPP-J client over
        the service return code, ERRID its positive form. */
pub(super) fn arm_fb_ocpp(off: u64, m: &mut Members) -> u64 {
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
            push(&mut off, "PATH", Type::Str { cap: 63 });
            push(&mut off, "OCPPID", Type::Str { cap: 63 });
            push(&mut off, "MSGID", Type::Str { cap: 63 });
            push(&mut off, "ACT", Type::Str { cap: 63 });
            push(&mut off, "CODE", Type::Str { cap: 63 });
            push(&mut off, "DESC", Type::Str { cap: 127 });
            push(&mut off, "PAYLOAD", Type::Str { cap: 255 });
            push(&mut off, "RXACTION", Type::Str { cap: 63 });
            push(&mut off, "RXPAYLOAD", Type::Str { cap: 255 });
            push(&mut off, "RXMSGTYPE", udint());
            push(&mut off, "STATE", dint());
            push(&mut off, "RXTOT", udint());
            push(&mut off, "TXTOT", udint());
            push(&mut off, "LASTERR", dint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
    off
}

        /* FB_Aes70 — tplc-aes70 service bridge (OCA/AES70 audio
        STRING[255] raw bytes. */
pub(super) fn arm_fb_aes70(off: u64, m: &mut Members) -> u64 {
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
            push(&mut off, "TARGET", udint());
            push(&mut off, "METH", udint());
            push(&mut off, "PARAMS", Type::Str { cap: 255 });
            push(&mut off, "HANDLE2", udint());
            push(&mut off, "STATUSCODE", udint());
            push(&mut off, "RXPAYLOAD", Type::Str { cap: 255 });
            push(&mut off, "STATE", dint());
            push(&mut off, "RXTOT", udint());
            push(&mut off, "TXTOT", udint());
            push(&mut off, "LASTERR", dint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
    off
}
