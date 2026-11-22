//! builtin_fb_layout arms — iec domain.
use super::*;

        /* FB_Iec870Conn — tplc-iec870 service bridge (IEC
        CA_SIZE/COT_SIZE 0=2 (1 or 2 octets each). */
pub(super) fn arm_fb_iec870_conn(off: u64, m: &mut Members) -> u64 {
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
            push(&mut off, "CA_SIZE", udint());
            push(&mut off, "COT_SIZE", udint());
            push(&mut off, "TIMEOUT", Type::Time);
            push(&mut off, "CONN_HANDLE", udint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "SUBSTATE", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "RX_DROPPED", udint());
            push(&mut off, "CLOSE", Type::Bool);
            push(&mut off, "EXEC", Type::Bool);
            push(&mut off, "BUSY", Type::Bool);
            push(&mut off, "DONE", Type::Bool);
            push(&mut off, "__last_exec", Type::Bool);
            push(&mut off, "__last_close", Type::Bool);
    off
}

        /* FB_Iec870Tx — tplc-iec870 service bridge for one ASDU
        transport errno. */
pub(super) fn arm_fb_iec870_tx(off: u64, m: &mut Members) -> u64 {
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
            push(&mut off, "TYPE_ID", udint());
            push(&mut off, "VSQ", udint());
            push(&mut off, "COT", udint());
            push(&mut off, "CA", udint());
            push(&mut off, "LEN", udint());
            push(&mut off, "HANDLE", udint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(
                &mut off,
                "PAYLOAD",
                Type::Array {
                    elem: Box::new(byte()),
                    lo: 0,
                    hi: 239,
                },
            );
            push(&mut off, "EXEC", Type::Bool);
            push(&mut off, "BUSY", Type::Bool);
            push(&mut off, "DONE", Type::Bool);
            push(&mut off, "__last_exec", Type::Bool);
    off
}

        /* FB_Iec870Rx — tplc-iec870 receive poll. Call every
        sequence number (diagnostic). */
pub(super) fn arm_fb_iec870_rx(off: u64, m: &mut Members) -> u64 {
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
            let byte = || Type::Int {
                signed: false,
                bits: 8,
            };
            push(&mut off, "CONN_HANDLE", udint());
            push(&mut off, "TYPE_ID", udint());
            push(&mut off, "VSQ", udint());
            push(&mut off, "COT", udint());
            push(&mut off, "CA", udint());
            push(&mut off, "LEN", udint());
            push(&mut off, "I_FRAME_SEQ", udint());
            push(&mut off, "ERRID", udint());
            push(
                &mut off,
                "PAYLOAD",
                Type::Array {
                    elem: Box::new(byte()),
                    lo: 0,
                    hi: 239,
                },
            );
            push(&mut off, "DONE", Type::Bool);
    off
}

        /* FB_Iec61850 — tplc-iec61850 service bridge (IEC 61850 MMS
        structured as their MmsValue string). */
pub(super) fn arm_fb_iec61850(off: u64, m: &mut Members) -> u64 {
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
            push(&mut off, "OP", udint());
            push(&mut off, "HANDLE", udint());
            push(&mut off, "HOST", Type::Str { cap: 63 });
            push(&mut off, "PORT", udint());
            push(&mut off, "DOMAIN", Type::Str { cap: 63 });
            push(&mut off, "ITEM", Type::Str { cap: 127 });
            push(&mut off, "VAL", Type::Str { cap: 255 });
            push(&mut off, "STATE", dint());
            push(&mut off, "INVOKE", udint());
            push(&mut off, "LASTERR", udint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            /* RCB ops (appended): OP 7=GetRCBValues (RCBREF ->
               RPTID/DATASET/RPTENA/RESV/BUFFERED/CONFREV/BUFTM/
               INTGPD/SQNUM/OPTFLDS/TRGOPS), 8=SetRCBValues (RCBREF +
               FLAGS bit0=rptEna bit1=resv bit2=GI), 9=report poll
               (-> VAL lines, RCNT bytes). */
            push(&mut off, "RCBREF", Type::Str { cap: 127 });
            push(&mut off, "FLAGS", udint());
            push(&mut off, "RPTENA", dint());
            push(&mut off, "RESV", dint());
            push(&mut off, "BUFFERED", dint());
            push(&mut off, "RPTID", Type::Str { cap: 31 });
            push(&mut off, "DATASET", Type::Str { cap: 63 });
            push(&mut off, "CONFREV", udint());
            push(&mut off, "BUFTM", udint());
            push(&mut off, "INTGPD", udint());
            push(&mut off, "SQNUM", udint());
            push(&mut off, "OPTFLDS", udint());
            push(&mut off, "TRGOPS", udint());
            push(&mut off, "RCNT", udint());
            /* control ops (appended): OP 10=control (CTRLREF + CMODE
               1=select 2=selectWithValue 3=operate 4=cancel + CTLVAL
               -> VAL echo), 11=control poll (-> CTLINVOKE/CTLSUCCESS/
               CTLERR/CTLTYPE/CTLNUM/CTLPEND). */
            push(&mut off, "CTRLREF", Type::Str { cap: 127 });
            push(&mut off, "CTLVAL", Type::Str { cap: 63 });
            push(&mut off, "CMODE", udint());
            push(&mut off, "CTLINVOKE", udint());
            push(&mut off, "CTLSUCCESS", dint());
            push(&mut off, "CTLERR", udint());
            push(&mut off, "CTLTYPE", udint());
            push(&mut off, "CTLNUM", udint());
            push(&mut off, "CTLPEND", udint());
            /* RCB snapshot extras (appended): OP 7 also fills
               GI/PURGEBUF/RESVTMS/ENTRYID. */
            push(&mut off, "GI", dint());
            push(&mut off, "PURGEBUF", dint());
            push(&mut off, "RESVTMS", udint());
            push(&mut off, "ENTRYID", udint());
            /* data-set ops (appended): OP 12=ReadDataSet (DSREF ->
               VAL "FCDA=value" lines, DSCOUNT elements), OP
               13=GetDataSetDirectory (DSREF -> VAL FCDA list). */
            push(&mut off, "DSREF", Type::Str { cap: 127 });
            push(&mut off, "DSCOUNT", udint());
    off
}
