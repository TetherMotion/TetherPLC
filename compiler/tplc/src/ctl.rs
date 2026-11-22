//! Control-channel client — native Rust port of `tools/ctl_client.h`.
//! Wire protocol: `include/tplc/tplc_ctl.h` (16-byte LE frame header:
//! u16 msg_type, u16 flags, u32 payload_len, u64 req_id).

use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::time::{Duration, Instant};

pub const PROTO_VER: u16 = 3;
pub const HDR_SIZE: usize = 16;
pub const F_MORE: u16 = 1 << 0;
pub const F_EVENT: u16 = 1 << 1;
pub const F_CANCEL: u16 = 1 << 2;
pub const F_ERROR: u16 = 1 << 15;
pub const MAX_PAYLOAD: u32 = 8 * 1024 * 1024;

pub mod op {
    pub const HELLO: u16 = 0x0101;
    pub const HELLO_ACK: u16 = 0x0102;
    pub const PING: u16 = 0x0103;
    pub const SET_NOTIFY: u16 = 0x0104;
    pub const DEPLOY_BEGIN: u16 = 0x0201;
    pub const DEPLOY_CHUNK: u16 = 0x0202;
    pub const DEPLOY_COMMIT: u16 = 0x0203;
    pub const DEPLOY_ABORT: u16 = 0x0204;
    pub const PKG_LIST: u16 = 0x0205;
    pub const PKG_INFO: u16 = 0x0206;
    pub const PROG_PREPARE: u16 = 0x0301;
    pub const PROG_START: u16 = 0x0302;
    pub const PROG_STOP: u16 = 0x0303;
    pub const PROG_KILL: u16 = 0x0304;
    pub const PROG_RESET: u16 = 0x0305;
    pub const PROG_UNLOAD: u16 = 0x0306;
    pub const PROG_STATUS: u16 = 0x0307;
    pub const PROG_LIST: u16 = 0x0308;
    pub const PROG_LAYOUT: u16 = 0x0309;
    pub const PROG_SWAP: u16 = 0x030A;
    pub const PROG_ROLLBACK: u16 = 0x030B;
    pub const TASK_STATS: u16 = 0x0401;
    pub const TASK_SUSPEND: u16 = 0x0402;
    pub const TASK_RESUME: u16 = 0x0403;
    pub const TASK_STEP: u16 = 0x0404;
    pub const TASK_EVENT: u16 = 0x0405;
    pub const DBG_RESOLVE: u16 = 0x0501;
    pub const DBG_READ: u16 = 0x0502;
    pub const DBG_WRITE: u16 = 0x0503;
    pub const DBG_STATE: u16 = 0x0504;
    pub const DBG_FORCE: u16 = 0x0505;
    pub const DBG_FORCE_LIST: u16 = 0x0506;
    pub const DBG_FORCE_CLEAR: u16 = 0x0507;
    pub const DBG_BREAK: u16 = 0x0508;
    pub const DBG_BREAK_LIST: u16 = 0x0509;
    pub const DBG_BREAK_CLEAR: u16 = 0x050A;
    pub const TRACE_CONF: u16 = 0x050B;
    pub const TRACE_STATUS: u16 = 0x050C;
    pub const TRACE_READ: u16 = 0x050D;
    pub const TRACE_CLEAR: u16 = 0x050E;
    pub const DBG_CAPTURE: u16 = 0x050F;
    pub const DBG_WATCH_ADD: u16 = 0x0510;
    pub const DBG_WATCH_GET: u16 = 0x0511;
    pub const DBG_WATCH_REMOVE: u16 = 0x0512;
    pub const DBG_WATCH_STREAM: u16 = 0x0513;
    pub const DBG_WATCH_LIST: u16 = 0x0514;
    pub const DBG_BREAK_EXPR: u16 = 0x0515;
    pub const DBG_CAPTURE_EXPR: u16 = 0x0516;
    pub const DBG_STATE_DIFF: u16 = 0x0517;
    pub const DBG_FIELDS: u16 = 0x0518;
    pub const RETAIN_STATUS: u16 = 0x0601;
    pub const RETAIN_FLUSH: u16 = 0x0602;
    pub const RETAIN_CLEAR: u16 = 0x0603;
    pub const RETAIN_DUMP: u16 = 0x0604;
    pub const IMAGE_STATS: u16 = 0x0701;
    pub const SHADOW_LOAD: u16 = 0x0801;
    pub const SHADOW_STATUS: u16 = 0x0802;
    pub const SHADOW_COMPARE: u16 = 0x0803;
    pub const SHADOW_PROMOTE: u16 = 0x0804;
    pub const SHADOW_DROP: u16 = 0x0805;
    pub const SHADOW_RESET: u16 = 0x0806;
    pub const SHADOW_IMAGE: u16 = 0x0807;
    pub const SHADOW_LIST: u16 = 0x0808;
    pub const RT_STATUS: u16 = 0x0901;
    pub const RT_SHUTDOWN: u16 = 0x0905;
    pub const OD_READ: u16 = 0x0A01;
    pub const OD_LIST: u16 = 0x0A02;
    pub const OD_WRITE: u16 = 0x0A03;
    pub const MAP_GET: u16 = 0x0C01;
    pub const JOURNAL_TAIL: u16 = 0x0D01;
    pub const EVTLOG_LIST: u16 = 0x0F01;
    pub const EVTLOG_QUERY: u16 = 0x0F02;
    pub const EVTLOG_STATS: u16 = 0x0F03;
    pub const EVTLOG_HISTORY: u16 = 0x0F04;
    pub const EVTLOG_CONFIRM: u16 = 0x0F05;
    pub const HIST_CONF: u16 = 0x0F10;
    pub const HIST_LIST: u16 = 0x0F11;
    pub const MQTT_STATUS: u16 = 0x0F20;
    pub const MQTT_JOURNAL: u16 = 0x0F21;
    pub const EV_PROG_TRANSITION: u16 = 0x0E01;
    pub const EV_FAULT: u16 = 0x0E02;
    pub const EV_DBG_BREAK: u16 = 0x0E03;
    pub const EV_DBG_WATCH: u16 = 0x0E04;
    pub const EV_USER: u16 = 0x0E05;
}

