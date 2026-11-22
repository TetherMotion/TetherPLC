//! tplc-rt builtin FB: event log.
use crate::*;

// ---- FB_EventLog builtin — tplc-eventlog journal bridge ----
// Member offsets (see builtin_fb_layout("FB_EventLog")).
pub(crate) const EV_OP: usize = 0;
pub(crate) const EV_ID: usize = 4;
pub(crate) const EV_ECLASS: usize = 8;
pub(crate) const EV_EVTID: usize = 12;
pub(crate) const EV_SEV: usize = 16;
pub(crate) const EV_ARGS: usize = 20;
pub(crate) const EV_STATE: usize = 24;
pub(crate) const EV_SEVO: usize = 28;
pub(crate) const EV_CLSO: usize = 32;
pub(crate) const EV_EVTIDO: usize = 36;
pub(crate) const EV_LIVE: usize = 40;
pub(crate) const EV_RAISED: usize = 44;
pub(crate) const EV_CLEARED: usize = 48;
pub(crate) const EV_CONFIRMED: usize = 52;
pub(crate) const EV_DROPPED: usize = 56;
pub(crate) const EV_STATUS: usize = 60;
pub(crate) const EV_ERRID: usize = 64;
pub(crate) const EV_TEXT_LEN: usize = 68;
pub(crate) const EV_TEXT: usize = 69;
pub(crate) const EV_TXTO_LEN: usize = 150;
pub(crate) const EV_TXTO: usize = 151;
pub(crate) const EV_TSTAMP: usize = 232;
pub(crate) const EV_CSTAMP: usize = 240;
pub(crate) const EV_FSTAMP: usize = 248;

// TplcEventlogService vtable: size@0 abi@4 host@8
pub(crate) const EV_SVC_RAISE: usize = 16;
pub(crate) const EV_SVC_CLEAR: usize = 24;
pub(crate) const EV_SVC_CONFIRM: usize = 32;
pub(crate) const EV_SVC_QUERY: usize = 40;
pub(crate) const EV_SVC_STATS: usize = 48;

pub(crate) unsafe fn eventlog_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-eventlog".as_ptr() as *const u8, 13) as *const u8
}

pub(crate) type FEvRaise = unsafe extern "C" fn(*const u8, u32, u32, u32, *const u8, u32, u32) -> i32;
pub(crate) type FEvId = unsafe extern "C" fn(*const u8, u32) -> i32;
pub(crate) type FEvQuery = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;
pub(crate) type FEvStats = unsafe extern "C" fn(*const u8, *mut u8) -> i32;

/// `__tplc_rt_FB_EventLog` — synchronous op dispatch over the
/// tplc-eventlog journal.  OP: 1=raise (-> ID), 2=clear,
/// 3=confirm, 4=query (-> STATE/SEVO/CLSO/EVTIDO/TXTO), 5=stats.
/// # Safety
/// `this` must point at a valid instance block of the
/// documented layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_EventLog(this: *mut u8, ctx: *mut u8) -> i32 {
    let op = ru32(this.add(EV_OP));
    wi32(this.add(EV_STATUS), 0);
    wu32(this.add(EV_ERRID), 0);
    let svc = eventlog_service(ctx);
    if svc.is_null() {
        wi32(this.add(EV_STATUS), -2);
        wu32(this.add(EV_ERRID), 99);
        return 0;
    }
    let set_rc = |rc: i32| {
        wi32(this.add(EV_STATUS), rc);
        wu32(this.add(EV_ERRID), (-rc).max(0) as u32);
    };
    match op {
        1 => {
            let mut buf = [0u8; 81];
            let n = uacstr(this, EV_TEXT_LEN, EV_TEXT, 80, buf.as_mut_ptr());
            let f: FEvRaise = core::mem::transmute(mbusfn(svc, EV_SVC_RAISE));
            let rc = f(svc, ru32(this.add(EV_ECLASS)), ru32(this.add(EV_EVTID)),
                       ru32(this.add(EV_SEV)), buf.as_ptr(), n,
                       ru32(this.add(EV_ARGS)));
            if rc > 0 {
                wu32(this.add(EV_ID), rc as u32);
                wi32(this.add(EV_STATUS), 0);
            } else {
                set_rc(rc);
            }
        }
        2 | 3 => {
            let off = if op == 2 { EV_SVC_CLEAR } else { EV_SVC_CONFIRM };
            let f: FEvId = core::mem::transmute(mbusfn(svc, off));
            set_rc(f(svc, ru32(this.add(EV_ID))));
        }
        4 => {
            /* TplcEventlogInfo is 136B — the old 112B buffer
               overflowed the stack on every query                 */
            let mut info = [0u8; 160];
            let f: FEvQuery = core::mem::transmute(mbusfn(svc, EV_SVC_QUERY));
            let rc = f(svc, ru32(this.add(EV_ID)), info.as_mut_ptr());
            wi32(this.add(EV_STATUS), rc);
            if rc == 0 {
                let rd = |off: usize| -> u32 {
                    let mut b = [0u8; 4];
                    core::ptr::copy_nonoverlapping(info.as_ptr().add(off), b.as_mut_ptr(), 4);
                    u32::from_le_bytes(b)
                };
                let rd64 = |off: usize| -> u64 {
                    let mut b = [0u8; 8];
                    core::ptr::copy_nonoverlapping(info.as_ptr().add(off), b.as_mut_ptr(), 8);
                    u64::from_le_bytes(b)
                };
                // TplcEventlogInfo: class@0 id@4 sev@8 state@12
                //   args@16 text@20 len@100 raised@104 cleared@112
                //   confirmed@120
                wu32(this.add(EV_CLSO), rd(0));
                wu32(this.add(EV_EVTIDO), rd(4));
                wu32(this.add(EV_SEVO), rd(8));
                wu32(this.add(EV_STATE), rd(12));
                wu64(this.add(EV_TSTAMP), rd64(104));
                wu64(this.add(EV_CSTAMP), rd64(112));
                wu64(this.add(EV_FSTAMP), rd64(120));
                let n = info[100].min(80) as usize;
                core::ptr::copy_nonoverlapping(info.as_ptr().add(20), this.add(EV_TXTO), n);
                *this.add(EV_TXTO_LEN) = n as u8;
            } else {
                wu32(this.add(EV_ERRID), (-rc).max(0) as u32);
            }
        }
        5 => {
            let mut st = [0u8; 48];
            let f: FEvStats = core::mem::transmute(mbusfn(svc, EV_SVC_STATS));
            let rc = f(svc, st.as_mut_ptr());
            wi32(this.add(EV_STATUS), rc);
            if rc == 0 {
                let rd = |off: usize| -> u32 {
                    let mut b = [0u8; 4];
                    core::ptr::copy_nonoverlapping(st.as_ptr().add(off), b.as_mut_ptr(), 4);
                    u32::from_le_bytes(b)
                };
                wu32(this.add(EV_LIVE), rd(0));
                wu32(this.add(EV_RAISED), rd(4));
                wu32(this.add(EV_CLEARED), rd(8));
                wu32(this.add(EV_CONFIRMED), rd(12));
                wu32(this.add(EV_DROPPED), rd(16));
            } else {
                wu32(this.add(EV_ERRID), (-rc).max(0) as u32);
            }
        }
        _ => {
            wi32(this.add(EV_STATUS), -1);
            wu32(this.add(EV_ERRID), 1);
        }
    }
    0
}
