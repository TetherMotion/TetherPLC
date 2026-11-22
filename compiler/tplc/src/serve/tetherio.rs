use super::*;

/* ---------------- /tether-io (WebSocket → TCP, SLIP) ---------------- */

pub(crate) fn sha1(data: &[u8]) -> [u8; 20] {
    // FIPS 180-1, ported from the C++ proxy
    let mut h: [u32; 5] = [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0];
    let bitlen = (data.len() as u64) * 8;
    let mut m = data.to_vec();
    m.push(0x80);
    while m.len() % 64 != 56 {
        m.push(0);
    }
    m.extend_from_slice(&bitlen.to_be_bytes());
    for chunk in m.chunks(64) {
        let mut w = [0u32; 80];
        for i in 0..16 {
            w[i] = u32::from_be_bytes(chunk[i * 4..i * 4 + 4].try_into().unwrap());
        }
        for i in 16..80 {
            let x = w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16];
            w[i] = x.rotate_left(1);
        }
        let (mut a, mut b, mut c, mut d, mut e) = (h[0], h[1], h[2], h[3], h[4]);
        for i in 0..80 {
            let (f, k) = match i {
                0..=19 => ((b & c) | (!b & d), 0x5A827999u32),
                20..=39 => (b ^ c ^ d, 0x6ED9EBA1),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1BBCDC),
                _ => (b ^ c ^ d, 0xCA62C1D6),
            };
            let t = a.rotate_left(5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(k)
                .wrapping_add(w[i]);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = t;
        }
        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
    }
    let mut out = [0u8; 20];
    for i in 0..5 {
        out[i * 4..i * 4 + 4].copy_from_slice(&h[i].to_be_bytes());
    }
    out
}

pub(crate) fn ws_read_frame(s: &mut TcpStream) -> Option<(u8, Vec<u8>, bool)> {
    let mut h = [0u8; 2];
    s.read_exact(&mut h).ok()?;
    let fin = h[0] & 0x80 != 0;
    let op = h[0] & 0x0F;
    let masked = h[1] & 0x80 != 0;
    let mut len = (h[1] & 0x7F) as u64;
    if len == 126 {
        let mut b = [0u8; 2];
        s.read_exact(&mut b).ok()?;
        len = u16::from_be_bytes(b) as u64;
    } else if len == 127 {
        let mut b = [0u8; 8];
        s.read_exact(&mut b).ok()?;
        len = u64::from_be_bytes(b);
    }
    if len > (64 << 20) {
        return None;
    }
    let mut mask = [0u8; 4];
    if masked {
        s.read_exact(&mut mask).ok()?;
    }
    let mut payload = vec![0u8; len as usize];
    s.read_exact(&mut payload).ok()?;
    if masked {
        for (i, b) in payload.iter_mut().enumerate() {
            *b ^= mask[i & 3];
        }
    }
    Some((op, payload, fin))
}

pub(crate) fn ws_send_frame(s: &mut TcpStream, op: u8, payload: &[u8]) -> bool {
    let n = payload.len();
    let mut hdr = vec![0x80 | op];
    if n < 126 {
        hdr.push(n as u8);
    } else if n <= 0xFFFF {
        hdr.push(126);
        hdr.extend_from_slice(&(n as u16).to_be_bytes());
    } else {
        hdr.push(127);
        hdr.extend_from_slice(&(n as u64).to_be_bytes());
    }
    s.write_all(&hdr).is_ok() && (n == 0 || s.write_all(payload).is_ok())
}

pub(crate) const SLIP_END: u8 = 0xC0;
pub(crate) const SLIP_ESC: u8 = 0xDB;
pub(crate) const SLIP_ESC_END: u8 = 0xDC;
pub(crate) const SLIP_ESC_ESC: u8 = 0xDD;
pub(crate) const SLIP_MAX: usize = 1 << 20;

pub(crate) fn slip_encode(p: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(p.len() + p.len() / 8 + 1);
    for &b in p {
        match b {
            SLIP_END => out.extend_from_slice(&[SLIP_ESC, SLIP_ESC_END]),
            SLIP_ESC => out.extend_from_slice(&[SLIP_ESC, SLIP_ESC_ESC]),
            _ => out.push(b),
        }
    }
    out.push(SLIP_END);
    out
}

