//! tplc-rt builtin FB: dyn mem.
use crate::*;

// ---- FB_DynMem builtin — tplc-dynmem bounded-arena bridge ----
// Member offsets (see builtin_fb_layout("FB_DynMem")).
pub(crate) const DM_OP: usize = 0;
pub(crate) const DM_ID: usize = 4;
pub(crate) const DM_SIZE: usize = 8;
pub(crate) const DM_RESET: usize = 12;
pub(crate) const DM_PRESERVE: usize = 16;
pub(crate) const DM_OFFSET: usize = 20;
pub(crate) const DM_BYTE: usize = 24;
pub(crate) const DM_LIVEBLKS: usize = 28;
pub(crate) const DM_LIVEBYTES: usize = 32;
pub(crate) const DM_TOTALALLOCS: usize = 36;
pub(crate) const DM_TOTALFREES: usize = 40;
pub(crate) const DM_ARENABYTES: usize = 44;
pub(crate) const DM_ARENAUSED: usize = 48;
pub(crate) const DM_STATUS: usize = 52;
pub(crate) const DM_ERRID: usize = 56;

// TplcDynmemService vtable (tplc_dynmem.h): size@0 abi@4 host@8
pub(crate) const DM_SVC_ALLOC: usize = 16;
pub(crate) const DM_SVC_FREE: usize = 24;
pub(crate) const DM_SVC_RESIZE: usize = 32;
pub(crate) const DM_SVC_READ: usize = 40;
pub(crate) const DM_SVC_WRITE: usize = 48;
pub(crate) const DM_SVC_STATS: usize = 56;

pub(crate) unsafe fn dynmem_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-dynmem".as_ptr() as *const u8, 11) as *const u8
}

pub(crate) type FDmAlloc = unsafe extern "C" fn(*const u8, u32, u32) -> i32;
pub(crate) type FDmFree = unsafe extern "C" fn(*const u8, u32) -> i32;
pub(crate) type FDmResize = unsafe extern "C" fn(*const u8, u32, u32, u32, u32) -> i32;
pub(crate) type FDmRw = unsafe extern "C" fn(*const u8, u32, u32, u32) -> i32;
pub(crate) type FDmStats = unsafe extern "C" fn(*const u8, *mut u8) -> i32;

/// `__tplc_rt_FB_DynMem` — synchronous op dispatch over the
/// tplc-dynmem arena service.  OP: 1=alloc (SIZE/RESET -> ID),
/// 2=free (ID), 3=resize (ID/SIZE/PRESERVE/RESET),
/// 4=read (ID/OFFSET -> BYTE), 5=write (ID/OFFSET/BYTE),
/// 6=stats -> counters.
/// # Safety
/// `this` must point at a valid instance block of the
/// documented layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_DynMem(this: *mut u8, ctx: *mut u8) -> i32 {
    let op = ru32(this.add(DM_OP));
    wi32(this.add(DM_STATUS), 0);
    wu32(this.add(DM_ERRID), 0);
    let svc = dynmem_service(ctx);
    if svc.is_null() {
        wi32(this.add(DM_STATUS), -2);
        wu32(this.add(DM_ERRID), 99);
        return 0;
    }
    let set_rc = |rc: i32| {
        wi32(this.add(DM_STATUS), rc);
        wu32(this.add(DM_ERRID), (-rc).max(0) as u32);
    };
    match op {
        1 => {
            let f: FDmAlloc = core::mem::transmute(mbusfn(svc, DM_SVC_ALLOC));
            let rc = f(svc, ru32(this.add(DM_SIZE)), ru32(this.add(DM_RESET)));
            if rc > 0 {
                wu32(this.add(DM_ID), rc as u32);
                wi32(this.add(DM_STATUS), 0);
            } else {
                set_rc(rc);
            }
        }
        2 => {
            let f: FDmFree = core::mem::transmute(mbusfn(svc, DM_SVC_FREE));
            set_rc(f(svc, ru32(this.add(DM_ID))));
        }
        3 => {
            let f: FDmResize = core::mem::transmute(mbusfn(svc, DM_SVC_RESIZE));
            set_rc(f(svc, ru32(this.add(DM_ID)), ru32(this.add(DM_SIZE)),
                     ru32(this.add(DM_PRESERVE)), ru32(this.add(DM_RESET))));
        }
        4 => {
            let f: FDmRw = core::mem::transmute(mbusfn(svc, DM_SVC_READ));
            let rc = f(svc, ru32(this.add(DM_ID)), ru32(this.add(DM_OFFSET)), 0);
            if rc >= 0 {
                wu32(this.add(DM_BYTE), rc as u32);
                wi32(this.add(DM_STATUS), 0);
            } else {
                set_rc(rc);
            }
        }
        5 => {
            let f: FDmRw = core::mem::transmute(mbusfn(svc, DM_SVC_WRITE));
            set_rc(f(svc, ru32(this.add(DM_ID)), ru32(this.add(DM_OFFSET)),
                     ru32(this.add(DM_BYTE))));
        }
        6 => {
            let mut st = [0u8; 56];
            let f: FDmStats = core::mem::transmute(mbusfn(svc, DM_SVC_STATS));
            let rc = f(svc, st.as_mut_ptr());
            wi32(this.add(DM_STATUS), rc);
            if rc == 0 {
                let rd = |off: usize| -> u32 {
                    let mut b = [0u8; 4];
                    core::ptr::copy_nonoverlapping(st.as_ptr().add(off), b.as_mut_ptr(), 4);
                    u32::from_le_bytes(b)
                };
                wu32(this.add(DM_LIVEBLKS), rd(0));
                wu32(this.add(DM_LIVEBYTES), rd(4));
                wu32(this.add(DM_TOTALALLOCS), rd(8));
                wu32(this.add(DM_TOTALFREES), rd(12));
                wu32(this.add(DM_ARENABYTES), rd(16));
                wu32(this.add(DM_ARENAUSED), rd(20));
            } else {
                wu32(this.add(DM_ERRID), (-rc).max(0) as u32);
            }
        }
        _ => {
            wi32(this.add(DM_STATUS), -1);
            wu32(this.add(DM_ERRID), 1);
        }
    }
    0
}
