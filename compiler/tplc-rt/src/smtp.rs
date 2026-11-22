//! tplc-rt builtin FB: smtp.
use crate::*;

// ---------------------------------------------------------------------------
// FB_SmtpSend — tplc-smtp service bridge (SMTP client, RFC 5321 over TCP
// 25/587, plain text + optional AUTH LOGIN). EXEC rising edge begins
// one send transaction; the rt feeds BODY[0..BODY_LEN) automatically
// and signals finish. STATUS mirrors the service (0 pending, 1 ok,
// 2 fail, 3 timeout, 4 error, 5 badresp, 6 more).
// ---------------------------------------------------------------------------

// FB_SmtpSend member offsets (see builtin_fb_layout("FB_SmtpSend"))
pub(crate) const SMTP_HOST_LEN: usize = 0;
pub(crate) const SMTP_HOST: usize = 1;
pub(crate) const SMTP_PORT: usize = 16;
pub(crate) const SMTP_USER_LEN: usize = 20;
pub(crate) const SMTP_USER: usize = 21;
pub(crate) const SMTP_PASS_LEN: usize = 52;
pub(crate) const SMTP_PASS: usize = 53;
pub(crate) const SMTP_FROM_LEN: usize = 84;
pub(crate) const SMTP_FROM: usize = 85;
pub(crate) const SMTP_TO_LEN: usize = 148;
pub(crate) const SMTP_TO: usize = 149;
pub(crate) const SMTP_SUBJECT_LEN: usize = 212;
pub(crate) const SMTP_SUBJECT: usize = 213;
pub(crate) const SMTP_TIMEOUT: usize = 280;
pub(crate) const SMTP_HANDLE: usize = 288;
pub(crate) const SMTP_STATUS: usize = 292;
pub(crate) const SMTP_RESP_CODE: usize = 296;
pub(crate) const SMTP_ERRID: usize = 300;
pub(crate) const SMTP_BODY: usize = 304;
pub(crate) const SMTP_EXEC: usize = 560;
pub(crate) const SMTP_BUSY: usize = 561;
pub(crate) const SMTP_DONE: usize = 562;
pub(crate) const SMTP_LAST_EXEC: usize = 563;
pub(crate) const SMTP_FEED_DONE: usize = 564;

// TplcSmtpService vtable offsets (struct_size@0, abi@4, host@8)
pub(crate) const SMTP_SVC_BEGIN: usize = 16;
pub(crate) const SMTP_SVC_POLL: usize = 24;
pub(crate) const SMTP_SVC_FEED: usize = 32;
pub(crate) const SMTP_SVC_FEED_FINISH: usize = 40;

pub(crate) unsafe fn smtp_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-smtp".as_ptr() as *const u8, 9) as *const u8
}

#[inline]
pub(crate) unsafe fn smtpfn(svc: *const u8, off: usize) -> usize {
    (svc.add(off) as *const usize).read()
}

pub(crate) type FSmtpBegin = unsafe extern "C" fn(*const u8, *const u8, *mut u32) -> i32;
pub(crate) type FSmtpPoll = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;
pub(crate) type FSmtpFeed = unsafe extern "C" fn(*const u8, u32, *const u8, u32) -> i32;
pub(crate) type FSmtpFeedFinish = unsafe extern "C" fn(*const u8, u32) -> i32;

#[inline]
pub(crate) unsafe fn smtp_fail(this: *mut u8, errid: u32) {
    wb(this.add(SMTP_BUSY), false);
    wb(this.add(SMTP_DONE), true);
    wu32(this.add(SMTP_HANDLE), 0);
    wu32(this.add(SMTP_ERRID), errid);
    wi32(this.add(SMTP_STATUS), 4);
}

