//! tplc-rt builtin FB: dmx.
use crate::*;

// ---------------------------------------------------------------------------
// FB_Dmx — DMX512 output over tplc-dmx (opt-in serial port).
// ---------------------------------------------------------------------------

// FB_Dmx member offsets (see builtin_fb_layout("FB_Dmx")). Strings
// are len-prefixed u8len+data.
pub(crate) const DX_OP: usize = 0;
pub(crate) const DX_HANDLE: usize = 4;
pub(crate) const DX_PATH_LEN: usize = 8;
pub(crate) const DX_PATH: usize = 9;
pub(crate) const DX_BAUD: usize = 136;
pub(crate) const DX_CHANNEL: usize = 140;
pub(crate) const DX_VALUE: usize = 144;
pub(crate) const DX_START: usize = 148;
pub(crate) const DX_COUNT: usize = 152;
pub(crate) const DX_VALUES_LEN: usize = 156;
pub(crate) const DX_VALUES: usize = 157;
pub(crate) const DX_STATE: usize = 412;
pub(crate) const DX_FRAMES: usize = 416;
pub(crate) const DX_LASTERR: usize = 420;
pub(crate) const DX_STATUS: usize = 424;
pub(crate) const DX_ERRID: usize = 428;

// tplc-dmx service vtable offsets (TplcDmxService):
// struct_size@0 u32, abi@4 u32, host@8 ptr(8B), then fn ptrs.
pub(crate) const DX_SVC_OPEN: usize = 16;
pub(crate) const DX_SVC_CLOSE: usize = 24;
pub(crate) const DX_SVC_STATUS: usize = 32;
pub(crate) const DX_SVC_SETCHANNEL: usize = 40;
pub(crate) const DX_SVC_SETCHANNELS: usize = 48;
pub(crate) const DX_SVC_SENDFRAME: usize = 56;

/// service lookup for "tplc-dmx" (8 chars)
pub(crate) unsafe fn dmx_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-dmx".as_ptr() as *const u8, 8) as *const u8
}

#[inline]
pub(crate) unsafe fn dxfn(svc: *const u8, off: usize) -> usize {
    (svc.add(off) as *const usize).read()
}

pub(crate) type FDmxOpen = unsafe extern "C" fn(*const u8, *const u8, u32, *mut u32) -> i32;
pub(crate) type FDmxClose = unsafe extern "C" fn(*const u8, u32) -> i32;
pub(crate) type FDmxStatus = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;
pub(crate) type FDmxSetChannel = unsafe extern "C" fn(*const u8, u32, u32, u32) -> i32;
pub(crate) type FDmxSetChannels = unsafe extern "C" fn(*const u8, u32, u32, *const u8, u32) -> i32;
pub(crate) type FDmxSendFrame = unsafe extern "C" fn(*const u8, u32) -> i32;

/// `__tplc_rt_FB_Dmx` — synchronous op dispatch over the tplc-dmx
/// DMX512 output service. OP: 1=open (PATH -> HANDLE), 2=close,
/// 3=status (-> STATE/FRAMES/LASTERR), 4=set_channel (CHANNEL/VALUE),
/// 5=set_channels (START/COUNT/VALUES), 6=send_frame. STATUS carries
/// the service return code, ERRID its positive form.  The serial
/// port is only opened when the surface is used.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_Dmx(this: *mut u8, ctx: *mut u8) -> i32 {
    let op = ru32(this.add(DX_OP));
    wi32(this.add(DX_STATUS), 0);
    wu32(this.add(DX_ERRID), 0);
    let svc = dmx_service(ctx);
    if svc.is_null() {
        wi32(this.add(DX_STATUS), -2);
        wu32(this.add(DX_ERRID), 99);
        return 0;
    }
    let set_rc = |rc: i32| {
        wi32(this.add(DX_STATUS), rc);
        wu32(this.add(DX_ERRID), (-rc).max(0) as u32);
    };
    match op {
        1 => {
            let mut buf = [0u8; 128];
            uacstr(this, DX_PATH_LEN, DX_PATH, 127, buf.as_mut_ptr());
            let f: FDmxOpen = core::mem::transmute(dxfn(svc, DX_SVC_OPEN));
            let mut h: u32 = 0;
            let rc = f(svc, buf.as_ptr(), ru32(this.add(DX_BAUD)), &mut h);
            set_rc(rc);
            if rc == 0 {
                wu32(this.add(DX_HANDLE), h);
            }
        }
        2 => {
            let f: FDmxClose = core::mem::transmute(dxfn(svc, DX_SVC_CLOSE));
            set_rc(f(svc, ru32(this.add(DX_HANDLE))));
        }
        3 => {
            // TplcDmxStatus (48B): state@0 frames@4 last_error@8
            let mut st = [0u8; 48];
            let f: FDmxStatus = core::mem::transmute(dxfn(svc, DX_SVC_STATUS));
            let rc = f(svc, ru32(this.add(DX_HANDLE)), st.as_mut_ptr());
            wi32(this.add(DX_STATUS), rc);
            if rc == 0 {
                let rd = |off: usize| -> i32 {
                    let mut b = [0u8; 4];
                    core::ptr::copy_nonoverlapping(st.as_ptr().add(off), b.as_mut_ptr(), 4);
                    i32::from_le_bytes(b)
                };
                wi32(this.add(DX_STATE), rd(0));
                wu32(this.add(DX_FRAMES), rd(4) as u32);
                wi32(this.add(DX_LASTERR), rd(8));
            } else {
                wu32(this.add(DX_ERRID), (-rc).max(0) as u32);
            }
        }
        4 => {
            let f: FDmxSetChannel = core::mem::transmute(dxfn(svc, DX_SVC_SETCHANNEL));
            set_rc(f(svc, ru32(this.add(DX_HANDLE)),
                     ru32(this.add(DX_CHANNEL)), ru32(this.add(DX_VALUE))));
        }
        5 => {
            let n = (*this.add(DX_VALUES_LEN) as usize).min(255);
            let f: FDmxSetChannels = core::mem::transmute(dxfn(svc, DX_SVC_SETCHANNELS));
            set_rc(f(svc, ru32(this.add(DX_HANDLE)),
                     ru32(this.add(DX_START)),
                     this.add(DX_VALUES), n.min(ru32(this.add(DX_COUNT)) as usize) as u32));
        }
        6 => {
            let f: FDmxSendFrame = core::mem::transmute(dxfn(svc, DX_SVC_SENDFRAME));
            set_rc(f(svc, ru32(this.add(DX_HANDLE))));
        }
        _ => {
            wi32(this.add(DX_STATUS), -1);
            wu32(this.add(DX_ERRID), 1);
        }
    }
    0
}
