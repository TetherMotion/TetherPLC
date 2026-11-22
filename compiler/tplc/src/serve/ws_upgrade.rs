use super::*;

/* ---------------- /ws — the primary UI channel ----------------
 *
 * One WebSocket per client, backed by a dedicated ctl UDS connection
 * (HELLO + SET_NOTIFY all-events at connect).
 *
 *   binary message → verbatim ctl frame forwarded to the runtime;
 *                    every reply/event frame comes back as a binary
 *                    message (client demuxes by req_id / F_EVENT).
 *   text message   → JSON meta-op, replied as a text message:
 *     {"id":N,"op":"compile","build":bool,"files":[{name,content}]}
 *     {"id":N,"op":"blocks","refresh":bool}
 *     {"id":N,"op":"libsrc","lib":"x","file":"y.st"}
 *     {"id":N,"op":"pkgmap","package":"<b64>","map":"<text>"}
 *     → {"id":N,"ok":true,"data":…}  or  {"id":N,"ok":false,"error":"…"}
 */

/// `Upgrade:` header presence, case-insensitively — HTTP field names are
/// case-insensitive (RFC 9110 §5.1); browsers send `Upgrade:` but node's
/// undici WebSocket lowercases to `upgrade:`.
pub(crate) fn wants_ws_upgrade(head: &str) -> bool {
    head.lines().any(|l| l.to_lowercase().starts_with("upgrade:"))
}

pub(crate) fn ws_accept(head: &str) -> Option<String> {
    let key = head
        .lines()
        .find(|l| l.to_lowercase().starts_with("sec-websocket-key:"))
        .and_then(|l| l.split_once(':').map(|x| x.1.trim().to_string()))?;
    if key.is_empty() {
        return None;
    }
    let magic = format!("{key}258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
    Some(b64e(&sha1(magic.as_bytes())))
}

pub(crate) fn ws_handshake(cfd: &mut TcpStream, head: &str) -> bool {
    let Some(accept) = ws_accept(head) else { return false };
    let resp = format!(
        "HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {accept}\r\n\r\n"
    );
    cfd.write_all(resp.as_bytes()).is_ok()
}

