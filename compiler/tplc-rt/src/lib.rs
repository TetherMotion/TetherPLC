//! `tplc-rt` — clean-room IEC 61131-3 standard FB implementations
//! (doc 07 Appendix C semantics). Compiled to bitcode and linked into
//! deployable artifacts; also callable directly in tests.
//!
//! Every fn takes `(this, ctx)`: `this` = the FB instance's byte block
//! inside the program state blob (member offsets from
//! `tplc_mir::layout::builtin_fb_layout`), `ctx` = TplcCtx*
//! (now_ns @ +48). Returns 0 — faults use the checkpoint path, not
//! per-FB codes.
//!
//! Member offsets (must stay in lockstep with builtin_fb_layout):
//!   TON/TOF/TP : IN@0 u8, PT@8 i64, Q@16 u8, ET@24 i64,
//!                __in_last@32 u8, __start@40 i64, __run@48 u8  (size 56)
//!   R_/F_TRIG  : CLK@0 u8, Q@1 u8, __clk_last@2 u8            (size 8)
//!   CTU/CTD/CTUD: CU@0 CD@1 R@2 LD@3 u8, PV@4 i32, QU@8 QD@9 u8,
//!                CV@12 i32, __cu_last@16 __cd_last@17 u8      (size 24)
//!   SR/RS      : S@0 R1@1 S1@2 R@3 Q1@4 u8                   (size 8)
//!   FB_Socket  : OP@0 i32, HOST@4(str), PORT@20 u32,
//!                TIMEOUT@24 i64, DATA@32(str), DLEN@292 u32,
//!                HANDLE@296 u32, AUX@300 u32, RESULT@304 i32,
//!                ERRID@308 u32, EXEC@312 u8, BUSY@313 u8,
//!                DONE@314 u8, __last_exec@315 u8,
//!                __deadline@320 i64, __sent@328 u32,
//!                __recvd@332 u32, __phase@336 i32   (size 344)
//!   FB_Serial  : OP@0 i32, DEV@4(str), BAUD@68 u32,
//!                PARITY@72 u32, DBITS@76 u32, SBITS@80 u32,
//!                FLAGS@84 u32, AUX@88 u32, TIMEOUT@96 i64,
//!                DATA@104(str), DLEN@364 u32, HANDLE@368 u32,
//!                RESULT@372 i32, ERRID@376 u32, EXEC@380 u8,
//!                BUSY@381 u8, DONE@382 u8, __last_exec@383 u8,
//!                __deadline@384 i64, __sent@392 u32,
//!                __recvd@396 u32, __phase@400 i32   (size 408)
//!   FB_Ads     : OP@0 i32, HOST@4(str), PORT@20 u32,
//!                IDXGRP@24 u32, IDXOFF@28 u32, STATE@32 i32,
//!                DEVSTATE@36 i32, AUX@40 u32,
//!                TIMEOUT@48 i64, DATA@56(str), DLEN@316 u32,
//!                HANDLE@320 u32, RESULT@324 i32, ERRID@328 u32,
//!                EXEC@332 u8, BUSY@333 u8, DONE@334 u8,
//!                __last_exec@335 u8, __deadline@336 i64,
//!                __sent@344 u32, __recvd@348 u32,
//!                __phase@352 i32   (size 360)

#![cfg_attr(feature = "nostd", no_std)]

#[cfg(feature = "nostd")]
#[panic_handler]
fn __tplc_rt_panic(_: &core::panic::PanicInfo) -> ! {
    unsafe extern "C" {
        fn abort() -> !;
    }
    unsafe { abort() }
}
const CTX_NOW_NS: usize = 48;

#[inline]
unsafe fn now_ns(ctx: *const u8) -> i64 {
    (ctx.add(CTX_NOW_NS) as *const i64).read()
}
#[inline]
unsafe fn rb(p: *const u8) -> bool {
    p.read() != 0
}
#[inline]
unsafe fn wb(p: *mut u8, v: bool) {
    p.write(v as u8);
}
#[inline]
unsafe fn ri(p: *const u8) -> i64 {
    (p as *const i64).read()
}
#[inline]
unsafe fn wi(p: *mut u8, v: i64) {
    (p as *mut i64).write(v);
}
#[inline]
unsafe fn ri32(p: *const u8) -> i32 {
    (p as *const i32).read()
}
#[inline]
unsafe fn wi32(p: *mut u8, v: i32) {
    (p as *mut i32).write(v);
}
#[inline]
unsafe fn ru32(p: *const u8) -> u32 {
    (p as *const u32).read()
}
#[inline]
unsafe fn wu32(p: *mut u8, v: u32) {
    (p as *mut u32).write(v);
}

/* ---------------- timers (shared block) ---------------- */

