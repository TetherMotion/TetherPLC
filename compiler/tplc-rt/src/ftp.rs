//! tplc-rt builtin FB: ftp.
use crate::*;

// ---------------------------------------------------------------------------
// FB_FtpConn / FB_FtpCmd / FB_FtpXfer — tplc-ftp service bridges (FTP
// client, RFC 959 over TCP with PASV data connections). FB_FtpConn owns
// the control session (open/login polled via BUSY, CLOSE edge tears it
// down); FB_FtpCmd runs one control command per EXEC edge;
// FB_FtpXfer runs one RETR/STOR/LIST transfer per EXEC edge (downloads
// stream into BUF, uploads feed from UP_DATA automatically).
// ---------------------------------------------------------------------------

// FB_FtpConn member offsets (see builtin_fb_layout("FB_FtpConn"))
pub(crate) const FTPC_HOST_LEN: usize = 0;
pub(crate) const FTPC_HOST: usize = 1;
pub(crate) const FTPC_PORT: usize = 16;
pub(crate) const FTPC_USER_LEN: usize = 20;
pub(crate) const FTPC_USER: usize = 21;
pub(crate) const FTPC_PASS_LEN: usize = 52;
pub(crate) const FTPC_PASS: usize = 53;
pub(crate) const FTPC_TIMEOUT: usize = 88;
pub(crate) const FTPC_CONN_HANDLE: usize = 96;
pub(crate) const FTPC_STATUS: usize = 100;
pub(crate) const FTPC_ERRID: usize = 104;
pub(crate) const FTPC_CLOSE: usize = 108;
pub(crate) const FTPC_EXEC: usize = 109;
pub(crate) const FTPC_BUSY: usize = 110;
pub(crate) const FTPC_DONE: usize = 111;
pub(crate) const FTPC_LAST_EXEC: usize = 112;
pub(crate) const FTPC_LAST_CLOSE: usize = 113;

// FB_FtpCmd member offsets
pub(crate) const FTC_CONN_HANDLE: usize = 0;
pub(crate) const FTC_CMD_LEN: usize = 4;
pub(crate) const FTC_CMD: usize = 5;
pub(crate) const FTC_TIMEOUT: usize = 136;
pub(crate) const FTC_HANDLE: usize = 144;
pub(crate) const FTC_STATUS: usize = 148;
pub(crate) const FTC_RESP_CODE: usize = 152;
pub(crate) const FTC_ERRID: usize = 156;
pub(crate) const FTC_TEXT_LEN: usize = 160;
pub(crate) const FTC_TEXT: usize = 161;
pub(crate) const FTC_EXEC: usize = 288;
pub(crate) const FTC_BUSY: usize = 289;
pub(crate) const FTC_DONE: usize = 290;
pub(crate) const FTC_LAST_EXEC: usize = 291;

// FB_FtpXfer member offsets
pub(crate) const FTX_CONN_HANDLE: usize = 0;
pub(crate) const FTX_OP: usize = 4;
pub(crate) const FTX_PATH_LEN: usize = 8;
pub(crate) const FTX_PATH: usize = 9;
pub(crate) const FTX_TIMEOUT: usize = 136;
pub(crate) const FTX_HANDLE: usize = 144;
pub(crate) const FTX_STATUS: usize = 148;
pub(crate) const FTX_RESP_CODE: usize = 152;
pub(crate) const FTX_CHUNK: usize = 156;
pub(crate) const FTX_TOTAL: usize = 160;
pub(crate) const FTX_ERRID: usize = 164;
pub(crate) const FTX_UP_LEN: usize = 168;
pub(crate) const FTX_UP_OFF: usize = 172;
pub(crate) const FTX_BUF: usize = 176;
pub(crate) const FTX_UP_DATA: usize = 688;
pub(crate) const FTX_EXEC: usize = 1200;
pub(crate) const FTX_BUSY: usize = 1201;
pub(crate) const FTX_DONE: usize = 1202;
pub(crate) const FTX_LAST_EXEC: usize = 1203;
pub(crate) const FTX_FEED_DONE: usize = 1204;