#[derive(Debug)]
pub struct CtlError {
    pub code: u32,
    pub subcode: u32,
    pub msg: String,
}

impl std::fmt::Display for CtlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error {}: {}", self.code, self.msg)
    }
}

pub struct Frame {
    pub msg_type: u16,
    pub flags: u16,
    pub req_id: u64,
    pub payload: Vec<u8>,
}

fn read_exact_timeout(s: &mut UnixStream, buf: &mut [u8]) -> std::io::Result<()> {
    s.read_exact(buf).map(|_| ())
}

pub struct Ctl {
    stream: UnixStream,
    req_id: u64,
}

impl Ctl {
    /// Connect with the same retry semantics as ctl_client.h: retry on
    /// ENOENT/ECONNREFUSED every 20 ms up to `timeout_ms`.
    pub fn connect(path: &str, timeout_ms: u64) -> std::io::Result<Ctl> {
        let deadline = Instant::now() + Duration::from_millis(timeout_ms);
        loop {
            match UnixStream::connect(path) {
                Ok(s) => {
                    return Ok(Ctl {
                        stream: s,
                        req_id: 1,
                    })
                }
                Err(e)
                    if (e.kind() == std::io::ErrorKind::NotFound
                        || e.raw_os_error() == Some(libc::ECONNREFUSED))
                        && Instant::now() < deadline =>
                {
                    std::thread::sleep(Duration::from_millis(20));
                }
                Err(e) => return Err(e),
            }
        }
    }

    /// Non-blocking probe: single connect attempt, no retry.
    pub fn probe(path: &str) -> bool {
        UnixStream::connect(path).is_ok()
    }

