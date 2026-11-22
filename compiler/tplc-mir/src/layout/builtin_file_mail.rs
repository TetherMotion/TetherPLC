//! builtin_fb_layout arms — file/mail domain.
use super::*;

        /* FB_FtpConn — tplc-ftp service bridge (FTP client,
        IPv4 string, PORT 0 = 21. */
pub(super) fn arm_fb_ftp_conn(off: u64, m: &mut Members) -> u64 {
    let mut push = |o: &mut u64, n: &str, t: Type| {
        let a = t.align();
        *o = (*o + a - 1) & !(a - 1);
        m.insert(n.to_string(), (*o, t.clone()));
        *o += t.size();
    };
    let mut off = off;
            let udint = || Type::Int {
                signed: false,
                bits: 32,
            };
            let dint = || Type::Int {
                signed: true,
                bits: 32,
            };
            push(&mut off, "HOST", Type::Str { cap: 15 });
            push(&mut off, "PORT", udint());
            push(&mut off, "USER", Type::Str { cap: 31 });
            push(&mut off, "PASS", Type::Str { cap: 31 });
            push(&mut off, "TIMEOUT", Type::Time);
            push(&mut off, "CONN_HANDLE", udint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "CLOSE", Type::Bool);
            push(&mut off, "EXEC", Type::Bool);
            push(&mut off, "BUSY", Type::Bool);
            push(&mut off, "DONE", Type::Bool);
            push(&mut off, "__last_exec", Type::Bool);
            push(&mut off, "__last_close", Type::Bool);
    off
}

        /* FB_FtpCmd — tplc-ftp service bridge for one FTP
        (RETR/STOR/LIST/PASV) must use FB_FtpXfer. */
pub(super) fn arm_fb_ftp_cmd(off: u64, m: &mut Members) -> u64 {
    let mut push = |o: &mut u64, n: &str, t: Type| {
        let a = t.align();
        *o = (*o + a - 1) & !(a - 1);
        m.insert(n.to_string(), (*o, t.clone()));
        *o += t.size();
    };
    let mut off = off;
            let udint = || Type::Int {
                signed: false,
                bits: 32,
            };
            let dint = || Type::Int {
                signed: true,
                bits: 32,
            };
            push(&mut off, "CONN_HANDLE", udint());
            push(&mut off, "CMD", Type::Str { cap: 127 });
            push(&mut off, "TIMEOUT", Type::Time);
            push(&mut off, "HANDLE", udint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "RESP_CODE", udint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "TEXT", Type::Str { cap: 127 });
            push(&mut off, "EXEC", Type::Bool);
            push(&mut off, "BUSY", Type::Bool);
            push(&mut off, "DONE", Type::Bool);
            push(&mut off, "__last_exec", Type::Bool);
    off
}

        /* FB_FtpXfer — tplc-ftp service bridge for one RETR /
        bytes. One outstanding transfer per connection. */
pub(super) fn arm_fb_ftp_xfer(off: u64, m: &mut Members) -> u64 {
    let mut push = |o: &mut u64, n: &str, t: Type| {
        let a = t.align();
        *o = (*o + a - 1) & !(a - 1);
        m.insert(n.to_string(), (*o, t.clone()));
        *o += t.size();
    };
    let mut off = off;
            let udint = || Type::Int {
                signed: false,
                bits: 32,
            };
            let dint = || Type::Int {
                signed: true,
                bits: 32,
            };
            let byte = || Type::Int {
                signed: false,
                bits: 8,
            };
            push(&mut off, "CONN_HANDLE", udint());
            push(&mut off, "OP", udint());
            push(&mut off, "PATH", Type::Str { cap: 127 });
            push(&mut off, "TIMEOUT", Type::Time);
            push(&mut off, "HANDLE", udint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "RESP_CODE", udint());
            push(&mut off, "CHUNK", udint());
            push(&mut off, "TOTAL", udint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "UP_LEN", udint());
            push(&mut off, "UP_OFF", udint());
            push(
                &mut off,
                "BUF",
                Type::Array {
                    elem: Box::new(byte()),
                    lo: 0,
                    hi: 511,
                },
            );
            push(
                &mut off,
                "UP_DATA",
                Type::Array {
                    elem: Box::new(byte()),
                    lo: 0,
                    hi: 511,
                },
            );
            push(&mut off, "EXEC", Type::Bool);
            push(&mut off, "BUSY", Type::Bool);
            push(&mut off, "DONE", Type::Bool);
            push(&mut off, "__last_exec", Type::Bool);
            push(&mut off, "__feed_done", Type::Bool);
    off
}

        /* FB_SmtpSend — tplc-smtp service bridge (SMTP client,
        reply code; ERRID transport errno. */
pub(super) fn arm_fb_smtp_send(off: u64, m: &mut Members) -> u64 {
    let mut push = |o: &mut u64, n: &str, t: Type| {
        let a = t.align();
        *o = (*o + a - 1) & !(a - 1);
        m.insert(n.to_string(), (*o, t.clone()));
        *o += t.size();
    };
    let mut off = off;
            let udint = || Type::Int {
                signed: false,
                bits: 32,
            };
            let dint = || Type::Int {
                signed: true,
                bits: 32,
            };
            push(&mut off, "HOST", Type::Str { cap: 15 });
            push(&mut off, "PORT", udint());
            push(&mut off, "USER", Type::Str { cap: 31 });
            push(&mut off, "PASS", Type::Str { cap: 31 });
            push(&mut off, "FROM", Type::Str { cap: 63 });
            push(&mut off, "RCPT", Type::Str { cap: 63 });
            push(&mut off, "SUBJECT", Type::Str { cap: 63 });
            push(&mut off, "TIMEOUT", Type::Time);
            push(&mut off, "HANDLE", udint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "RESP_CODE", udint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "BODY", Type::Str { cap: 255 });
            push(&mut off, "EXEC", Type::Bool);
            push(&mut off, "BUSY", Type::Bool);
            push(&mut off, "DONE", Type::Bool);
            push(&mut off, "__last_exec", Type::Bool);
            push(&mut off, "__feed_done", Type::Bool);
    off
}