// TplcFtpService vtable offsets (struct_size@0, abi@4, host@8)
pub(crate) const FTP_SVC_OPEN: usize = 16;
pub(crate) const FTP_SVC_CONN_POLL: usize = 24;
pub(crate) const FTP_SVC_CMD: usize = 32;
pub(crate) const FTP_SVC_TXN_POLL: usize = 40;
pub(crate) const FTP_SVC_XFER_BEGIN: usize = 48;
pub(crate) const FTP_SVC_XFER_POLL: usize = 56;
pub(crate) const FTP_SVC_XFER_FEED: usize = 64;
pub(crate) const FTP_SVC_XFER_FINISH: usize = 72;
pub(crate) const FTP_SVC_CLOSE: usize = 88;

pub(crate) unsafe fn ftp_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-ftp".as_ptr() as *const u8, 8) as *const u8
}

#[inline]
pub(crate) unsafe fn ftpfn(svc: *const u8, off: usize) -> usize {
    (svc.add(off) as *const usize).read()
}

pub(crate) type FFtpOpen = unsafe extern "C" fn(*const u8, *const u8, *mut u32) -> i32;
pub(crate) type FFtpConnPoll = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;
pub(crate) type FFtpCmd = unsafe extern "C" fn(*const u8, u32, *const u8, *mut u32) -> i32;
pub(crate) type FFtpTxnPoll = unsafe extern "C" fn(*const u8, u32, *mut u8, *mut u8, u32) -> i32;
pub(crate) type FFtpXferBegin = unsafe extern "C" fn(*const u8, u32, *const u8, *mut u32) -> i32;
pub(crate) type FFtpXferPoll = unsafe extern "C" fn(*const u8, u32, *mut u8, *mut u8, u32) -> i32;
pub(crate) type FFtpXferFeed = unsafe extern "C" fn(*const u8, u32, *const u8, u32) -> i32;
pub(crate) type FFtpXferFinish = unsafe extern "C" fn(*const u8, u32) -> i32;
pub(crate) type FFtpClose = unsafe extern "C" fn(*const u8, u32) -> i32;

/// copy an FB Str field (len byte + chars) into a NUL-terminated
/// fixed char buffer for the service request structs
#[inline]
pub(crate) unsafe fn ftp_str_to_buf(
    this: *const u8,
    len_off: usize,
    ch_off: usize,
    str_cap: usize,
    dst: *mut u8,
    dst_cap: usize,
) {
    let n = (*this.add(len_off) as usize).min(str_cap).min(dst_cap - 1);
    if n > 0 {
        core::ptr::copy_nonoverlapping(this.add(ch_off), dst, n);
    }
    *dst.add(n) = 0;
}

/// ns -> ms for the service timeout fields (0 = service default)
#[inline]
pub(crate) unsafe fn ftp_timeout_ms(tns: i64) -> u32 {
    if tns <= 0 {
        0
    } else {
        ((tns / 1_000_000).min(300000)) as u32
    }
}

#[inline]
pub(crate) unsafe fn ftpc_fail(this: *mut u8, errid: u32) {
    wb(this.add(FTPC_BUSY), false);
    wb(this.add(FTPC_DONE), true);
    wu32(this.add(FTPC_ERRID), errid);
    wi32(this.add(FTPC_STATUS), 2);
}

#[inline]
pub(crate) unsafe fn ftpcmd_fail(this: *mut u8, errid: u32) {
    wb(this.add(FTC_BUSY), false);
    wb(this.add(FTC_DONE), true);
    wu32(this.add(FTC_HANDLE), 0);
    wu32(this.add(FTC_ERRID), errid);
    wi32(this.add(FTC_STATUS), 4);
}