    fn send_frame(&mut self, msg_type: u16, flags: u16, payload: &[u8]) -> std::io::Result<()> {
        let req = self.req_id;
        self.req_id = self.req_id.wrapping_add(1);
        let mut hdr = [0u8; HDR_SIZE];
        hdr[0..2].copy_from_slice(&msg_type.to_le_bytes());
        hdr[2..4].copy_from_slice(&flags.to_le_bytes());
        hdr[4..8].copy_from_slice(&(payload.len() as u32).to_le_bytes());
        hdr[8..16].copy_from_slice(&req.to_le_bytes());
        self.stream.write_all(&hdr)?;
        if !payload.is_empty() {
            self.stream.write_all(payload)?;
        }
        Ok(())
    }

    fn recv_frame(&mut self) -> std::io::Result<Frame> {
        let mut hdr = [0u8; HDR_SIZE];
        read_exact_timeout(&mut self.stream, &mut hdr)?;
        let msg_type = u16::from_le_bytes([hdr[0], hdr[1]]);
        let flags = u16::from_le_bytes([hdr[2], hdr[3]]);
        let len = u32::from_le_bytes([hdr[4], hdr[5], hdr[6], hdr[7]]);
        let req_id = u64::from_le_bytes(hdr[8..16].try_into().unwrap());
        if len > MAX_PAYLOAD {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "ctl frame too large",
            ));
        }
        let mut payload = vec![0u8; len as usize];
        if len > 0 {
            read_exact_timeout(&mut self.stream, &mut payload)?;
        }
        Ok(Frame {
            msg_type,
            flags,
            req_id,
            payload,
        })
    }

    /// Session bootstrap: HELLO {u16 proto_ver, u16 pad, u32 caps} +
    /// u16 name_len + name. Expects HELLO_ACK.
    pub fn hello(&mut self, name: &str) -> std::io::Result<()> {
        let mut buf = Vec::with_capacity(8 + 2 + name.len());
        buf.extend_from_slice(&PROTO_VER.to_le_bytes());
        buf.extend_from_slice(&0u16.to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes()); // caps
        buf.extend_from_slice(&(name.len() as u16).to_le_bytes());
        buf.extend_from_slice(name.as_bytes());
        self.send_frame(op::HELLO, 0, &buf)?;
        let f = self.recv_frame()?;
        if f.msg_type != op::HELLO_ACK {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "hello: no HELLO_ACK",
            ));
        }
        Ok(())
    }

    /// One request → first reply frame (flags + payload).
    /// `payload` may exceed 64 KiB — replies come back whole.
    pub fn call(&mut self, msg_type: u16, payload: &[u8]) -> std::io::Result<(u16, Vec<u8>)> {
        self.send_frame(msg_type, 0, payload)?;
        let f = self.recv_frame()?;
        Ok((f.flags, f.payload))
    }

    /// `call` + F_ERROR check → Err(CtlError) on error frames.
    pub fn call_ok(&mut self, msg_type: u16, payload: &[u8]) -> Result<Vec<u8>, CtlError> {
        match self.call(msg_type, payload) {
            Ok((flags, pl)) => {
                if flags & F_ERROR != 0 {
                    Err(parse_ctl_error(&pl))
                } else {
                    Ok(pl)
                }
            }
            Err(e) => Err(CtlError {
                code: u32::MAX,
                subcode: 0,
                msg: format!("io: {e}"),
            }),
        }
    }

    /// Arm the notify bitmask (SET_NOTIFY {u32 mask}).
    pub fn notify(&mut self, mask: u32) -> std::io::Result<()> {
        self.call_ok(op::SET_NOTIFY, &mask.to_le_bytes())
            .map(|_| ())
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
    }

    /// Next frame on the wire — used by event streams (F_EVENT frames
    /// arrive unsolicited after SET_NOTIFY).
    pub fn next_frame(&mut self) -> std::io::Result<Frame> {
        self.recv_frame()
    }

    /// Take the raw stream clone for protocol bridging (proxy /ctl).
    pub fn into_inner(self) -> UnixStream {
        self.stream
    }

    pub fn stream_mut(&mut self) -> &mut UnixStream {
        &mut self.stream
    }

    pub fn path_exists(path: &str) -> bool {
        Path::new(path).exists()
    }
}

