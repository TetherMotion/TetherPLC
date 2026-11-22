//! tplc-rt builtin FB: io link.
use crate::*;

// FB_IoLink — IO-Link master data-link over tplc-iolink (IEC 61131-9
// SDCI wire protocol; per-port byte transports configured via
// TPLC_IOLINK_IO).
// ---------------------------------------------------------------------------

// FB_IoLink member offsets (see builtin_fb_layout("FB_IoLink")).
pub(crate) const IL_OP: usize = 0;
pub(crate) const IL_PORT: usize = 4;
pub(crate) const IL_MSEQ: usize = 8;
pub(crate) const IL_ADDR: usize = 12;
pub(crate) const IL_DLEN: usize = 16;
pub(crate) const IL_STATUS: usize = 20;
pub(crate) const IL_ERRID: usize = 24;
pub(crate) const IL_CKSFLAGS: usize = 28;
pub(crate) const IL_IDENT: usize = 32; // u8[16]
pub(crate) const IL_PDIN: usize = 48; // u8[40]
pub(crate) const IL_PDOUT: usize = 88; // u8[32]
pub(crate) const IL_OD: usize = 120; // u8[8]
pub(crate) const IL_ISDUERR: usize = 128; // u32
pub(crate) const IL_ISDU: usize = 132; // u8[240]
pub(crate) const IL_DIAG: usize = 372; // u8[19] event memory
pub(crate) const IL_ALPDO: usize = 392; // u32 negotiated PD-out octets
pub(crate) const IL_ALPDI: usize = 396; // u32 negotiated PD-in octets

// tplc-iolink service vtable offsets (TplcIoLinkService):
// struct_size@0 u32, abi@4 u32, host@8 ptr(8B),
// il_state@16, il_open@24, il_close@32, il_ident@40,
// il_pd_exchange@48, il_isdu_read@56, il_isdu_write@64, il_port_info@72,
// il_isdu_read_ex@80, il_isdu_write_ex@88, il_master_cmd@96,
// il_diag_read@104, il_al_neg@112.
pub(crate) const IL_SVC_STATE: usize = 16;
pub(crate) const IL_SVC_OPEN: usize = 24;
pub(crate) const IL_SVC_CLOSE: usize = 32;
pub(crate) const IL_SVC_IDENT: usize = 40;
pub(crate) const IL_SVC_PDEX: usize = 48;
pub(crate) const IL_SVC_ISDU_R: usize = 56;
pub(crate) const IL_SVC_ISDU_W: usize = 64;
pub(crate) const IL_SVC_ISDU_RX: usize = 80;
pub(crate) const IL_SVC_ISDU_WX: usize = 88;
pub(crate) const IL_SVC_MCMD: usize = 96;
pub(crate) const IL_SVC_DIAG: usize = 104;
pub(crate) const IL_SVC_ALNEG: usize = 112;

/// service lookup for "tplc-iolink" (11 chars)
pub(crate) unsafe fn iolink_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-iolink".as_ptr() as *const u8, 11) as *const u8
}

