//! tplc-rt builtin FB: serial.
use crate::*;

/* ---------------- FB_Serial (tplc-serial service bridge) ---------------- */

// TplcSerialService fn ptrs start @16 (after struct_size/abi_version/
// host), eight bytes apart.
pub(crate) const SER_OPEN: usize = 16;
pub(crate) const SER_WRITE: usize = 24;
pub(crate) const SER_READ: usize = 32;
pub(crate) const SER_FLUSH: usize = 40;
pub(crate) const SER_RS485: usize = 48;
pub(crate) const SER_CLOSE: usize = 56;

// member offsets — keep in lockstep with builtin_fb_layout "FB_Serial"
pub(crate) const SE_OP: usize = 0;
pub(crate) const SE_DEV_LEN: usize = 4;
pub(crate) const SE_DEV: usize = 5;
pub(crate) const SE_BAUD: usize = 68;
pub(crate) const SE_PARITY: usize = 72;
pub(crate) const SE_DBITS: usize = 76;
pub(crate) const SE_SBITS: usize = 80;
pub(crate) const SE_FLAGS: usize = 84;
pub(crate) const SE_AUX: usize = 88;
pub(crate) const SE_TIMEOUT: usize = 96;
pub(crate) const SE_DATA_LEN: usize = 104;
pub(crate) const SE_DATA: usize = 105;
pub(crate) const SE_DLEN: usize = 364;
pub(crate) const SE_HANDLE: usize = 368;
pub(crate) const SE_RESULT: usize = 372;
pub(crate) const SE_ERRID: usize = 376;
pub(crate) const SE_EXEC: usize = 380;
pub(crate) const SE_BUSY: usize = 381;
pub(crate) const SE_DONE: usize = 382;
pub(crate) const SE_LAST_EXEC: usize = 383;
pub(crate) const SE_DEADLINE: usize = 384;
pub(crate) const SE_SENT: usize = 392;

pub(crate) const SER_OP_OPEN: i32 = 1;
pub(crate) const SER_OP_WRITE: i32 = 2;
pub(crate) const SER_OP_READ: i32 = 3;
pub(crate) const SER_OP_CLOSE: i32 = 4;
pub(crate) const SER_OP_FLUSH: i32 = 5;
pub(crate) const SER_OP_RS485: i32 = 6;

// mirrored TplcSerialService result codes (positive form for ERRID)
pub(crate) const SERERR_PARAM: u32 = 1;
pub(crate) const SERERR_NOSERVICE: u32 = 2;
pub(crate) const SERERR_TIMEOUT: u32 = 5;

pub(crate) type FSerOpen = unsafe extern "C" fn(*const u8, *const u8, u32, u8, u8, u8, u32, *mut u32) -> i32;
pub(crate) type FSerWrite = unsafe extern "C" fn(*const u8, u32, *const u8, u32, *mut u32) -> i32;
pub(crate) type FSerRead = unsafe extern "C" fn(*const u8, u32, *mut u8, u32, *mut u32) -> i32;
pub(crate) type FSerFlush = unsafe extern "C" fn(*const u8, u32, u8) -> i32;
pub(crate) type FSerRs485 = unsafe extern "C" fn(*const u8, u32, u8) -> i32;
pub(crate) type FSerClose = unsafe extern "C" fn(*const u8, u32) -> i32;

pub(crate) unsafe fn ser_fail(this: *mut u8, errid: u32) {
    wb(this.add(SE_BUSY), false);
    wb(this.add(SE_DONE), true);
    wu32(this.add(SE_ERRID), errid);
    wi32(this.add(SE_RESULT), 0);
}

pub(crate) unsafe fn ser_done(this: *mut u8, result: i32) {
    wb(this.add(SE_BUSY), false);
    wb(this.add(SE_DONE), true);
    wu32(this.add(SE_ERRID), 0);
    wi32(this.add(SE_RESULT), result);
}

/// copy the len-prefixed DEV member into a NUL-terminated buffer
pub(crate) unsafe fn dev_cstr(this: *const u8, out: *mut u8) {
    let n = (*this.add(SE_DEV_LEN) as usize).min(63);
    core::ptr::copy_nonoverlapping(this.add(SE_DEV), out, n);
    *out.add(n) = 0;
}

/// resolve the tplc-serial service table through ctx->svc->get_service
pub(crate) unsafe fn ser_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-serial".as_ptr() as *const u8, 11) as *const u8
}

/// load a TplcSerialService slot as a raw pointer-sized value
#[inline]
pub(crate) unsafe fn serfn(ser: *const u8, off: usize) -> usize {
    (ser.add(off) as *const usize).read()
}

