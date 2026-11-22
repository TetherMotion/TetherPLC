use super::*;

/* ---------------- ctl UDS helpers ---------------- */

pub(crate) fn ctl_frame(msg_type: u16, flags: u16, req: u64, payload: &[u8]) -> Vec<u8> {
    let mut fr = Vec::with_capacity(16 + payload.len());
    fr.extend_from_slice(&msg_type.to_le_bytes());
    fr.extend_from_slice(&flags.to_le_bytes());
    fr.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    fr.extend_from_slice(&req.to_le_bytes());
    fr.extend_from_slice(payload);
    fr
}

/// HELLO handshake on an already-connected ctl socket.
pub(crate) fn ctl_hello(s: &mut std::os::unix::net::UnixStream, name: &str) -> bool {
    let mut pl = Vec::with_capacity(8 + 2 + name.len());
    pl.extend_from_slice(&3u16.to_le_bytes()); // proto ver
    pl.extend_from_slice(&0u16.to_le_bytes());
    pl.extend_from_slice(&0u32.to_le_bytes()); // caps
    pl.extend_from_slice(&(name.len() as u16).to_le_bytes());
    pl.extend_from_slice(name.as_bytes());
    if s.write_all(&ctl_frame(op::HELLO, 0, 1, &pl)).is_err() {
        return false;
    }
    let mut hdr = [0u8; 16];
    if !read_full(s, &mut hdr) {
        return false;
    }
    let rlen = u32::from_le_bytes(hdr[4..8].try_into().unwrap()) as usize;
    if rlen > (1 << 20) {
        return false;
    }
    let mut pl = vec![0u8; rlen];
    if rlen > 0 && !read_full(s, &mut pl) {
        return false;
    }
    u16::from_le_bytes([hdr[0], hdr[1]]) == op::HELLO_ACK && hdr[3] & 0x80 == 0
}

/// SET_NOTIFY with the all-events mask; drains the ack.
pub(crate) fn ctl_notify_all(s: &mut std::os::unix::net::UnixStream) -> bool {
    if s
        .write_all(&ctl_frame(op::SET_NOTIFY, 0, 2, &0xFFFF_FFFFu32.to_le_bytes()))
        .is_err()
    {
        return false;
    }
    let mut hdr = [0u8; 16];
    if !read_full(s, &mut hdr) {
        return false;
    }
    let rlen = u32::from_le_bytes(hdr[4..8].try_into().unwrap()) as usize;
    if rlen > (1 << 20) {
        return false;
    }
    let mut pl = vec![0u8; rlen];
    if rlen > 0 && !read_full(s, &mut pl) {
        return false;
    }
    hdr[3] & 0x80 == 0
}

