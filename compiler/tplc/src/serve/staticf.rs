use super::*;

/* ---------------- static files ---------------- */

pub(crate) fn mime_for(path: &Path) -> &'static str {
    match path.extension().and_then(|x| x.to_str()).unwrap_or("") {
        "html" => "text/html; charset=utf-8",
        "js" | "mjs" => "text/javascript; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "json" | "map" | "webmanifest" => "application/json",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "ico" => "image/x-icon",
        "wasm" => "application/wasm",
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        "ttf" => "font/ttf",
        "txt" | "md" | "st" => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}

pub(crate) fn pct_decode(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'%' && i + 2 < b.len() {
            if let Ok(h) = u8::from_str_radix(&s[i + 1..i + 3], 16) {
                out.push(h);
                i += 3;
                continue;
            }
        }
        out.push(if b[i] == b'+' { b' ' } else { b[i] });
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

pub(crate) fn serve_static(fd: &mut TcpStream, webui: &str, path: &str) {
    // sanitize: no .., decode %XX
    let rel = pct_decode(path.trim_start_matches('/'));
    if rel.contains("..") {
        reply_str(fd, 403, "Forbidden", "bad path", "text/plain");
        return;
    }
    let root = Path::new(webui);
    let mut fp = root.join(&rel);
    if fp.is_dir() {
        fp = fp.join("index.html");
    }
    let mut body = std::fs::read(&fp).ok();
    if body.is_none() {
        // SPA fallback: unknown non-file paths → index.html
        let last = rel.rsplit('/').next().unwrap_or("");
        if !last.contains('.') {
            body = std::fs::read(root.join("index.html")).ok();
        }
    }
    match body {
        Some(b) => reply(fd, 200, "OK", &b, mime_for(&fp)),
        None => reply_str(fd, 404, "Not Found", "not found", "text/plain"),
    }
}

