//! tplc-rt builtin FB: iec870.
use crate::*;

// ---------------------------------------------------------------------------
// FB_Iec870Conn / FB_Iec870Tx / FB_Iec870Rx — tplc-iec870 service bridges
// (IEC 60870-5-104 client, TCP 2404). FB_Iec870Conn owns the session
// (open handshake polled via BUSY, CLOSE edge sends STOPDT + closes);
// FB_Iec870Tx runs one ASDU send per EXEC edge; FB_Iec870Rx polls the
// received-ASDU queue every scan (consume-on-read, DONE pulses per ASDU).
// ---------------------------------------------------------------------------

// FB_Iec870Conn member offsets (see builtin_fb_layout("FB_Iec870Conn"))
pub(crate) const I70C_HOST_LEN: usize = 0;
pub(crate) const I70C_HOST: usize = 1;
pub(crate) const I70C_PORT: usize = 16;
pub(crate) const I70C_CA_SIZE: usize = 20;
pub(crate) const I70C_COT_SIZE: usize = 24;
pub(crate) const I70C_TIMEOUT: usize = 32;
pub(crate) const I70C_CONN_HANDLE: usize = 40;
pub(crate) const I70C_STATUS: usize = 44;
pub(crate) const I70C_SUBSTATE: usize = 48;
pub(crate) const I70C_ERRID: usize = 52;
pub(crate) const I70C_RX_DROPPED: usize = 56;
pub(crate) const I70C_CLOSE: usize = 60;
pub(crate) const I70C_EXEC: usize = 61;
pub(crate) const I70C_BUSY: usize = 62;
pub(crate) const I70C_DONE: usize = 63;
pub(crate) const I70C_LAST_EXEC: usize = 64;
pub(crate) const I70C_LAST_CLOSE: usize = 65;

// FB_Iec870Tx member offsets (see builtin_fb_layout("FB_Iec870Tx"))
pub(crate) const I70T_CONN_HANDLE: usize = 0;
pub(crate) const I70T_TYPE_ID: usize = 4;
pub(crate) const I70T_VSQ: usize = 8;
pub(crate) const I70T_COT: usize = 12;
pub(crate) const I70T_CA: usize = 16;
pub(crate) const I70T_LEN: usize = 20;
pub(crate) const I70T_HANDLE: usize = 24;
pub(crate) const I70T_STATUS: usize = 28;
pub(crate) const I70T_ERRID: usize = 32;
pub(crate) const I70T_PAYLOAD: usize = 36;
pub(crate) const I70T_EXEC: usize = 276;
pub(crate) const I70T_BUSY: usize = 277;
pub(crate) const I70T_DONE: usize = 278;
pub(crate) const I70T_LAST_EXEC: usize = 279;

// FB_Iec870Rx member offsets (see builtin_fb_layout("FB_Iec870Rx"))
pub(crate) const I70R_CONN_HANDLE: usize = 0;
pub(crate) const I70R_TYPE_ID: usize = 4;
pub(crate) const I70R_VSQ: usize = 8;
pub(crate) const I70R_COT: usize = 12;
pub(crate) const I70R_CA: usize = 16;
pub(crate) const I70R_LEN: usize = 20;
pub(crate) const I70R_I_FRAME_SEQ: usize = 24;
pub(crate) const I70R_ERRID: usize = 28;
pub(crate) const I70R_PAYLOAD: usize = 32;
pub(crate) const I70R_DONE: usize = 272;

// TplcIec870Service vtable slots
pub(crate) const I70_SVC_OPEN: usize = 16;
pub(crate) const I70_SVC_CONN_POLL: usize = 24;
pub(crate) const I70_SVC_SEND: usize = 32;
pub(crate) const I70_SVC_TX_POLL: usize = 40;
pub(crate) const I70_SVC_RX_POLL: usize = 56;
pub(crate) const I70_SVC_CLOSE: usize = 64;

/// service lookup for "tplc-iec870" (11 chars)
pub(crate) unsafe fn iec870_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-iec870".as_ptr() as *const u8, 11) as *const u8
}

#[inline]
pub(crate) unsafe fn iec870fn(svc: *const u8, off: usize) -> usize {
    (svc.add(off) as *const usize).read()
}

