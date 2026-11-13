//! State-blob layout algorithm (doc 07 §3).
//!
//! Per program instance: collect fields into regions
//! `[global | fb_instances | task_local | retain | internal]`, order
//! each region by qualified name, assign aligned offsets, emit the
//! `layout_desc` (LAYD) field table.

use crate::ir::*;
use std::collections::BTreeMap;
use tplc_pkg::sections::{Layd, LaydField, LaydRegion};

pub const REGION_GLOBAL: u8 = 0;
pub const REGION_FB: u8 = 1;
pub const REGION_TASK_LOCAL: u8 = 2;
pub const REGION_RETAIN: u8 = 3;
pub const REGION_INTERNAL: u8 = 4;

#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub offset: u64,
    pub size: u64,
    pub ty: Type,
    pub retain: bool,
    pub is_const: bool,
    pub field_kind: u8, // 0 var, 1 fb_inst, 4 retain_wrapper
}

/// Layout of one FB *type* (doc 07 §3.2): iface + internal vars +
/// bookkeeping, declaration order.
#[derive(Debug, Clone)]
pub struct FbLayout {
    pub size: u64,
    pub align: u64,
    /// member name → (offset, type). Includes builtin bookkeeping fields.
    pub members: BTreeMap<String, (u64, Type)>,
}

/// Complete blob layout for one program instance.
#[derive(Debug)]
pub struct Layout {
    pub total_size: u64,
    pub fields: BTreeMap<String, FieldInfo>, // qualified path → leaf field
    pub fb_types: BTreeMap<String, FbLayout>,
    pub retain_offset: u64,
    pub retain_size: u64,
    pub align: u64,
    pub layd: Layd,
}

impl Layout {
    /// Layout for a library package: only the FB type table; no
    /// program fields, image regions, or retain area.
    pub fn library(fb_types: BTreeMap<String, FbLayout>) -> Layout {
        Layout {
            total_size: 0,
            fields: BTreeMap::new(),
            fb_types,
            retain_offset: 0,
            retain_size: 0,
            align: 1,
            layd: Layd {
                version: 1,
                total_size: 0,
                regions: vec![],
                fields: vec![],
            },
        }
    }
}

/// FB layouts for every FUNCTION_BLOCK in the module — the library
/// counterpart of the pre-layout pass inside `layout_program`.
pub fn library_fb_types(module: &Module) -> BTreeMap<String, FbLayout> {
    let mut fb_types = BTreeMap::new();
    for pou in &module.pous {
        if pou.kind == PouKind::FunctionBlock {
            let l = layout_fb(pou, &mut fb_types);
            fb_types.insert(pou.name.clone(), l);
        }
    }
    fb_types
}