/// TON: IN↑ captures start; while IN, ET=min(now-start,PT), Q=(ET>=PT).
#[no_mangle]
/// # Safety
/// `this` must point at a valid instance block of the
/// documented layout; `ctx` at a live `TplcCtx`.
pub unsafe extern "C" fn __tplc_rt_TON(this: *mut u8, ctx: *mut u8) -> i32 {
    let inn = rb(this);
    let pt = ri(this.add(8));
    let last = rb(this.add(32));
    let now = now_ns(ctx);
    if inn && !last {
        wi(this.add(40), now);
    }
    let (q, et) = if inn {
        let el = now - ri(this.add(40));
        (el >= pt, el.min(pt))
    } else {
        (false, 0)
    };
    wb(this.add(16), q);
    wi(this.add(24), et);
    wb(this.add(32), inn);
    0
}

/// TOF: IN → Q=1 immediately, ET=0; IN↓ starts off-timer, Q drops at PT.
#[no_mangle]
/// # Safety
/// `this` must point at a valid instance block of the
/// documented layout; `ctx` at a live `TplcCtx`.
pub unsafe extern "C" fn __tplc_rt_TOF(this: *mut u8, ctx: *mut u8) -> i32 {
    let inn = rb(this);
    let pt = ri(this.add(8));
    let last = rb(this.add(32));
    let now = now_ns(ctx);
    if inn {
        wb(this.add(16), true);
        wi(this.add(24), 0);
        wb(this.add(48), false); // off-running cancelled
    } else {
        if last {
            wi(this.add(40), now); // falling edge: capture off-start
            wb(this.add(48), true);
        }
        if rb(this.add(48)) {
            let et = (now - ri(this.add(40))).min(pt);
            wi(this.add(24), et);
            if et >= pt {
                wb(this.add(16), false);
                wb(this.add(48), false);
            }
        }
    }
    wb(this.add(32), inn);
    0
}

/// TP: IN↑ starts a non-retriggerable PT pulse; Q high until ET>=PT.
#[no_mangle]
/// # Safety
/// `this` must point at a valid instance block of the
/// documented layout; `ctx` at a live `TplcCtx`.
pub unsafe extern "C" fn __tplc_rt_TP(this: *mut u8, ctx: *mut u8) -> i32 {
    let inn = rb(this);
    let pt = ri(this.add(8));
    let last = rb(this.add(32));
    let now = now_ns(ctx);
    if inn && !last && !rb(this.add(48)) {
        wb(this.add(48), true);
        wi(this.add(40), now);
        wb(this.add(16), true);
    }
    if rb(this.add(48)) {
        let et = (now - ri(this.add(40))).min(pt);
        wi(this.add(24), et);
        if et >= pt {
            wb(this.add(16), false);
            wb(this.add(48), false);
        }
    }
    wb(this.add(32), inn);
    0
}

/* ---------------- edge detectors ---------------- */

#[no_mangle]
/// # Safety
/// `this` must point at a valid instance block of the
/// documented layout; `ctx` at a live `TplcCtx`.
pub unsafe extern "C" fn __tplc_rt_R_TRIG(this: *mut u8, _ctx: *mut u8) -> i32 {
    let clk = rb(this);
    let last = rb(this.add(2));
    wb(this.add(1), clk && !last);
    wb(this.add(2), clk);
    0
}

#[no_mangle]
/// # Safety
/// `this` must point at a valid instance block of the
/// documented layout; `ctx` at a live `TplcCtx`.
pub unsafe extern "C" fn __tplc_rt_F_TRIG(this: *mut u8, _ctx: *mut u8) -> i32 {
    let clk = rb(this);
    let last = rb(this.add(2));
    wb(this.add(1), !clk && last);
    wb(this.add(2), clk);
    0
}

/* ---------------- counters ---------------- */

unsafe fn ctud_step(this: *mut u8) {
    let cu = rb(this);
    let cd = rb(this.add(1));
    let r = rb(this.add(2));
    let ld = rb(this.add(3));
    let pv = ri32(this.add(4));
    let cu_last = rb(this.add(16));
    let cd_last = rb(this.add(17));
    let mut cv = ri32(this.add(12));
    if ld {
        cv = pv;
    }
    if r {
        cv = 0;
    }
    if !r && !ld {
        if cu && !cu_last {
            cv = cv.saturating_add(1);
        }
        if cd && !cd_last {
            cv = cv.saturating_sub(1);
        }
    }
    wi32(this.add(12), cv);
    wb(this.add(8), cv >= pv); // QU
    wb(this.add(9), cv <= 0); // QD
    wb(this.add(16), cu);
    wb(this.add(17), cd);
}

