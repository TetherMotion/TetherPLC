//! tplc-rt builtin FB: s7.
use crate::*;

// ---------------------------------------------------------------------------
// FB_S7Conn / FB_S7Io — tplc-s7 service bridges (Siemens S7comm,
// RFC1006 ISO-on-TCP). FB_S7Conn owns the persistent COTP session
// (open handshake polled via BUSY, CLOSE edge tears it down);
// FB_S7Io runs one Read Var / Write Var transaction per EXEC edge
// against a live CONN_HANDLE (one outstanding io per connection).
// ---------------------------------------------------------------------------

// FB_S7Conn member offsets (see builtin_fb_layout("FB_S7Conn"))
pub(crate) const S7C_HOST_LEN: usize = 0;
pub(crate) const S7C_HOST: usize = 1;
pub(crate) const S7C_PORT: usize = 16;
pub(crate) const S7C_RACK: usize = 20;
pub(crate) const S7C_SLOT: usize = 24;
pub(crate) const S7C_TIMEOUT: usize = 32;
pub(crate) const S7C_CONN_HANDLE: usize = 40;
pub(crate) const S7C_STATUS: usize = 44;
pub(crate) const S7C_ERRID: usize = 48;
pub(crate) const S7C_CLOSE: usize = 52;
pub(crate) const S7C_EXEC: usize = 53;
pub(crate) const S7C_BUSY: usize = 54;
pub(crate) const S7C_DONE: usize = 55;
pub(crate) const S7C_LAST_EXEC: usize = 56;
pub(crate) const S7C_LAST_CLOSE: usize = 57;

// FB_S7Io member offsets (see builtin_fb_layout("FB_S7Io"))
pub(crate) const S7I_CONN_HANDLE: usize = 0;
pub(crate) const S7I_MODE: usize = 4;
pub(crate) const S7I_AREA: usize = 8;
pub(crate) const S7I_DB: usize = 12;
pub(crate) const S7I_BYTE_OFF: usize = 16;
pub(crate) const S7I_BIT_ACCESS: usize = 20;
pub(crate) const S7I_BIT_NO: usize = 24;
pub(crate) const S7I_LEN: usize = 28;
pub(crate) const S7I_TIMEOUT: usize = 32;
pub(crate) const S7I_HANDLE: usize = 40;
pub(crate) const S7I_STATUS: usize = 44;
pub(crate) const S7I_ITEM_RC: usize = 48;
pub(crate) const S7I_DATA_LEN: usize = 52;
pub(crate) const S7I_ERRID: usize = 56;
pub(crate) const S7I_DATA: usize = 60;
pub(crate) const S7I_R_DATA: usize = 300;
pub(crate) const S7I_EXEC: usize = 540;
pub(crate) const S7I_BUSY: usize = 541;
pub(crate) const S7I_DONE: usize = 542;
pub(crate) const S7I_LAST_EXEC: usize = 543;

// TplcS7Service vtable slots
pub(crate) const S7_SVC_OPEN: usize = 16;
pub(crate) const S7_SVC_CONN_POLL: usize = 24;
pub(crate) const S7_SVC_READ: usize = 32;
pub(crate) const S7_SVC_WRITE: usize = 40;
pub(crate) const S7_SVC_POLL: usize = 48;
pub(crate) const S7_SVC_CLOSE: usize = 64;

/// service lookup for "tplc-s7" (7 chars)
pub(crate) unsafe fn s7_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-s7".as_ptr() as *const u8, 7) as *const u8
}

#[inline]
pub(crate) unsafe fn s7fn(svc: *const u8, off: usize) -> usize {
    (svc.add(off) as *const usize).read()
}

pub(crate) type FS7Open = unsafe extern "C" fn(*const u8, *const u8, *mut u32) -> i32;
pub(crate) type FS7ConnPoll = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;
pub(crate) type FS7IoBegin = unsafe extern "C" fn(*const u8, u32, *const u8, *mut u32) -> i32;
pub(crate) type FS7Poll = unsafe extern "C" fn(*const u8, u32, *mut u8, *mut u8, u32) -> i32;
pub(crate) type FS7Close = unsafe extern "C" fn(*const u8, u32) -> i32;

