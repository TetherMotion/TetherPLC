//! tplc-rt builtin FB: s5.
use crate::*;

// ---------------------------------------------------------------------------
// FB_S5Conn / FB_S5Io — tplc-s5 service bridges (Siemens S5, RK512 job
// protocol over TCP with 3964 STX/ETX+BCC framing). FB_S5Conn owns the
// connection (plain TCP connect, no handshake); FB_S5Io runs one RK512
// FETCH / SEND transaction per EXEC edge on the DB area.
// ---------------------------------------------------------------------------

// FB_S5Conn member offsets (see builtin_fb_layout("FB_S5Conn"))
pub(crate) const S5C_HOST_LEN: usize = 0;
pub(crate) const S5C_HOST: usize = 1;
pub(crate) const S5C_PORT: usize = 16;
pub(crate) const S5C_TIMEOUT: usize = 24;
pub(crate) const S5C_CONN_HANDLE: usize = 32;
pub(crate) const S5C_STATUS: usize = 36;
pub(crate) const S5C_ERRID: usize = 40;
pub(crate) const S5C_CLOSE: usize = 44;
pub(crate) const S5C_EXEC: usize = 45;
pub(crate) const S5C_BUSY: usize = 46;
pub(crate) const S5C_DONE: usize = 47;
pub(crate) const S5C_LAST_EXEC: usize = 48;
pub(crate) const S5C_LAST_CLOSE: usize = 49;

// FB_S5Io member offsets (see builtin_fb_layout("FB_S5Io"))
pub(crate) const S5I_CONN_HANDLE: usize = 0;
pub(crate) const S5I_MODE: usize = 4;
pub(crate) const S5I_DB: usize = 8;
pub(crate) const S5I_BYTE_OFF: usize = 12;
pub(crate) const S5I_LEN: usize = 16;
pub(crate) const S5I_TIMEOUT: usize = 24;
pub(crate) const S5I_HANDLE: usize = 32;
pub(crate) const S5I_STATUS: usize = 36;
pub(crate) const S5I_ERRID: usize = 40;
pub(crate) const S5I_DATA_LEN: usize = 44;
pub(crate) const S5I_DATA: usize = 48;
pub(crate) const S5I_R_DATA: usize = 558;
pub(crate) const S5I_EXEC: usize = 1068;
pub(crate) const S5I_BUSY: usize = 1069;
pub(crate) const S5I_DONE: usize = 1070;
pub(crate) const S5I_LAST_EXEC: usize = 1071;

// TplcS5Service vtable slots (same layout as TplcS7Service)
pub(crate) const S5_SVC_OPEN: usize = 16;
pub(crate) const S5_SVC_CONN_POLL: usize = 24;
pub(crate) const S5_SVC_READ: usize = 32;
pub(crate) const S5_SVC_WRITE: usize = 40;
pub(crate) const S5_SVC_POLL: usize = 48;
pub(crate) const S5_SVC_CLOSE: usize = 64;

/// service lookup for "tplc-s5" (7 chars)
pub(crate) unsafe fn s5_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-s5".as_ptr() as *const u8, 7) as *const u8
}

#[inline]
pub(crate) unsafe fn s5fn(svc: *const u8, off: usize) -> usize {
    (svc.add(off) as *const usize).read()
}

pub(crate) type FS5Open = unsafe extern "C" fn(*const u8, *const u8, *mut u32) -> i32;
pub(crate) type FS5ConnPoll = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;
pub(crate) type FS5IoBegin = unsafe extern "C" fn(*const u8, u32, *const u8, *mut u32) -> i32;
pub(crate) type FS5Poll = unsafe extern "C" fn(*const u8, u32, *mut u8, *mut u8, u32) -> i32;
pub(crate) type FS5Close = unsafe extern "C" fn(*const u8, u32) -> i32;

#[inline]
pub(crate) unsafe fn s5c_fail(this: *mut u8, errid: u32) {
    wb(this.add(S5C_BUSY), false);
    wb(this.add(S5C_DONE), true);
    wu32(this.add(S5C_ERRID), errid);
    wi32(this.add(S5C_STATUS), 2);
}

#[inline]
pub(crate) unsafe fn s5i_fail(this: *mut u8, errid: u32) {
    wb(this.add(S5I_BUSY), false);
    wb(this.add(S5I_DONE), true);
    wu32(this.add(S5I_HANDLE), 0);
    wu32(this.add(S5I_ERRID), errid);
    wi32(this.add(S5I_STATUS), 3);
}

/// close a live connection handle through the service (best effort)
#[inline]
pub(crate) unsafe fn s5_close_handle(svc: *const u8, h: u32) {
    if h == 0 || svc.is_null() {
        return;
    }
    let f: FS5Close = core::mem::transmute(s5fn(svc, S5_SVC_CLOSE));
    f(svc, h);
}

