use super::*;


/// One ctl frame carrying a TplcCtlError payload (flags bit15).
pub(crate) fn ctl_error_frame(req_type: u16, req: u64, msg: &str) -> Vec<u8> {
    let mut p = Vec::with_capacity(10 + msg.len());
    p.extend_from_slice(&(-1i32).to_le_bytes()); // TPLC_E_IO
    p.extend_from_slice(&0u32.to_le_bytes()); // subcode
    p.extend_from_slice(&(msg.len() as u16).to_le_bytes());
    p.extend_from_slice(msg.as_bytes());
    ctl_frame(req_type, 0x8000, req, &p)
}

pub(crate) fn handle_ws(mut cfd: TcpStream, head: String, cfg: &ServeCfg, cache: &Arc<Mutex<(String, bool)>>) {
    // ctl backend is best-effort: meta-ops (compile/blocks/…) work
    // without a runtime; ctl frames get a synthesized F_ERROR reply.
    // Handshake + meta worker come up FIRST — uds_connect retries a
    // missing ctl socket for up to 2s and must not stall the ws upgrade
    // or the meta-ops that never needed the runtime (iface_list took a
    // 2s+ hit on every connect before this reorder).
    if !ws_handshake(&mut cfd, &head) {
        return;
    }
    cfd.set_read_timeout(None).ok();
    cfd.set_nodelay(true).ok();

    let cfg2 = cfg.clone();
    let cache2 = cache.clone();
    let cfd_w = match cfd.try_clone() {
        Ok(s) => Arc::new(Mutex::new(s)),
        Err(_) => return,
    };
    let (tx, rx) = std::sync::mpsc::channel::<serde_json::Value>();

    // meta-op worker: JSON text requests → JSON text replies
    {
        let w = cfd_w.clone();
        std::thread::spawn(move || {
            while let Ok(req) = rx.recv() {
                let id = req.get("id").cloned().unwrap_or(serde_json::Value::Null);
                let mut rep = meta_dispatch(&cfg2, &cache2, &req);
                rep["id"] = id;
                let text = rep.to_string();
                let mut g = w.lock().unwrap();
                if !ws_send_frame(&mut g, 0x1, text.as_bytes()) {
                    break;
                }
            }
        });
    }

    /* ctl UDS connects in the background — set OnceLock when the retry
       budget resolves either way. ctl-frame writers wait on it (same
       ≤2s semantics as before); meta-ops and the ws upgrade don't. */
    let uds_slot = Arc::new(std::sync::OnceLock::<Mutex<Option<UnixStream>>>::new());
    {
        let slot = uds_slot.clone();
        let sock = cfg.ctl_sock.clone();
        let w = cfd_w.clone();
        std::thread::spawn(move || {
            let mut uds = uds_connect(&sock, 2000);
            if let Some(s) = uds.as_mut() {
                s.set_read_timeout(None).ok();
                if !ctl_hello(s, "tplc-webui") || !ctl_notify_all(s) {
                    uds = None;
                }
            }
            // UDS → WS: every ctl frame (replies + F_EVENT pushes) →
            // binary msg — spawned once the stream exists.
            if let Some(uds0) = uds {
                if let Ok(mut uds_r) = uds0.try_clone() {
                    uds_r.set_read_timeout(None).ok();
                    let w2 = w.clone();
                    std::thread::spawn(move || {
                        let mut hdr = [0u8; 16];
                        loop {
                            if !read_full(&mut uds_r, &mut hdr) {
                                break;
                            }
                            let len =
                                u32::from_le_bytes(hdr[4..8].try_into().unwrap()) as usize;
                            if len > 8 * 1024 * 1024 {
                                break;
                            }
                            let mut fr = hdr.to_vec();
                            fr.resize(16 + len, 0);
                            if len > 0 && !read_full(&mut uds_r, &mut fr[16..]) {
                                break;
                            }
                            let mut g = w2.lock().unwrap();
                            if !ws_send_frame(&mut g, 0x2, &fr) {
                                break;
                            }
                        }
                        // ctl went away — drop the ws too so the client reconnects
                        let _ = uds_r.shutdown(std::net::Shutdown::Both);
                    });
                }
                let _ = slot.set(Mutex::new(Some(uds0)));
            } else {
                let _ = slot.set(Mutex::new(None));
            }
        });
    }

    // WS → UDS (this thread): binary → ctl frame, text → meta channel
    let mut wsmsg: Vec<u8> = Vec::new();
    let mut ws_is_text = false;
    loop {
        let Some((op, payload, fin)) = ws_read_frame(&mut cfd) else { break };
        match op {
            0x8 => {
                let _ = ws_send_frame(&mut cfd, 0x8, &payload);
                break;
            }
            0x9 => {
                let _ = ws_send_frame(&mut cfd, 0xA, &payload);
            }
            0xA => {}
            0x0 | 0x1 | 0x2 => {
                if op != 0x0 {
                    wsmsg.clear();
                    ws_is_text = op == 0x1;
                }
                wsmsg.extend_from_slice(&payload);
                if !fin {
                    continue;
                }
                let msg = std::mem::take(&mut wsmsg);
                if ws_is_text {
                    if let Ok(v) = serde_json::from_slice::<serde_json::Value>(&msg) {
                        if tx.send(v).is_err() {
                            break;
                        }
                    }
                } else if msg.len() >= 16 {
                    // wait (bounded by the connect budget) for the ctl
                    // backend — the connect thread sets the slot whether
                    // it landed a stream or exhausted retries.
                    let wrote = match uds_slot.wait().lock().unwrap().as_mut() {
                        Some(u) => u.write_all(&msg).is_ok(),
                        None => false,
                    };
                    if !wrote {
                        // no runtime: parse req id and error the call
                        let t = u16::from_le_bytes([msg[0], msg[1]]);
                        let req = u64::from_le_bytes(msg[8..16].try_into().unwrap());
                        let err = ctl_error_frame(t, req, "ctl backend down");
                        let mut g = cfd_w.lock().unwrap();
                        if !ws_send_frame(&mut g, 0x2, &err) {
                            break;
                        }
                    }
                }
            }
            _ => {}
        }
        if STOP.load(Ordering::Relaxed) {
            break;
        }
    }
    if let Some(mu) = uds_slot.get() {
        if let Some(u) = mu.lock().unwrap().take() {
            let _ = u.shutdown(std::net::Shutdown::Both);
        }
    }
}