/// Builtin FB bookkeeping layouts (doc 07 §3.2 rt fields). The names are
/// reserved (`__` prefix) and live inside the instance sub-object.
pub fn builtin_fb_layout(name: &str) -> Option<FbLayout> {
    let mut m = BTreeMap::new();
    let mut push = |o: &mut u64, n: &str, t: Type| {
        let a = t.align();
        *o = (*o + a - 1) & !(a - 1);
        m.insert(n.to_string(), (*o, t.clone()));
        *o += t.size();
    };
    let mut off = 0u64;
    match name {
        "TON" | "TOF" | "TP" => {
            push(&mut off, "IN", Type::Bool);
            push(&mut off, "PT", Type::Time);
            push(&mut off, "Q", Type::Bool);
            push(&mut off, "ET", Type::Time);
            push(&mut off, "__in_last", Type::Bool);
            push(&mut off, "__start", Type::Time);
            push(&mut off, "__run", Type::Bool);
        }
        "R_TRIG" | "F_TRIG" => {
            push(&mut off, "CLK", Type::Bool);
            push(&mut off, "Q", Type::Bool);
            push(&mut off, "__clk_last", Type::Bool);
        }
        "CTU" | "CTD" | "CTUD" => {
            push(&mut off, "CU", Type::Bool);
            push(&mut off, "CD", Type::Bool);
            push(&mut off, "R", Type::Bool);
            push(&mut off, "LD", Type::Bool);
            push(
                &mut off,
                "PV",
                Type::Int {
                    signed: true,
                    bits: 32,
                },
            );
            push(&mut off, "QU", Type::Bool);
            push(&mut off, "QD", Type::Bool);
            push(
                &mut off,
                "CV",
                Type::Int {
                    signed: true,
                    bits: 32,
                },
            );
            push(&mut off, "__cu_last", Type::Bool);
            push(&mut off, "__cd_last", Type::Bool);
        }
        "RS" | "SR" => {
            push(&mut off, "S", Type::Bool);
            push(&mut off, "R1", Type::Bool);
            push(&mut off, "S1", Type::Bool);
            push(&mut off, "R", Type::Bool);
            push(&mut off, "Q1", Type::Bool);
        }
        /* FB_Socket — tplc-net service bridge (plan 21 C2). One
        generic op-multiplexed FB; the compat layer's
        FB_SocketConnect/Listen/Accept/Send/Receive/Close wrap
        it with Beckhoff-shaped signatures. DATA is a STRING
        (Beckhoff uses POINTER TO BYTE — no pointers in ST). */
        "FB_Socket" => {
            let dint = || Type::Int {
                signed: true,
                bits: 32,
            };
            let udint = || Type::Int {
                signed: false,
                bits: 32,
            };
            push(&mut off, "OP", dint());
            push(&mut off, "HOST", Type::Str { cap: 15 });
            push(&mut off, "PORT", udint());
            push(&mut off, "TIMEOUT", Type::Time);
            push(&mut off, "DATA", Type::Str { cap: 256 });
            push(&mut off, "DLEN", udint());
            push(&mut off, "HANDLE", udint());
            push(&mut off, "AUX", udint());
            push(&mut off, "RESULT", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "EXEC", Type::Bool);
            push(&mut off, "BUSY", Type::Bool);
            push(&mut off, "DONE", Type::Bool);
            push(&mut off, "__last_exec", Type::Bool);
            push(&mut off, "__deadline", Type::Time);
            push(&mut off, "__sent", udint());
            push(&mut off, "__recvd", udint());
            push(&mut off, "__phase", dint());
        }
        /* FB_Serial — tplc-serial service bridge. Same op-multiplexed
        shape as FB_Socket; DEV is the device path (STRING, no
        pointers in ST), DATA the bounded payload. */
        "FB_Serial" => {
            let dint = || Type::Int {
                signed: true,
                bits: 32,
            };
            let udint = || Type::Int {
                signed: false,
                bits: 32,
            };
            push(&mut off, "OP", dint());
            push(&mut off, "DEV", Type::Str { cap: 63 });
            push(&mut off, "BAUD", udint());
            push(&mut off, "PARITY", udint());
            push(&mut off, "DBITS", udint());
            push(&mut off, "SBITS", udint());
            push(&mut off, "FLAGS", udint());
            push(&mut off, "AUX", udint());
            push(&mut off, "TIMEOUT", Type::Time);
            push(&mut off, "DATA", Type::Str { cap: 256 });
            push(&mut off, "DLEN", udint());
            push(&mut off, "HANDLE", udint());
            push(&mut off, "RESULT", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "EXEC", Type::Bool);
            push(&mut off, "BUSY", Type::Bool);
            push(&mut off, "DONE", Type::Bool);
            push(&mut off, "__last_exec", Type::Bool);
            push(&mut off, "__deadline", Type::Time);
            push(&mut off, "__sent", udint());
            push(&mut off, "__recvd", udint());
            push(&mut off, "__phase", dint());
        }
        /* FB_Ads — tplc-ads service bridge (AMS/TCP ADS client).
        Same op-multiplexed shape as FB_Socket/FB_Serial; STATE and
        DEVSTATE carry the ADS/device states for read-state and
        write-control, RESULT carries the ADS-layer error code. */
        "FB_Ads" => {
            let dint = || Type::Int {
                signed: true,
                bits: 32,
            };
            let udint = || Type::Int {
                signed: false,
                bits: 32,
            };
            push(&mut off, "OP", dint());
            push(&mut off, "HOST", Type::Str { cap: 15 });
            push(&mut off, "PORT", udint());
            push(&mut off, "IDXGRP", udint());
            push(&mut off, "IDXOFF", udint());
            push(&mut off, "STATE", dint());
            push(&mut off, "DEVSTATE", dint());
            push(&mut off, "AUX", udint());
            push(&mut off, "TIMEOUT", Type::Time);
            push(&mut off, "DATA", Type::Str { cap: 256 });
            push(&mut off, "DLEN", udint());
            push(&mut off, "HANDLE", udint());
            push(&mut off, "RESULT", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "EXEC", Type::Bool);
            push(&mut off, "BUSY", Type::Bool);
            push(&mut off, "DONE", Type::Bool);
            push(&mut off, "__last_exec", Type::Bool);
            push(&mut off, "__deadline", Type::Time);
            push(&mut off, "__sent", udint());
            push(&mut off, "__recvd", udint());
            push(&mut off, "__phase", dint());
        }
        /* FB_RtMon — tplc-rtmon service bridge (Tc3_RealtimeMonitoring).
        OP selects the op: 1=mark (append CONTEXT/GROUP/EVENT/MARKCTRL),
        2=count (RESULT=retained marks), 3=get (AUX=index, fills
        CONTEXT/GROUP/EVENT/MARKCTRL from the record), 4=clear.
        Marks are synchronous O(1) ring appends, so no deadline/poll
        state. */
        "FB_RtMon" => {
            let dint = || Type::Int {
                signed: true,
                bits: 32,
            };
            let udint = || Type::Int {
                signed: false,
                bits: 32,
            };
            push(&mut off, "OP", dint());
            push(&mut off, "CONTEXT", udint());
            push(&mut off, "GROUP", udint());
            push(&mut off, "EVENT", udint());
            push(&mut off, "MARKCTRL", udint());
            push(&mut off, "AUX", udint());
            push(&mut off, "RESULT", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "EXEC", Type::Bool);
            push(&mut off, "BUSY", Type::Bool);
            push(&mut off, "DONE", Type::Bool);
            push(&mut off, "__last_exec", Type::Bool);
        }
        /* FB_Modbus — tplc-modbus service bridge (Modbus TCP client).
        One transaction per EXEC edge: FUNC is the Modbus function
        code, DATA is a 256-byte array carrying the request PDU data
        in (RLEN bytes) and the response PDU data out (the service
        copies the request at begin, so the buffer can be reused),
        DLEN the response length, RESULT 0=ok /1=exception (EXC =
        exception code), ERRID transport errors. The service enforces
        TIMEOUT itself, so no deadline fields. */
        "FB_Modbus" => {
            let dint = || Type::Int {
                signed: true,
                bits: 32,
            };
            let udint = || Type::Int {
                signed: false,
                bits: 32,
            };
            let byte = || Type::Int {
                signed: false,
                bits: 8,
            };
            push(&mut off, "HOST", Type::Str { cap: 15 });
            push(&mut off, "PORT", udint());
            push(&mut off, "UNIT", udint());
            push(&mut off, "TIMEOUT", Type::Time);
            push(&mut off, "FUNC", udint());
            push(
                &mut off,
                "DATA",
                Type::Array {
                    elem: Box::new(byte()),
                    lo: 0,
                    hi: 255,
                },
            );
            push(&mut off, "RLEN", udint());
            push(&mut off, "DLEN", udint());
            push(&mut off, "HANDLE", udint());
            push(&mut off, "RESULT", dint());
            push(&mut off, "EXC", udint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "EXEC", Type::Bool);
            push(&mut off, "BUSY", Type::Bool);
            push(&mut off, "DONE", Type::Bool);
            push(&mut off, "__last_exec", Type::Bool);
            /* UDP selects the datagram transport (mb_begin_udp);
            appended last so earlier member offsets stay fixed */
            push(&mut off, "UDP", Type::Bool);
        }
        /* FB_ModbusRtu — tplc-modbus RTU transport (Modbus RTU
        master). Same transaction shape as FB_Modbus but over a
        serial device: DEV is the device path, BAUD/PARITY/DBITS/
        SBITS the termios settings (tplc-serial numeric codes),
        UNIT the station address. DATA/RLEN/DLEN/RESULT/EXC/ERRID
        mirror FB_Modbus. The service enforces TIMEOUT itself. */
        "FB_ModbusRtu" => {
            let dint = || Type::Int {
                signed: true,
                bits: 32,
            };
            let udint = || Type::Int {
                signed: false,
                bits: 32,
            };
            let byte = || Type::Int {
                signed: false,
                bits: 8,
            };
            push(&mut off, "DEV", Type::Str { cap: 63 });
            push(&mut off, "BAUD", udint());
            push(&mut off, "PARITY", udint());
            push(&mut off, "DBITS", udint());
            push(&mut off, "SBITS", udint());
            push(&mut off, "UNIT", udint());
            push(&mut off, "TIMEOUT", Type::Time);
            push(&mut off, "FUNC", udint());
            push(
                &mut off,
                "DATA",
                Type::Array {
                    elem: Box::new(byte()),
                    lo: 0,
                    hi: 255,
                },
            );
            push(&mut off, "RLEN", udint());
            push(&mut off, "DLEN", udint());
            push(&mut off, "HANDLE", udint());
            push(&mut off, "RESULT", dint());
            push(&mut off, "EXC", udint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "EXEC", Type::Bool);
            push(&mut off, "BUSY", Type::Bool);
            push(&mut off, "DONE", Type::Bool);
            push(&mut off, "__last_exec", Type::Bool);
        }
        /* FB_SacnSource — tplc-sacn source role (ANSI E1.31
        streaming ACN). ENABLE rising edge opens the source (config
        fields are latched at begin), falling edge closes it; while
        enabled the service transmits the DMX frame every INTERVAL
        ms. START_CODE is the DMX start code byte, DMX the slot
        data, DLEN its byte count (<=512). SENT/AUX count data and
        sync+discovery packets, SEQ the last data-packet sequence,
        LAST_ERR the last socket errno. */
        "FB_SacnSource" => {
            let udint = || Type::Int {
                signed: false,
                bits: 32,
            };
            let dint = || Type::Int {
                signed: true,
                bits: 32,
            };
            let ulint = || Type::Int {
                signed: false,
                bits: 64,
            };
            let byte = || Type::Int {
                signed: false,
                bits: 8,
            };
            push(&mut off, "ENABLE", Type::Bool);
            push(&mut off, "UNIVERSE", udint());
            push(&mut off, "PRIORITY", udint());
            push(&mut off, "SYNC_ADDR", udint());
            push(&mut off, "INTERVAL", udint());
            push(&mut off, "TTL", udint());
            push(&mut off, "OPTIONS", byte());
            push(&mut off, "FLAGS", byte());
            push(&mut off, "START_CODE", byte());
            push(&mut off, "NAME", Type::Str { cap: 63 });
            push(
                &mut off,
                "CID",
                Type::Array {
                    elem: Box::new(byte()),
                    lo: 0,
                    hi: 15,
                },
            );
            push(&mut off, "DLEN", udint());
            push(
                &mut off,
                "DMX",
                Type::Array {
                    elem: Box::new(byte()),
                    lo: 0,
                    hi: 511,
                },
            );
            push(&mut off, "HANDLE", udint());
            push(&mut off, "SEQ", udint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "LAST_ERR", dint());
            push(&mut off, "SENT", ulint());
            push(&mut off, "AUX", ulint());
            push(&mut off, "BUSY", Type::Bool);
            push(&mut off, "__err_latch", Type::Bool);
        }
        /* FB_SacnReceiver — tplc-sacn receiver role. ENABLE rising
        edge binds the sACN port and joins the universe multicast
        group(s), falling edge closes. While enabled each scan drains
        pending datagrams; the outputs mirror the latest accepted
        data/sync/discovery packet. DMX holds the latest payload
        (DLEN bytes), SRC_NAME the sender name, UNIS the first 64
        universes of the last discovery list (UNI_COUNT the full
        count). Counters are u64. */
        "FB_SacnReceiver" => {
            let udint = || Type::Int {
                signed: false,
                bits: 32,
            };
            let ulint = || Type::Int {
                signed: false,
                bits: 64,
            };
            let u16t = || Type::Int {
                signed: false,
                bits: 16,
            };
            let byte = || Type::Int {
                signed: false,
                bits: 8,
            };
            push(&mut off, "ENABLE", Type::Bool);
            push(&mut off, "RCV_UNIVERSE", udint());
            push(&mut off, "SYNC_UNIVERSE", udint());
            push(&mut off, "PORT", udint());
            push(&mut off, "DATA_RX", ulint());
            push(&mut off, "DATA_DROP", ulint());
            push(&mut off, "SYNC_RX", ulint());
            push(&mut off, "DISC_RX", ulint());
            push(&mut off, "REMOTE_IP", udint());
            push(&mut off, "HANDLE", udint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "DLEN", udint());
            push(&mut off, "SYNC_ADDR", u16t());
            push(&mut off, "UNIVERSE", u16t());
            push(&mut off, "SYNC_ADDR_SEEN", u16t());
            push(&mut off, "UNI_COUNT", u16t());
            push(&mut off, "START_CODE", byte());
            push(&mut off, "SEQ", byte());
            push(&mut off, "PRIORITY", byte());
            push(&mut off, "SYNC_SEQ", byte());
            push(&mut off, "UNI_PAGE", byte());
            push(&mut off, "UNI_LAST", byte());
            push(&mut off, "HAVE_DATA", Type::Bool);
            push(&mut off, "PREVIEW", Type::Bool);
            push(&mut off, "STREAM_TERM", Type::Bool);
            push(&mut off, "FORCE_SYNC", Type::Bool);
            push(&mut off, "HAVE_SYNC", Type::Bool);
            push(&mut off, "HAVE_DISC", Type::Bool);
            push(&mut off, "BUSY", Type::Bool);
            push(&mut off, "SRC_NAME", Type::Str { cap: 63 });
            push(
                &mut off,
                "DMX",
                Type::Array {
                    elem: Box::new(byte()),
                    lo: 0,
                    hi: 511,
                },
            );
            push(
                &mut off,
                "UNIS",
                Type::Array {
                    elem: Box::new(u16t()),
                    lo: 0,
                    hi: 63,
                },
            );
            push(&mut off, "__err_latch", Type::Bool);
        }
        /* FB_Bacnet — tplc-bacnet service bridge (BACnet/IP client,
        ASHRAE 135 over UDP 47808). One transaction per EXEC edge:
        FUNC selects Who-Is (1) / ReadProperty (2) / WriteProperty
        (3); HOST is a dotted-quad peer ("" or "255.255.255.255" =
        broadcast for Who-Is), PORT 0 = 47808. OBJ_TYPE/OBJ_INST/
        PROP_ID address the object/property; HAS_IDX+PROP_IDX an
        optional array index; HAS_PRIO+PRIORITY a write priority;
        HAS_RANGE+RANGE_LO/HI the Who-Is device-instance window.
        VAL_TYPE/VLEN/VAL carry the write value as raw application-
        tag payload bytes. Outputs mirror TplcBacnetRespInfo:
        STATUS 0=pending 1=ok 2=timeout 3=error 4=badresp, R_* the
        read-ack fields, R_VAL the returned value payload, and the
        DEV_* who-is responder table (DEV_ID = type<<22|instance,
        DEV_IP flat 4-bytes-per-device). The service enforces
        TIMEOUT itself. */
        "FB_Bacnet" => {
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
            push(&mut off, "HOST", Type::Str { cap: 15 });
            push(&mut off, "PORT", udint());
            push(&mut off, "FUNC", udint());
            push(&mut off, "OBJ_TYPE", udint());
            push(&mut off, "OBJ_INST", udint());
            push(&mut off, "PROP_ID", udint());
            push(&mut off, "HAS_IDX", Type::Bool);
            push(&mut off, "HAS_PRIO", Type::Bool);
            push(&mut off, "HAS_RANGE", Type::Bool);
            push(&mut off, "PROP_IDX", udint());
            push(&mut off, "PRIORITY", udint());
            push(&mut off, "RANGE_LO", udint());
            push(&mut off, "RANGE_HI", udint());
            push(&mut off, "VAL_TYPE", udint());
            push(&mut off, "VLEN", udint());
            push(&mut off, "TIMEOUT", Type::Time);
            push(
                &mut off,
                "VAL",
                Type::Array {
                    elem: Box::new(byte()),
                    lo: 0,
                    hi: 255,
                },
            );
            push(&mut off, "HANDLE", udint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "R_SERVICE", udint());
            push(&mut off, "R_APDU", udint());
            push(&mut off, "R_OBJ_TYPE", udint());
            push(&mut off, "R_OBJ_INST", udint());
            push(&mut off, "R_PROP_ID", udint());
            push(&mut off, "R_VTYPE", udint());
            push(&mut off, "R_VLEN", udint());
            push(
                &mut off,
                "R_VAL",
                Type::Array {
                    elem: Box::new(byte()),
                    lo: 0,
                    hi: 255,
                },
            );
            push(&mut off, "DEV_COUNT", udint());
            push(
                &mut off,
                "DEV_ID",
                Type::Array {
                    elem: Box::new(udint()),
                    lo: 0,
                    hi: 7,
                },
            );
            push(
                &mut off,
                "DEV_IP",
                Type::Array {
                    elem: Box::new(byte()),
                    lo: 0,
                    hi: 31,
                },
            );
            push(&mut off, "DEV_VENDOR", udint());
            push(&mut off, "DEV_MAX_APDU", udint());
            push(&mut off, "DEV_SEG", udint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "EXEC", Type::Bool);
            push(&mut off, "BUSY", Type::Bool);
            push(&mut off, "DONE", Type::Bool);
            push(&mut off, "__last_exec", Type::Bool);
        }
        /* FB_Pjlink — tplc-pjlink service bridge (JBMIA PJLink
        projector client, TCP 4352). One transaction per EXEC edge:
        HOST is a dotted-quad projector address, PORT 0 = 4352,
        CLS the PJLink class digit (0..9, standard commands use
        1), CMD the 4-char command body ("POWR","INPT","AVMT",
        "ERST","INFO","NAME","INFM","LAMP", ...), PARAM the
        transmission parameter ("?" for queries, <=128 chars),
        PASSWORD the optional PJLink auth password (<=32 chars;
        when the projector greets with `PJLINK 1 <nonce>` the
        service prepends MD5(nonce+password) automatically).
        R_VALUE is the response value text ("1","OK","ERR3",...),
        R_CLS/R_CMD echo the response header, R_VLEN the full
        value length, AUTHED 1 when the digest exchange ran.
        STATUS 0=pending 1=ok 2=timeout 3=error 4=badresp
        5=authfail. The service enforces TIMEOUT itself. */
        "FB_Pjlink" => {
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
            push(&mut off, "CLS", udint());
            push(&mut off, "CMD", Type::Str { cap: 4 });
            push(&mut off, "PARAM", Type::Str { cap: 128 });
            push(&mut off, "PASSWORD", Type::Str { cap: 32 });
            push(&mut off, "TIMEOUT", Type::Time);
            push(&mut off, "HANDLE", udint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "R_ERR", dint());
            push(&mut off, "R_CLS", udint());
            push(&mut off, "R_CMD", udint());
            push(&mut off, "R_VLEN", udint());
            push(&mut off, "R_VALUE", Type::Str { cap: 127 });
            push(&mut off, "AUTHED", Type::Bool);
            push(&mut off, "EXEC", Type::Bool);
            push(&mut off, "BUSY", Type::Bool);
            push(&mut off, "DONE", Type::Bool);
            push(&mut off, "__last_exec", Type::Bool);
        }
        /* FB_S7Conn — tplc-s7 service bridge (Siemens S7comm
        client, RFC1006/TCP 102). EXEC rising edge opens a
        persistent connection (TCP connect -> COTP CR/CC -> S7
        Setup Communication) polled via BUSY until STATUS leaves
        0=pending; 1=ready, 2=error (ERRID carries errno or the
        S7 error word). CONN_HANDLE stays valid while STATUS=1
        and feeds FB_S7Io. CLOSE rising edge tears the session
        down (CONN_HANDLE -> 0). HOST is a dotted-quad IPv4
        string, PORT 0 = 102, RACK 0..15, SLOT 0..31 folded
        into the COTP called TSAP. */
        "FB_S7Conn" => {
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
            push(&mut off, "RACK", udint());
            push(&mut off, "SLOT", udint());
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
        }
        /* FB_S7Io — tplc-s7 service bridge for one Read Var /
        Write Var transaction on an FB_S7Conn connection.
        CONN_HANDLE comes from FB_S7Conn.CONN_HANDLE. MODE 0=read 1=write. AREA is the S7-Any area
        code (0x81 PE, 0x82 PA, 0x83 MK, 0x84 DB, 0x1C CT,
        0x1D TM); DB the data block number; BYTE_OFF the byte
        offset (24-bit); BIT_ACCESS+BIT_NO select single-bit
        access (BIT_NO 0..7); LEN the byte count for word access
        (<=240). DATA carries the write payload (LEN bytes,
        DATA[0]&1 for bit writes); R_DATA receives read bytes
        (DATA_LEN valid). STATUS 0=pending 1=ok 2=timeout
        3=error 4=badresp 5=s7err; ITEM_RC is the response item
        return code (0xFF ok); ERRID the transport errno or S7
        error word. One outstanding transaction per connection. */
        "FB_S7Io" => {
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
            push(&mut off, "MODE", udint());
            push(&mut off, "AREA", udint());
            push(&mut off, "DB", udint());
            push(&mut off, "BYTE_OFF", udint());
            push(&mut off, "BIT_ACCESS", Type::Bool);
            push(&mut off, "BIT_NO", udint());
            push(&mut off, "LEN", udint());
            push(&mut off, "TIMEOUT", Type::Time);
            push(&mut off, "HANDLE", udint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ITEM_RC", udint());
            push(&mut off, "DATA_LEN", udint());
            push(&mut off, "ERRID", udint());
            push(
                &mut off,
                "DATA",
                Type::Array {
                    elem: Box::new(byte()),
                    lo: 0,
                    hi: 239,
                },
            );
            push(
                &mut off,
                "R_DATA",
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
        }
        /* FB_S5Conn — tplc-s5 service bridge (Siemens S5, RK512
        job protocol over TCP with 3964 STX/ETX+BCC framing).
        EXEC rising edge opens a connection (plain TCP connect;
        the RK512 layer has no handshake) polled via BUSY until
        STATUS leaves 0=pending; 1=ready, 2=error (ERRID carries
        errno). CONN_HANDLE stays valid while STATUS=1 and feeds
        FB_S5Io. CLOSE rising edge tears the session down
        (CONN_HANDLE -> 0). HOST is a dotted-quad IPv4 string,
        PORT 0 = 102, TIMEOUT bounds the connect. */
        "FB_S5Conn" => {
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
        }
        /* FB_S5Io — tplc-s5 service bridge for one RK512 FETCH
        / SEND transaction on an FB_S5Conn connection.
        CONN_HANDLE comes from FB_S5Conn.CONN_HANDLE. MODE
        0=read(FETCH) 1=write(SEND). DB is the data block
        number (0..255); BYTE_OFF an even byte offset within the
        DB (<=510, the wire addresses words); LEN the byte count
        (<=510). DATA carries the write payload (LEN bytes);
        R_DATA receives read bytes (DATA_LEN valid, wire-order
        big-endian words). STATUS 0=pending 1=ok 2=timeout
        3=error 4=badresp 5=s5err; ERRID the transport errno or
        RK512 status byte. One outstanding transaction per
        connection. */
        "FB_S5Io" => {
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
            push(&mut off, "MODE", udint());
            push(&mut off, "DB", udint());
            push(&mut off, "BYTE_OFF", udint());
            push(&mut off, "LEN", udint());
            push(&mut off, "TIMEOUT", Type::Time);
            push(&mut off, "HANDLE", udint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "DATA_LEN", udint());
            push(
                &mut off,
                "DATA",
                Type::Array {
                    elem: Box::new(byte()),
                    lo: 0,
                    hi: 509,
                },
            );
            push(
                &mut off,
                "R_DATA",
                Type::Array {
                    elem: Box::new(byte()),
                    lo: 0,
                    hi: 509,
                },
            );
            push(&mut off, "EXEC", Type::Bool);
            push(&mut off, "BUSY", Type::Bool);
            push(&mut off, "DONE", Type::Bool);
            push(&mut off, "__last_exec", Type::Bool);
        }
        /* FB_FtpConn — tplc-ftp service bridge (FTP client,
        RFC 959 over TCP). EXEC rising edge opens a control
        connection (TCP connect -> 220 greeting -> USER ->
        PASS) polled via BUSY until STATUS leaves
        0=pending; 1=ready, 2=error (ERRID carries errno).
        CONN_HANDLE stays valid while STATUS=1 and feeds
        FB_FtpCmd/FB_FtpXfer. CLOSE rising edge tears the
        session down (CONN_HANDLE -> 0). HOST is a dotted-quad
        IPv4 string, PORT 0 = 21. */
        "FB_FtpConn" => {
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
        }
        /* FB_FtpCmd — tplc-ftp service bridge for one FTP
        control command on an FB_FtpConn connection
        (MKD/RMD/DELE/RNFR/RNTO/TYPE/SIZE/CWD/PWD/SYST/NOOP/
        SITE/...). CONN_HANDLE comes from FB_FtpConn.
        RESP_CODE is the final 3-digit reply code; TEXT the
        reply text (last line, NUL-trimmed). STATUS
        0=pending 1=ok (2xx) 2=fail (4xx/5xx) 3=timeout
        4=error 5=badresp; ERRID the transport errno. One
        outstanding command per connection. Data commands
        (RETR/STOR/LIST/PASV) must use FB_FtpXfer. */
        "FB_FtpCmd" => {
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
        }
        /* FB_FtpXfer — tplc-ftp service bridge for one RETR /
        STOR / LIST transfer on an FB_FtpConn connection.
        CONN_HANDLE comes from FB_FtpConn. OP 0=RETR
        (download), 1=STOR (upload), 2=LIST/NLST (download
        listing). For RETR/LIST each BUSY scan copies up to
        512 received bytes into BUF (CHUNK = bytes this
        scan, TOTAL cumulative); STATUS 6=more while data
        flows, then 1=ok / 2=fail / 3=timeout / 4=error /
        5=badresp terminal with RESP_CODE the final reply
        code. For STOR the rt feeds UP_DATA[0..UP_LEN) into
        the upload automatically; UP_OFF tracks accepted
        bytes. One outstanding transfer per connection. */
        "FB_FtpXfer" => {
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
        }
        /* FB_SmtpSend — tplc-smtp service bridge (SMTP client,
        RFC 5321 over TCP 25/587, plain text + optional AUTH
        LOGIN). EXEC rising edge begins one send transaction:
        connect -> 220 -> EHLO -> [AUTH LOGIN] -> MAIL FROM ->
        RCPT TO -> DATA -> service-generated headers + BODY
        body (dot-stuffed) -> "." -> QUIT. BODY is a STRING[255]
        fed automatically on the DATA phase; empty BODY sends
        headers only. STATUS 0=pending 1=ok 2=fail(smtp code)
        3=timeout 4=error 5=badresp; RESP_CODE the last SMTP
        reply code; ERRID transport errno. */
        "FB_SmtpSend" => {
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
        }
        /* FB_Iec870Conn — tplc-iec870 service bridge (IEC
        60870-5-104 client, TCP 2404). EXEC rising edge opens a
        connection (TCP connect -> STARTDT act/con) polled via
        BUSY until STATUS leaves 0=pending; 1=ready, 2=error
        (ERRID carries errno or a protocol code). SUBSTATE
        0=none 1=tcp 2=startdt 3=active. CONN_HANDLE stays valid
        while STATUS=1 and feeds FB_Iec870Tx/FB_Iec870Rx. CLOSE
        rising edge sends STOPDT act and tears the session down
        (CONN_HANDLE -> 0). HOST dotted-quad IPv4, PORT 0=2404,
        CA_SIZE/COT_SIZE 0=2 (1 or 2 octets each). */
        "FB_Iec870Conn" => {
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
        }
        /* FB_Iec870Tx — tplc-iec870 service bridge for one ASDU
        send on an FB_Iec870Conn connection. CONN_HANDLE comes
        from FB_Iec870Conn.CONN_HANDLE. TYPE_ID/VSQ/COT/CA are
        the raw ASDU header fields (VSQ bit7=sequence flag,
        bits0..6 object count; COT bits0..7 encoded COT octet,
        bits8..15 originator address sent only when COT_SIZE=2;
        CA low octets per CA_SIZE). PAYLOAD carries the
        info-object bytes (LEN <= 240). One outstanding tx per
        connection. STATUS 0=pending 1=ok 2=error; ERRID the
        transport errno. */
        "FB_Iec870Tx" => {
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
        }
        /* FB_Iec870Rx — tplc-iec870 receive poll. Call every
        scan with CONN_HANDLE from FB_Iec870Conn; DONE pulses
        TRUE on the scan an ASDU is delivered (consume-on-read,
        one ASDU per DONE). TYPE_ID/VSQ/COT/CA echo the raw ASDU
        header fields parsed per the conn's COT_SIZE/CA_SIZE;
        LEN the info-object byte count; PAYLOAD the info-object
        bytes (LEN valid, <=240). I_FRAME_SEQ the I-frame send
        sequence number (diagnostic). */
        "FB_Iec870Rx" => {
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
        }
        /* FB_MotionAxis — tplc-motion service bridge (simulated
        CiA402/DS402-shaped axis). Synchronous op dispatch: set
        OP + fields and call; STATUS reports the service return
        code (for OP=6 info it carries the DS402 state). OP
        1=create (uses the CFG fields, returns AXIS handle),
        2=destroy, 3=poll (advance all axes), 4=read object
        (INDEX -> VALUE), 5=write object (INDEX+VALUE), 6=info
        (full axis snapshot into MODE/FAULT/TARGET_REACHED/
        BUSY/POS_ACTUAL/VEL_ACTUAL/TORQUE_ACTUAL). All service
        calls are immediate; the compat layer gates create/
        destroy invocation, so no EXEC edge is needed. */
        "FB_MotionAxis" => {
            let udint = || Type::Int {
                signed: false,
                bits: 32,
            };
            let dint = || Type::Int {
                signed: true,
                bits: 32,
            };
            push(&mut off, "OP", udint());
            push(&mut off, "AXIS", udint());
            push(&mut off, "UNITS_PER_REV", dint());
            push(&mut off, "MAX_VEL", dint());
            push(&mut off, "MAX_ACCEL", dint());
            push(&mut off, "MAX_DECEL", dint());
            push(&mut off, "POS_MIN", dint());
            push(&mut off, "POS_MAX", dint());
            push(&mut off, "HOME_OFFSET", dint());
            push(&mut off, "FOLLOW_WINDOW", dint());
            push(&mut off, "FLAGS", udint());
            push(&mut off, "INDEX", udint());
            push(&mut off, "VALUE", dint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "MODE", dint());
            push(&mut off, "FAULT", dint());
            push(&mut off, "TARGET_REACHED", dint());
            push(&mut off, "BUSY", dint());
            push(&mut off, "POS_ACTUAL", dint());
            push(&mut off, "VEL_ACTUAL", dint());
        }
        /* FB_NciChan — tplc-nci service bridge (NC interpreter channel
        on a Tether IO endpoint). Synchronous op dispatch like
        FB_MotionAxis: set OP + fields and call. OP 1=load program
        text (PROG string), 2=load program file on the server host
        (PATH string), 3=command (CMD code), 4=status (fills
        STATE/LINE/TOTAL/NERR/PROGRESS/DURATION). STATUS carries the
        service return code, ERRID its positive form; NERR is the
        channel's own error code from the last status read. */
        "FB_NciChan" => {
            let udint = || Type::Int {
                signed: false,
                bits: 32,
            };
            let dint = || Type::Int {
                signed: true,
                bits: 32,
            };
            push(&mut off, "OP", udint());
            push(&mut off, "CHAN", udint());
            push(&mut off, "PATH", Type::Str { cap: 127 });
            push(&mut off, "PROG", Type::Str { cap: 4095 });
            push(&mut off, "CMD", udint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "STATE", dint());
            push(&mut off, "LINE", udint());
            push(&mut off, "TOTAL", udint());
            push(&mut off, "NERR", dint());
            push(&mut off, "PROGRESS", dint());
            push(&mut off, "DURATION", dint());
            push(&mut off, "AUX", udint());
            push(&mut off, "RVAL", Type::Real { bits: 64 });
            push(&mut off, "ZSX", Type::Real { bits: 64 });
            push(&mut off, "ZSY", Type::Real { bits: 64 });
            push(&mut off, "ZSZ", Type::Real { bits: 64 });
            push(&mut off, "MFUNC", udint());
            push(&mut off, "FIFOLEN", udint());
            push(&mut off, "FIFOSEQ", udint());
        }
        /* FB_UaClient — tplc-opcua service bridge (OPC UA client via
        libopen62541). Synchronous op dispatch like FB_NciChan: set
        OP + fields and call. OP 1=connect (URL string → HANDLE),
        2=disconnect, 3=status (→STATE), 4=read (NODE string → VAL/TYPE
        or SVAL for strings), 5=write (NODE + VAL/TYPE), 6=namespace
        index lookup (URL string reused as the URI → IDX). STATUS
        carries the service return code, ERRID its positive form.
        Numeric values widen into VAL (LREAL); signed/unsigned 64-bit
        nodes lose precision above 2^53 — an honest LREAL limit. */
        "FB_UaClient" => {
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
            push(&mut off, "URL", Type::Str { cap: 127 });
            push(&mut off, "NODE", Type::Str { cap: 255 });
            push(&mut off, "SVAL", Type::Str { cap: 255 });
            push(&mut off, "VAL", Type::Real { bits: 64 });
            push(&mut off, "VARTYPE", udint());
            push(&mut off, "STATE", dint());
            push(&mut off, "IDX", udint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
        }
        /* FB_Eip — tplc-eip service bridge (EtherNet/IP explicit
        messaging). Synchronous op dispatch like FB_UaClient: set OP +
        fields and call. OP 1=open+RegisterSession (HOST/PORT →
        HANDLE), 2=close+unregister, 3=status (→ STATE/SESSION/
        LASTCIP), 4=ListIdentity (HOST/PORT → DATA summary),
        5=Get_Attribute_Single (CLS/INST/ATTR → DATA bytes +
        DATALEN), 6=Set_Attribute_Single (CLS/INST/ATTR + DATA/DATALEN).
        STATUS carries the service return code, ERRID its positive
        form. Attribute payloads travel as bounded STRING bytes
        (TwinCAT uses PVOID buffers — the string is the practical
        carrier here). */
        "FB_Eip" => {
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
            push(&mut off, "CLS", udint());
            push(&mut off, "INST", udint());
            push(&mut off, "ATTR", udint());
            push(&mut off, "DATALEN", udint());
            push(&mut off, "DATA", Type::Str { cap: 255 });
            push(&mut off, "STATE", dint());
            push(&mut off, "SESSION", udint());
            push(&mut off, "LASTCIP", udint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
        }
        /* FB_Iec61850 — tplc-iec61850 service bridge (IEC 61850 MMS
        client via libiec61850). Synchronous op dispatch like FB_Eip:
        set OP + fields and call. OP 1=connect+associate (HOST/PORT →
        HANDLE), 2=close+release, 3=status (→ STATE/INVOKE/LASTERR),
        4=read (DOMAIN/ITEM → VAL), 5=write (DOMAIN/ITEM + VAL),
        6=GetNameList (DOMAIN → VAL), 7=GetRCBValues (RCBREF → RCB
        attrs), 8=SetRCBValues (RCBREF + FLAGS), 9=report poll
        (→ VAL/RCNT). STATUS carries the service
        return code, ERRID its positive form. Data attributes are
        addressed by domain (LD) + item (VariableAccess name); values
        travel as bounded STRING[255] (numerics as their ASCII repr,
        structured as their MmsValue string). */
        "FB_Iec61850" => {
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
        }
        /* FB_Mqtt — tplc-mqtt service bridge (MQTT client via
        libpaho-mqtt3c). Synchronous op dispatch like FB_Eip: set OP +
        fields and call. OP 1=connect (HOST/PORT/CLEAN/KEEPALIVE ->
        HANDLE), 2=disconnect, 3=status (-> STATE/LASTMSGID/LASTERR),
        4=publish (TOPIC/PAYLOAD/QOS/RETAIN), 5=subscribe (TOPIC/QOS),
        6=unsubscribe (TOPIC), 7=receive (-> RXTOPIC/RXPAYLOAD/RXQOS/
        RXMSGID/RXPLEN or NOMSG), 8=isConnected (-> CONNECTED).
        STATUS carries the service return code, ERRID its positive
        form.  Topics are bounded STRING[127], payloads STRING[255]. */
        "FB_Mqtt" => {
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
            push(&mut off, "CLEAN", udint());
            push(&mut off, "KEEPALIVE", udint());
            push(&mut off, "TOPIC", Type::Str { cap: 127 });
            push(&mut off, "PAYLOAD", Type::Str { cap: 255 });
            push(&mut off, "QOS", udint());
            push(&mut off, "FLG_RETAIN", udint());
            push(&mut off, "RXTOPIC", Type::Str { cap: 127 });
            push(&mut off, "RXPAYLOAD", Type::Str { cap: 255 });
            push(&mut off, "RXQOS", udint());
            push(&mut off, "RXMSGID", udint());
            push(&mut off, "RXPLEN", udint());
            push(&mut off, "RXRETAIN", udint());
            push(&mut off, "STATE", dint());
            push(&mut off, "LASTMSGID", udint());
            push(&mut off, "LASTERR", dint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "CONNECTED", udint());
            /* last-will + abort (appended): OP1 reads WILLTOPIC/
               WILLPAYLOAD/WILLQOS/WILLRETAIN; OP9 aborts the
               connection without DISCONNECT so the broker fires
               the will. */
            push(&mut off, "WILLTOPIC", Type::Str { cap: 127 });
            push(&mut off, "WILLPAYLOAD", Type::Str { cap: 255 });
            push(&mut off, "WILLQOS", udint());
            push(&mut off, "WILLRETAIN", udint());
            /* TLS (appended): OP1 reads USETLS/TLSINSECURE/
               TLSCAFILE/TLSCERTFILE/TLSKEYFILE into the connect
               cfg's ssl fields. */
            push(&mut off, "USETLS", udint());
            push(&mut off, "TLSINSECURE", udint());
            push(&mut off, "TLSCAFILE", Type::Str { cap: 127 });
            push(&mut off, "TLSCERTFILE", Type::Str { cap: 127 });
            push(&mut off, "TLSKEYFILE", Type::Str { cap: 127 });
            /* broker-message age (appended): OP3 copies
               TplcMqttStatus.last_msg_age_ms here. */
            push(&mut off, "LASTMSGSINCE", udint());
            /* backoff (appended): OP10 reads BACKOFFMIN/MAX/EN ->
               mqtt_backoff vtable op. */
            push(&mut off, "BACKOFFMIN", udint());
            push(&mut off, "BACKOFFMAX", udint());
            push(&mut off, "BACKOFFEN", udint());
            /* MQTT5 (appended): OP1 reads MQTTVER (3|5) + the five
               connect-property udints into the cfg tail. */
            push(&mut off, "MQTTVER", udint());
            push(&mut off, "M5SESSIONEXP", udint());
            push(&mut off, "M5RECVMAX", udint());
            push(&mut off, "M5TOPICALIAS", udint());
            push(&mut off, "M5REQRESP", udint());
            push(&mut off, "M5REQPROB", udint());
            /* MQTT5 publish properties (appended): OP11 reads these
               into TplcMqtt5PubProps for mqtt_publish5. */
            push(&mut off, "M5PUBFMT", udint());
            push(&mut off, "M5PUBEXPIRY", udint());
            push(&mut off, "M5PUBCTYPE", Type::Str { cap: 127 });
            push(&mut off, "M5PUBRTOPIC", Type::Str { cap: 127 });
            push(&mut off, "M5PUBCORR", Type::Str { cap: 127 });
            push(&mut off, "M5PUBCORR_LEN", udint());
            push(&mut off, "M5PUBUPROPS", Type::Str { cap: 255 });
            /* MQTT5 subscribe properties (appended): OP12 reads these
               into TplcMqtt5SubProps for mqtt_subscribe5. */
            push(&mut off, "M5SUBNOLOCAL", udint());
            push(&mut off, "M5SUBRAP", udint());
            push(&mut off, "M5SUBRH", udint());
            push(&mut off, "M5SUBSID", udint());
            push(&mut off, "M5SUBUPROPS", Type::Str { cap: 255 });
            /* MQTT5 receive properties (appended): OP13 reads these
               from TplcMqttRx5 for mqtt_receive5. */
            push(&mut off, "M5RXFMT", udint());
            push(&mut off, "M5RXEXPIRY", udint());
            push(&mut off, "M5RXSUBID", udint());
            push(&mut off, "M5RXCTYPE", Type::Str { cap: 63 });
            push(&mut off, "M5RXRTOPIC", Type::Str { cap: 127 });
            push(&mut off, "M5RXUPROPS", Type::Str { cap: 255 });
            push(&mut off, "M5RXCORR_LEN", udint());
            push(&mut off, "M5RXCORR", Type::Str { cap: 127 });
        }
        /* FB_EcSync — tplc-ecsync service bridge (EtherCAT DC sync
        state over Tether IO ec.dc.* params). Synchronous op dispatch:
        OP 1=status (-> CONNECTED/SYNCEN/PARAMSOK/LASTERR/DIFFNS/
        REFNS), 2=set_sync (ENABLE -> STATUS). STATUS carries the
        service return code, ERRID its positive form. */
        "FB_EcSync" => {
            let udint = || Type::Int {
                signed: false,
                bits: 32,
            };
            let dint = || Type::Int {
                signed: true,
                bits: 32,
            };
            push(&mut off, "OP", udint());
            push(&mut off, "ENABLE", udint());
            push(&mut off, "CONNECTED", dint());
            push(&mut off, "SYNCEN", dint());
            push(&mut off, "PARAMSOK", dint());
            push(&mut off, "LASTERR", dint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "DIFFNS", Type::Real { bits: 64 });
            push(&mut off, "REFNS", Type::Int {
                signed: false,
                bits: 64,
            });
        }
        /* FB_Ocpp — tplc-ocpp service bridge (OCPP-J client over
        WebSocket + JSON-RPC). Synchronous op dispatch like FB_Eip:
        set OP + fields and call. OP 1=connect (HOST/PORT/PATH/OCPPID
        -> HANDLE), 2=disconnect, 3=status (-> STATE/RXTOT/TXTOT/
        LASTERR), 4=call (MSGID/ACTION/PAYLOAD), 5=result (MSGID/
        PAYLOAD), 6=callerror (MSGID/CODE/DESC/PAYLOAD), 7=receive
        (-> RXACTION/RXPAYLOAD/RXMSGTYPE or NOMSG). STATUS carries
        the service return code, ERRID its positive form. */
        "FB_Ocpp" => {
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
            push(&mut off, "PATH", Type::Str { cap: 63 });
            push(&mut off, "OCPPID", Type::Str { cap: 63 });
            push(&mut off, "MSGID", Type::Str { cap: 63 });
            push(&mut off, "ACT", Type::Str { cap: 63 });
            push(&mut off, "CODE", Type::Str { cap: 63 });
            push(&mut off, "DESC", Type::Str { cap: 127 });
            push(&mut off, "PAYLOAD", Type::Str { cap: 255 });
            push(&mut off, "RXACTION", Type::Str { cap: 63 });
            push(&mut off, "RXPAYLOAD", Type::Str { cap: 255 });
            push(&mut off, "RXMSGTYPE", udint());
            push(&mut off, "STATE", dint());
            push(&mut off, "RXTOT", udint());
            push(&mut off, "TXTOT", udint());
            push(&mut off, "LASTERR", dint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
        }
        /* FB_Aes70 — tplc-aes70 service bridge (OCA/AES70 audio
        control over OSCP TCP). Synchronous op dispatch like FB_Eip:
        set OP + fields and call. OP 1=connect (HOST/PORT -> HANDLE),
        2=disconnect, 3=status (-> STATE/RXTOT/TXTOT/LASTERR),
        4=call (TARGET/METHOD/PARAMS -> HANDLE2), 5=keepalive,
        6=receive (-> HANDLE2/STATUSCODE/RXPAYLOAD or NOMSG).
        STATUS carries the service return code, ERRID its positive
        form.  TARGET is the OCA object number, METHOD the
        defLevel<<8 + methodIndex, PARAMS/RXPAYLOAD bounded
        STRING[255] raw bytes. */
        "FB_Aes70" => {
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
            push(&mut off, "TARGET", udint());
            push(&mut off, "METH", udint());
            push(&mut off, "PARAMS", Type::Str { cap: 255 });
            push(&mut off, "HANDLE2", udint());
            push(&mut off, "STATUSCODE", udint());
            push(&mut off, "RXPAYLOAD", Type::Str { cap: 255 });
            push(&mut off, "STATE", dint());
            push(&mut off, "RXTOT", udint());
            push(&mut off, "TXTOT", udint());
            push(&mut off, "LASTERR", dint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
        }
        /* FB_Dmx — tplc-dmx service bridge (DMX512 output over an
        opt-in serial port). Synchronous op dispatch like FB_Eip:
        set OP + fields and call. OP 1=open (PATH -> HANDLE),
        2=close, 3=status (-> STATE/FRAMES/LASTERR), 4=set_channel
        (CHANNEL/VALUE), 5=set_channels (START/COUNT/VALUES),
        6=send_frame. STATUS carries the service return code,
        ERRID its positive form.  The serial port is only opened
        when the surface is used. */
        "FB_Dmx" => {
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
            push(&mut off, "PATH", Type::Str { cap: 127 });
            push(&mut off, "BAUD", udint());
            push(&mut off, "CHANNEL", udint());
            push(&mut off, "VALUE", udint());
            push(&mut off, "START", udint());
            push(&mut off, "COUNT", udint());
            push(&mut off, "VALUES", Type::Str { cap: 255 });
            push(&mut off, "STATE", dint());
            push(&mut off, "FRAMES", udint());
            push(&mut off, "LASTERR", dint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
        }
        /* FB_Mbus — tplc-mbus service bridge (wired M-Bus / EN 13757
        metering master over an opt-in serial port at 2400 8E1).
        Synchronous op dispatch like FB_Eip: set OP + fields and call.
        OP 1=open (PATH -> HANDLE), 2=close, 3=status (-> STATE/
        TXTOT/RXTOT/LASTERR), 4=short frame (CONTROL/ADDRESS),
        5=long frame (CONTROL/ADDRESS/CI/DATA), 6=receive (-> KIND/
        RXPAYLOAD or NOMSG). STATUS carries the service return code,
        ERRID its positive form.  The serial port is only opened
        when the surface is used. */
        /* FB_EnOcean — tplc-enocean service bridge (EnOcean ESP3
        transceiver over an opt-in serial port at 57600 8N1).
        OP 1=open (PATH -> HANDLE), 2=close, 3=status, 4=send_radio
        (ORG/SENDER/DEST/DATA), 5=receive (-> PTYPE/RORG/RSENDER/
        RXPAYLOAD/RXB0 or NOMSG).  STATUS carries the service return
        code, ERRID its positive form. */
        "FB_EnOcean" => {
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
            push(&mut off, "PATH", Type::Str { cap: 127 });
            push(&mut off, "ORG", udint());
            push(&mut off, "SENDER", udint());
            push(&mut off, "DEST", udint());
            push(&mut off, "DATA", Type::Str { cap: 255 });
            push(&mut off, "PTYPE", udint());
            push(&mut off, "RORG", udint());
            push(&mut off, "RSENDER", udint());
            push(&mut off, "RXPAYLOAD", Type::Str { cap: 255 });
            push(&mut off, "STATE", dint());
            push(&mut off, "TXTOT", udint());
            push(&mut off, "RXTOT", udint());
            push(&mut off, "LASTERR", dint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "RXB0", udint());
        }
        /* FB_Genibus — tplc-genibus service bridge (GENIbus
        telegrams over an opt-in serial port at 9600 8E1).
        OP 1=open (PATH -> HANDLE), 2=close, 3=status, 4=send
        (DA/SA/APDU), 5=receive (-> RDA/RSA/RXAPDU/RXB0 or NOMSG). */
        "FB_Genibus" => {
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
            push(&mut off, "PATH", Type::Str { cap: 127 });
            push(&mut off, "DA", udint());
            push(&mut off, "SA", udint());
            push(&mut off, "APDU", Type::Str { cap: 250 });
            push(&mut off, "RDA", udint());
            push(&mut off, "RSA", udint());
            push(&mut off, "RXAPDU", Type::Str { cap: 250 });
            push(&mut off, "STATE", dint());
            push(&mut off, "TXTOT", udint());
            push(&mut off, "RXTOT", udint());
            push(&mut off, "LASTERR", dint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "RXB0", udint());
        }
        /* FB_Mpbus — tplc-mpbus service bridge (Belimo MP-Bus
        master over an opt-in serial adapter at 9600 8N1).
        OP 1=open (PATH -> HANDLE), 2=close, 3=status,
        4=transact (DA/CMD/APDU -> RDA/RCMD/RXAPDU/RXB0 or NOMSG),
        5=receive (same outs). */
        "FB_Mpbus" => {
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
            push(&mut off, "PATH", Type::Str { cap: 127 });
            push(&mut off, "DA", udint());
            push(&mut off, "CMD", udint());
            push(&mut off, "APDU", Type::Str { cap: 250 });
            push(&mut off, "RDA", udint());
            push(&mut off, "RCMD", udint());
            push(&mut off, "RXAPDU", Type::Str { cap: 250 });
            push(&mut off, "STATE", dint());
            push(&mut off, "TXTOT", udint());
            push(&mut off, "RXTOT", udint());
            push(&mut off, "LASTERR", dint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "RXB0", udint());
        }
        /* FB_DynMem — tplc-dynmem bounded-arena bridge
        (Tc3_DynamicMemory).  OP 1=alloc (SIZE/RESET -> ID),
        2=free (ID), 3=resize (ID/SIZE/PRESERVE/RESET),
        4=read (ID/OFFSET -> BYTE), 5=write (ID/OFFSET/BYTE),
        6=stats -> LIVE/TOTAL/ARENA counters. */
        "FB_DynMem" => {
            let udint = || Type::Int {
                signed: false,
                bits: 32,
            };
            let dint = || Type::Int {
                signed: true,
                bits: 32,
            };
            push(&mut off, "OP", udint());
            push(&mut off, "ID", udint());
            push(&mut off, "SIZE", udint());
            push(&mut off, "RESET", udint());
            push(&mut off, "PRESERVE", udint());
            push(&mut off, "OFFSET", udint());
            push(&mut off, "BYTE", udint());
            push(&mut off, "LIVEBLKS", udint());
            push(&mut off, "LIVEBYTES", udint());
            push(&mut off, "TOTALALLOCS", udint());
            push(&mut off, "TOTALFREES", udint());
            push(&mut off, "ARENABYTES", udint());
            push(&mut off, "ARENAUSED", udint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
        }
        /* FB_EventLog — tplc-eventlog journal bridge
        (Tc3_EventLogger).  OP 1=raise (EECLASS/EVTID/SEV/TEXT/ARGS
        -> ID), 2=clear (ID), 3=confirm (ID), 4=query (ID ->
        STATE/SEVO/CLSO/EVTIDO/TXTO), 5=stats -> counters. */
        "FB_EventLog" => {
            let udint = || Type::Int {
                signed: false,
                bits: 32,
            };
            let dint = || Type::Int {
                signed: true,
                bits: 32,
            };
            push(&mut off, "OP", udint());
            push(&mut off, "ID", udint());
            push(&mut off, "ECLASS", udint());
            push(&mut off, "EVTID", udint());
            push(&mut off, "SEV", udint());
            push(&mut off, "ARGS", udint());
            push(&mut off, "STATE", udint());
            push(&mut off, "SEVO", udint());
            push(&mut off, "CLSO", udint());
            push(&mut off, "EVTIDO", udint());
            push(&mut off, "LIVE", udint());
            push(&mut off, "RAISED", udint());
            push(&mut off, "CLEARED", udint());
            push(&mut off, "CONFIRMED", udint());
            push(&mut off, "DROPPED", udint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "TEXT", Type::Str { cap: 80 });
            push(&mut off, "TXTO", Type::Str { cap: 80 });
        }
        /* FB_Db — tplc-db SQLite bridge (Tc2/Tc3_Database).
        OP 1=open (PATH -> HANDLE), 2=exec (SQL -> CHANGES),
        3=query (SQL -> BUF/ROWS/COLS), 4=close, 5=error (-> ERR). */
        "FB_Db" => {
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
            push(&mut off, "CHANGES", udint());
            push(&mut off, "ROWS", udint());
            push(&mut off, "COLS", udint());
            push(&mut off, "ERRLEN", udint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "PATH", Type::Str { cap: 127 });
            push(&mut off, "SQL", Type::Str { cap: 255 });
            push(&mut off, "BUF", Type::Str { cap: 255 });
            push(&mut off, "ERR", Type::Str { cap: 127 });
        }
        /* FB_XmlSrv — tplc-xmlsrv file-backed XML store bridge
        (Tc2_XmlDataSrv).  OP 1=read (PATH/XPATH -> VAL),
        2=write (PATH/XPATH/VAL). */
        "FB_XmlSrv" => {
            let udint = || Type::Int {
                signed: false,
                bits: 32,
            };
            let dint = || Type::Int {
                signed: true,
                bits: 32,
            };
            push(&mut off, "OP", udint());
            push(&mut off, "VALLEN", udint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "PATH", Type::Str { cap: 127 });
            push(&mut off, "XPATH", Type::Str { cap: 127 });
            push(&mut off, "VAL", Type::Str { cap: 255 });
        }
        /* FB_Sym — tplc-sym by-name variable access
        (Tc2_DataExchange).  OP 1=read bytes (-> VAL/TYPE/SIZE),
        2=write bytes (VAL), 3=write int (IB), 4=write real (FB),
        5=write bool (BB), 6=read int (-> IB), 7=read real (-> FB),
        8=read bool (-> BB), 9=find (-> TYPE/SIZE/FOUND). */
        "FB_Sym" => {
            let udint = || Type::Int {
                signed: false,
                bits: 32,
            };
            let dint = || Type::Int {
                signed: true,
                bits: 32,
            };
            push(&mut off, "OP", udint());
            push(&mut off, "VTYP", udint());
            push(&mut off, "SIZE", udint());
            push(&mut off, "VALLEN", udint());
            push(&mut off, "FOUND", Type::Bool);
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "IB", dint());
            push(&mut off, "FB", Type::Real { bits: 64 });
            push(&mut off, "BB", Type::Bool);
            push(&mut off, "NAME", Type::Str { cap: 63 });
            push(&mut off, "VAL", Type::Str { cap: 255 });
        }
        /* FB_Iof — I/O device/box enumeration (Tc2_IoFunctions
        general FBs) over the package variable table.
        OP 1=box count, 2=box by index (-> NAME/REGION/ADDR/UNIT),
        3=find by name (-> REGION/ADDR/UNIT/FOUND),
        4=device count, 5=device name (-> VAL). */
        "FB_Iof" => {
            let udint = || Type::Int {
                signed: false,
                bits: 32,
            };
            let dint = || Type::Int {
                signed: true,
                bits: 32,
            };
            push(&mut off, "OP", udint());
            push(&mut off, "IDX", udint());
            push(&mut off, "COUNT", udint());
            push(&mut off, "REGION", udint());
            push(&mut off, "ADDR", udint());
            push(&mut off, "UNIT", udint());
            push(&mut off, "FOUND", Type::Bool);
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
            push(&mut off, "NAME", Type::Str { cap: 63 });
            push(&mut off, "VAL", Type::Str { cap: 63 });
        }
        "FB_Mbus" => {
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
            push(&mut off, "PATH", Type::Str { cap: 127 });
            push(&mut off, "CONTROL", udint());
            push(&mut off, "ADDRESS", udint());
            push(&mut off, "CI", udint());
            push(&mut off, "DATA", Type::Str { cap: 255 });
            push(&mut off, "KIND", udint());
            push(&mut off, "RXPAYLOAD", Type::Str { cap: 255 });
            push(&mut off, "STATE", dint());
            push(&mut off, "TXTOT", udint());
            push(&mut off, "RXTOT", udint());
            push(&mut off, "LASTERR", dint());
            push(&mut off, "STATUS", dint());
            push(&mut off, "ERRID", udint());
        }
        _ => return None,
    }
    let align = 8u64;
    Some(FbLayout {
        size: (off + align - 1) & !(align - 1),
        align,
        members: m,
    })
}

/// Lay out an ordered VarDecl list as a struct (declaration order,
/// natural alignment ≤8). Used for FB types, FUNCTION call scopes, and
/// STRUCT member tables. VAR_IN_OUT is laid out as a pointer-sized
/// `Type::Ref` slot (doc 07 §3.2): the caller stores the bound lvalue's
/// address/offset at call time and the callee dereferences on access.
pub fn layout_members(vars: &[VarDecl], fb_types: BTreeMap<String, FbLayout>) -> FbLayout {
    let mut members = BTreeMap::new();
    let mut off = 0u64;
    let mut align = 1u64;
    for v in vars {
        let ty = if v.kind == VarKind::InOut {
            Type::Ref {
                to: Box::new(v.ty.clone()),
            }
        } else {
            v.ty.clone()
        };
        let (size, a) = if let Type::Fb { name } = &ty {
            let sub = fb_types
                .get(name)
                .cloned()
                .or_else(|| builtin_fb_layout(name))
                .unwrap_or_else(|| panic!("unknown FB type {name}"));
            (sub.size, sub.align)
        } else {
            (ty.size(), ty.align())
        };
        align = align.max(a);
        off = (off + a - 1) & !(a - 1);
        members.insert(v.name.clone(), (off, ty));
        off += size;
    }
    let size = (off + align - 1) & !(align - 1);
    FbLayout {
        size,
        align,
        members,
    }
}

/// Layout an FB *type* from its MIR declaration (declaration order).
fn layout_fb(pou: &Pou, fb_types: &mut BTreeMap<String, FbLayout>) -> FbLayout {
    if let Some(b) = builtin_fb_layout(&pou.name) {
        return b;
    }
    if let Some(l) = fb_types.get(&pou.name) {
        return l.clone();
    }
    layout_members(&pou.iface, fb_types.clone())
}

fn layout_fb_named(name: &str, fb_types: &mut BTreeMap<String, FbLayout>) -> FbLayout {
    if let Some(l) = fb_types.get(name) {
        return l.clone();
    }
    if let Some(b) = builtin_fb_layout(name) {
        fb_types.insert(name.to_string(), b.clone());
        return b;
    }
    panic!("unknown FB type {name}");
}

/// Collect one program's leaf fields (qualified paths) into regions.
pub fn layout_program(module: &Module, program: &Pou) -> Layout {
    layout_impl(module, &[program.clone()], false)
}

/// Collect *all* programs' leaf fields into one shared state blob
/// (multi-program packages: FBD + ST programs in one image). GVL
/// fields keep their `GVL.x` names; each program's own vars qualify
/// as `PROGRAM.var` — only when more than one program exists, so a
/// single-program package keeps byte-identical field names/hash.
pub fn layout_module(module: &Module) -> Layout {
    let progs: Vec<Pou> = module
        .pous
        .iter()
        .filter(|p| p.kind == PouKind::Program)
        .cloned()
        .collect();
    layout_impl(module, &progs, progs.len() > 1)
}

fn layout_impl(module: &Module, programs: &[Pou], qualify: bool) -> Layout {
    let mut fb_types: BTreeMap<String, FbLayout> = BTreeMap::new();
    // Pre-layout all declared FB types (decl-order within each type).
    for pou in &module.pous {
        if pou.kind == PouKind::FunctionBlock {
            let l = layout_fb(pou, &mut fb_types);
            fb_types.insert(pou.name.clone(), l);
        }
    }

    // Region → (qualified name, decl) pending list.
    let mut buckets: [Vec<(&VarDecl, String)>; 5] = Default::default();
    for gvl in &module.gvls {
        for v in &gvl.vars {
            let region = if v.retain || gvl.retain {
                REGION_RETAIN
            } else {
                REGION_GLOBAL
            };
            buckets[region as usize].push((v, format!("{}.{}", gvl.name, v.name)));
        }
    }
    for pou in programs {
        for v in &pou.iface {
            // VAR_IN_OUT is host-provided; VAR_EXTERNAL only names a GVL
            // var — it aliases that storage instead of owning a field
            if matches!(v.kind, VarKind::InOut | VarKind::External) {
                continue;
            }
            let region = if v.retain {
                REGION_RETAIN
            } else if matches!(v.ty, Type::Fb { .. }) {
                REGION_FB
            } else {
                REGION_GLOBAL
            };
            let qname = if qualify {
                format!("{}.{}", pou.name, v.name)
            } else {
                v.name.clone()
            };
            buckets[region as usize].push((v, qname));
        }
    }

    // Name-sort within each region; assign offsets (doc 07 §3.3).
    let mut fields = BTreeMap::new();
    let mut off = 0u64;
    let max_align = 1u64;
    let mut layd_fields = Vec::new();
    let mut region_ranges: [(u64, u64, u32); 5] = [(0, 0, 0); 5];
    for (ri, bucket) in buckets.iter_mut().enumerate() {
        bucket.sort_by(|a, b| a.1.cmp(&b.1));
        let region_start = off;
        let mut region_align = 1u64;
        for (v, qname) in bucket.iter() {
            let is_fb = matches!(v.ty, Type::Fb { .. });
            let (size, a) = if let Type::Fb { name } = &v.ty {
                let sub = layout_fb_named(name, &mut fb_types);
                (sub.size, sub.align)
            } else {
                (v.ty.size(), v.ty.align())
            };
            region_align = region_align.max(a);
            off = (off + a - 1) & !(a - 1);
            let is_const = v.kind == VarKind::Constant;
            fields.insert(
                qname.clone(),
                FieldInfo {
                    offset: off,
                    size,
                    ty: v.ty.clone(),
                    retain: v.retain,
                    is_const,
                    field_kind: if is_fb { 1 } else { 0 },
                },
            );
            layd_fields.push(LaydField {
                sym_entry: u32::MAX,
                field_kind: if is_fb { 1 } else { 0 },
                // bit0 retain, bit1 const (read-only, not writeable
                // or forceable through the debug plane)
                flags: (v.retain as u8) | ((is_const as u8) << 1),
                gen: 0,
                offset: off,
                size,
                type_ref: u32::MAX,
            });
            off += size;
        }
        region_ranges[ri] = (region_start, off - region_start, region_align as u32);
    }

    // VAR_EXTERNAL decls get no storage — they alias the matching GVL
    // var's field so reads/writes hit shared global state
    for pou in programs {
        for v in &pou.iface {
            let alias = if qualify {
                format!("{}.{}", pou.name, v.name)
            } else {
                v.name.clone()
            };
            if v.kind != VarKind::External || fields.contains_key(&alias) {
                continue;
            }
            let target = module.gvls.iter().find_map(|g| {
                g.vars
                    .iter()
                    .find(|gv| gv.name.eq_ignore_ascii_case(&v.name))
                    .map(|gv| format!("{}.{}", g.name, gv.name))
            });
            if let Some(tq) = target {
                if let Some(tf) = fields.get(&tq) {
                    fields.insert(alias, tf.clone());
                }
            }
        }
    }

    let total = (off + max_align - 1) & !(max_align - 1);
    let (retain_offset, retain_size, _) = region_ranges[REGION_RETAIN as usize];

    let regions: Vec<LaydRegion> = region_ranges
        .iter()
        .enumerate()
        .filter(|(_, (_, size, _))| *size > 0)
        .map(|(kind, (start, size, al))| LaydRegion {
            kind: kind as u8,
            align: *al,
            offset: *start,
            size: *size,
        })
        .collect();

    Layout {
        total_size: total.max(1),
        fields,
        fb_types,
        retain_offset,
        retain_size,
        align: max_align,
        layd: Layd {
            version: 1,
            total_size: total.max(1),
            regions,
            fields: layd_fields,
        },
    }
}