/// `__tplc_rt_FB_S5Conn` — EXEC rising edge opens an S5 connection
/// (TCP connect only; RK512 needs no handshake); BUSY stays set
/// while s5_conn_poll drives it. STATUS mirrors the conn info
/// (0 pending, 1 ready, 2 error); DONE pulses on the terminal
/// poll. CLOSE rising edge closes the session.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_S5Conn(this: *mut u8, ctx: *mut u8) -> i32 {
    let exec = rb(this.add(S5C_EXEC));
    let last = rb(this.add(S5C_LAST_EXEC));
    wb(this.add(S5C_LAST_EXEC), exec);
    let cls = rb(this.add(S5C_CLOSE));
    let clast = rb(this.add(S5C_LAST_CLOSE));
    wb(this.add(S5C_LAST_CLOSE), cls);

    // CLOSE rising edge: tear down any live session
    if cls && !clast {
        let h = ru32(this.add(S5C_CONN_HANDLE));
        let svc = s5_service(ctx);
        s5_close_handle(svc, h);
        wu32(this.add(S5C_CONN_HANDLE), 0);
        wb(this.add(S5C_BUSY), false);
        wb(this.add(S5C_DONE), true);
        wi32(this.add(S5C_STATUS), 0);
        wu32(this.add(S5C_ERRID), 0);
    }

    if !rb(this.add(S5C_BUSY)) {
        if !(exec && !last) {
            return 0;
        }
        // EXEC rising edge: drop a stale handle, open a fresh conn
        let old = ru32(this.add(S5C_CONN_HANDLE));
        let svc = s5_service(ctx);
        if svc.is_null() {
            s5c_fail(this, 99);
            return 0;
        }
        s5_close_handle(svc, old);
        wu32(this.add(S5C_CONN_HANDLE), 0);

        /* TplcS5OpenReq (32 bytes): host[16] @0, port u16 @16,
        reserved u16 @18, timeout_ms u32 @20, reserved2 u32 @24,
        reserved3 u32 @28 */
        let mut req = [0u8; 32];
        let hn = (*this.add(S5C_HOST_LEN) as usize).min(15);
        if hn > 0 {
            core::ptr::copy_nonoverlapping(this.add(S5C_HOST), req.as_mut_ptr(), hn);
        }
        req[16..18].copy_from_slice(&(ru32(this.add(S5C_PORT)) as u16).to_le_bytes());
        req[20..24].copy_from_slice(&s7_timeout_ms(ri(this.add(S5C_TIMEOUT))).to_le_bytes());
        let mut h: u32 = 0;
        let f: FS5Open = core::mem::transmute(s5fn(svc, S5_SVC_OPEN));
        let rc = f(svc, req.as_ptr(), &mut h);
        if rc != 0 {
            s5c_fail(this, (-rc) as u32);
            return 0;
        }
        wu32(this.add(S5C_CONN_HANDLE), h);
        wb(this.add(S5C_DONE), false);
        wu32(this.add(S5C_ERRID), 0);
        wi32(this.add(S5C_STATUS), 0);
        wb(this.add(S5C_BUSY), true);
    }

    if !rb(this.add(S5C_BUSY)) {
        return 0;
    }
    let svc = s5_service(ctx);
    if svc.is_null() {
        s5c_fail(this, 99);
        return 0;
    }
    /* s5_conn_poll(h, info64) — TplcS5ConnInfo: status i32 @0,
    err i32 @4 */
    let mut info = [0u8; 64];
    let f: FS5ConnPoll = core::mem::transmute(s5fn(svc, S5_SVC_CONN_POLL));
    let rc = f(svc, ru32(this.add(S5C_CONN_HANDLE)), info.as_mut_ptr());
    if rc != 0 {
        s5c_fail(this, (-rc) as u32);
        return 0;
    }
    wi32(this.add(S5C_STATUS), ri32(info.as_ptr()));
    wu32(this.add(S5C_ERRID), ri32(info.as_ptr().add(4)) as u32);
    if ri32(info.as_ptr()) != 0 {
        wb(this.add(S5C_BUSY), false);
        wb(this.add(S5C_DONE), true);
    }
    0
}

