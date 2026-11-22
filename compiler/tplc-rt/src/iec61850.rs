//! tplc-rt builtin FB: iec61850.
use crate::*;

// ---------------------------------------------------------------------------
// FB_Iec61850 — IEC 61850 MMS client over tplc-iec61850 (libiec61850).
// ---------------------------------------------------------------------------

// FB_Iec61850 member offsets (see builtin_fb_layout("FB_Iec61850")).
// Strings are len-prefixed: HOST u8len@8+data@9 cap63, DOMAIN
// u8len@76+data@77 cap63, ITEM u8len@140+data@141 cap127, VAL
// u8len@268+data@269 cap255.
pub(crate) const IEC_OP: usize = 0;
pub(crate) const IEC_HANDLE: usize = 4;
pub(crate) const IEC_HOST_LEN: usize = 8;
pub(crate) const IEC_HOST: usize = 9;
pub(crate) const IEC_PORT: usize = 72;
pub(crate) const IEC_DOMAIN_LEN: usize = 76;
pub(crate) const IEC_DOMAIN: usize = 77;
pub(crate) const IEC_ITEM_LEN: usize = 140;
pub(crate) const IEC_ITEM: usize = 141;
pub(crate) const IEC_VAL_LEN: usize = 268;
pub(crate) const IEC_VAL: usize = 269;
pub(crate) const IEC_STATE: usize = 524;
pub(crate) const IEC_INVOKE: usize = 528;
pub(crate) const IEC_LASTERR: usize = 532;
pub(crate) const IEC_STATUS: usize = 536;
pub(crate) const IEC_ERRID: usize = 540;
// RCB members (appended): RCBREF u8len@544+data@545 cap127, FLAGS@672,
// RPTENA@676, RESV@680, BUFFERED@684, RPTID u8len@688+data@689 cap31,
// DATASET u8len@720+data@721 cap63, CONFREV@784, BUFTM@788,
// INTGPD@792, SQNUM@796, OPTFLDS@800, TRGOPS@804, RCNT@808.
pub(crate) const IEC_RCBREF_LEN: usize = 544;
pub(crate) const IEC_RCBREF: usize = 545;
pub(crate) const IEC_FLAGS: usize = 672;
pub(crate) const IEC_RPTENA: usize = 676;
pub(crate) const IEC_RESV: usize = 680;
pub(crate) const IEC_BUFFERED: usize = 684;
pub(crate) const IEC_RPTID_LEN: usize = 688;
pub(crate) const IEC_RPTID: usize = 689;
pub(crate) const IEC_DATASET_LEN: usize = 720;
pub(crate) const IEC_DATASET: usize = 721;
pub(crate) const IEC_CONFREV: usize = 784;
pub(crate) const IEC_BUFTM: usize = 788;
pub(crate) const IEC_INTGPD: usize = 792;
pub(crate) const IEC_SQNUM: usize = 796;
pub(crate) const IEC_OPTFLDS: usize = 800;
pub(crate) const IEC_TRGOPS: usize = 804;
pub(crate) const IEC_RCNT: usize = 808;
// Control members (appended): CTRLREF u8len@812+data@813 cap127,
// CTLVAL u8len@940+data@941 cap63, CMODE@1004, CTLINVOKE@1008,
// CTLSUCCESS@1012, CTLERR@1016, CTLTYPE@1020, CTLNUM@1024,
// CTLPEND@1028.
pub(crate) const IEC_CTRLREF_LEN: usize = 812;
pub(crate) const IEC_CTRLREF: usize = 813;
pub(crate) const IEC_CTLVAL_LEN: usize = 940;
pub(crate) const IEC_CTLVAL: usize = 941;
pub(crate) const IEC_CMODE: usize = 1004;
pub(crate) const IEC_CTLINVOKE: usize = 1008;
pub(crate) const IEC_CTLSUCCESS: usize = 1012;
pub(crate) const IEC_CTLERR: usize = 1016;
pub(crate) const IEC_CTLTYPE: usize = 1020;
pub(crate) const IEC_CTLNUM: usize = 1024;
pub(crate) const IEC_CTLPEND: usize = 1028;
// RCB snapshot extras (appended): GI@1032, PURGEBUF@1036,
// RESVTMS@1040, ENTRYID@1044.
pub(crate) const IEC_GI: usize = 1032;
pub(crate) const IEC_PURGEBUF: usize = 1036;
pub(crate) const IEC_RESVTMS: usize = 1040;
pub(crate) const IEC_ENTRYID: usize = 1044;
// Data-set members (appended): DSREF u8len@1048+data@1049 cap127,
// DSCOUNT@1176.
pub(crate) const IEC_DSREF_LEN: usize = 1048;
pub(crate) const IEC_DSREF: usize = 1049;
pub(crate) const IEC_DSCOUNT: usize = 1176;

