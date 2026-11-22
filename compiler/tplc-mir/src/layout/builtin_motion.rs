//! builtin_fb_layout arms — motion domain.
use super::*;

        /* FB_MotionAxis — tplc-motion service bridge (simulated
        destroy invocation, so no EXEC edge is needed. */
pub(super) fn arm_fb_motion_axis(off: u64, m: &mut Members) -> u64 {
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
            push(&mut off, "AXIS", udint());
            push(&mut off, "UNITS_PER_REV", dint());
            push(&mut off, "MAX_VEL", dint());
            push(&mut off, "MAX_ACCEL", dint());
            push(&mut off, "MAX_DECEL", dint());
            push(&mut off, "POS_MIN", dint());
            push(&mut off, "POS_MAX", dint());
            push(&mut off, "HOME_OFFSET", dint());
            push(&mut off, "FOLLOW_WINDOW", dint());
            push(&mut off, "FLAGS", udint());
            push(&mut off, "INDEX", udint());
            push(&mut off, "VALUE", dint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "MODE", dint());
            push(&mut off, "FAULT", dint());
            push(&mut off, "TARGET_REACHED", dint());
            push(&mut off, "BUSY", dint());
            push(&mut off, "POS_ACTUAL", dint());
            push(&mut off, "VEL_ACTUAL", dint());
    off
}

        /* FB_NciChan — tplc-nci service bridge (NC interpreter channel
        channel's own error code from the last status read. */
pub(super) fn arm_fb_nci_chan(off: u64, m: &mut Members) -> u64 {
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
            push(&mut off, "CHAN", udint());
            push(&mut off, "PATH", Type::Str { cap: 127 });
            push(&mut off, "PROG", Type::Str { cap: 4095 });
            push(&mut off, "CMD", udint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "STATE", dint());
            push(&mut off, "LINE", udint());
            push(&mut off, "TOTAL", udint());
            push(&mut off, "NERR", dint());
            push(&mut off, "PROGRESS", dint());
            push(&mut off, "DURATION", dint());
            push(&mut off, "AUX", udint());
            push(&mut off, "RVAL", Type::Real { bits: 64 });
            push(&mut off, "ZSX", Type::Real { bits: 64 });
            push(&mut off, "ZSY", Type::Real { bits: 64 });
            push(&mut off, "ZSZ", Type::Real { bits: 64 });
            push(&mut off, "MFUNC", udint());
            push(&mut off, "FIFOLEN", udint());
            push(&mut off, "FIFOSEQ", udint());
    off
}

        /* FB_Fifo — tplc-fifo service bridge (Tc2_NcFifoAxes numeric
        return code, ERRID its positive form. */
pub(super) fn arm_fb_fifo(off: u64, m: &mut Members) -> u64 {
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
            push(&mut off, "CHAN", udint());
            push(&mut off, "AUX", udint());
            push(&mut off, "NROWS", udint());
            push(&mut off, "VAL", Type::Real { bits: 64 });
            push(&mut off, "STATE", dint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "QLEN", udint());
            push(&mut off, "SEQ", udint());
            push(&mut off, "NAXES", udint());
            push(&mut off, "MAXROWS", udint());
            push(
                &mut off,
                "ROWS",
                Type::Array {
                    elem: Box::new(Type::Real { bits: 64 }),
                    lo: 0,
                    hi: 255,
                },
            );
    off
}

        /* FB_Kin — tplc-kin service bridge (Tc2_NcKinematicTransformation
        return code, ERRID its positive form. */