/// `__tplc_rt_FB_S5Io` — EXEC rising edge begins one RK512 FETCH
/// (MODE=0) or SEND (MODE=1) transaction on CONN_HANDLE; BUSY
/// stays set while s5_poll drives it. R_DATA receives the read
/// payload (DATA_LEN bytes); ERRID mirrors the transport errno
/// or RK512 status byte.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_S5Io(this: *mut u8, ctx: *mut u8) -> i32 {
    let exec = rb(this.add(S5I_EXEC));
    let last = rb(this.add(S5I_LAST_EXEC));
    wb(this.add(S5I_LAST_EXEC), exec);

    if !rb(this.add(S5I_BUSY)) {
        if !(exec && !last) {
            return 0;
        }
        let svc = s5_service(ctx);
        if svc.is_null() {
            s5i_fail(this, 99);
            return 0;
        }
        let is_write = ru32(this.add(S5I_MODE)) != 0;

        /* TplcS5Addr (16 bytes): db u32 @0, byte_off u32 @4,
        len u32 @8, reserved u32 @12 — little-endian scalars */
        let fill_addr = |p: *mut u8| {
            core::ptr::write_bytes(p, 0, 16);
            let w = |off: usize, v: u32| {
                core::ptr::copy_nonoverlapping(v.to_le_bytes().as_ptr(), p.add(off), 4);
            };
            w(0, ru32(this.add(S5I_DB)));
            w(4, ru32(this.add(S5I_BYTE_OFF)));
            w(8, ru32(this.add(S5I_LEN)));
        };

        let mut h: u32 = 0;
        let rc = if is_write {
            /* TplcS5WriteReq (538 bytes): addr @0..16, dlen u32
            @16, timeout_ms u32 @20, reserved u32 @24,
            data[510] @28 */
            let mut req = [0u8; 538];
            fill_addr(req.as_mut_ptr());
            let dlen = ru32(this.add(S5I_LEN)).min(510);
            let mut w = |off: usize, v: u32| {
                core::ptr::copy_nonoverlapping(
                    v.to_le_bytes().as_ptr(),
                    req.as_mut_ptr().add(off),
                    4,
                );
            };
            w(16, dlen);
            w(20, s7_timeout_ms(ri(this.add(S5I_TIMEOUT))));
            if dlen > 0 {
                core::ptr::copy_nonoverlapping(
                    this.add(S5I_DATA),
                    req.as_mut_ptr().add(28),
                    dlen as usize,
                );
            }
            let f: FS5IoBegin = core::mem::transmute(s5fn(svc, S5_SVC_WRITE));
            f(svc, ru32(this.add(S5I_CONN_HANDLE)), req.as_ptr(), &mut h)
        } else {
            /* TplcS5ReadReq (24 bytes): addr @0..16,
            timeout_ms u32 @16, reserved u32 @20 */
            let mut req = [0u8; 24];
            fill_addr(req.as_mut_ptr());
            core::ptr::copy_nonoverlapping(
                s7_timeout_ms(ri(this.add(S5I_TIMEOUT)))
                    .to_le_bytes()
                    .as_ptr(),
                req.as_mut_ptr().add(16),
                4,
            );
            let f: FS5IoBegin = core::mem::transmute(s5fn(svc, S5_SVC_READ));
            f(svc, ru32(this.add(S5I_CONN_HANDLE)), req.as_ptr(), &mut h)
        };
        if rc != 0 {
            s5i_fail(this, (-rc) as u32);
            return 0;
        }
        wu32(this.add(S5I_HANDLE), h);
        wb(this.add(S5I_DONE), false);
        wu32(this.add(S5I_ERRID), 0);
        wu32(this.add(S5I_DATA_LEN), 0);
        wi32(this.add(S5I_STATUS), 0);
        wb(this.add(S5I_BUSY), true);
    }

    if !rb(this.add(S5I_BUSY)) {
        return 0;
    }
    let svc = s5_service(ctx);
    if svc.is_null() {
        s5i_fail(this, 99);
        return 0;
    }
    /* s5_poll(h, info64, val, val_cap) — TplcS5RespInfo:
    status i32 @0, err i32 @4, data_len u32 @8 */
    let mut info = [0u8; 64];
    let mut vbuf = [0u8; 510];
    let f: FS5Poll = core::mem::transmute(s5fn(svc, S5_SVC_POLL));
    let rc = f(
        svc,
        ru32(this.add(S5I_HANDLE)),
        info.as_mut_ptr(),
        vbuf.as_mut_ptr(),
        510,
    );
    if rc != 0 {
        s5i_fail(this, (-rc) as u32);
        return 0;
    }
    let st = ri32(info.as_ptr());
    wi32(this.add(S5I_STATUS), st);
    wu32(this.add(S5I_ERRID), ri32(info.as_ptr().add(4)) as u32);
    let dlen = ru32(info.as_ptr().add(8));
    wu32(this.add(S5I_DATA_LEN), dlen);
    if ru32(this.add(S5I_MODE)) == 0 && dlen > 0 {
        let cp = (dlen as usize).min(510);
        core::ptr::copy_nonoverlapping(vbuf.as_ptr(), this.add(S5I_R_DATA), cp);
    }
    if st != 0 {
        wb(this.add(S5I_BUSY), false);
        wb(this.add(S5I_DONE), true);
        wu32(this.add(S5I_HANDLE), 0);
    }
    0
}