/// `__tplc_rt_FB_Serial` — EXEC rising edge arms an op; while BUSY the
/// op is polled each scan until DONE or the TIMEOUT deadline. OP codes
/// mirror the TPLC_SERIAL_* op constants; ERRID carries the positive
/// service code.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_Serial(this: *mut u8, ctx: *mut u8) -> i32 {
    let exec = rb(this.add(SE_EXEC));
    let last = rb(this.add(SE_LAST_EXEC));
    let now = now_ns(ctx);

    if !rb(this.add(SE_BUSY)) {
        if !(exec && !last) {
            wb(this.add(SE_LAST_EXEC), exec);
            return 0;
        }
        // rising edge: arm the op
        wb(this.add(SE_DONE), false);
        wu32(this.add(SE_ERRID), 0);
        wi32(this.add(SE_RESULT), 0);
        wu32(this.add(SE_SENT), 0);
        wb(this.add(SE_BUSY), true);
        let to = ri(this.add(SE_TIMEOUT));
        wi(
            this.add(SE_DEADLINE),
            if to > 0 {
                now.saturating_add(to)
            } else {
                i64::MAX
            },
        );
    }
    wb(this.add(SE_LAST_EXEC), exec);

    let ser = ser_service(ctx);
    if ser.is_null() {
        ser_fail(this, SERERR_NOSERVICE);
        return 0;
    }
    if now > ri(this.add(SE_DEADLINE)) {
        ser_fail(this, SERERR_TIMEOUT);
        return 0;
    }

    let op = ri32(this.add(SE_OP));
    let mut devbuf = [0u8; 64];
    match op {
        SER_OP_OPEN => {
            dev_cstr(this, devbuf.as_mut_ptr());
            let mut out: u32 = 0;
            let f: FSerOpen = core::mem::transmute(serfn(ser, SER_OPEN));
            let rc = f(
                ser,
                devbuf.as_ptr(),
                ru32(this.add(SE_BAUD)),
                ru32(this.add(SE_DBITS)) as u8,
                ru32(this.add(SE_PARITY)) as u8,
                ru32(this.add(SE_SBITS)) as u8,
                ru32(this.add(SE_FLAGS)),
                &mut out,
            );
            if rc < 0 {
                ser_fail(this, (-rc) as u32);
            } else {
                wu32(this.add(SE_HANDLE), out);
                ser_done(this, 0);
            }
        }
        SER_OP_WRITE => {
            let dlen = ru32(this.add(SE_DLEN)) as usize;
            let slen = (*this.add(SE_DATA_LEN) as usize).min(256);
            let total = if dlen > 0 { dlen.min(256) } else { slen };
            let mut sent = ru32(this.add(SE_SENT)) as usize;
            let f: FSerWrite = core::mem::transmute(serfn(ser, SER_WRITE));
            for _ in 0..8 {
                if sent >= total {
                    break;
                }
                let mut done: u32 = 0;
                match f(
                    ser,
                    ru32(this.add(SE_HANDLE)),
                    this.add(SE_DATA).add(sent),
                    (total - sent) as u32,
                    &mut done,
                ) {
                    0 => {
                        sent += done as usize;
                        if done == 0 {
                            break;
                        }
                    }
                    -6 => break, // E_AGAIN: nothing moved this scan
                    e => {
                        ser_fail(this, (-e) as u32);
                        return 0;
                    }
                }
            }
            wu32(this.add(SE_SENT), sent as u32);
            if sent >= total {
                ser_done(this, sent as i32);
            }
        }
        SER_OP_READ => {
            let mut done: u32 = 0;
            let f: FSerRead = core::mem::transmute(serfn(ser, SER_READ));
            match f(
                ser,
                ru32(this.add(SE_HANDLE)),
                this.add(SE_DATA),
                256,
                &mut done,
            ) {
                0 => {
                    *this.add(SE_DATA_LEN) = done.min(256) as u8;
                    wu32(this.add(SE_DLEN), done);
                    ser_done(this, done as i32);
                }
                -6 => {}
                e => ser_fail(this, (-e) as u32),
            }
        }
        SER_OP_CLOSE => {
            let f: FSerClose = core::mem::transmute(serfn(ser, SER_CLOSE));
            f(ser, ru32(this.add(SE_HANDLE)));
            ser_done(this, 0);
        }
        SER_OP_FLUSH => {
            let f: FSerFlush = core::mem::transmute(serfn(ser, SER_FLUSH));
            let rc = f(ser, ru32(this.add(SE_HANDLE)), ru32(this.add(SE_AUX)) as u8);
            if rc < 0 {
                ser_fail(this, (-rc) as u32);
            } else {
                ser_done(this, 0);
            }
        }
        SER_OP_RS485 => {
            let f: FSerRs485 = core::mem::transmute(serfn(ser, SER_RS485));
            let rc = f(ser, ru32(this.add(SE_HANDLE)), ru32(this.add(SE_AUX)) as u8);
            if rc < 0 {
                ser_fail(this, (-rc) as u32);
            } else {
                ser_done(this, 0);
            }
        }
        _ => ser_fail(this, SERERR_PARAM),
    }
    0
}