/// `__tplc_rt_FB_IoLink` — OP 1=open, 2=close, 3=state, 4=ident,
/// 5=pd_exchange, 6=isdu_read, 7=isdu_write, 8=master_cmd,
/// 9=isdu_read_ex, 10=isdu_write_ex.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_IoLink(this: *mut u8, ctx: *mut u8) -> i32 {
    let op = ru32(this.add(IL_OP));
    wi32(this.add(IL_STATUS), 0);
    wu32(this.add(IL_ERRID), 0);
    wu32(this.add(IL_CKSFLAGS), 0);
    let svc = iolink_service(ctx);
    if svc.is_null() {
        wi32(this.add(IL_STATUS), -3);
        wu32(this.add(IL_ERRID), 99);
        return 0;
    }
    let set_rc = |rc: i32| {
        wi32(this.add(IL_STATUS), rc);
        wu32(this.add(IL_ERRID), (-rc).max(0) as u32);
    };
    let port = ru32(this.add(IL_PORT));
    match op {
        1 => {
            let f: unsafe extern "C" fn(*const u8, u32) -> i32 =
                core::mem::transmute(
                    (svc.add(IL_SVC_OPEN) as *const usize).read(),
                );
            set_rc(f(svc, port));
        }
        2 => {
            let f: unsafe extern "C" fn(*const u8, u32) -> i32 =
                core::mem::transmute(
                    (svc.add(IL_SVC_CLOSE) as *const usize).read(),
                );
            set_rc(f(svc, port));
        }
        3 => {
            let f: unsafe extern "C" fn(*const u8) -> i32 =
                core::mem::transmute(
                    (svc.add(IL_SVC_STATE) as *const usize).read(),
                );
            set_rc(f(svc));
        }
        4 => {
            /* il_ident(svc, port, out, cap, out_len*) -> i32 */
            let f: unsafe extern "C" fn(
                *const u8, u32, *mut u8, u32, *mut u32,
            ) -> i32 = core::mem::transmute(
                (svc.add(IL_SVC_IDENT) as *const usize).read(),
            );
            let mut len: u32 = 0;
            let rc = f(svc, port, this.add(IL_IDENT), 16, &mut len);
            wu32(this.add(IL_DLEN), len);
            set_rc(rc);
        }
        5 => {
            /* il_pd_exchange(svc, port, mseq, pdo, pdo_len, pdi, cap,
               pdi_len*, cks_flags*) -> i32 */
            let f: unsafe extern "C" fn(
                *const u8, u32, u32, *const u8, u32, *mut u8, u32,
                *mut u32, *mut u32,
            ) -> i32 = core::mem::transmute(
                (svc.add(IL_SVC_PDEX) as *const usize).read(),
            );
            let pdo_len = ru32(this.add(IL_DLEN)).min(32);
            let mut pdi_len: u32 = 0;
            let mut cks: u32 = 0;
            let rc = f(
                svc,
                port,
                ru32(this.add(IL_MSEQ)),
                this.add(IL_PDOUT),
                pdo_len,
                this.add(IL_PDIN),
                40,
                &mut pdi_len,
                &mut cks,
            );
            wu32(this.add(IL_DLEN), pdi_len);
            wu32(this.add(IL_CKSFLAGS), cks);
            set_rc(rc);
        }
        6 => {
            /* il_isdu_read(svc, port, addr u8, od, od_len, resp,
               cap, resp_len*) -> i32 */
            let f: unsafe extern "C" fn(
                *const u8, u32, u8, *const u8, u32, *mut u8, u32,
                *mut u32,
            ) -> i32 = core::mem::transmute(
                (svc.add(IL_SVC_ISDU_R) as *const usize).read(),
            );
            let od_len = ru32(this.add(IL_DLEN)).min(8);
            let mut resp_len: u32 = 0;
            let rc = f(
                svc,
                port,
                ru32(this.add(IL_ADDR)) as u8,
                this.add(IL_OD),
                od_len,
                this.add(IL_OD),
                8,
                &mut resp_len,
            );
            wu32(this.add(IL_DLEN), resp_len);
            set_rc(rc);
        }
        7 => {
            /* il_isdu_write(svc, port, addr u8, od, od_len) -> i32 */
            let f: unsafe extern "C" fn(
                *const u8, u32, u8, *const u8, u32,
            ) -> i32 = core::mem::transmute(
                (svc.add(IL_SVC_ISDU_W) as *const usize).read(),
            );
            let od_len = ru32(this.add(IL_DLEN)).min(8);
            set_rc(f(
                svc,
                port,
                ru32(this.add(IL_ADDR)) as u8,
                this.add(IL_OD),
                od_len,
            ));
        }
        8 => {
            /* il_master_cmd(svc, port, cmd u8) -> i32 */
            let f: unsafe extern "C" fn(*const u8, u32, u8) -> i32 =
                core::mem::transmute(
                    (svc.add(IL_SVC_MCMD) as *const usize).read(),
                );
            set_rc(f(
                svc,
                port,
                ru32(this.add(IL_ADDR)) as u8,
            ));
        }
        9 | 10 => {
            /* segmented ISDU (A.5):
               il_isdu_read_ex(svc, port, index u16, sub u8, data,
                 cap, len*, err*) / il_isdu_write_ex(svc, port,
                 index, sub, data, len, err*) -> i32.
               MSEQ carries the 16-bit index, ADDR the subindex,
               ISDU the payload (write) / response (read), DLEN the
               payload length, ISDUERR the device ErrorType. */
            let index = (ru32(this.add(IL_MSEQ)) & 0xFFFF) as u16;
            let sub = (ru32(this.add(IL_ADDR)) & 0xFF) as u8;
            let mut err: u32 = 0;
            let rc = if op == 9 {
                let f: unsafe extern "C" fn(
                    *const u8, u32, u16, u8, *mut u8, u32,
                    *mut u32, *mut u32,
                ) -> i32 = core::mem::transmute(
                    (svc.add(IL_SVC_ISDU_RX) as *const usize).read(),
                );
                let mut len: u32 = 0;
                let rc = f(
                    svc, port, index, sub,
                    this.add(IL_ISDU), 240, &mut len, &mut err,
                );
                wu32(this.add(IL_DLEN), len);
                rc
            } else {
                let f: unsafe extern "C" fn(
                    *const u8, u32, u16, u8, *const u8, u32,
                    *mut u32,
                ) -> i32 = core::mem::transmute(
                    (svc.add(IL_SVC_ISDU_WX) as *const usize).read(),
                );
                f(
                    svc, port, index, sub,
                    this.add(IL_ISDU),
                    ru32(this.add(IL_DLEN)).min(232),
                    &mut err,
                )
            };
            wu32(this.add(IL_ISDUERR), err);
            set_rc(rc);
        }
        11 => {
            /* il_diag_read(svc, port, mem, cap, len*) -> i32:
               diagnosis-channel event memory (Table 58) + ack.
               DIAG receives up to 19 octets, DLEN the count. */
            let f: unsafe extern "C" fn(
                *const u8, u32, *mut u8, u32, *mut u32,
            ) -> i32 = core::mem::transmute(
                (svc.add(IL_SVC_DIAG) as *const usize).read(),
            );
            let mut len: u32 = 0;
            let rc = f(
                svc, port, this.add(IL_DIAG), 19, &mut len,
            );
            wu32(this.add(IL_DLEN), len);
            set_rc(rc);
        }
        12 => {
            /* il_al_neg(svc, port, pdo_req, pdi_req, pdo*, pdi*)
               -> i32: AL negotiation — MSEQ=pdo request, ADDR=pdi
               request (0 = device max), results in ALPDO/ALPDI. */
            let f: unsafe extern "C" fn(
                *const u8, u32, u32, u32, *mut u32, *mut u32,
            ) -> i32 = core::mem::transmute(
                (svc.add(IL_SVC_ALNEG) as *const usize).read(),
            );
            set_rc(f(
                svc, port,
                ru32(this.add(IL_MSEQ)),
                ru32(this.add(IL_ADDR)),
                this.add(IL_ALPDO) as *mut u32,
                this.add(IL_ALPDI) as *mut u32,
            ));
        }
        _ => {
            wi32(this.add(IL_STATUS), -1);
            wu32(this.add(IL_ERRID), 1);
        }
    }
    0
}