pub(crate) type FI70Open = unsafe extern "C" fn(*const u8, *const u8, *mut u32) -> i32;
pub(crate) type FI70ConnPoll = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;
pub(crate) type FI70Send = unsafe extern "C" fn(*const u8, u32, *const u8, *mut u32) -> i32;
pub(crate) type FI70TxPoll = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;
pub(crate) type FI70RxPoll = unsafe extern "C" fn(*const u8, u32, *mut u8, *mut u8, u32) -> i32;
pub(crate) type FI70Close = unsafe extern "C" fn(*const u8, u32) -> i32;

#[inline]
pub(crate) unsafe fn i70c_fail(this: *mut u8, errid: u32) {
    wb(this.add(I70C_BUSY), false);
    wb(this.add(I70C_DONE), true);
    wu32(this.add(I70C_ERRID), errid);
    wi32(this.add(I70C_STATUS), 2);
}

#[inline]
pub(crate) unsafe fn i70t_fail(this: *mut u8, errid: u32) {
    wb(this.add(I70T_BUSY), false);
    wb(this.add(I70T_DONE), true);
    wu32(this.add(I70T_HANDLE), 0);
    wu32(this.add(I70T_ERRID), errid);
    wi32(this.add(I70T_STATUS), 2);
}

/// ns -> ms for the service timeout fields (0 = service default)
#[inline]
pub(crate) unsafe fn i70_timeout_ms(tns: i64) -> u32 {
    if tns <= 0 {
        0
    } else {
        ((tns / 1_000_000).min(60000)) as u32
    }
}

/// close a live connection handle through the service (best effort)
#[inline]
pub(crate) unsafe fn i70_close_handle(svc: *const u8, h: u32) {
    if h == 0 || svc.is_null() {
        return;
    }
    let f: FI70Close = core::mem::transmute(iec870fn(svc, I70_SVC_CLOSE));
    f(svc, h);
}