#[inline]
pub(crate) unsafe fn s7c_fail(this: *mut u8, errid: u32) {
    wb(this.add(S7C_BUSY), false);
    wb(this.add(S7C_DONE), true);
    wu32(this.add(S7C_ERRID), errid);
    wi32(this.add(S7C_STATUS), 2);
}

#[inline]
pub(crate) unsafe fn s7i_fail(this: *mut u8, errid: u32) {
    wb(this.add(S7I_BUSY), false);
    wb(this.add(S7I_DONE), true);
    wu32(this.add(S7I_HANDLE), 0);
    wu32(this.add(S7I_ERRID), errid);
    wi32(this.add(S7I_STATUS), 3);
}

/// ns -> ms for the service timeout fields (0 = service default)
#[inline]
pub(crate) unsafe fn s7_timeout_ms(tns: i64) -> u32 {
    if tns <= 0 {
        0
    } else {
        ((tns / 1_000_000).min(60000)) as u32
    }
}

/// close a live connection handle through the service (best effort)
#[inline]
pub(crate) unsafe fn s7_close_handle(svc: *const u8, h: u32) {
    if h == 0 || svc.is_null() {
        return;
    }
    let f: FS7Close = core::mem::transmute(s7fn(svc, S7_SVC_CLOSE));
    f(svc, h);
}

/// `__tplc_rt_FB_S7Conn` — EXEC rising edge opens a persistent S7
/// connection; BUSY stays set while the TCP/COTP/Setup-Comm
/// handshake is driven through s7_conn_poll. STATUS mirrors the
/// conn info (0 pending, 1 ready, 2 error); DONE pulses on the
/// terminal poll. CLOSE rising edge closes the session.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_S7Conn(this: *mut u8, ctx: *mut u8) -> i32 {
    let exec = rb(this.add(S7C_EXEC));
    let last = rb(this.add(S7C_LAST_EXEC));
    wb(this.add(S7C_LAST_EXEC), exec);
    let cls = rb(this.add(S7C_CLOSE));
    let clast = rb(this.add(S7C_LAST_CLOSE));
    wb(this.add(S7C_LAST_CLOSE), cls);

    // CLOSE rising edge: tear down any live session
    if cls && !clast {
        let h = ru32(this.add(S7C_CONN_HANDLE));
        let svc = s7_service(ctx);
        s7_close_handle(svc, h);
        wu32(this.add(S7C_CONN_HANDLE), 0);
        wb(this.add(S7C_BUSY), false);
        wb(this.add(S7C_DONE), true);
        wi32(this.add(S7C_STATUS), 0);
        wu32(this.add(S7C_ERRID), 0);
    }

    if !rb(this.add(S7C_BUSY)) {
        if !(exec && !last) {
            return 0;
        }
        // EXEC rising edge: drop a stale handle, open a fresh conn
        let old = ru32(this.add(S7C_CONN_HANDLE));
        let svc = s7_service(ctx);
        if svc.is_null() {
            s7c_fail(this, 99);
            return 0;
        }
        s7_close_handle(svc, old);
        wu32(this.add(S7C_CONN_HANDLE), 0);

        /* TplcS7OpenReq (32 bytes): host[16] @0, port u16 @16,
        rack u16 @18, slot u16 @20, reserved u16 @22,
        timeout_ms u32 @24, reserved2 u32 @28 */
        let mut req = [0u8; 32];
        let hn = (*this.add(S7C_HOST_LEN) as usize).min(15);
        if hn > 0 {
            core::ptr::copy_nonoverlapping(this.add(S7C_HOST), req.as_mut_ptr(), hn);
        }
        req[16..18].copy_from_slice(&(ru32(this.add(S7C_PORT)) as u16).to_le_bytes());
        req[18..20].copy_from_slice(&(ru32(this.add(S7C_RACK)) as u16).to_le_bytes());
        req[20..22].copy_from_slice(&(ru32(this.add(S7C_SLOT)) as u16).to_le_bytes());
        req[24..28].copy_from_slice(&s7_timeout_ms(ri(this.add(S7C_TIMEOUT))).to_le_bytes());
        let mut h: u32 = 0;
        let f: FS7Open = core::mem::transmute(s7fn(svc, S7_SVC_OPEN));
        let rc = f(svc, req.as_ptr(), &mut h);
        if rc != 0 {
            s7c_fail(this, (-rc) as u32);
            return 0;
        }
        wu32(this.add(S7C_CONN_HANDLE), h);
        wb(this.add(S7C_DONE), false);
        wu32(this.add(S7C_ERRID), 0);
        wi32(this.add(S7C_STATUS), 0);
        wb(this.add(S7C_BUSY), true);
    }

    if !rb(this.add(S7C_BUSY)) {
        return 0;
    }
    let svc = s7_service(ctx);
    if svc.is_null() {
        s7c_fail(this, 99);
        return 0;
    }
    /* s7_conn_poll(h, info64) — TplcS7ConnInfo: status i32 @0,
    err i32 @4 */
    let mut info = [0u8; 64];
    let f: FS7ConnPoll = core::mem::transmute(s7fn(svc, S7_SVC_CONN_POLL));
    let rc = f(svc, ru32(this.add(S7C_CONN_HANDLE)), info.as_mut_ptr());
    if rc != 0 {
        s7c_fail(this, (-rc) as u32);
        return 0;
    }
    wi32(this.add(S7C_STATUS), ri32(info.as_ptr()));
    wu32(this.add(S7C_ERRID), ri32(info.as_ptr().add(4)) as u32);
    if ri32(info.as_ptr()) != 0 {
        wb(this.add(S7C_BUSY), false);
        wb(this.add(S7C_DONE), true);
    }
    0
}

