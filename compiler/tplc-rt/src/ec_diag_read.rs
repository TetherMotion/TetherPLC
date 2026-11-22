//! tplc-rt builtin FB: ec diag read.
use crate::*;

// FB_EcDiagRead — EtherCAT CoE 0x10F3 diag history over
// tplc-ecdiag (Tether IO ec.coediag.* params).
// ---------------------------------------------------------------------------

// FB_EcDiagRead member offsets (see builtin_fb_layout("FB_EcDiagRead")).
pub(crate) const ED_OP: usize = 0;
pub(crate) const ED_SLAVE: usize = 4;
pub(crate) const ED_ACKVAL: usize = 8;
pub(crate) const ED_COUNT: usize = 12;
pub(crate) const ED_NEWEST: usize = 16;
pub(crate) const ED_ACKED: usize = 20;
pub(crate) const ED_NEWMSGS: usize = 24;
pub(crate) const ED_INFOFLAGS: usize = 28;
pub(crate) const ED_STATUS: usize = 32;
pub(crate) const ED_ERRID: usize = 36;
pub(crate) const ED_MSGTYPE: usize = 40; // u32[16]
pub(crate) const ED_ACKST: usize = 104;
pub(crate) const ED_TSMS: usize = 168;
pub(crate) const ED_CODE: usize = 232;
pub(crate) const ED_CODETYPE: usize = 296;
pub(crate) const ED_TEXTID: usize = 360;
pub(crate) const ED_MODULE: usize = 424;
pub(crate) const ED_CHANNEL: usize = 488;
pub(crate) const ED_MSGTEXT: usize = 552; // Str{63}[16], stride 64

// tplc-ecdiag service vtable offsets (TplcEcDiagService):
// struct_size@0 u32, abi@4 u32, host@8 ptr(8B), diag_read@16,
// diag_ack@24.
pub(crate) const ED_SVC_READ: usize = 16;
pub(crate) const ED_SVC_ACK: usize = 24;

pub(crate) const ED_MAX: usize = 16;
// TplcEcDiagEntry stride 104: ts_ns@0 u64, msg_type@8, ack@12,
// diag_code@16, code_type@20, text_id@24, module@28, channel@32,
// msg_text@36 char[64].
pub(crate) const ED_ENT_SIZE: usize = 104;

/// service lookup for "tplc-ecdiag" (11 chars)
pub(crate) unsafe fn ecdiag_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-ecdiag".as_ptr() as *const u8, 11) as *const u8
}

/// `__tplc_rt_FB_EcDiagRead` — OP 1=read diag history, OP 2=ack.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_EcDiagRead(this: *mut u8, ctx: *mut u8) -> i32 {
    let op = ru32(this.add(ED_OP));
    wi32(this.add(ED_STATUS), 0);
    wu32(this.add(ED_ERRID), 0);
    let svc = ecdiag_service(ctx);
    if svc.is_null() {
        wi32(this.add(ED_STATUS), -2);
        wu32(this.add(ED_ERRID), 99);
        return 0;
    }
    let set_rc = |rc: i32| {
        wi32(this.add(ED_STATUS), rc);
        wu32(this.add(ED_ERRID), (-rc).max(0) as u32);
    };
    match op {
        1 => {
            // entries[16] + info[24] on stack
            let mut ents = [0u8; ED_MAX * ED_ENT_SIZE];
            let mut info = [0u8; 24];
            let mut count: u32 = 0;
            let f: unsafe extern "C" fn(
                *const u8,
                u32,
                *mut u8,
                u32,
                *mut u32,
                *mut u8,
            ) -> i32 = core::mem::transmute(
                (svc.add(ED_SVC_READ) as *const usize).read(),
            );
            let rc = f(
                svc,
                ru32(this.add(ED_SLAVE)),
                ents.as_mut_ptr(),
                ED_MAX as u32,
                &mut count,
                info.as_mut_ptr(),
            );
            set_rc(rc);
            wu32(this.add(ED_COUNT), count);
            // TplcEcDiagInfo: newest@0 acked@4 newmsgs@8 flags@12
            wu32(this.add(ED_NEWEST), ru32(info.as_ptr()));
            wu32(this.add(ED_ACKED), ru32(info.as_ptr().add(4)));
            wu32(this.add(ED_NEWMSGS), ru32(info.as_ptr().add(8)));
            wu32(this.add(ED_INFOFLAGS), ru32(info.as_ptr().add(12)));
            let n = (count as usize).min(ED_MAX);
            for i in 0..n {
                let e = ents.as_ptr().add(i * ED_ENT_SIZE);
                let ebase = |off: usize| e.add(off);
                let rd_e32 = |off: usize| -> u32 {
                    let mut b = [0u8; 4];
                    core::ptr::copy_nonoverlapping(ebase(off), b.as_mut_ptr(), 4);
                    u32::from_le_bytes(b)
                };
                let arr = |base: usize| base + i * 4;
                wu32(this.add(arr(ED_MSGTYPE)), rd_e32(8));
                wu32(this.add(arr(ED_ACKST)), rd_e32(12));
                wu32(this.add(arr(ED_TSMS)), (ru64(e) / 1_000_000) as u32);
                wu32(this.add(arr(ED_CODE)), rd_e32(16));
                wu32(this.add(arr(ED_CODETYPE)), rd_e32(20));
                wu32(this.add(arr(ED_TEXTID)), rd_e32(24));
                wu32(this.add(arr(ED_MODULE)), rd_e32(28));
                wu32(this.add(arr(ED_CHANNEL)), rd_e32(32));
                // msg_text char[64] -> Str blob: u8 len + bytes
                let mt = this.add(ED_MSGTEXT + i * 64);
                let mut len = 0usize;
                while len < 63 && *ebase(36 + len) != 0 {
                    len += 1;
                }
                *mt = len as u8;
                core::ptr::copy_nonoverlapping(ebase(36), mt.add(1), len);
            }
        }
        2 => {
            let f: unsafe extern "C" fn(*const u8, u32, u32) -> i32 =
                core::mem::transmute((svc.add(ED_SVC_ACK) as *const usize).read());
            set_rc(f(svc, ru32(this.add(ED_SLAVE)), ru32(this.add(ED_ACKVAL))));
        }
        _ => {
            wi32(this.add(ED_STATUS), -1);
            wu32(this.add(ED_ERRID), 1);
        }
    }
    0
}
