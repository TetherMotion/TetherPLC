//! tplc-rt builtin FB: profinet.
use crate::*;

// FB_Profinet — PROFINET DCP + cyclic IO records over tplc-profinet
// (per-port UDP transports; real L2 Ethertype-0x8892 path staged).
// ---------------------------------------------------------------------------

// FB_Profinet member offsets (see builtin_fb_layout("FB_Profinet")).
pub(crate) const PN_OP: usize = 0;
pub(crate) const PN_PORT: usize = 4;
pub(crate) const PN_XID: usize = 8;
pub(crate) const PN_OPTION: usize = 12;
pub(crate) const PN_SUB: usize = 16;
pub(crate) const PN_DLEN: usize = 20;
pub(crate) const PN_STATUS: usize = 24;
pub(crate) const PN_ERRID: usize = 28;
pub(crate) const PN_ROLE: usize = 32;
pub(crate) const PN_VENDOR: usize = 36;
pub(crate) const PN_DEVID: usize = 40;
pub(crate) const PN_DEVINST: usize = 44;
pub(crate) const PN_OEM: usize = 48;
pub(crate) const PN_NAME: usize = 52; // Str{31}: u8 len + chars
pub(crate) const PN_OD: usize = 84; // u8[64]
pub(crate) const PN_IOOUT: usize = 148; // u8[64]
pub(crate) const PN_IOIN: usize = 212; // u8[64]
pub(crate) const PN_PNCYC: usize = 276; // u32 device cycle counter (OP8)
pub(crate) const PN_PNSTAT: usize = 280; // u32 DataStatus byte (OP8)
pub(crate) const PN_DEVOBJ: usize = 284; // u8[16] device object UUID (OP9)
pub(crate) const PN_ACTUUID: usize = 300; // u8[16] activity UUID (OP9)
pub(crate) const PN_ARUUID: usize = 316; // u8[16] AR UUID (OP9 in, OP13 out)
pub(crate) const PN_ARBLK: usize = 332; // u8[512] PNIO blocks in/out (OP9-11,14)
pub(crate) const PN_ARSTATE: usize = 844; // u32 AR state (OP9,12,13)
pub(crate) const PN_ARKEY: usize = 848; // u32 session key (OP9,13)
pub(crate) const PN_ARPSTAT: usize = 852; // u32 NDR pnio_status (OP9-11,14)

// tplc-profinet service vtable offsets (TplcProfinetService):
// struct_size@0 u32, abi@4 u32, host@8 ptr(8B),
// pn_state@16, pn_open@24, pn_close@32, pn_identify@40,
// pn_dcp_get@48, pn_dcp_set@56, pn_io_exchange@64, pn_io_cr@72,
// pn_port_info@80, pn_ar_connect@88, pn_ar_write@96,
// pn_ar_control@104, pn_ar_service@112, pn_ar_state@120, pn_ar_read@128.
pub(crate) const PN_SVC_STATE: usize = 16;
pub(crate) const PN_SVC_OPEN: usize = 24;
pub(crate) const PN_SVC_CLOSE: usize = 32;
pub(crate) const PN_SVC_IDENT: usize = 40;
pub(crate) const PN_SVC_DCPGET: usize = 48;
pub(crate) const PN_SVC_DCPSET: usize = 56;
pub(crate) const PN_SVC_IOEX: usize = 64;
pub(crate) const PN_SVC_IOCR: usize = 72;
pub(crate) const PN_SVC_ARCONN: usize = 88;
pub(crate) const PN_SVC_ARWRITE: usize = 96;
pub(crate) const PN_SVC_ARCTRL: usize = 104;
pub(crate) const PN_SVC_ARSRV: usize = 112;
pub(crate) const PN_SVC_ARSTATE: usize = 120;
pub(crate) const PN_SVC_ARREAD: usize = 128;

/// service lookup for "tplc-profinet" (14 chars)
pub(crate) unsafe fn profinet_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-profinet".as_ptr() as *const u8, 14) as *const u8
}