/// `__tplc_rt_FB_Iec870Conn` — EXEC rising edge opens an IEC
/// 60870-5-104 connection; BUSY stays set while the TCP/STARTDT
/// handshake is driven through iec870_conn_poll. STATUS mirrors
/// the conn info (0 pending, 1 ready, 2 error); SUBSTATE
/// 0=none 1=tcp 2=startdt 3=active; DONE pulses on the terminal
/// poll. CLOSE rising edge sends STOPDT act and closes.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_Iec870Conn(this: *mut u8, ctx: *mut u8) -> i32 {
    let exec = rb(this.add(I70C_EXEC));
    let last = rb(this.add(I70C_LAST_EXEC));
    wb(this.add(I70C_LAST_EXEC), exec);
    let cls = rb(this.add(I70C_CLOSE));
    let clast = rb(this.add(I70C_LAST_CLOSE));
    wb(this.add(I70C_LAST_CLOSE), cls);

    // CLOSE rising edge: tear down any live session
    if cls && !clast {
        let h = ru32(this.add(I70C_CONN_HANDLE));
        let svc = iec870_service(ctx);
        i70_close_handle(svc, h);
        wu32(this.add(I70C_CONN_HANDLE), 0);
        wb(this.add(I70C_BUSY), false);
        wb(this.add(I70C_DONE), true);
        wi32(this.add(I70C_STATUS), 0);
        wi32(this.add(I70C_SUBSTATE), 0);
        wu32(this.add(I70C_ERRID), 0);
    }

    if !rb(this.add(I70C_BUSY)) {
        if !(exec && !last) {
            return 0;
        }
        // EXEC rising edge: drop a stale handle, open a fresh conn
        let old = ru32(this.add(I70C_CONN_HANDLE));
        let svc = iec870_service(ctx);
        if svc.is_null() {
            i70c_fail(this, 99);
            return 0;
        }
        i70_close_handle(svc, old);
        wu32(this.add(I70C_CONN_HANDLE), 0);

        /* TplcIec870OpenReq (36 bytes): host[16] @0, port u16
        @16, ca_size u16 @18, cot_size u16 @20, reserved u16 @22,
        timeout_ms u32 @24, keepalive_ms u32 @28, reserved2 @32 */
        let mut req = [0u8; 36];
        let hn = (*this.add(I70C_HOST_LEN) as usize).min(15);
        if hn > 0 {
            core::ptr::copy_nonoverlapping(this.add(I70C_HOST), req.as_mut_ptr(), hn);
        }
        let mut w16 = |off: usize, v: u16| {
            core::ptr::copy_nonoverlapping(v.to_le_bytes().as_ptr(), req.as_mut_ptr().add(off), 2);
        };
        w16(16, ru32(this.add(I70C_PORT)) as u16);
        w16(18, ru32(this.add(I70C_CA_SIZE)) as u16);
        w16(20, ru32(this.add(I70C_COT_SIZE)) as u16);
        let mut w32 = |off: usize, v: u32| {
            core::ptr::copy_nonoverlapping(v.to_le_bytes().as_ptr(), req.as_mut_ptr().add(off), 4);
        };
        w32(24, i70_timeout_ms(ri(this.add(I70C_TIMEOUT))));
        w32(28, 0); /* keepalive: service default */
        let mut h: u32 = 0;
        let f: FI70Open = core::mem::transmute(iec870fn(svc, I70_SVC_OPEN));
        let rc = f(svc, req.as_ptr(), &mut h);
        if rc != 0 {
            i70c_fail(this, (-rc) as u32);
            return 0;
        }
        wu32(this.add(I70C_CONN_HANDLE), h);
        wb(this.add(I70C_DONE), false);
        wu32(this.add(I70C_ERRID), 0);
        wi32(this.add(I70C_STATUS), 0);
        wi32(this.add(I70C_SUBSTATE), 0);
        wb(this.add(I70C_BUSY), true);
    }

    if !rb(this.add(I70C_BUSY)) {
        return 0;
    }
    let svc = iec870_service(ctx);
    if svc.is_null() {
        i70c_fail(this, 99);
        return 0;
    }
    /* iec870_conn_poll(h, info64) — TplcIec870ConnInfo: status i32
    @0, substate i32 @4, err i32 @8, rx_dropped u32 @12 */
    let mut info = [0u8; 64];
    let f: FI70ConnPoll = core::mem::transmute(iec870fn(svc, I70_SVC_CONN_POLL));
    let rc = f(svc, ru32(this.add(I70C_CONN_HANDLE)), info.as_mut_ptr());
    if rc != 0 {
        i70c_fail(this, (-rc) as u32);
        return 0;
    }
    wi32(this.add(I70C_STATUS), ri32(info.as_ptr()));
    wi32(this.add(I70C_SUBSTATE), ri32(info.as_ptr().add(4)));
    wu32(this.add(I70C_ERRID), ri32(info.as_ptr().add(8)) as u32);
    wu32(this.add(I70C_RX_DROPPED), ru32(info.as_ptr().add(12)));
    if ri32(info.as_ptr()) != 0 {
        wb(this.add(I70C_BUSY), false);
        wb(this.add(I70C_DONE), true);
    }
    0
}

