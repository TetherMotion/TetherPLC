//! tplc-rt builtin FB: ec sync.
use crate::*;

// ---------------------------------------------------------------------------
// FB_EcSync — EtherCAT DC sync state over tplc-ecsync (Tether IO
// ec.dc.* params).  Synchronous op dispatch like FB_Mqtt.
// ---------------------------------------------------------------------------

// FB_EcSync member offsets (see builtin_fb_layout("FB_EcSync")).
pub(crate) const ES_OP: usize = 0;
pub(crate) const ES_ENABLE: usize = 4;
pub(crate) const ES_CONNECTED: usize = 8;
pub(crate) const ES_SYNCEN: usize = 12;
pub(crate) const ES_PARAMSOK: usize = 16;
pub(crate) const ES_LASTERR: usize = 20;
pub(crate) const ES_STATUS: usize = 24;
pub(crate) const ES_ERRID: usize = 28;
pub(crate) const ES_DIFFNS: usize = 32;   // f64
pub(crate) const ES_REFNS: usize = 40;    // u64

// tplc-ecsync service vtable offsets (TplcEcsyncService):
// struct_size@0 u32, abi@4 u32, host@8 ptr(8B), ec_status@16,
// ec_set_sync@24.
pub(crate) const ES_SVC_STATUS: usize = 16;
pub(crate) const ES_SVC_SET_SYNC: usize = 24;

/// service lookup for "tplc-ecsync" (11 chars)
pub(crate) unsafe fn ecsync_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-ecsync".as_ptr() as *const u8, 11) as *const u8
}

/// `__tplc_rt_FB_EcSync` — OP 1=status (polls ec.dc.* params),
/// OP 2=set_sync (ENABLE 0/1). STATUS carries the service return
/// code, ERRID its positive form. TplcEcsyncStatus is 32 bytes:
/// connected@0 sync_enabled@4 params_ok@8 last_error@12 (i32),
/// time_diff_ns@16 (f64), ref_time_ns@24 (u64).
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_EcSync(this: *mut u8, ctx: *mut u8) -> i32 {
    let op = ru32(this.add(ES_OP));
    wi32(this.add(ES_STATUS), 0);
    wu32(this.add(ES_ERRID), 0);
    let svc = ecsync_service(ctx);
    if svc.is_null() {
        wi32(this.add(ES_STATUS), -2);
        wu32(this.add(ES_ERRID), 99);
        return 0;
    }
    let set_rc = |rc: i32| {
        wi32(this.add(ES_STATUS), rc);
        wu32(this.add(ES_ERRID), (-rc).max(0) as u32);
    };
    match op {
        1 => {
            // TplcEcsyncStatus (32B)
            let mut st = [0u8; 32];
            let f: unsafe extern "C" fn(*const u8, *mut u8) -> i32 =
                core::mem::transmute((svc.add(ES_SVC_STATUS) as *const usize).read());
            let rc = f(svc, st.as_mut_ptr());
            set_rc(rc);
            let rd32 = |off: usize| -> i32 {
                let mut b = [0u8; 4];
                core::ptr::copy_nonoverlapping(st.as_ptr().add(off), b.as_mut_ptr(), 4);
                i32::from_le_bytes(b)
            };
            let rd64 = |off: usize| -> u64 {
                let mut b = [0u8; 8];
                core::ptr::copy_nonoverlapping(st.as_ptr().add(off), b.as_mut_ptr(), 8);
                u64::from_le_bytes(b)
            };
            wi32(this.add(ES_CONNECTED), rd32(0));
            wi32(this.add(ES_SYNCEN), rd32(4));
            wi32(this.add(ES_PARAMSOK), rd32(8));
            wi32(this.add(ES_LASTERR), rd32(12));
            // time_diff_ns f64 @16 -> store raw bits
            let mut db = [0u8; 8];
            core::ptr::copy_nonoverlapping(st.as_ptr().add(16), db.as_mut_ptr(), 8);
            core::ptr::copy_nonoverlapping(db.as_ptr(), this.add(ES_DIFFNS), 8);
            let r = rd64(24);
            core::ptr::copy_nonoverlapping(r.to_le_bytes().as_ptr(), this.add(ES_REFNS), 8);
        }
        2 => {
            let f: unsafe extern "C" fn(*const u8, u32) -> i32 =
                core::mem::transmute((svc.add(ES_SVC_SET_SYNC) as *const usize).read());
            set_rc(f(svc, ru32(this.add(ES_ENABLE))));
        }
        _ => {
            wi32(this.add(ES_STATUS), -1);
            wu32(this.add(ES_ERRID), 1);
        }
    }
    0
}