#[inline]
pub(crate) unsafe fn ftx_fail(this: *mut u8, errid: u32) {
    wb(this.add(FTX_BUSY), false);
    wb(this.add(FTX_DONE), true);
    wu32(this.add(FTX_HANDLE), 0);
    wu32(this.add(FTX_ERRID), errid);
    wi32(this.add(FTX_STATUS), 4);
}

/// close a live connection handle through the service (best effort)
#[inline]
pub(crate) unsafe fn ftp_close_handle(svc: *const u8, h: u32) {
    if h == 0 || svc.is_null() {
        return;
    }
    let f: FFtpClose = core::mem::transmute(ftpfn(svc, FTP_SVC_CLOSE));
    f(svc, h);
}

/// `__tplc_rt_FB_FtpConn` — EXEC rising edge opens an FTP control
/// connection; BUSY stays set while the TCP connect + USER/PASS
/// login is driven through ftp_conn_poll. STATUS mirrors the conn
/// info (0 pending, 1 ready, 2 error); DONE pulses on the terminal
/// poll. CLOSE rising edge closes the session.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_FtpConn(this: *mut u8, ctx: *mut u8) -> i32 {
    let exec = rb(this.add(FTPC_EXEC));
    let last = rb(this.add(FTPC_LAST_EXEC));
    wb(this.add(FTPC_LAST_EXEC), exec);
    let cls = rb(this.add(FTPC_CLOSE));
    let clast = rb(this.add(FTPC_LAST_CLOSE));
    wb(this.add(FTPC_LAST_CLOSE), cls);

    if cls && !clast {
        let h = ru32(this.add(FTPC_CONN_HANDLE));
        let svc = ftp_service(ctx);
        ftp_close_handle(svc, h);
        wu32(this.add(FTPC_CONN_HANDLE), 0);
        wb(this.add(FTPC_BUSY), false);
        wb(this.add(FTPC_DONE), true);
        wi32(this.add(FTPC_STATUS), 0);
        wu32(this.add(FTPC_ERRID), 0);
    }

    if !rb(this.add(FTPC_BUSY)) {
        if !(exec && !last) {
            return 0;
        }
        let old = ru32(this.add(FTPC_CONN_HANDLE));
        let svc = ftp_service(ctx);
        if svc.is_null() {
            ftpc_fail(this, 99);
            return 0;
        }
        ftp_close_handle(svc, old);
        wu32(this.add(FTPC_CONN_HANDLE), 0);

        /* TplcFtpOpenReq (280 bytes): host[16] @0, port u16 @16,
        reserved u16 @18, timeout_ms u32 @20, user[128] @24,
        pass[128] @152 */
        let mut req = [0u8; 280];
        ftp_str_to_buf(this, FTPC_HOST_LEN, FTPC_HOST, 15, req.as_mut_ptr(), 16);
        core::ptr::copy_nonoverlapping(
            (ru32(this.add(FTPC_PORT)) as u16).to_le_bytes().as_ptr(),
            req.as_mut_ptr().add(16),
            2,
        );
        core::ptr::copy_nonoverlapping(
            ftp_timeout_ms(ri(this.add(FTPC_TIMEOUT)))
                .to_le_bytes()
                .as_ptr(),
            req.as_mut_ptr().add(20),
            4,
        );
        ftp_str_to_buf(
            this,
            FTPC_USER_LEN,
            FTPC_USER,
            31,
            req.as_mut_ptr().add(24),
            128,
        );
        ftp_str_to_buf(
            this,
            FTPC_PASS_LEN,
            FTPC_PASS,
            31,
            req.as_mut_ptr().add(152),
            128,
        );
        let mut h: u32 = 0;
        let f: FFtpOpen = core::mem::transmute(ftpfn(svc, FTP_SVC_OPEN));
        let rc = f(svc, req.as_ptr(), &mut h);
        if rc != 0 {
            ftpc_fail(this, (-rc) as u32);
            return 0;
        }
        wu32(this.add(FTPC_CONN_HANDLE), h);
        wb(this.add(FTPC_DONE), false);
        wu32(this.add(FTPC_ERRID), 0);
        wi32(this.add(FTPC_STATUS), 0);
        wb(this.add(FTPC_BUSY), true);
    }

    if !rb(this.add(FTPC_BUSY)) {
        return 0;
    }
    let svc = ftp_service(ctx);
    if svc.is_null() {
        ftpc_fail(this, 99);
        return 0;
    }
    /* ftp_conn_poll(h, info64) — TplcFtpConnInfo: status i32 @0,
    err i32 @4, resp_code u32 @8 */
    let mut info = [0u8; 64];
    let f: FFtpConnPoll = core::mem::transmute(ftpfn(svc, FTP_SVC_CONN_POLL));
    let rc = f(svc, ru32(this.add(FTPC_CONN_HANDLE)), info.as_mut_ptr());
    if rc != 0 {
        ftpc_fail(this, (-rc) as u32);
        return 0;
    }
    wi32(this.add(FTPC_STATUS), ri32(info.as_ptr()));
    wu32(this.add(FTPC_ERRID), ri32(info.as_ptr().add(4)) as u32);
    if ri32(info.as_ptr()) != 0 {
        wb(this.add(FTPC_BUSY), false);
        wb(this.add(FTPC_DONE), true);
    }
    0
}