/// `__tplc_rt_FB_Iec870Tx` — EXEC rising edge begins one ASDU
/// send on CONN_HANDLE; BUSY stays set while iec870_tx_poll
/// drives the I-frame flush. STATUS 0=pending 1=ok 2=error;
/// ERRID the transport errno.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_Iec870Tx(this: *mut u8, ctx: *mut u8) -> i32 {
    let exec = rb(this.add(I70T_EXEC));
    let last = rb(this.add(I70T_LAST_EXEC));
    wb(this.add(I70T_LAST_EXEC), exec);

    if !rb(this.add(I70T_BUSY)) {
        if !(exec && !last) {
            return 0;
        }
        let svc = iec870_service(ctx);
        if svc.is_null() {
            i70t_fail(this, 99);
            return 0;
        }

        /* TplcIec870TxReq (264 bytes): type_id u32 @0, vsq @4,
        cot @8, ca @12, plen @16, reserved @20, payload[240] @24 */
        let mut req = [0u8; 264];
        let mut w = |off: usize, v: u32| {
            core::ptr::copy_nonoverlapping(v.to_le_bytes().as_ptr(), req.as_mut_ptr().add(off), 4);
        };
        w(0, ru32(this.add(I70T_TYPE_ID)));
        w(4, ru32(this.add(I70T_VSQ)));
        w(8, ru32(this.add(I70T_COT)));
        w(12, ru32(this.add(I70T_CA)));
        let plen = ru32(this.add(I70T_LEN)).min(240);
        w(16, plen);
        if plen > 0 {
            core::ptr::copy_nonoverlapping(
                this.add(I70T_PAYLOAD),
                req.as_mut_ptr().add(24),
                plen as usize,
            );
        }
        let mut h: u32 = 0;
        let f: FI70Send = core::mem::transmute(iec870fn(svc, I70_SVC_SEND));
        let rc = f(svc, ru32(this.add(I70T_CONN_HANDLE)), req.as_ptr(), &mut h);
        if rc != 0 {
            i70t_fail(this, (-rc) as u32);
            return 0;
        }
        wu32(this.add(I70T_HANDLE), h);
        wb(this.add(I70T_DONE), false);
        wu32(this.add(I70T_ERRID), 0);
        wi32(this.add(I70T_STATUS), 0);
        wb(this.add(I70T_BUSY), true);
    }

    if !rb(this.add(I70T_BUSY)) {
        return 0;
    }
    let svc = iec870_service(ctx);
    if svc.is_null() {
        i70t_fail(this, 99);
        return 0;
    }
    /* iec870_tx_poll(h, info64) — TplcIec870TxInfo: status i32
    @0, err i32 @4 */
    let mut info = [0u8; 64];
    let f: FI70TxPoll = core::mem::transmute(iec870fn(svc, I70_SVC_TX_POLL));
    let rc = f(svc, ru32(this.add(I70T_HANDLE)), info.as_mut_ptr());
    if rc != 0 {
        i70t_fail(this, (-rc) as u32);
        return 0;
    }
    wi32(this.add(I70T_STATUS), ri32(info.as_ptr()));
    wu32(this.add(I70T_ERRID), ri32(info.as_ptr().add(4)) as u32);
    if ri32(info.as_ptr()) != 0 {
        wb(this.add(I70T_BUSY), false);
        wb(this.add(I70T_DONE), true);
        wu32(this.add(I70T_HANDLE), 0);
    }
    0
}

/// `__tplc_rt_FB_Iec870Rx` — polls the received-ASDU queue every
/// scan. DONE pulses TRUE on the scan an ASDU is delivered
/// (consume-on-read); TYPE_ID/VSQ/COT/CA/LEN/I_FRAME_SEQ and
/// PAYLOAD describe it.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_Iec870Rx(this: *mut u8, ctx: *mut u8) -> i32 {
    wb(this.add(I70R_DONE), false);
    let svc = iec870_service(ctx);
    if svc.is_null() {
        wu32(this.add(I70R_ERRID), 99);
        return 0;
    }
    /* iec870_rx_poll(ch, info64, payload, cap) — TplcIec870RxInfo:
    status i32 @0, err i32 @4, type_id @8, vsq @12, cot @16,
    ca @20, payload_len @24, i_frame_seq @28 */
    let mut info = [0u8; 64];
    let f: FI70RxPoll = core::mem::transmute(iec870fn(svc, I70_SVC_RX_POLL));
    let rc = f(
        svc,
        ru32(this.add(I70R_CONN_HANDLE)),
        info.as_mut_ptr(),
        this.add(I70R_PAYLOAD),
        240,
    );
    if rc != 0 {
        wu32(this.add(I70R_ERRID), (-rc) as u32);
        return 0;
    }
    wu32(this.add(I70R_ERRID), ri32(info.as_ptr().add(4)) as u32);
    if ri32(info.as_ptr()) == 1 {
        let rd = |off: usize| -> u32 {
            let mut b = [0u8; 4];
            core::ptr::copy_nonoverlapping(info.as_ptr().add(off), b.as_mut_ptr(), 4);
            u32::from_le_bytes(b)
        };
        wu32(this.add(I70R_TYPE_ID), rd(8));
        wu32(this.add(I70R_VSQ), rd(12));
        wu32(this.add(I70R_COT), rd(16));
        wu32(this.add(I70R_CA), rd(20));
        wu32(this.add(I70R_LEN), rd(24));
        wu32(this.add(I70R_I_FRAME_SEQ), rd(28));
        wb(this.add(I70R_DONE), true);
    }
    0
}
