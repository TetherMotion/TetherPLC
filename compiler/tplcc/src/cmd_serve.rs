use std::process::ExitCode;
use crate::helpers::*;
pub(crate) fn cmd_serve(args: &[String]) -> ExitCode {
    use std::io::Write;
    use std::os::unix::net::UnixListener;

    let sock = args
        .first()
        .map(String::as_str)
        .unwrap_or("/tmp/tplccd.sock");
    let _ = std::fs::remove_file(sock);
    let listener = match UnixListener::bind(sock) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("tplcc serve: bind {sock}: {e}");
            return ExitCode::FAILURE;
        }
    };
    eprintln!("tplcc serve: listening on {sock}");
    for conn in listener.incoming() {
        match conn {
            Ok(mut s) => {
                let reply = serve_check(&mut s);
                let _ = s.write_all(reply.as_bytes());
            }
            Err(e) => eprintln!("tplcc serve: accept: {e}"),
        }
    }
    ExitCode::SUCCESS
}

fn serve_check(s: &mut std::os::unix::net::UnixStream) -> String {
    use std::io::Read;
    let mut rd = |n: usize| -> Option<Vec<u8>> {
        let mut b = vec![0u8; n];
        s.read_exact(&mut b).ok()?;
        Some(b)
    };
    let u16be = |b: &[u8]| u16::from_le_bytes([b[0], b[1]]) as usize;
    let frame_err = |msg: &str| -> String {
        serde_json::json!({"compile": {"ok": false, "errors": 1,
            "diags": [{"code": "TPLC_E_DRIVER", "severity": "error",
                       "message": msg, "notes": []}]}, "exit": 2})
        .to_string()
    };
    let Some(hdr) = rd(2) else {
        return frame_err("short frame");
    };
    let count = u16be(&hdr);
    if count > 64 {
        return frame_err("too many files");
    }
    let mut srcs: Vec<(String, String)> = Vec::with_capacity(count);
    for _ in 0..count {
        let Some(nl) = rd(2) else {
            return frame_err("short name len");
        };
        let Some(nb) = rd(u16be(&nl)) else {
            return frame_err("short name");
        };
        let Some(cl) = rd(4) else {
            return frame_err("short content len");
        };
        let clen = u32::from_le_bytes([cl[0], cl[1], cl[2], cl[3]]) as usize;
        if clen > 4 * 1024 * 1024 {
            return frame_err("file too large");
        }
        let Some(cb) = rd(clen) else {
            return frame_err("short content");
        };
        let name = String::from_utf8_lossy(&nb).into_owned();
        let text = String::from_utf8_lossy(&cb).into_owned();
        srcs.push((name, text));
    }
    let (mut out, eff) = compile_srcs(&srcs);
    let mut diags = std::mem::take(&mut out.diags);
    if !diags.iter().any(|d| d.severity == tplc_st::Severity::Error) {
        diags.extend(tplc_st::sema::check(&mut out.module));
    }
    let errs = diags
        .iter()
        .filter(|d| d.severity == tplc_st::Severity::Error)
        .count();
    serde_json::json!({
        "compile": diag_report(&diags, &eff, errs, None),
        "exit": if errs > 0 { 1 } else { 0 },
    })
    .to_string()
}