/// `__tplc_rt_FB_Profinet` — OP 1=open, 2=close, 3=state,
/// 4=identify, 5=dcp_get, 6=dcp_set, 7=io_exchange.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_Profinet(this: *mut u8, ctx: *mut u8) -> i32 {
    let op = ru32(this.add(PN_OP));
    wi32(this.add(PN_STATUS), 0);
    wu32(this.add(PN_ERRID), 0);
    let svc = profinet_service(ctx);
    if svc.is_null() {
        wi32(this.add(PN_STATUS), -3);
        wu32(this.add(PN_ERRID), 99);
        return 0;
    }
    let set_rc = |rc: i32| {
        wi32(this.add(PN_STATUS), rc);
        wu32(this.add(PN_ERRID), (-rc).max(0) as u32);
    };
    let port = ru32(this.add(PN_PORT));
    let xid = ru32(this.add(PN_XID));
    match op {
        1 => {
            let f: unsafe extern "C" fn(*const u8, u32) -> i32 =
                core::mem::transmute(
                    (svc.add(PN_SVC_OPEN) as *const usize).read(),
                );
            set_rc(f(svc, port));
        }
        2 => {
            let f: unsafe extern "C" fn(*const u8, u32) -> i32 =
                core::mem::transmute(
                    (svc.add(PN_SVC_CLOSE) as *const usize).read(),
                );
            set_rc(f(svc, port));
        }
        3 => {
            let f: unsafe extern "C" fn(*const u8) -> i32 =
                core::mem::transmute(
                    (svc.add(PN_SVC_STATE) as *const usize).read(),
                );
            set_rc(f(svc));
        }
        4 => {
            /* pn_identify(svc, port, xid, name*, cap, vendor*, device*,
               role*, instance*, oem*, raw*, raw_cap, raw_len*) */
            let f: unsafe extern "C" fn(
                *const u8, u32, u32, *mut u8, u32, *mut u32, *mut u32,
                *mut u32, *mut u32, *mut u32, *mut u8, u32,
                *mut u32,
            ) -> i32 = core::mem::transmute(
                (svc.add(PN_SVC_IDENT) as *const usize).read(),
            );
            let mut nbuf = [0u8; 40];
            let mut vendor: u32 = 0;
            let mut device: u32 = 0;
            let mut role: u32 = 0;
            let mut inst: u32 = 0;
            let mut oem: u32 = 0;
            let rc = f(
                svc, port, xid, nbuf.as_mut_ptr(), 32,
                &mut vendor, &mut device, &mut role, &mut inst,
                &mut oem, core::ptr::null_mut(), 0,
                core::ptr::null_mut(),
            );
            /* Str{31}: u8 len + chars */
            let mut nl = 0usize;
            while nl < 31 && nbuf[nl] != 0 {
                nl += 1;
            }
            let nm = this.add(PN_NAME);
            *nm = nl as u8;
            core::ptr::copy_nonoverlapping(nbuf.as_ptr(), nm.add(1), nl);
            wu32(this.add(PN_VENDOR), vendor);
            wu32(this.add(PN_DEVID), device);
            wu32(this.add(PN_ROLE), role);
            wu32(this.add(PN_DEVINST), inst);
            wu32(this.add(PN_OEM), oem);
            set_rc(rc);
        }
        5 => {
            /* pn_dcp_get(svc, port, xid, option u8, sub u8, data*,
               cap, out_len*) -> i32 */
            let f: unsafe extern "C" fn(
                *const u8, u32, u32, u8, u8, *mut u8, u32, *mut u32,
            ) -> i32 = core::mem::transmute(
                (svc.add(PN_SVC_DCPGET) as *const usize).read(),
            );
            let mut out_len: u32 = 0;
            let rc = f(
                svc, port, xid,
                ru32(this.add(PN_OPTION)) as u8,
                ru32(this.add(PN_SUB)) as u8,
                this.add(PN_OD), 64, &mut out_len,
            );
            wu32(this.add(PN_DLEN), out_len);
            set_rc(rc);
        }
        6 => {
            /* pn_dcp_set(svc, port, xid, option u8, sub u8, data*,
               len) -> i32 */
            let f: unsafe extern "C" fn(
                *const u8, u32, u32, u8, u8, *const u8, u32,
            ) -> i32 = core::mem::transmute(
                (svc.add(PN_SVC_DCPSET) as *const usize).read(),
            );
            let len = ru32(this.add(PN_DLEN)).min(64);
            set_rc(f(
                svc, port, xid,
                ru32(this.add(PN_OPTION)) as u8,
                ru32(this.add(PN_SUB)) as u8,
                this.add(PN_OD), len,
            ));
        }
        7 => {
            /* pn_io_exchange(svc, port, out*, out_len, in*, in_cap,
               in_len*) -> i32 */
            let f: unsafe extern "C" fn(
                *const u8, u32, *const u8, u32, *mut u8, u32,
                *mut u32,
            ) -> i32 = core::mem::transmute(
                (svc.add(PN_SVC_IOEX) as *const usize).read(),
            );
            let out_len = ru32(this.add(PN_DLEN)).min(64);
            let mut in_len: u32 = 0;
            let rc = f(
                svc, port,
                this.add(PN_IOOUT), out_len,
                this.add(PN_IOIN), 64, &mut in_len,
            );
            wu32(this.add(PN_DLEN), in_len);
            set_rc(rc);
        }
        8 => {
            /* pn_io_cr(svc, port, frame_id, out*, out_len, in*,
               in_cap, in_len*, cyc*, stat*) -> i32: IOCR APDU
               exchange — XID=frame_id (0=keep), DLEN=out len,
               PNCYC/PNSTAT receive the device trailer. */
            let f: unsafe extern "C" fn(
                *const u8, u32, u32, *const u8, u32, *mut u8, u32,
                *mut u32, *mut u32, *mut u32,
            ) -> i32 = core::mem::transmute(
                (svc.add(PN_SVC_IOCR) as *const usize).read(),
            );
            let out_len = ru32(this.add(PN_DLEN)).min(64);
            let mut in_len: u32 = 0;
            let rc = f(
                svc, port, ru32(this.add(PN_XID)),
                this.add(PN_IOOUT), out_len,
                this.add(PN_IOIN), 64, &mut in_len,
                this.add(PN_PNCYC) as *mut u32,
                this.add(PN_PNSTAT) as *mut u32,
            );
            wu32(this.add(PN_DLEN), in_len);
            set_rc(rc);
        }
        /* PNIO-CM AR ops: ARBLK[0..DLEN) carries encoded PNIO blocks
           in; the service copies input before writing the response, so
           the same buffer receives res_blocks and DLEN becomes
           res_len. ARPSTAT receives the NDR pnio_status. */
        9..=11 | 14 => {
            let vt = match op {
                9 => PN_SVC_ARCONN,
                10 => PN_SVC_ARWRITE,
                11 => PN_SVC_ARCTRL,
                _ => PN_SVC_ARREAD,
            };
            let blk_len = ru32(this.add(PN_DLEN)).min(512);
            let mut res_len: u32 = 0;
            let mut pstat: u32 = 0;
            let rc = if op == 9 {
                /* pn_ar_connect(svc, port, dev*, act*, ar*, blocks,
                   blen, res*, res_cap, res_len*, pnio_status*) */
                let f: unsafe extern "C" fn(
                    *const u8, u32, *const u8, *const u8, *const u8,
                    *const u8, u32, *mut u8, u32, *mut u32, *mut u32,
                ) -> i32 = core::mem::transmute(
                    (svc.add(vt) as *const usize).read(),
                );
                f(
                    svc, port,
                    this.add(PN_DEVOBJ), this.add(PN_ACTUUID),
                    this.add(PN_ARUUID),
                    this.add(PN_ARBLK), blk_len,
                    this.add(PN_ARBLK), 512, &mut res_len, &mut pstat,
                )
            } else {
                /* pn_ar_write/control/read(svc, port, blocks, blen,
                   res*, res_cap, res_len*, pnio_status*) */
                let f: unsafe extern "C" fn(
                    *const u8, u32, *const u8, u32, *mut u8, u32,
                    *mut u32, *mut u32,
                ) -> i32 = core::mem::transmute(
                    (svc.add(vt) as *const usize).read(),
                );
                f(
                    svc, port,
                    this.add(PN_ARBLK), blk_len,
                    this.add(PN_ARBLK), 512, &mut res_len, &mut pstat,
                )
            };
            wu32(this.add(PN_DLEN), res_len);
            wu32(this.add(PN_ARPSTAT), pstat);
            set_rc(rc);
        }
        12 => {
            /* pn_ar_service(svc, port, timeout_ms, ar_state*) -> i32:
               pumps one CM datagram; STATUS 1 = AppReady consumed. */
            let f: unsafe extern "C" fn(*const u8, u32, u32, *mut u32) -> i32 =
                core::mem::transmute(
                    (svc.add(PN_SVC_ARSRV) as *const usize).read(),
                );
            let rc = f(svc, port, ru32(this.add(PN_XID)),
                       this.add(PN_ARSTATE) as *mut u32);
            set_rc(rc);
        }
        13 => {
            /* pn_ar_state(svc, port, ar_state*, session_key*,
               ar_uuid*) -> i32 */
            let f: unsafe extern "C" fn(
                *const u8, u32, *mut u32, *mut u16, *mut u8,
            ) -> i32 = core::mem::transmute(
                (svc.add(PN_SVC_ARSTATE) as *const usize).read(),
            );
            let mut key: u16 = 0;
            let rc = f(svc, port,
                       this.add(PN_ARSTATE) as *mut u32,
                       &mut key, this.add(PN_ARUUID));
            wu32(this.add(PN_ARKEY), u32::from(key));
            set_rc(rc);
        }
        _ => {
            wi32(this.add(PN_STATUS), -1);
            wu32(this.add(PN_ERRID), 1);
        }
    }
    0
}