#[no_mangle]
/// # Safety
/// `this` must point at a valid instance block of the
/// documented layout; `ctx` at a live `TplcCtx`.
pub unsafe extern "C" fn __tplc_rt_CTU(this: *mut u8, _ctx: *mut u8) -> i32 {
    ctud_step(this);
    0
}
#[no_mangle]
/// # Safety
/// `this` must point at a valid instance block of the
/// documented layout; `ctx` at a live `TplcCtx`.
pub unsafe extern "C" fn __tplc_rt_CTD(this: *mut u8, _ctx: *mut u8) -> i32 {
    ctud_step(this);
    0
}
#[no_mangle]
/// # Safety
/// `this` must point at a valid instance block of the
/// documented layout; `ctx` at a live `TplcCtx`.
pub unsafe extern "C" fn __tplc_rt_CTUD(this: *mut u8, _ctx: *mut u8) -> i32 {
    ctud_step(this);
    0
}

/* ---------------- bistables ---------------- */

/// SR: set-dominant (S1 wins over R).
#[no_mangle]
/// # Safety
/// `this` must point at a valid instance block of the
/// documented layout; `ctx` at a live `TplcCtx`.
pub unsafe extern "C" fn __tplc_rt_SR(this: *mut u8, _ctx: *mut u8) -> i32 {
    let s1 = rb(this.add(2));
    let r = rb(this.add(3));
    let q1 = if s1 {
        true
    } else if r {
        false
    } else {
        rb(this.add(4))
    };
    wb(this.add(4), q1);
    0
}

/// RS: reset-dominant (R1 wins over S).
#[no_mangle]
/// # Safety
/// `this` must point at a valid instance block of the
/// documented layout; `ctx` at a live `TplcCtx`.
pub unsafe extern "C" fn __tplc_rt_RS(this: *mut u8, _ctx: *mut u8) -> i32 {
    let s = rb(this);
    let r1 = rb(this.add(1));
    let q1 = if r1 {
        false
    } else if s {
        true
    } else {
        rb(this.add(4))
    };
    wb(this.add(4), q1);
    0
}

/* ---------------- FB_Socket (tplc-net service bridge) ---------------- */

// TplcCtx.svc @72; TplcServices.get_service @48, .host @56.
// TplcNetService fn ptrs start @16, eight bytes apart.
const CTX_SVC: usize = 72;
const SVC_GET_SERVICE: usize = 48;
const SVC_HOST: usize = 56;
const NET_TCP_CONNECT: usize = 16;
const NET_TCP_CONNECT_POLL: usize = 24;
const NET_TCP_LISTEN: usize = 32;
const NET_TCP_ACCEPT: usize = 40;
const NET_TCP_SEND: usize = 48;
const NET_TCP_RECV: usize = 56;
const NET_TCP_CLOSE: usize = 64;
const NET_UDP_BIND: usize = 72;
const NET_UDP_SENDTO: usize = 80;
const NET_UDP_RECVFROM: usize = 88;
const NET_UDP_CLOSE: usize = 96;

// member offsets — keep in lockstep with builtin_fb_layout "FB_Socket"
const SO_OP: usize = 0;
const SO_HOST_LEN: usize = 4;
const SO_HOST: usize = 5;
const SO_PORT: usize = 20;
const SO_TIMEOUT: usize = 24;
const SO_DATA_LEN: usize = 32;
const SO_DATA: usize = 33;
const SO_DLEN: usize = 292;
const SO_HANDLE: usize = 296;
const SO_AUX: usize = 300;
const SO_RESULT: usize = 304;
const SO_ERRID: usize = 308;
const SO_EXEC: usize = 312;
const SO_BUSY: usize = 313;
const SO_DONE: usize = 314;
const SO_LAST_EXEC: usize = 315;
const SO_DEADLINE: usize = 320;
const SO_SENT: usize = 328;
const SO_PHASE: usize = 336;

const SOCK_OP_CONNECT: i32 = 1;
const SOCK_OP_LISTEN: i32 = 2;
const SOCK_OP_ACCEPT: i32 = 3;
const SOCK_OP_SEND: i32 = 4;
const SOCK_OP_RECV: i32 = 5;
const SOCK_OP_CLOSE: i32 = 6;
const SOCK_OP_UDP_BIND: i32 = 7;
const SOCK_OP_UDP_SENDTO: i32 = 8;
const SOCK_OP_UDP_RECV: i32 = 9;
const SOCK_OP_UDP_CLOSE: i32 = 10;

// mirrored TplcNetService result codes (positive form for ERRID);
// service errors arrive as negatives and are stored as -rc
const NETERR_PARAM: u32 = 1;
const NETERR_NOSERVICE: u32 = 2;
const NETERR_TIMEOUT: u32 = 5;