/// Parse a TplcCtlError payload {u32 code, u32 subcode, str msg}.
pub fn parse_ctl_error(pl: &[u8]) -> CtlError {
    let mut code = 0u32;
    let mut subcode = 0u32;
    if pl.len() >= 8 {
        code = u32::from_le_bytes(pl[0..4].try_into().unwrap());
        subcode = u32::from_le_bytes(pl[4..8].try_into().unwrap());
    }
    let mut msg = String::new();
    if pl.len() >= 10 {
        let ml = u16::from_le_bytes([pl[8], pl[9]]) as usize;
        let avail = pl.len().saturating_sub(10);
        let n = ml.min(avail).min(200);
        msg = String::from_utf8_lossy(&pl[10..10 + n]).into_owned();
    }
    CtlError { code, subcode, msg }
}

/* ---------------- little reader helpers for reply payloads --------- */

pub struct Rd<'a> {
    pub b: &'a [u8],
    pub o: usize,
}

impl<'a> Rd<'a> {
    pub fn new(b: &'a [u8]) -> Self {
        Rd { b, o: 0 }
    }
    pub fn left(&self) -> usize {
        self.b.len().saturating_sub(self.o)
    }
    pub fn u8(&mut self) -> u8 {
        let v = self.b.get(self.o).copied().unwrap_or(0);
        self.o += 1;
        v
    }
    pub fn u16(&mut self) -> u16 {
        let v = u16::from_le_bytes([
            self.b.get(self.o).copied().unwrap_or(0),
            self.b.get(self.o + 1).copied().unwrap_or(0),
        ]);
        self.o += 2;
        v
    }
    pub fn u32(&mut self) -> u32 {
        let v = u32::from_le_bytes(self.b[self.o..self.o + 4].try_into().unwrap_or([0; 4]));
        self.o += 4;
        v
    }
    pub fn i32(&mut self) -> i32 {
        self.u32() as i32
    }
    pub fn u64(&mut self) -> u64 {
        let mut a = [0u8; 8];
        for i in 0..8 {
            a[i] = self.b.get(self.o + i).copied().unwrap_or(0);
        }
        self.o += 8;
        u64::from_le_bytes(a)
    }
    /// TLV string: u16 len + bytes.
    pub fn str(&mut self) -> String {
        let n = self.u16() as usize;
        let s = String::from_utf8_lossy(&self.b[self.o..self.o + n.min(self.left())]).into_owned();
        self.o += n.min(self.left());
        s
    }
    pub fn bytes(&mut self, n: usize) -> &'a [u8] {
        let end = (self.o + n).min(self.b.len());
        let s = &self.b[self.o..end];
        self.o = end;
        s
    }
    pub fn skip(&mut self, n: usize) {
        self.o = (self.o + n).min(self.b.len());
    }
}

/// Little-endian payload builder.
pub struct Wr {
    pub b: Vec<u8>,
}

impl Wr {
    pub fn new() -> Self {
        Wr { b: Vec::new() }
    }
    pub fn u8(&mut self, v: u8) -> &mut Self {
        self.b.push(v);
        self
    }
    pub fn u16(&mut self, v: u16) -> &mut Self {
        self.b.extend_from_slice(&v.to_le_bytes());
        self
    }
    pub fn u32(&mut self, v: u32) -> &mut Self {
        self.b.extend_from_slice(&v.to_le_bytes());
        self
    }
    pub fn u64(&mut self, v: u64) -> &mut Self {
        self.b.extend_from_slice(&v.to_le_bytes());
        self
    }
    pub fn bytes(&mut self, v: &[u8]) -> &mut Self {
        self.b.extend_from_slice(v);
        self
    }
    pub fn zeros(&mut self, n: usize) -> &mut Self {
        self.b.resize(self.b.len() + n, 0);
        self
    }
    pub fn str(&mut self, s: &str) -> &mut Self {
        self.u16(s.len() as u16);
        self.bytes(s.as_bytes());
        self
    }
}