/// `__tplc_rt_FB_S7Io` — EXEC rising edge begins one S7 Read Var
/// (MODE=0) or Write Var (MODE=1) transaction on CONN_HANDLE;
/// BUSY stays set while s7_poll drives it. R_DATA receives the
/// read payload (DATA_LEN bytes); ITEM_RC/ERRID mirror the
/// response item return code and transport/S7 error word.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_S7Io(this: *mut u8, ctx: *mut u8) -> i32 {
    let exec = rb(this.add(S7I_EXEC));
    let last = rb(this.add(S7I_LAST_EXEC));
    wb(this.add(S7I_LAST_EXEC), exec);

    if !rb(this.add(S7I_BUSY)) {
        if !(exec && !last) {
            return 0;
        }
        let svc = s7_service(ctx);
        if svc.is_null() {
            s7i_fail(this, 99);
            return 0;
        }
        let is_write = ru32(this.add(S7I_MODE)) != 0;
        let bit = rb(this.add(S7I_BIT_ACCESS));
        let bit_no = ru32(this.add(S7I_BIT_NO));

        /* TplcS7Addr (24 bytes): area u32 @0, db u32 @4,
        byte_off u32 @8, bit_off u32 @12, len u32 @16,
        reserved u32 @20 — little-endian scalars */
        let fill_addr = |p: *mut u8| {
            core::ptr::write_bytes(p, 0, 24);
            let w = |off: usize, v: u32| {
                core::ptr::copy_nonoverlapping(v.to_le_bytes().as_ptr(), p.add(off), 4);
            };
            w(0, ru32(this.add(S7I_AREA)));
            w(4, ru32(this.add(S7I_DB)));
            w(8, ru32(this.add(S7I_BYTE_OFF)));
            w(12, if bit { bit_no } else { 0xFFFF_FFFF });
            w(16, ru32(this.add(S7I_LEN)));
        };

        let mut h: u32 = 0;
        let rc = if is_write {
            /* TplcS7WriteReq (276 bytes): addr @0..24, dlen u32
            @24, timeout_ms u32 @28, reserved u32 @32,
            data[240] @36 */
            let mut req = [0u8; 276];
            fill_addr(req.as_mut_ptr());
            let dlen = ru32(this.add(S7I_LEN)).min(240);
            let mut w = |off: usize, v: u32| {
                core::ptr::copy_nonoverlapping(
                    v.to_le_bytes().as_ptr(),
                    req.as_mut_ptr().add(off),
                    4,
                );
            };
            w(24, dlen);
            w(28, s7_timeout_ms(ri(this.add(S7I_TIMEOUT))));
            if dlen > 0 {
                core::ptr::copy_nonoverlapping(
                    this.add(S7I_DATA),
                    req.as_mut_ptr().add(36),
                    dlen as usize,
                );
            }
            let f: FS7IoBegin = core::mem::transmute(s7fn(svc, S7_SVC_WRITE));
            f(svc, ru32(this.add(S7I_CONN_HANDLE)), req.as_ptr(), &mut h)
        } else {
            /* TplcS7ReadReq (32 bytes): addr @0..24,
            timeout_ms u32 @24, reserved u32 @28 */
            let mut req = [0u8; 32];
            fill_addr(req.as_mut_ptr());
            core::ptr::copy_nonoverlapping(
                s7_timeout_ms(ri(this.add(S7I_TIMEOUT)))
                    .to_le_bytes()
                    .as_ptr(),
                req.as_mut_ptr().add(24),
                4,
            );
            let f: FS7IoBegin = core::mem::transmute(s7fn(svc, S7_SVC_READ));
            f(svc, ru32(this.add(S7I_CONN_HANDLE)), req.as_ptr(), &mut h)
        };
        if rc != 0 {
            s7i_fail(this, (-rc) as u32);
            return 0;
        }
        wu32(this.add(S7I_HANDLE), h);
        wb(this.add(S7I_DONE), false);
        wu32(this.add(S7I_ERRID), 0);
        wu32(this.add(S7I_ITEM_RC), 0);
        wu32(this.add(S7I_DATA_LEN), 0);
        wi32(this.add(S7I_STATUS), 0);
        wb(this.add(S7I_BUSY), true);
    }

    if !rb(this.add(S7I_BUSY)) {
        return 0;
    }
    let svc = s7_service(ctx);
    if svc.is_null() {
        s7i_fail(this, 99);
        return 0;
    }
    /* s7_poll(h, info64, val, val_cap) — TplcS7RespInfo:
    status i32 @0, err i32 @4, item_rc u32 @8, data_len u32 @12 */
    let mut info = [0u8; 64];
    let mut vbuf = [0u8; 240];
    let f: FS7Poll = core::mem::transmute(s7fn(svc, S7_SVC_POLL));
    let rc = f(
        svc,
        ru32(this.add(S7I_HANDLE)),
        info.as_mut_ptr(),
        vbuf.as_mut_ptr(),
        240,
    );
    if rc != 0 {
        s7i_fail(this, (-rc) as u32);
        return 0;
    }
    let st = ri32(info.as_ptr());
    wi32(this.add(S7I_STATUS), st);
    wu32(this.add(S7I_ERRID), ri32(info.as_ptr().add(4)) as u32);
    wu32(this.add(S7I_ITEM_RC), ru32(info.as_ptr().add(8)));
    let dlen = ru32(info.as_ptr().add(12));
    wu32(this.add(S7I_DATA_LEN), dlen);
    if ru32(this.add(S7I_MODE)) == 0 && dlen > 0 {
        let cp = (dlen as usize).min(240);
        core::ptr::copy_nonoverlapping(vbuf.as_ptr(), this.add(S7I_R_DATA), cp);
    }
    if st != 0 {
        wb(this.add(S7I_BUSY), false);
        wb(this.add(S7I_DONE), true);
        wu32(this.add(S7I_HANDLE), 0);
    }
    0
}