pub(super) fn arm_fb_kin(off: u64, m: &mut Members) -> u64 {
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
            let f64arr = |n: u64| Type::Array {
                elem: Box::new(lreal()),
                lo: 0,
                hi: n as i64 - 1,
            };
            let u32arr = |n: u64| Type::Array {
                elem: Box::new(udint()),
                lo: 0,
                hi: n as i64 - 1,
            };
            push(&mut off, "OP", udint());
            push(&mut off, "OBJ", udint());
            push(&mut off, "TYPEIN", dint());
            push(&mut off, "PRESET", udint());
            push(&mut off, "NCFG", udint());
            push(&mut off, "NIN", udint());
            push(&mut off, "NACS", udint());
            push(&mut off, "NMCS", udint());
            push(&mut off, "CMD", udint());
            push(&mut off, "ROP", udint());
            push(&mut off, "CHN", udint());
            push(&mut off, "LOCKED", udint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "STATE", dint());
            push(&mut off, "KTYPE", dint());
            push(&mut off, "DOF", dint());
            push(&mut off, "ERR", dint());
            push(&mut off, "CHNST", dint());
            push(&mut off, "CFG", f64arr(32));
            push(&mut off, "IN", f64arr(8));
            push(&mut off, "OUT", f64arr(8));
            push(&mut off, "Q", f64arr(8));
            push(&mut off, "ACS", u32arr(8));
            push(&mut off, "MCS", u32arr(8));
            push(&mut off, "ROT", f64arr(3));
    off
}

        /* FB_McGroup — tplc-mcgroup service bridge
           the group snapshot for OP 6/10. */
pub(super) fn arm_fb_mc_group(off: u64, m: &mut Members) -> u64 {
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
            let u32arr = |n: u64| Type::Array {
                elem: Box::new(udint()),
                lo: 0,
                hi: n as i64 - 1,
            };
            push(&mut off, "OP", udint());
            push(&mut off, "GRP", udint());
            push(&mut off, "AXIS", udint());
            push(&mut off, "IDENT", udint());
            push(&mut off, "ENABLE", dint());
            push(&mut off, "SOURCE", dint());
            push(&mut off, "COORD", udint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "STATE", dint());
            push(&mut off, "NAXES", dint());
            push(&mut off, "ENABLED", dint());
            push(&mut off, "SRC", dint());
            push(&mut off, "OVR", lreal());
            push(&mut off, "OVRACT", lreal());
            push(&mut off, "AXES", u32arr(16));
    off
}

        /* FB_Cam — tplc-cam service bridge (Tc3_Mc3Camming cam
        the service return code, ERRID its magnitude. */
pub(super) fn arm_fb_cam(off: u64, m: &mut Members) -> u64 {
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
            push(&mut off, "HANDLE", udint());
            push(&mut off, "AUX", udint());
            push(&mut off, "AUX2", udint());
            push(&mut off, "FLAGS", udint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "NPTS", udint());
            push(&mut off, "F1", lreal());
            push(&mut off, "F2", lreal());
            push(&mut off, "F3", lreal());
            push(&mut off, "F4", lreal());
    off
}

        /* FB_Path — tplc-path service bridge (Tc3_McCoordinatedMotion
        code, ERRID its magnitude. */
pub(super) fn arm_fb_path(off: u64, m: &mut Members) -> u64 {
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
            let f64arr = |n: u64| Type::Array {
                elem: Box::new(lreal()),
                lo: 0,
                hi: n as i64 - 1,
            };
            push(&mut off, "OP", udint());
            push(&mut off, "HANDLE", udint());
            push(&mut off, "AUX", udint());
            push(&mut off, "AUX2", udint());
            push(&mut off, "FLAGS", udint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "NSEG", udint());
            push(&mut off, "DONE", udint());
            push(&mut off, "NOW", lreal());
            push(&mut off, "F1", lreal());
            push(&mut off, "F2", lreal());
            push(&mut off, "F3", lreal());
            push(&mut off, "POS", f64arr(16));
            push(&mut off, "AUXP", f64arr(16));
            let u32arr = |n: u64| Type::Array {
                elem: Box::new(udint()),
                lo: 0,
                hi: n as i64 - 1,
            };
            push(&mut off, "TRID", u32arr(10));
            push(&mut off, "TACT", u32arr(10));
    off
}
