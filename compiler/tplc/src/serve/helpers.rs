use super::*;

/* ---------------- small helpers ---------------- */

pub(crate) fn b64e(data: &[u8]) -> String {
    const T: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut o = String::with_capacity((data.len() + 2) / 3 * 4);
    for c in data.chunks(3) {
        let v = ((c[0] as u32) << 16)
            | ((*c.get(1).unwrap_or(&0) as u32) << 8)
            | (*c.get(2).unwrap_or(&0) as u32);
        o.push(T[(v >> 18) as usize & 63] as char);
        o.push(T[(v >> 12) as usize & 63] as char);
        o.push(if c.len() > 1 { T[(v >> 6) as usize & 63] as char } else { '=' });
        o.push(if c.len() > 2 { T[v as usize & 63] as char } else { '=' });
    }
    o
}

pub(crate) fn b64d(s: &str) -> Vec<u8> {
    fn val(c: u8) -> i32 {
        match c {
            b'A'..=b'Z' => (c - b'A') as i32,
            b'a'..=b'z' => (c - b'a' + 26) as i32,
            b'0'..=b'9' => (c - b'0' + 52) as i32,
            b'+' => 62,
            b'/' => 63,
            _ => -1,
        }
    }
    let mut out = Vec::new();
    let mut acc = 0u32;
    let mut nbits = 0;
    for &c in s.as_bytes() {
        if c == b'=' {
            break;
        }
        let v = val(c);
        if v < 0 {
            continue;
        }
        acc = (acc << 6) | v as u32;
        nbits += 6;
        if nbits >= 8 {
            nbits -= 8;
            out.push((acc >> nbits) as u8);
        }
    }
    out
}

pub(crate) fn json_escape(s: &str) -> String {
    let mut o = String::with_capacity(s.len() + 8);
    for c in s.chars() {
        match c {
            '"' | '\\' => {
                o.push('\\');
                o.push(c);
            }
            '\n' => o.push_str("\\n"),
            '\r' => {}
            c => o.push(c),
        }
    }
    o
}

pub(crate) fn sanitize_name(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        if c == '/' || c == '\\' {
            out.clear();
            continue;
        }
        if c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '-' {
            out.push(c);
        }
    }
    if out.is_empty() || out.starts_with('.') {
        out = format!("src{out}");
    }
    out
}

pub(crate) fn read_full(s: &mut impl Read, buf: &mut [u8]) -> bool {
    s.read_exact(buf).is_ok()
}

pub(crate) fn reply(fd: &mut TcpStream, code: u16, text: &str, body: &[u8], ctype: &str) {
    let hdr = format!(
        "HTTP/1.1 {} {}\r\nContent-Length: {}\r\nContent-Type: {}\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Headers: content-type\r\nAccess-Control-Allow-Methods: GET,POST,OPTIONS\r\nConnection: close\r\n\r\n",
        code, text, body.len(), ctype
    );
    let _ = fd.write_all(hdr.as_bytes());
    let _ = fd.write_all(body);
}

pub(crate) fn reply_str(fd: &mut TcpStream, code: u16, text: &str, body: &str, ctype: &str) {
    reply(fd, code, text, body.as_bytes(), ctype);
}

pub(crate) fn uds_connect(path: &str, timeout_ms: u64) -> Option<std::os::unix::net::UnixStream> {
    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(timeout_ms);
    loop {
        match std::os::unix::net::UnixStream::connect(path) {
            Ok(s) => {
                s.set_read_timeout(Some(std::time::Duration::from_secs(5))).ok();
                return Some(s);
            }
            Err(e)
                if (e.kind() == std::io::ErrorKind::NotFound
                    || e.raw_os_error() == Some(libc::ECONNREFUSED))
                    && std::time::Instant::now() < deadline =>
            {
                std::thread::sleep(std::time::Duration::from_millis(20));
            }
            Err(_) => return None,
        }
    }
}