type FNetConnect = unsafe extern "C" fn(*const u8, *const u8, u16, *mut u32) -> i32;
type FNetConnectPoll = unsafe extern "C" fn(*const u8, u32) -> i32;
type FNetListen = unsafe extern "C" fn(*const u8, *const u8, u16, *mut u32) -> i32;
type FNetAccept = unsafe extern "C" fn(*const u8, u32, *mut u32) -> i32;
type FNetSend = unsafe extern "C" fn(*const u8, u32, *const u8, u32, *mut u32) -> i32;
type FNetRecv = unsafe extern "C" fn(*const u8, u32, *mut u8, u32, *mut u32) -> i32;
type FNetClose = unsafe extern "C" fn(*const u8, u32) -> i32;
type FNetBind = unsafe extern "C" fn(*const u8, *const u8, u16, *mut u32) -> i32;
type FNetSendTo =
    unsafe extern "C" fn(*const u8, u32, *const u8, u16, *const u8, u32, *mut u32) -> i32;
type FNetRecvFrom =
    unsafe extern "C" fn(*const u8, u32, *mut u8, u32, *mut u8, *mut u16, *mut u32) -> i32;

unsafe fn sock_fail(this: *mut u8, errid: u32) {
    wb(this.add(SO_BUSY), false);
    wb(this.add(SO_DONE), true);
    wu32(this.add(SO_ERRID), errid);
    wi32(this.add(SO_RESULT), 0);
}

unsafe fn sock_done(this: *mut u8, result: i32) {
    wb(this.add(SO_BUSY), false);
    wb(this.add(SO_DONE), true);
    wu32(this.add(SO_ERRID), 0);
    wi32(this.add(SO_RESULT), result);
}

/// copy the len-prefixed HOST member into a NUL-terminated buffer
unsafe fn host_cstr(this: *const u8, out: *mut u8) {
    let n = (*this.add(SO_HOST_LEN) as usize).min(15);
    core::ptr::copy_nonoverlapping(this.add(SO_HOST), out, n);
    *out.add(n) = 0;
}

/// resolve the tplc-net service table through ctx->svc->get_service
unsafe fn net_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-net".as_ptr() as *const u8, 8) as *const u8
}

/// load a TplcNetService slot as a raw pointer-sized value
#[inline]
unsafe fn netfn(net: *const u8, off: usize) -> usize {
    (net.add(off) as *const usize).read()
}

