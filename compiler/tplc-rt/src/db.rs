//! tplc-rt builtin FB: db.
use crate::*;

// ---- FB_Db builtin — tplc-db SQLite bridge ----
// Member offsets (see builtin_fb_layout("FB_Db")).
pub(crate) const DB_OP: usize = 0;
pub(crate) const DB_HANDLE: usize = 4;
pub(crate) const DB_CHANGES: usize = 8;
pub(crate) const DB_ROWS: usize = 12;
pub(crate) const DB_COLS: usize = 16;
pub(crate) const DB_ERRLEN: usize = 20;
pub(crate) const DB_STATUS: usize = 24;
pub(crate) const DB_ERRID: usize = 28;
pub(crate) const DB_PATH_LEN: usize = 32;
pub(crate) const DB_PATH: usize = 33;
pub(crate) const DB_SQL_LEN: usize = 160;
pub(crate) const DB_SQL: usize = 161;
pub(crate) const DB_BUF_LEN: usize = 416;
pub(crate) const DB_BUF: usize = 417;
pub(crate) const DB_ERR_LEN: usize = 672;
pub(crate) const DB_ERR: usize = 673;

// TplcDbService vtable: size@0 abi@4 host@8
pub(crate) const DB_SVC_OPEN: usize = 16;
pub(crate) const DB_SVC_CLOSE: usize = 24;
pub(crate) const DB_SVC_EXEC: usize = 32;
pub(crate) const DB_SVC_QUERY: usize = 40;
pub(crate) const DB_SVC_ERROR: usize = 48;

pub(crate) unsafe fn db_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-db".as_ptr() as *const u8, 7) as *const u8
}

pub(crate) type FDbOpen = unsafe extern "C" fn(*const u8, *const u8, *mut u32) -> i32;
pub(crate) type FDbClose = unsafe extern "C" fn(*const u8, u32) -> i32;
pub(crate) type FDbExec = unsafe extern "C" fn(*const u8, u32, *const u8, *mut u32) -> i32;
pub(crate) type FDbQuery = unsafe extern "C" fn(*const u8, u32, *const u8, *mut u8, u32, *mut u32, *mut u32) -> i32;
pub(crate) type FDbError = unsafe extern "C" fn(*const u8, u32, *mut u8, u32, *mut u32) -> i32;

/// `__tplc_rt_FB_Db` — synchronous op dispatch over the tplc-db
/// SQLite service.  OP: 1=open (PATH -> HANDLE), 2=exec
/// (SQL -> CHANGES), 3=query (SQL -> BUF/ROWS/COLS), 4=close,
/// 5=error (-> ERR).
/// # Safety
/// `this` must point at a valid instance block of the
/// documented layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_Db(this: *mut u8, ctx: *mut u8) -> i32 {
    let op = ru32(this.add(DB_OP));
    wi32(this.add(DB_STATUS), 0);
    wu32(this.add(DB_ERRID), 0);
    let svc = db_service(ctx);
    if svc.is_null() {
        wi32(this.add(DB_STATUS), -2);
        wu32(this.add(DB_ERRID), 99);
        return 0;
    }
    let set_rc = |rc: i32| {
        wi32(this.add(DB_STATUS), rc);
        wu32(this.add(DB_ERRID), (-rc).max(0) as u32);
    };
    match op {
        1 => {
            let mut buf = [0u8; 128];
            uacstr(this, DB_PATH_LEN, DB_PATH, 127, buf.as_mut_ptr());
            let f: FDbOpen = core::mem::transmute(mbusfn(svc, DB_SVC_OPEN));
            let mut h: u32 = 0;
            let rc = f(svc, buf.as_ptr(), &mut h);
            set_rc(rc);
            if rc == 0 {
                wu32(this.add(DB_HANDLE), h);
            }
        }
        2 => {
            let mut buf = [0u8; 256];
            uacstr(this, DB_SQL_LEN, DB_SQL, 255, buf.as_mut_ptr());
            let f: FDbExec = core::mem::transmute(mbusfn(svc, DB_SVC_EXEC));
            let mut ch: u32 = 0;
            let rc = f(svc, ru32(this.add(DB_HANDLE)), buf.as_ptr(), &mut ch);
            set_rc(rc);
            if rc == 0 {
                wu32(this.add(DB_CHANGES), ch);
            }
        }
        3 => {
            let mut sql = [0u8; 256];
            uacstr(this, DB_SQL_LEN, DB_SQL, 255, sql.as_mut_ptr());
            let mut out = [0u8; 256];
            let f: FDbQuery = core::mem::transmute(mbusfn(svc, DB_SVC_QUERY));
            let mut rows: u32 = 0;
            let mut cols: u32 = 0;
            let rc = f(svc, ru32(this.add(DB_HANDLE)), sql.as_ptr(),
                       out.as_mut_ptr(), 255, &mut rows, &mut cols);
            set_rc(rc);
            if rc == 0 {
                wu32(this.add(DB_ROWS), rows);
                wu32(this.add(DB_COLS), cols);
                let n = (0..256).position(|i| out[i] == 0).unwrap_or(255);
                core::ptr::copy_nonoverlapping(out.as_ptr(), this.add(DB_BUF), n);
                *this.add(DB_BUF_LEN) = n as u8;
            }
        }
        4 => {
            let f: FDbClose = core::mem::transmute(mbusfn(svc, DB_SVC_CLOSE));
            set_rc(f(svc, ru32(this.add(DB_HANDLE))));
        }
        5 => {
            let mut out = [0u8; 128];
            let mut n: u32 = 0;
            let f: FDbError = core::mem::transmute(mbusfn(svc, DB_SVC_ERROR));
            let rc = f(svc, ru32(this.add(DB_HANDLE)), out.as_mut_ptr(),
                       127, &mut n);
            set_rc(rc);
            if rc == 0 {
                core::ptr::copy_nonoverlapping(out.as_ptr(), this.add(DB_ERR), n as usize);
                *this.add(DB_ERR_LEN) = n as u8;
                wu32(this.add(DB_ERRLEN), n);
            }
        }
        _ => {
            wi32(this.add(DB_STATUS), -1);
            wu32(this.add(DB_ERRID), 1);
        }
    }
    0
}