// tplc-iec61850 service vtable offsets (TplcIec61850Service):
// struct_size@0 u32, abi@4 u32, host@8 ptr(8B), then fn ptrs.
pub(crate) const IEC_SVC_CONNECT: usize = 16;
pub(crate) const IEC_SVC_CLOSE: usize = 24;
pub(crate) const IEC_SVC_STATUS: usize = 32;
pub(crate) const IEC_SVC_READ: usize = 40;
pub(crate) const IEC_SVC_WRITE: usize = 48;
pub(crate) const IEC_SVC_GETNAMELIST: usize = 56;
pub(crate) const IEC_SVC_RCB_GET: usize = 64;
pub(crate) const IEC_SVC_RCB_SET: usize = 72;
pub(crate) const IEC_SVC_REPORT_POLL: usize = 80;
pub(crate) const IEC_SVC_CONTROL: usize = 88;
pub(crate) const IEC_SVC_CONTROL_POLL: usize = 96;
pub(crate) const IEC_SVC_DS_READ: usize = 104;
pub(crate) const IEC_SVC_DS_DIRECTORY: usize = 112;
pub(crate) const IEC_SVC_DS_WRITE: usize = 120;
pub(crate) const IEC_SVC_DS_CREATE: usize = 128;
pub(crate) const IEC_SVC_DS_DELETE: usize = 136;

/// service lookup for "tplc-iec61850" (12 chars)
pub(crate) unsafe fn iec_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-iec61850".as_ptr() as *const u8, 12) as *const u8
}

#[inline]
pub(crate) unsafe fn iecfn(svc: *const u8, off: usize) -> usize {
    (svc.add(off) as *const usize).read()
}

pub(crate) type FIecConnect = unsafe extern "C" fn(*const u8, *const u8, u32, *mut u32) -> i32;
pub(crate) type FIecClose = unsafe extern "C" fn(*const u8, u32) -> i32;
pub(crate) type FIecStatus = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;
pub(crate) type FIecRead = unsafe extern "C" fn(*const u8, u32, *const u8, *const u8, *mut u8, u32) -> i32;
pub(crate) type FIecWrite = unsafe extern "C" fn(*const u8, u32, *const u8, *const u8, *const u8, u32) -> i32;
pub(crate) type FIecGetNameList = unsafe extern "C" fn(*const u8, u32, *const u8, *mut u8, u32) -> i32;
pub(crate) type FIecRcbGet = unsafe extern "C" fn(*const u8, u32, *const u8, *mut u8) -> i32;
pub(crate) type FIecRcbSet = unsafe extern "C" fn(*const u8, u32, *const u8, u32) -> i32;
pub(crate) type FIecReportPoll = unsafe extern "C" fn(*const u8, u32, *mut u8, u32) -> i32;
pub(crate) type FIecControl = unsafe extern "C" fn(*const u8, u32, *const u8, u32,
                                       *const u8, *mut u8, u32) -> i32;
pub(crate) type FIecControlPoll = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;
pub(crate) type FIecDsRead = unsafe extern "C" fn(*const u8, u32, *const u8,
                                      *mut u8, u32) -> i32;
pub(crate) type FIecDsDelete = unsafe extern "C" fn(*const u8, u32, *const u8) -> i32;