/// `__tplc_rt_FB_Socket` — EXEC rising edge arms an op; while BUSY the
/// op is polled each scan until DONE or the TIMEOUT deadline. OP codes
/// mirror TPLC_SOCK_OP_*; ERRID carries the positive service code.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_Socket(this: *mut u8, ctx: *mut u8) -> i32 {
    let exec = rb(this.add(SO_EXEC));
    let last = rb(this.add(SO_LAST_EXEC));
    let now = now_ns(ctx);

    if !rb(this.add(SO_BUSY)) {
        if !(exec && !last) {
            wb(this.add(SO_LAST_EXEC), exec);
            return 0;
        }
        // rising edge: arm the op
        wb(this.add(SO_DONE), false);
        wu32(this.add(SO_ERRID), 0);
        wi32(this.add(SO_RESULT), 0);
        wu32(this.add(SO_SENT), 0);
        wb(this.add(SO_BUSY), true);
        let to = ri(this.add(SO_TIMEOUT));
        wi(
            this.add(SO_DEADLINE),
            if to > 0 {
                now.saturating_add(to)
            } else {
                i64::MAX
            },
        );
    }
    wb(this.add(SO_LAST_EXEC), exec);

    let net = net_service(ctx);
    if net.is_null() {
        sock_fail(this, NETERR_NOSERVICE);
        return 0;
    }
    if now > ri(this.add(SO_DEADLINE)) {
        sock_fail(this, NETERR_TIMEOUT);
        return 0;
    }

    let op = ri32(this.add(SO_OP));
    let mut hbuf = [0u8; 16];
    match op {
        SOCK_OP_CONNECT => {
            if ri32(this.add(SO_PHASE)) == 0 {
                host_cstr(this, hbuf.as_mut_ptr());
                let mut out: u32 = 0;
                let f: FNetConnect = core::mem::transmute(netfn(net, NET_TCP_CONNECT));
                let rc = f(net, hbuf.as_ptr(), ru32(this.add(SO_PORT)) as u16, &mut out);
                if rc < 0 {
                    sock_fail(this, (-rc) as u32);
                    return 0;
                }
                wu32(this.add(SO_HANDLE), out);
                wi32(this.add(SO_PHASE), 1);
            }
            let f: FNetConnectPoll = core::mem::transmute(netfn(net, NET_TCP_CONNECT_POLL));
            match f(net, ru32(this.add(SO_HANDLE))) {
                0 => sock_done(this, 0),
                -7 => {} // still pending
                e => sock_fail(this, (-e) as u32),
            }
        }
        SOCK_OP_LISTEN => {
            host_cstr(this, hbuf.as_mut_ptr());
            let mut out: u32 = 0;
            let f: FNetListen = core::mem::transmute(netfn(net, NET_TCP_LISTEN));
            let rc = f(net, hbuf.as_ptr(), ru32(this.add(SO_PORT)) as u16, &mut out);
            if rc < 0 {
                sock_fail(this, (-rc) as u32);
            } else {
                wu32(this.add(SO_HANDLE), out);
                sock_done(this, 0);
            }
        }
        SOCK_OP_ACCEPT => {
            let mut out: u32 = 0;
            let f: FNetAccept = core::mem::transmute(netfn(net, NET_TCP_ACCEPT));
            match f(net, ru32(this.add(SO_AUX)), &mut out) {
                0 => {
                    wu32(this.add(SO_HANDLE), out);
                    sock_done(this, 0);
                }
                -7 => {}
                e => sock_fail(this, (-e) as u32),
            }
        }
        SOCK_OP_SEND => {
            let dlen = ru32(this.add(SO_DLEN)) as usize;
            let slen = (*this.add(SO_DATA_LEN) as usize).min(256);
            let total = if dlen > 0 { dlen.min(256) } else { slen };
            let mut sent = ru32(this.add(SO_SENT)) as usize;
            let f: FNetSend = core::mem::transmute(netfn(net, NET_TCP_SEND));
            for _ in 0..8 {
                if sent >= total {
                    break;
                }
                let mut done: u32 = 0;
                match f(
                    net,
                    ru32(this.add(SO_HANDLE)),
                    this.add(SO_DATA).add(sent),
                    (total - sent) as u32,
                    &mut done,
                ) {
                    0 => {
                        sent += done as usize;
                        if done == 0 {
                            break;
                        }
                    }
                    -7 => break,
                    e => {
                        sock_fail(this, (-e) as u32);
                        return 0;
                    }
                }
            }
            wu32(this.add(SO_SENT), sent as u32);
            if sent >= total {
                sock_done(this, sent as i32);
            }
        }
        SOCK_OP_RECV => {
            // DLEN is a pure output here: recv always reads up to the
            // full DATA capacity so a re-armed op cannot inherit a
            // stale cap from the previous transfer.
            let mut done: u32 = 0;
            let f: FNetRecv = core::mem::transmute(netfn(net, NET_TCP_RECV));
            match f(
                net,
                ru32(this.add(SO_HANDLE)),
                this.add(SO_DATA),
                256,
                &mut done,
            ) {
                0 => {
                    *this.add(SO_DATA_LEN) = done.min(256) as u8;
                    wu32(this.add(SO_DLEN), done);
                    sock_done(this, done as i32);
                }
                -7 => {}
                e => sock_fail(this, (-e) as u32),
            }
        }
        SOCK_OP_CLOSE => {
            let f: FNetClose = core::mem::transmute(netfn(net, NET_TCP_CLOSE));
            f(net, ru32(this.add(SO_HANDLE)));
            sock_done(this, 0);
        }
        SOCK_OP_UDP_BIND => {
            host_cstr(this, hbuf.as_mut_ptr());
            let mut out: u32 = 0;
            let f: FNetBind = core::mem::transmute(netfn(net, NET_UDP_BIND));
            let rc = f(net, hbuf.as_ptr(), ru32(this.add(SO_PORT)) as u16, &mut out);
            if rc < 0 {
                sock_fail(this, (-rc) as u32);
            } else {
                wu32(this.add(SO_HANDLE), out);
                sock_done(this, 0);
            }
        }
        SOCK_OP_UDP_SENDTO => {
            host_cstr(this, hbuf.as_mut_ptr());
            let dlen = ru32(this.add(SO_DLEN)) as usize;
            let slen = (*this.add(SO_DATA_LEN) as usize).min(256);
            let total = if dlen > 0 { dlen.min(256) } else { slen };
            let mut done: u32 = 0;
            let f: FNetSendTo = core::mem::transmute(netfn(net, NET_UDP_SENDTO));
            let rc = f(
                net,
                ru32(this.add(SO_HANDLE)),
                hbuf.as_ptr(),
                ru32(this.add(SO_PORT)) as u16,
                this.add(SO_DATA),
                total as u32,
                &mut done,
            );
            match rc {
                0 => sock_done(this, done as i32),
                e => sock_fail(this, (-e) as u32),
            }
        }
        SOCK_OP_UDP_RECV => {
            let mut peer = [0u8; 16];
            let mut pport: u16 = 0;
            let mut done: u32 = 0;
            let f: FNetRecvFrom = core::mem::transmute(netfn(net, NET_UDP_RECVFROM));
            match f(
                net,
                ru32(this.add(SO_HANDLE)),
                this.add(SO_DATA),
                256,
                peer.as_mut_ptr(),
                &mut pport,
                &mut done,
            ) {
                0 => {
                    *this.add(SO_DATA_LEN) = done.min(256) as u8;
                    wu32(this.add(SO_DLEN), done);
                    // peer endpoint -> HOST string + AUX port
                    let mut n = 0usize;
                    while n < 15 && peer[n] != 0 {
                        n += 1;
                    }
                    *this.add(SO_HOST_LEN) = n as u8;
                    core::ptr::copy_nonoverlapping(peer.as_ptr(), this.add(SO_HOST), n);
                    wu32(this.add(SO_AUX), pport as u32);
                    sock_done(this, done as i32);
                }
                -7 => {}
                e => sock_fail(this, (-e) as u32),
            }
        }
        SOCK_OP_UDP_CLOSE => {
            let f: FNetClose = core::mem::transmute(netfn(net, NET_UDP_CLOSE));
            f(net, ru32(this.add(SO_HANDLE)));
            sock_done(this, 0);
        }
        _ => sock_fail(this, NETERR_PARAM),
    }
    0
}