/// `__tplc_rt_FB_SmtpSend` — EXEC rising edge begins one SMTP send
/// transaction; BUSY stays set while smtp_poll drives it. The rt
/// feeds BODY[0..BODY_LEN) once the DATA phase opens and calls
/// feed_finish when the body is fully queued. STATUS 6=more while
/// the body streams; terminal 1=ok / 2=fail / 3=timeout /
/// 4=error / 5=badresp.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_SmtpSend(this: *mut u8, ctx: *mut u8) -> i32 {
    let exec = rb(this.add(SMTP_EXEC));
    let last = rb(this.add(SMTP_LAST_EXEC));
    wb(this.add(SMTP_LAST_EXEC), exec);

    if !rb(this.add(SMTP_BUSY)) {
        if !(exec && !last) {
            return 0;
        }
        let svc = smtp_service(ctx);
        if svc.is_null() {
            smtp_fail(this, 99);
            return 0;
        }
        /* TplcSmtpReq (344 bytes): host[16] @0, port u16 @16,
        reserved u16 @18, timeout_ms u32 @20, user[64] @24,
        pass[64] @88, from[64] @152, to[64] @216, subject[64] @280 */
        let mut req = [0u8; 344];
        ftp_str_to_buf(this, SMTP_HOST_LEN, SMTP_HOST, 15, req.as_mut_ptr(), 16);
        core::ptr::copy_nonoverlapping(
            (ru32(this.add(SMTP_PORT)) as u16).to_le_bytes().as_ptr(),
            req.as_mut_ptr().add(16),
            2,
        );
        core::ptr::copy_nonoverlapping(
            ftp_timeout_ms(ri(this.add(SMTP_TIMEOUT)))
                .to_le_bytes()
                .as_ptr(),
            req.as_mut_ptr().add(20),
            4,
        );
        ftp_str_to_buf(
            this,
            SMTP_USER_LEN,
            SMTP_USER,
            31,
            req.as_mut_ptr().add(24),
            64,
        );
        ftp_str_to_buf(
            this,
            SMTP_PASS_LEN,
            SMTP_PASS,
            31,
            req.as_mut_ptr().add(88),
            64,
        );
        ftp_str_to_buf(
            this,
            SMTP_FROM_LEN,
            SMTP_FROM,
            63,
            req.as_mut_ptr().add(152),
            64,
        );
        ftp_str_to_buf(
            this,
            SMTP_TO_LEN,
            SMTP_TO,
            63,
            req.as_mut_ptr().add(216),
            64,
        );
        ftp_str_to_buf(
            this,
            SMTP_SUBJECT_LEN,
            SMTP_SUBJECT,
            63,
            req.as_mut_ptr().add(280),
            64,
        );
        let mut h: u32 = 0;
        let f: FSmtpBegin = core::mem::transmute(smtpfn(svc, SMTP_SVC_BEGIN));
        let rc = f(svc, req.as_ptr(), &mut h);
        if rc != 0 {
            smtp_fail(this, (-rc) as u32);
            return 0;
        }
        wu32(this.add(SMTP_HANDLE), h);
        wb(this.add(SMTP_DONE), false);
        wb(this.add(SMTP_FEED_DONE), false);
        wu32(this.add(SMTP_ERRID), 0);
        wu32(this.add(SMTP_RESP_CODE), 0);
        wi32(this.add(SMTP_STATUS), 0);
        wb(this.add(SMTP_BUSY), true);
    }

    if !rb(this.add(SMTP_BUSY)) {
        return 0;
    }
    let svc = smtp_service(ctx);
    if svc.is_null() {
        smtp_fail(this, 99);
        return 0;
    }
    /* TplcSmtpInfo (64 bytes): status i32 @0, err i32 @4,
    resp_code u32 @8, body_total u64 @16 */
    let mut info = [0u8; 64];
    let fp: FSmtpPoll = core::mem::transmute(smtpfn(svc, SMTP_SVC_POLL));
    let rc = fp(svc, ru32(this.add(SMTP_HANDLE)), info.as_mut_ptr());
    if rc != 0 {
        smtp_fail(this, (-rc) as u32);
        return 0;
    }
    /* feed BODY once the DATA phase is streaming (status 6);
    BODY is a STRING[255] (len byte @SMTP_BODY, chars @+1) */
    let st = ri32(info.as_ptr());
    if st == 6 && !rb(this.add(SMTP_FEED_DONE)) {
        let blen = (*this.add(SMTP_BODY) as usize).min(255);
        if blen > 0 {
            let ff: FSmtpFeed = core::mem::transmute(smtpfn(svc, SMTP_SVC_FEED));
            ff(
                svc,
                ru32(this.add(SMTP_HANDLE)),
                this.add(SMTP_BODY).add(1),
                blen as u32,
            );
        }
        let ff: FSmtpFeedFinish = core::mem::transmute(smtpfn(svc, SMTP_SVC_FEED_FINISH));
        ff(svc, ru32(this.add(SMTP_HANDLE)));
        wb(this.add(SMTP_FEED_DONE), true);
    }
    wi32(this.add(SMTP_STATUS), ri32(info.as_ptr()));
    wu32(this.add(SMTP_ERRID), ri32(info.as_ptr().add(4)) as u32);
    wu32(this.add(SMTP_RESP_CODE), ru32(info.as_ptr().add(8)));
    let st2 = ri32(info.as_ptr());
    if (1..=5).contains(&st2) {
        wb(this.add(SMTP_BUSY), false);
        wb(this.add(SMTP_DONE), true);
        wu32(this.add(SMTP_HANDLE), 0);
    }
    0
}