pub(crate) struct SlipDec {
    msg: Vec<u8>,
    in_esc: bool,
    discard: bool,
}
impl SlipDec {
    fn new() -> Self {
        SlipDec { msg: vec![], in_esc: false, discard: false }
    }
    /// feed bytes; complete messages pushed to `out`
    fn feed(&mut self, p: &[u8], out: &mut Vec<Vec<u8>>) {
        for &b in p {
            if self.discard {
                if b == SLIP_END {
                    self.discard = false;
                    self.msg.clear();
                    self.in_esc = false;
                }
                continue;
            }
            if self.in_esc {
                self.in_esc = false;
                match b {
                    SLIP_ESC_END => self.msg.push(SLIP_END),
                    SLIP_ESC_ESC => self.msg.push(SLIP_ESC),
                    _ => {
                        self.msg.clear();
                        self.discard = true;
                    }
                }
                continue;
            }
            if b == SLIP_END {
                if !self.msg.is_empty() && self.msg.len() <= SLIP_MAX {
                    out.push(std::mem::take(&mut self.msg));
                } else {
                    self.msg.clear();
                }
            } else if b == SLIP_ESC {
                self.in_esc = true;
            } else {
                self.msg.push(b);
                if self.msg.len() > SLIP_MAX {
                    self.msg.clear();
                    self.discard = true;
                }
            }
        }
    }
}

pub(crate) fn tcp_connect(hostport: &str) -> Option<TcpStream> {
    use std::net::ToSocketAddrs;
    hostport.to_socket_addrs().ok()?.find_map(|a| TcpStream::connect(a).ok())
}

pub(crate) fn handle_tetherio(mut cfd: TcpStream, head: String, upstream: String) {
    std::thread::spawn(move || {
        // Sec-WebSocket-Key → accept digest
        let key = head
            .lines()
            .find(|l| l.to_lowercase().starts_with("sec-websocket-key:"))
            .and_then(|l| l.split_once(':').map(|x| x.1.trim().to_string()))
            .unwrap_or_default();
        if key.is_empty() {
            return;
        }
        let magic = format!("{key}258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
        let accept = b64e(&sha1(magic.as_bytes()));
        let resp = format!(
            "HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {accept}\r\n\r\n"
        );
        if cfd.write_all(resp.as_bytes()).is_err() {
            return;
        }
        let Some(tfd) = tcp_connect(&upstream) else {
            let pl = [0x03u8, 0xE8];
            let _ = ws_send_frame(&mut cfd, 0x8, &pl);
            return;
        };
        let tfd_w = match tfd.try_clone() {
            Ok(s) => s,
            Err(_) => return,
        };
        let mut cfd_r = match cfd.try_clone() {
            Ok(s) => s,
            Err(_) => return,
        };
        // reader thread: WS → SLIP → TCP
        let mut tfd_w2 = match tfd_w.try_clone() {
            Ok(s) => s,
            Err(_) => return,
        };
        let reader = std::thread::spawn(move || {
            let mut wsmsg: Vec<u8> = Vec::new();
            loop {
                let Some((op, payload, fin)) = ws_read_frame(&mut cfd_r) else { break };
                match op {
                    0x8 => {
                        let _ = ws_send_frame(&mut cfd_r, 0x8, &payload);
                        break;
                    }
                    0x9 => {
                        let _ = ws_send_frame(&mut cfd_r, 0xA, &payload);
                    }
                    0xA => {}
                    0x0 | 0x1 | 0x2 => {
                        if op != 0x0 {
                            wsmsg.clear();
                        }
                        wsmsg.extend_from_slice(&payload);
                        if !fin {
                            continue;
                        }
                        let pkt = slip_encode(&wsmsg);
                        wsmsg.clear();
                        if tfd_w2.write_all(&pkt).is_err() {
                            break;
                        }
                    }
                    _ => {}
                }
            }
            // unblock the main pump
            let _ = tfd_w.shutdown(std::net::Shutdown::Both);
        });
        // main: TCP → SLIP decode → WS binary frames
        let mut tfd_r = tfd;
        tfd_r.set_read_timeout(Some(std::time::Duration::from_millis(500))).ok();
        let mut slip = SlipDec::new();
        let mut buf = [0u8; 16384];
        let mut msgs: Vec<Vec<u8>> = Vec::new();
        loop {
            match tfd_r.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    msgs.clear();
                    slip.feed(&buf[..n], &mut msgs);
                    for m in msgs.drain(..) {
                        if !ws_send_frame(&mut cfd, 0x2, &m) {
                            let _ = reader.join();
                            return;
                        }
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock
                    || e.kind() == std::io::ErrorKind::TimedOut => {}
                Err(_) => break,
            }
            if STOP.load(Ordering::Relaxed) {
                break;
            }
        }
        let _ = cfd.shutdown(std::net::Shutdown::Both);
        let _ = reader.join();
    });
}