/// `__tplc_rt_FB_FtpCmd` — EXEC rising edge begins one FTP control
/// command transaction on CONN_HANDLE; BUSY stays set while
/// ftp_txn_poll drives it. RESP_CODE is the final 3-digit reply
/// code; TEXT receives the reply text. STATUS mirrors the txn
/// info (0 pending, 1 ok/2xx, 2 fail/4xx-5xx, 3 timeout,
/// 4 error, 5 badresp).
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_FtpCmd(this: *mut u8, ctx: *mut u8) -> i32 {
    let exec = rb(this.add(FTC_EXEC));
    let last = rb(this.add(FTC_LAST_EXEC));
    wb(this.add(FTC_LAST_EXEC), exec);

    if !rb(this.add(FTC_BUSY)) {
        if !(exec && !last) {
            return 0;
        }
        let svc = ftp_service(ctx);
        if svc.is_null() {
            ftpcmd_fail(this, 99);
            return 0;
        }
        /* TplcFtpCmdReq (136 bytes): cmd[128] @0,
        timeout_ms u32 @128, reserved u32 @132 */
        let mut req = [0u8; 136];
        ftp_str_to_buf(this, FTC_CMD_LEN, FTC_CMD, 127, req.as_mut_ptr(), 128);
        core::ptr::copy_nonoverlapping(
            ftp_timeout_ms(ri(this.add(FTC_TIMEOUT)))
                .to_le_bytes()
                .as_ptr(),
            req.as_mut_ptr().add(128),
            4,
        );
        let mut h: u32 = 0;
        let f: FFtpCmd = core::mem::transmute(ftpfn(svc, FTP_SVC_CMD));
        let rc = f(svc, ru32(this.add(FTC_CONN_HANDLE)), req.as_ptr(), &mut h);
        if rc != 0 {
            ftpcmd_fail(this, (-rc) as u32);
            return 0;
        }
        wu32(this.add(FTC_HANDLE), h);
        wb(this.add(FTC_DONE), false);
        wu32(this.add(FTC_ERRID), 0);
        wu32(this.add(FTC_RESP_CODE), 0);
        wi32(this.add(FTC_STATUS), 0);
        wb(this.add(FTC_BUSY), true);
    }

    if !rb(this.add(FTC_BUSY)) {
        return 0;
    }
    let svc = ftp_service(ctx);
    if svc.is_null() {
        ftpcmd_fail(this, 99);
        return 0;
    }
    /* ftp_txn_poll(h, info64, textbuf, text_cap) — TplcFtpTxnInfo:
    status i32 @0, err i32 @4, resp_code u32 @8, total u64 @16,
    chunk u32 @24 */
    let mut info = [0u8; 64];
    let mut tbuf = [0u8; 128];
    let f: FFtpTxnPoll = core::mem::transmute(ftpfn(svc, FTP_SVC_TXN_POLL));
    let rc = f(
        svc,
        ru32(this.add(FTC_HANDLE)),
        info.as_mut_ptr(),
        tbuf.as_mut_ptr(),
        128,
    );
    if rc != 0 {
        ftpcmd_fail(this, (-rc) as u32);
        return 0;
    }
    let st = ri32(info.as_ptr());
    wi32(this.add(FTC_STATUS), st);
    wu32(this.add(FTC_ERRID), ri32(info.as_ptr().add(4)) as u32);
    wu32(this.add(FTC_RESP_CODE), ru32(info.as_ptr().add(8)));
    /* copy the NUL-terminated reply text into TEXT (len + chars) */
    let mut tl = 0usize;
    while tl < 127 && tbuf[tl] != 0 {
        tl += 1;
    }
    *this.add(FTC_TEXT_LEN) = tl as u8;
    if tl > 0 {
        core::ptr::copy_nonoverlapping(tbuf.as_ptr(), this.add(FTC_TEXT), tl);
    }
    if st != 0 {
        wb(this.add(FTC_BUSY), false);
        wb(this.add(FTC_DONE), true);
        wu32(this.add(FTC_HANDLE), 0);
    }
    0
}