/* ---------------- FB_Serial (tplc-serial service bridge) ---------------- */

// TplcSerialService fn ptrs start @16 (after struct_size/abi_version/
// host), eight bytes apart.
const SER_OPEN: usize = 16;
const SER_WRITE: usize = 24;
const SER_READ: usize = 32;
const SER_FLUSH: usize = 40;
const SER_RS485: usize = 48;
const SER_CLOSE: usize = 56;

// member offsets — keep in lockstep with builtin_fb_layout "FB_Serial"
const SE_OP: usize = 0;
const SE_DEV_LEN: usize = 4;
const SE_DEV: usize = 5;
const SE_BAUD: usize = 68;
const SE_PARITY: usize = 72;
const SE_DBITS: usize = 76;
const SE_SBITS: usize = 80;
const SE_FLAGS: usize = 84;
const SE_AUX: usize = 88;
const SE_TIMEOUT: usize = 96;
const SE_DATA_LEN: usize = 104;
const SE_DATA: usize = 105;
const SE_DLEN: usize = 364;
const SE_HANDLE: usize = 368;
const SE_RESULT: usize = 372;
const SE_ERRID: usize = 376;
const SE_EXEC: usize = 380;
const SE_BUSY: usize = 381;
const SE_DONE: usize = 382;
const SE_LAST_EXEC: usize = 383;
const SE_DEADLINE: usize = 384;
const SE_SENT: usize = 392;

const SER_OP_OPEN: i32 = 1;
const SER_OP_WRITE: i32 = 2;
const SER_OP_READ: i32 = 3;
const SER_OP_CLOSE: i32 = 4;
const SER_OP_FLUSH: i32 = 5;
const SER_OP_RS485: i32 = 6;

// mirrored TplcSerialService result codes (positive form for ERRID)
const SERERR_PARAM: u32 = 1;
const SERERR_NOSERVICE: u32 = 2;
const SERERR_TIMEOUT: u32 = 5;

type FSerOpen = unsafe extern "C" fn(*const u8, *const u8, u32, u8, u8, u8, u32, *mut u32) -> i32;
type FSerWrite = unsafe extern "C" fn(*const u8, u32, *const u8, u32, *mut u32) -> i32;
type FSerRead = unsafe extern "C" fn(*const u8, u32, *mut u8, u32, *mut u32) -> i32;
type FSerFlush = unsafe extern "C" fn(*const u8, u32, u8) -> i32;
type FSerRs485 = unsafe extern "C" fn(*const u8, u32, u8) -> i32;
type FSerClose = unsafe extern "C" fn(*const u8, u32) -> i32;

unsafe fn ser_fail(this: *mut u8, errid: u32) {
    wb(this.add(SE_BUSY), false);
    wb(this.add(SE_DONE), true);
    wu32(this.add(SE_ERRID), errid);
    wi32(this.add(SE_RESULT), 0);
}

unsafe fn ser_done(this: *mut u8, result: i32) {
    wb(this.add(SE_BUSY), false);
    wb(this.add(SE_DONE), true);
    wu32(this.add(SE_ERRID), 0);
    wi32(this.add(SE_RESULT), result);
}

/// copy the len-prefixed DEV member into a NUL-terminated buffer
unsafe fn dev_cstr(this: *const u8, out: *mut u8) {
    let n = (*this.add(SE_DEV_LEN) as usize).min(63);
    core::ptr::copy_nonoverlapping(this.add(SE_DEV), out, n);
    *out.add(n) = 0;
}

