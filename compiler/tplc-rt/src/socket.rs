//! tplc-rt builtin FB: socket.
use crate::*;

/* ---------------- FB_Socket (tplc-net service bridge) ---------------- */

// TplcCtx.svc @72; TplcServices.get_service @48, .host @56.
// TplcNetService fn ptrs start @16, eight bytes apart.
pub(crate) const CTX_SVC: usize = 72;
pub(crate) const SVC_GET_SERVICE: usize = 48;
pub(crate) const SVC_HOST: usize = 56;
pub(crate) const NET_TCP_CONNECT: usize = 16;
pub(crate) const NET_TCP_CONNECT_POLL: usize = 24;
pub(crate) const NET_TCP_LISTEN: usize = 32;
pub(crate) const NET_TCP_ACCEPT: usize = 40;
pub(crate) const NET_TCP_SEND: usize = 48;
pub(crate) const NET_TCP_RECV: usize = 56;
pub(crate) const NET_TCP_CLOSE: usize = 64;
pub(crate) const NET_UDP_BIND: usize = 72;
pub(crate) const NET_UDP_SENDTO: usize = 80;
pub(crate) const NET_UDP_RECVFROM: usize = 88;
pub(crate) const NET_UDP_CLOSE: usize = 96;

// member offsets — keep in lockstep with builtin_fb_layout "FB_Socket"
pub(crate) const SO_OP: usize = 0;
pub(crate) const SO_HOST_LEN: usize = 4;
pub(crate) const SO_HOST: usize = 5;
pub(crate) const SO_PORT: usize = 20;
pub(crate) const SO_TIMEOUT: usize = 24;
pub(crate) const SO_DATA_LEN: usize = 32;
pub(crate) const SO_DATA: usize = 33;
pub(crate) const SO_DLEN: usize = 292;
pub(crate) const SO_HANDLE: usize = 296;
pub(crate) const SO_AUX: usize = 300;
pub(crate) const SO_RESULT: usize = 304;
pub(crate) const SO_ERRID: usize = 308;
pub(crate) const SO_EXEC: usize = 312;
pub(crate) const SO_BUSY: usize = 313;
pub(crate) const SO_DONE: usize = 314;
pub(crate) const SO_LAST_EXEC: usize = 315;
pub(crate) const SO_DEADLINE: usize = 320;
pub(crate) const SO_SENT: usize = 328;
pub(crate) const SO_PHASE: usize = 336;

pub(crate) const SOCK_OP_CONNECT: i32 = 1;
pub(crate) const SOCK_OP_LISTEN: i32 = 2;
pub(crate) const SOCK_OP_ACCEPT: i32 = 3;
pub(crate) const SOCK_OP_SEND: i32 = 4;
pub(crate) const SOCK_OP_RECV: i32 = 5;
pub(crate) const SOCK_OP_CLOSE: i32 = 6;
pub(crate) const SOCK_OP_UDP_BIND: i32 = 7;
pub(crate) const SOCK_OP_UDP_SENDTO: i32 = 8;
pub(crate) const SOCK_OP_UDP_RECV: i32 = 9;
pub(crate) const SOCK_OP_UDP_CLOSE: i32 = 10;

// mirrored TplcNetService result codes (positive form for ERRID);
// service errors arrive as negatives and are stored as -rc
pub(crate) const NETERR_PARAM: u32 = 1;
pub(crate) const NETERR_NOSERVICE: u32 = 2;
pub(crate) const NETERR_TIMEOUT: u32 = 5;

pub(crate) type FNetConnect = unsafe extern "C" fn(*const u8, *const u8, u16, *mut u32) -> i32;
pub(crate) type FNetConnectPoll = unsafe extern "C" fn(*const u8, u32) -> i32;
pub(crate) type FNetListen = unsafe extern "C" fn(*const u8, *const u8, u16, *mut u32) -> i32;
pub(crate) type FNetAccept = unsafe extern "C" fn(*const u8, u32, *mut u32) -> i32;
pub(crate) type FNetSend = unsafe extern "C" fn(*const u8, u32, *const u8, u32, *mut u32) -> i32;
pub(crate) type FNetRecv = unsafe extern "C" fn(*const u8, u32, *mut u8, u32, *mut u32) -> i32;
pub(crate) type FNetClose = unsafe extern "C" fn(*const u8, u32) -> i32;
pub(crate) type FNetBind = unsafe extern "C" fn(*const u8, *const u8, u16, *mut u32) -> i32;
pub(crate) type FNetSendTo =
    unsafe extern "C" fn(*const u8, u32, *const u8, u16, *const u8, u32, *mut u32) -> i32;
pub(crate) type FNetRecvFrom =
    unsafe extern "C" fn(*const u8, u32, *mut u8, u32, *mut u8, *mut u16, *mut u32) -> i32;

pub(crate) unsafe fn sock_fail(this: *mut u8, errid: u32) {
    wb(this.add(SO_BUSY), false);
    wb(this.add(SO_DONE), true);
    wu32(this.add(SO_ERRID), errid);
    wi32(this.add(SO_RESULT), 0);
}

pub(crate) unsafe fn sock_done(this: *mut u8, result: i32) {
    wb(this.add(SO_BUSY), false);
    wb(this.add(SO_DONE), true);
    wu32(this.add(SO_ERRID), 0);
    wi32(this.add(SO_RESULT), result);
}

/// copy the len-prefixed HOST member into a NUL-terminated buffer
pub(crate) unsafe fn host_cstr(this: *const u8, out: *mut u8) {
    let n = (*this.add(SO_HOST_LEN) as usize).min(15);
    core::ptr::copy_nonoverlapping(this.add(SO_HOST), out, n);
    *out.add(n) = 0;
}

/// resolve the tplc-net service table through ctx->svc->get_service
pub(crate) unsafe fn net_service(ctx: *const u8) -> *const u8 {
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
pub(crate) unsafe fn netfn(net: *const u8, off: usize) -> usize {
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