/// `__tplc_rt_FB_FtpXfer` — EXEC rising edge begins one FTP data
/// transfer (OP 0=RETR, 1=STOR, 2=LIST) on CONN_HANDLE; BUSY stays
/// set while ftp_xfer_poll drives it. For RETR/LIST each scan copies
/// up to 512 received bytes into BUF (CHUNK bytes this scan, TOTAL
/// cumulative); STATUS 6=more while data flows, terminal 1=ok /
/// 2=fail / 3=timeout / 4=error / 5=badresp. For STOR the rt feeds
/// UP_DATA[0..UP_LEN) automatically and signals finish.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_FtpXfer(this: *mut u8, ctx: *mut u8) -> i32 {
    let exec = rb(this.add(FTX_EXEC));
    let last = rb(this.add(FTX_LAST_EXEC));
    wb(this.add(FTX_LAST_EXEC), exec);

    if !rb(this.add(FTX_BUSY)) {
        if !(exec && !last) {
            return 0;
        }
        let svc = ftp_service(ctx);
        if svc.is_null() {
            ftx_fail(this, 99);
            return 0;
        }
        /* TplcFtpXferReq (136 bytes): op u32 @0, timeout_ms u32 @4,
        path[128] @8 */
        let mut req = [0u8; 136];
        core::ptr::copy_nonoverlapping(
            ru32(this.add(FTX_OP)).to_le_bytes().as_ptr(),
            req.as_mut_ptr(),
            4,
        );
        core::ptr::copy_nonoverlapping(
            ftp_timeout_ms(ri(this.add(FTX_TIMEOUT)))
                .to_le_bytes()
                .as_ptr(),
            req.as_mut_ptr().add(4),
            4,
        );
        ftp_str_to_buf(
            this,
            FTX_PATH_LEN,
            FTX_PATH,
            127,
            req.as_mut_ptr().add(8),
            128,
        );
        let mut h: u32 = 0;
        let f: FFtpXferBegin = core::mem::transmute(ftpfn(svc, FTP_SVC_XFER_BEGIN));
        let rc = f(svc, ru32(this.add(FTX_CONN_HANDLE)), req.as_ptr(), &mut h);
        if rc != 0 {
            ftx_fail(this, (-rc) as u32);
            return 0;
        }
        wu32(this.add(FTX_HANDLE), h);
        wb(this.add(FTX_DONE), false);
        wb(this.add(FTX_FEED_DONE), false);
        wu32(this.add(FTX_UP_OFF), 0);
        wu32(this.add(FTX_ERRID), 0);
        wu32(this.add(FTX_RESP_CODE), 0);
        wu32(this.add(FTX_CHUNK), 0);
        wu32(this.add(FTX_TOTAL), 0);
        wi32(this.add(FTX_STATUS), 0);
        wb(this.add(FTX_BUSY), true);
    }

    if !rb(this.add(FTX_BUSY)) {
        return 0;
    }
    let svc = ftp_service(ctx);
    if svc.is_null() {
        ftx_fail(this, 99);
        return 0;
    }
    let is_stor = ru32(this.add(FTX_OP)) == 1;
    /* TplcFtpTxnInfo (64 bytes): status i32 @0, err i32 @4,
    resp_code u32 @8, total u64 @16, chunk u32 @24 */
    let mut info = [0u8; 64];
    if is_stor {
        /* drive the transfer, then feed remaining UP_DATA */
        let fp: FFtpXferPoll = core::mem::transmute(ftpfn(svc, FTP_SVC_XFER_POLL));
        let rc = fp(
            svc,
            ru32(this.add(FTX_HANDLE)),
            info.as_mut_ptr(),
            core::ptr::null_mut(),
            0,
        );
        if rc != 0 {
            ftx_fail(this, (-rc) as u32);
            return 0;
        }
        let ulen = ru32(this.add(FTX_UP_LEN)).min(512);
        let mut uoff = ru32(this.add(FTX_UP_OFF)).min(ulen);
        if !rb(this.add(FTX_FEED_DONE)) {
            if uoff < ulen {
                let ff: FFtpXferFeed = core::mem::transmute(ftpfn(svc, FTP_SVC_XFER_FEED));
                let n = ff(
                    svc,
                    ru32(this.add(FTX_HANDLE)),
                    this.add(FTX_UP_DATA).add(uoff as usize),
                    ulen - uoff,
                );
                if n > 0 {
                    uoff += n as u32;
                }
            }
            if uoff >= ulen {
                let ff: FFtpXferFinish = core::mem::transmute(ftpfn(svc, FTP_SVC_XFER_FINISH));
                ff(svc, ru32(this.add(FTX_HANDLE)));
                wb(this.add(FTX_FEED_DONE), true);
            }
            wu32(this.add(FTX_UP_OFF), uoff);
        }
    } else {
        /* RETR/LIST: poll copies up to 512 bytes into BUF */
        let fp: FFtpXferPoll = core::mem::transmute(ftpfn(svc, FTP_SVC_XFER_POLL));
        let rc = fp(
            svc,
            ru32(this.add(FTX_HANDLE)),
            info.as_mut_ptr(),
            this.add(FTX_BUF),
            512,
        );
        if rc != 0 {
            ftx_fail(this, (-rc) as u32);
            return 0;
        }
    }
    let st = ri32(info.as_ptr());
    wi32(this.add(FTX_STATUS), st);
    wu32(this.add(FTX_ERRID), ri32(info.as_ptr().add(4)) as u32);
    wu32(this.add(FTX_RESP_CODE), ru32(info.as_ptr().add(8)));
    wu32(this.add(FTX_CHUNK), ru32(info.as_ptr().add(24)));
    let tot = u64::from_le_bytes(info[16..24].try_into().unwrap_or([0; 8]));
    wu32(this.add(FTX_TOTAL), tot.min(u64::from(u32::MAX)) as u32);
    /* terminal: ok/fail/timeout/error/badresp (1..=5); 0 pending
    and 6 more-data keep BUSY */
    if (1..=5).contains(&st) {
        wb(this.add(FTX_BUSY), false);
        wb(this.add(FTX_DONE), true);
        wu32(this.add(FTX_HANDLE), 0);
    }
    0
}