/// resolve the tplc-serial service table through ctx->svc->get_service
unsafe fn ser_service(ctx: *const u8) -> *const u8 {
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
unsafe fn serfn(ser: *const u8, off: usize) -> usize {
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

/* ---------------- FB_Ads (tplc-ads service bridge) ---------------- */

// TplcAdsService fn ptrs start @16, eight bytes apart.
const ADS_OPEN: usize = 16;
const ADS_READ: usize = 24;
const ADS_WRITE: usize = 32;
const ADS_READ_STATE: usize = 40;
const ADS_WRITE_CTRL: usize = 48;
const ADS_CLOSE: usize = 56;

// member offsets — keep in lockstep with builtin_fb_layout "FB_Ads"
const AD_OP: usize = 0;
const AD_HOST_LEN: usize = 4;
const AD_HOST: usize = 5;
const AD_PORT: usize = 20;
const AD_IDXGRP: usize = 24;
const AD_IDXOFF: usize = 28;
const AD_STATE: usize = 32;
const AD_DEVSTATE: usize = 36;
const AD_TIMEOUT: usize = 48;
const AD_DATA_LEN: usize = 56;
const AD_DATA: usize = 57;
const AD_DLEN: usize = 316;
const AD_HANDLE: usize = 320;
const AD_RESULT: usize = 324;
const AD_ERRID: usize = 328;
const AD_EXEC: usize = 332;
const AD_BUSY: usize = 333;
const AD_DONE: usize = 334;
const AD_LAST_EXEC: usize = 335;
const AD_DEADLINE: usize = 336;
const AD_SENT: usize = 344;

const ADS_OP_OPEN: i32 = 1;
const ADS_OP_READ: i32 = 2;
const ADS_OP_WRITE: i32 = 3;
const ADS_OP_READ_STATE: i32 = 4;
const ADS_OP_WRITE_CTRL: i32 = 5;
const ADS_OP_CLOSE: i32 = 6;

// mirrored TplcAdsService result codes (positive form for ERRID)
const ADSERR_PARAM: u32 = 1;
const ADSERR_NOSERVICE: u32 = 2;
const ADSERR_TIMEOUT: u32 = 5;

type FAdsOpen = unsafe extern "C" fn(*const u8, *const u8, u16, *mut u32) -> i32;
type FAdsRead =
    unsafe extern "C" fn(*const u8, u32, u32, u32, u32, *mut u8, u32, *mut u32, *mut u32) -> i32;
type FAdsWrite = unsafe extern "C" fn(*const u8, u32, u32, u32, *const u8, u32, *mut u32) -> i32;
type FAdsReadState = unsafe extern "C" fn(*const u8, u32, *mut u32, *mut u32, *mut u32) -> i32;
type FAdsWriteCtrl = unsafe extern "C" fn(*const u8, u32, u32, u32, *mut u32) -> i32;
type FAdsClose = unsafe extern "C" fn(*const u8, u32) -> i32;

unsafe fn ads_fail(this: *mut u8, errid: u32) {
    wb(this.add(AD_BUSY), false);
    wb(this.add(AD_DONE), true);
    wu32(this.add(AD_ERRID), errid);
    wi32(this.add(AD_RESULT), 0);
}

unsafe fn ads_done(this: *mut u8, result: i32) {
    wb(this.add(AD_BUSY), false);
    wb(this.add(AD_DONE), true);
    wu32(this.add(AD_ERRID), 0);
    wi32(this.add(AD_RESULT), result);
}

/// copy the len-prefixed HOST member into a NUL-terminated buffer
unsafe fn ad_host_cstr(this: *const u8, out: *mut u8) {
    let n = (*this.add(AD_HOST_LEN) as usize).min(15);
    core::ptr::copy_nonoverlapping(this.add(AD_HOST), out, n);
    *out.add(n) = 0;
}

/// resolve the tplc-ads service table through ctx->svc->get_service
unsafe fn ads_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-ads".as_ptr() as *const u8, 8) as *const u8
}

/// load a TplcAdsService slot as a raw pointer-sized value
#[inline]
unsafe fn adsfn(ads: *const u8, off: usize) -> usize {
    (ads.add(off) as *const usize).read()
}

/// `__tplc_rt_FB_Ads` — EXEC rising edge arms an op; while BUSY the
/// op is polled each scan until DONE or the TIMEOUT deadline. RESULT
/// carries the ADS-layer error code; ERRID carries service errors.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_Ads(this: *mut u8, ctx: *mut u8) -> i32 {
    let exec = rb(this.add(AD_EXEC));
    let last = rb(this.add(AD_LAST_EXEC));
    let now = now_ns(ctx);

    if !rb(this.add(AD_BUSY)) {
        if !(exec && !last) {
            wb(this.add(AD_LAST_EXEC), exec);
            return 0;
        }
        // rising edge: arm the op
        wb(this.add(AD_DONE), false);
        wu32(this.add(AD_ERRID), 0);
        wi32(this.add(AD_RESULT), 0);
        wu32(this.add(AD_SENT), 0);
        wb(this.add(AD_BUSY), true);
        let to = ri(this.add(AD_TIMEOUT));
        wi(
            this.add(AD_DEADLINE),
            if to > 0 {
                now.saturating_add(to)
            } else {
                i64::MAX
            },
        );
    }
    wb(this.add(AD_LAST_EXEC), exec);

    let ads = ads_service(ctx);
    if ads.is_null() {
        ads_fail(this, ADSERR_NOSERVICE);
        return 0;
    }
    if now > ri(this.add(AD_DEADLINE)) {
        ads_fail(this, ADSERR_TIMEOUT);
        return 0;
    }

    let op = ri32(this.add(AD_OP));
    let mut hbuf = [0u8; 16];
    match op {
        ADS_OP_OPEN => {
            ad_host_cstr(this, hbuf.as_mut_ptr());
            let mut out: u32 = 0;
            let f: FAdsOpen = core::mem::transmute(adsfn(ads, ADS_OPEN));
            let rc = f(ads, hbuf.as_ptr(), ru32(this.add(AD_PORT)) as u16, &mut out);
            if rc < 0 {
                ads_fail(this, (-rc) as u32);
            } else {
                wu32(this.add(AD_HANDLE), out);
                ads_done(this, 0);
            }
        }
        ADS_OP_READ => {
            let req_len = ru32(this.add(AD_DLEN)).min(256);
            let mut rlen: u32 = 0;
            let mut adserr: u32 = 0;
            let f: FAdsRead = core::mem::transmute(adsfn(ads, ADS_READ));
            let rc = f(
                ads,
                ru32(this.add(AD_HANDLE)),
                ru32(this.add(AD_IDXGRP)),
                ru32(this.add(AD_IDXOFF)),
                req_len,
                this.add(AD_DATA),
                256,
                &mut rlen,
                &mut adserr,
            );
            match rc {
                0 => {
                    *this.add(AD_DATA_LEN) = rlen.min(256) as u8;
                    wu32(this.add(AD_DLEN), rlen);
                    ads_done(this, adserr as i32);
                }
                -6 => {}
                e => ads_fail(this, (-e) as u32),
            }
        }
        ADS_OP_WRITE => {
            let dlen = ru32(this.add(AD_DLEN)) as usize;
            let slen = (*this.add(AD_DATA_LEN) as usize).min(256);
            let total = if dlen > 0 { dlen.min(256) } else { slen };
            let mut adserr: u32 = 0;
            let f: FAdsWrite = core::mem::transmute(adsfn(ads, ADS_WRITE));
            let rc = f(
                ads,
                ru32(this.add(AD_HANDLE)),
                ru32(this.add(AD_IDXGRP)),
                ru32(this.add(AD_IDXOFF)),
                this.add(AD_DATA),
                total as u32,
                &mut adserr,
            );
            match rc {
                0 => ads_done(this, adserr as i32),
                -6 => {}
                e => ads_fail(this, (-e) as u32),
            }
        }
        ADS_OP_READ_STATE => {
            let mut ads_state: u32 = 0;
            let mut dev_state: u32 = 0;
            let mut adserr: u32 = 0;
            let f: FAdsReadState = core::mem::transmute(adsfn(ads, ADS_READ_STATE));
            let rc = f(
                ads,
                ru32(this.add(AD_HANDLE)),
                &mut ads_state,
                &mut dev_state,
                &mut adserr,
            );
            match rc {
                0 => {
                    wi32(this.add(AD_STATE), ads_state as i32);
                    wi32(this.add(AD_DEVSTATE), dev_state as i32);
                    ads_done(this, adserr as i32);
                }
                -6 => {}
                e => ads_fail(this, (-e) as u32),
            }
        }
        ADS_OP_WRITE_CTRL => {
            let mut adserr: u32 = 0;
            let f: FAdsWriteCtrl = core::mem::transmute(adsfn(ads, ADS_WRITE_CTRL));
            let rc = f(
                ads,
                ru32(this.add(AD_HANDLE)),
                ru32(this.add(AD_STATE)),
                ru32(this.add(AD_DEVSTATE)),
                &mut adserr,
            );
            match rc {
                0 => ads_done(this, adserr as i32),
                -6 => {}
                e => ads_fail(this, (-e) as u32),
            }
        }
        ADS_OP_CLOSE => {
            let f: FAdsClose = core::mem::transmute(adsfn(ads, ADS_CLOSE));
            f(ads, ru32(this.add(AD_HANDLE)));
            ads_done(this, 0);
        }
        _ => ads_fail(this, ADSERR_PARAM),
    }
    0
}