/// `__tplc_rt_FB_Iec61850` — synchronous op dispatch over the
/// tplc-iec61850 MMS client. OP: 1=connect+associate (HOST/PORT ->
/// HANDLE), 2=close+release, 3=status (-> STATE/INVOKE/LASTERR),
/// 4=read (DOMAIN/ITEM -> VAL), 5=write (DOMAIN/ITEM + VAL),
/// 6=GetNameList (DOMAIN -> VAL). STATUS carries the service return
/// code, ERRID its positive form.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_Iec61850(this: *mut u8, ctx: *mut u8) -> i32 {
    let op = ru32(this.add(IEC_OP));
    wi32(this.add(IEC_STATUS), 0);
    wu32(this.add(IEC_ERRID), 0);
    let svc = iec_service(ctx);
    if svc.is_null() {
        wi32(this.add(IEC_STATUS), -2);
        wu32(this.add(IEC_ERRID), 99);
        return 0;
    }
    match op {
        1 => {
            let mut buf = [0u8; 64];
            uacstr(this, IEC_HOST_LEN, IEC_HOST, 63, buf.as_mut_ptr());
            let f: FIecConnect = core::mem::transmute(iecfn(svc, IEC_SVC_CONNECT));
            let mut h: u32 = 0;
            let rc = f(svc, buf.as_ptr(), ru32(this.add(IEC_PORT)), &mut h);
            wi32(this.add(IEC_STATUS), rc);
            wu32(this.add(IEC_ERRID), (-rc).max(0) as u32);
            if rc == 0 {
                wu32(this.add(IEC_HANDLE), h);
            }
        }
        2 => {
            let f: FIecClose = core::mem::transmute(iecfn(svc, IEC_SVC_CLOSE));
            let rc = f(svc, ru32(this.add(IEC_HANDLE)));
            wi32(this.add(IEC_STATUS), rc);
            wu32(this.add(IEC_ERRID), (-rc).max(0) as u32);
        }
        3 => {
            // TplcIec61850Status (32 bytes): state@0 invoke_id@4 last_error@8
            let mut st = [0u8; 32];
            let f: FIecStatus = core::mem::transmute(iecfn(svc, IEC_SVC_STATUS));
            let rc = f(svc, ru32(this.add(IEC_HANDLE)), st.as_mut_ptr());
            wi32(this.add(IEC_STATUS), rc);
            if rc == 0 {
                let rd = |off: usize| -> i32 {
                    let mut b = [0u8; 4];
                    core::ptr::copy_nonoverlapping(st.as_ptr().add(off), b.as_mut_ptr(), 4);
                    i32::from_le_bytes(b)
                };
                wi32(this.add(IEC_STATE), rd(0));
                wu32(this.add(IEC_INVOKE), rd(4) as u32);
                wu32(this.add(IEC_LASTERR), rd(8) as u32);
            } else {
                wu32(this.add(IEC_ERRID), (-rc).max(0) as u32);
            }
        }
        4 => {
            let mut dbuf = [0u8; 64];
            let mut ibuf = [0u8; 128];
            uacstr(this, IEC_DOMAIN_LEN, IEC_DOMAIN, 63, dbuf.as_mut_ptr());
            uacstr(this, IEC_ITEM_LEN, IEC_ITEM, 127, ibuf.as_mut_ptr());
            let mut out = [0u8; 255];
            let f: FIecRead = core::mem::transmute(iecfn(svc, IEC_SVC_READ));
            let rc = f(svc, ru32(this.add(IEC_HANDLE)),
                       dbuf.as_ptr(), ibuf.as_ptr(), out.as_mut_ptr(), 255);
            wi32(this.add(IEC_STATUS), rc);
            wu32(this.add(IEC_ERRID), (-rc).max(0) as u32);
            if rc >= 0 {
                let n = (rc as usize).min(255);
                core::ptr::copy_nonoverlapping(out.as_ptr(), this.add(IEC_VAL), n);
                *this.add(IEC_VAL_LEN) = n as u8;
                *this.add(IEC_VAL + n) = 0;
            }
        }
        5 => {
            let mut dbuf = [0u8; 64];
            let mut ibuf = [0u8; 128];
            uacstr(this, IEC_DOMAIN_LEN, IEC_DOMAIN, 63, dbuf.as_mut_ptr());
            uacstr(this, IEC_ITEM_LEN, IEC_ITEM, 127, ibuf.as_mut_ptr());
            let n = (*this.add(IEC_VAL_LEN) as usize).min(255);
            let f: FIecWrite = core::mem::transmute(iecfn(svc, IEC_SVC_WRITE));
            let rc = f(svc, ru32(this.add(IEC_HANDLE)),
                       dbuf.as_ptr(), ibuf.as_ptr(),
                       this.add(IEC_VAL), n as u32);
            wi32(this.add(IEC_STATUS), rc);
            wu32(this.add(IEC_ERRID), (-rc).max(0) as u32);
        }
        6 => {
            let mut dbuf = [0u8; 64];
            uacstr(this, IEC_DOMAIN_LEN, IEC_DOMAIN, 63, dbuf.as_mut_ptr());
            let mut out = [0u8; 255];
            let f: FIecGetNameList = core::mem::transmute(iecfn(svc, IEC_SVC_GETNAMELIST));
            let rc = f(svc, ru32(this.add(IEC_HANDLE)),
                       dbuf.as_ptr(), out.as_mut_ptr(), 255);
            wi32(this.add(IEC_STATUS), rc);
            wu32(this.add(IEC_ERRID), (-rc).max(0) as u32);
            if rc >= 0 {
                let mut n = 0usize;
                while n < 255 && out[n] != 0 { n += 1; }
                core::ptr::copy_nonoverlapping(out.as_ptr(), this.add(IEC_VAL), n);
                *this.add(IEC_VAL_LEN) = n as u8;
                *this.add(IEC_VAL + n) = 0;
            }
        }
        7 => {
            // GetRCBValues: RCBREF -> TplcIec61850Rcb (152B)
            let mut rbuf = [0u8; 128];
            uacstr(this, IEC_RCBREF_LEN, IEC_RCBREF, 127, rbuf.as_mut_ptr());
            let mut rcb = [0u8; 152];
            let f: FIecRcbGet = core::mem::transmute(iecfn(svc, IEC_SVC_RCB_GET));
            let rc = f(svc, ru32(this.add(IEC_HANDLE)),
                       rbuf.as_ptr(), rcb.as_mut_ptr());
            wi32(this.add(IEC_STATUS), rc);
            wu32(this.add(IEC_ERRID), (-rc).max(0) as u32);
            if rc == 0 {
                let rd32 = |off: usize| -> i32 {
                    let mut b = [0u8; 4];
                    core::ptr::copy_nonoverlapping(rcb.as_ptr().add(off), b.as_mut_ptr(), 4);
                    i32::from_le_bytes(b)
                };
                let wr_str = |dst_len: usize, dst: usize, src: usize, cap: usize| {
                    let mut n = 0usize;
                    while n < cap && rcb[src + n] != 0 { n += 1; }
                    core::ptr::copy_nonoverlapping(rcb.as_ptr().add(src), this.add(dst), n);
                    *this.add(dst_len) = n as u8;
                    *this.add(dst + n) = 0;
                };
                wr_str(IEC_RPTID_LEN, IEC_RPTID, 0, 31);
                wr_str(IEC_DATASET_LEN, IEC_DATASET, 32, 63);
                wu32(this.add(IEC_CONFREV), rd32(96) as u32);
                wu32(this.add(IEC_BUFTM), rd32(100) as u32);
                wu32(this.add(IEC_INTGPD), rd32(104) as u32);
                wu32(this.add(IEC_SQNUM), rd32(108) as u32);
                wu32(this.add(IEC_OPTFLDS), rd32(112) as u32);
                wu32(this.add(IEC_TRGOPS), rd32(116) as u32);
                wi32(this.add(IEC_RPTENA), rd32(120));
                wi32(this.add(IEC_RESV), rd32(124));
                wi32(this.add(IEC_BUFFERED), rd32(128));
                wi32(this.add(IEC_GI), rd32(132));
                wi32(this.add(IEC_PURGEBUF), rd32(136));
                wu32(this.add(IEC_RESVTMS), rd32(140) as u32);
                wu32(this.add(IEC_ENTRYID), rd32(144) as u32);
            }
        }
        8 => {
            // SetRCBValues: RCBREF + FLAGS (bit0 ena, bit1 resv, bit2 GI)
            let mut rbuf = [0u8; 128];
            uacstr(this, IEC_RCBREF_LEN, IEC_RCBREF, 127, rbuf.as_mut_ptr());
            let f: FIecRcbSet = core::mem::transmute(iecfn(svc, IEC_SVC_RCB_SET));
            let rc = f(svc, ru32(this.add(IEC_HANDLE)),
                       rbuf.as_ptr(), ru32(this.add(IEC_FLAGS)));
            wi32(this.add(IEC_STATUS), rc);
            wu32(this.add(IEC_ERRID), (-rc).max(0) as u32);
        }
        9 => {
            // report poll: drain queued reports into VAL, RCNT bytes
            let mut out = [0u8; 255];
            let f: FIecReportPoll = core::mem::transmute(iecfn(svc, IEC_SVC_REPORT_POLL));
            let rc = f(svc, ru32(this.add(IEC_HANDLE)),
                       out.as_mut_ptr(), 255);
            wi32(this.add(IEC_STATUS), rc);
            wu32(this.add(IEC_ERRID), (-rc).max(0) as u32);
            if rc >= 0 {
                wu32(this.add(IEC_RCNT), rc as u32);
                let n = (rc as usize).min(255);
                core::ptr::copy_nonoverlapping(out.as_ptr(), this.add(IEC_VAL), n);
                *this.add(IEC_VAL_LEN) = n as u8;
                *this.add(IEC_VAL + n) = 0;
            }
        }
        10 => {
            // control: CTRLREF + CMODE (1=select 2=selectWithValue
            // 3=operate 4=cancel) + CTLVAL -> VAL echo
            let mut cbuf = [0u8; 128];
            let mut vbuf = [0u8; 64];
            uacstr(this, IEC_CTRLREF_LEN, IEC_CTRLREF, 127, cbuf.as_mut_ptr());
            uacstr(this, IEC_CTLVAL_LEN, IEC_CTLVAL, 63, vbuf.as_mut_ptr());
            let mut out = [0u8; 64];
            let f: FIecControl = core::mem::transmute(iecfn(svc, IEC_SVC_CONTROL));
            let rc = f(svc, ru32(this.add(IEC_HANDLE)),
                       cbuf.as_ptr(), ru32(this.add(IEC_CMODE)),
                       vbuf.as_ptr(), out.as_mut_ptr(), 64);
            wi32(this.add(IEC_STATUS), rc);
            wu32(this.add(IEC_ERRID), (-rc).max(0) as u32);
            if rc == 0 {
                let mut n = 0usize;
                while n < 64 && out[n] != 0 { n += 1; }
                core::ptr::copy_nonoverlapping(out.as_ptr(), this.add(IEC_VAL), n);
                *this.add(IEC_VAL_LEN) = n as u8;
                *this.add(IEC_VAL + n) = 0;
            }
        }
        11 => {
            // control poll: drain one CommandTermination event ->
            // CTLINVOKE/CTLSUCCESS/CTLERR/CTLTYPE/CTLNUM/CTLPEND
            let mut ev = [0u8; 32];
            let f: FIecControlPoll =
                core::mem::transmute(iecfn(svc, IEC_SVC_CONTROL_POLL));
            let rc = f(svc, ru32(this.add(IEC_HANDLE)), ev.as_mut_ptr());
            wi32(this.add(IEC_STATUS), rc);
            wu32(this.add(IEC_ERRID), (-rc).max(0) as u32);
            if rc == 0 {
                let rd32 = |off: usize| -> u32 {
                    let mut b = [0u8; 4];
                    core::ptr::copy_nonoverlapping(ev.as_ptr().add(off),
                                                 b.as_mut_ptr(), 4);
                    u32::from_le_bytes(b)
                };
                wu32(this.add(IEC_CTLINVOKE), rd32(0));
                wi32(this.add(IEC_CTLSUCCESS), rd32(4) as i32);
                wu32(this.add(IEC_CTLERR), rd32(8));
                wu32(this.add(IEC_CTLTYPE), rd32(12));
                wu32(this.add(IEC_CTLNUM), rd32(16));
                wu32(this.add(IEC_CTLPEND), rd32(20));
            }
        }
        12 | 13 => {
            // data set: DSREF -> VAL text, DSCOUNT elements.
            // 12=ReadDataSet ("FCDA=value" lines), 13=directory.
            let mut rbuf = [0u8; 128];
            uacstr(this, IEC_DSREF_LEN, IEC_DSREF, 127, rbuf.as_mut_ptr());
            let mut out = [0u8; 255];
            let off = if op == 12 { IEC_SVC_DS_READ } else { IEC_SVC_DS_DIRECTORY };
            let f: FIecDsRead = core::mem::transmute(iecfn(svc, off));
            let rc = f(svc, ru32(this.add(IEC_HANDLE)),
                       rbuf.as_ptr(), out.as_mut_ptr(), 255);
            wi32(this.add(IEC_STATUS), rc);
            wu32(this.add(IEC_ERRID), (-rc).max(0) as u32);
            if rc >= 0 {
                wu32(this.add(IEC_DSCOUNT), rc as u32);
                let mut n = 0usize;
                while n < 255 && out[n] != 0 { n += 1; }
                core::ptr::copy_nonoverlapping(out.as_ptr(), this.add(IEC_VAL), n);
                *this.add(IEC_VAL_LEN) = n as u8;
                *this.add(IEC_VAL + n) = 0;
            }
        }
        14 => {
            // WriteDataSet: DSREF + VAL (newline-separated values,
            // optional "name=" prefixes) -> STATUS only
            let mut rbuf = [0u8; 128];
            uacstr(this, IEC_DSREF_LEN, IEC_DSREF, 127, rbuf.as_mut_ptr());
            let n = (*this.add(IEC_VAL_LEN) as usize).min(255);
            let f: FIecDsRead = core::mem::transmute(iecfn(svc, IEC_SVC_DS_WRITE));
            let rc = f(svc, ru32(this.add(IEC_HANDLE)),
                       rbuf.as_ptr(), this.add(IEC_VAL), n as u32);
            wi32(this.add(IEC_STATUS), rc);
            wu32(this.add(IEC_ERRID), (-rc).max(0) as u32);
        }
        15 => {
            // CreateDataSet: DSREF + VAL (newline-separated member
            // FCDA refs) -> STATUS only
            let mut rbuf = [0u8; 128];
            uacstr(this, IEC_DSREF_LEN, IEC_DSREF, 127, rbuf.as_mut_ptr());
            let n = (*this.add(IEC_VAL_LEN) as usize).min(255);
            let f: FIecDsRead = core::mem::transmute(iecfn(svc, IEC_SVC_DS_CREATE));
            let rc = f(svc, ru32(this.add(IEC_HANDLE)),
                       rbuf.as_ptr(), this.add(IEC_VAL), n as u32);
            wi32(this.add(IEC_STATUS), rc);
            wu32(this.add(IEC_ERRID), (-rc).max(0) as u32);
        }
        16 => {
            // DeleteDataSet: DSREF -> STATUS only
            let mut rbuf = [0u8; 128];
            uacstr(this, IEC_DSREF_LEN, IEC_DSREF, 127, rbuf.as_mut_ptr());
            let f: FIecDsDelete = core::mem::transmute(iecfn(svc, IEC_SVC_DS_DELETE));
            let rc = f(svc, ru32(this.add(IEC_HANDLE)), rbuf.as_ptr());
            wi32(this.add(IEC_STATUS), rc);
            wu32(this.add(IEC_ERRID), (-rc).max(0) as u32);
        }
        _ => {
            wi32(this.add(IEC_STATUS), -1);
            wu32(this.add(IEC_ERRID), 1);
        }
    }
    0
}
