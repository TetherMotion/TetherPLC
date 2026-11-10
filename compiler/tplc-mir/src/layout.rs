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
/// STRUCT member tables. VAR_IN_OUT gets no stored field (call-scoped
/// binding, doc 07 §3.2).
pub fn layout_members(vars: &[VarDecl], fb_types: BTreeMap<String, FbLayout>) -> FbLayout {
    let mut members = BTreeMap::new();
    let mut off = 0u64;
    let mut align = 1u64;
    for v in vars {
        if v.kind == VarKind::InOut {
            continue;
        }
        let (size, a) = if let Type::Fb { name } = &v.ty {
            let sub = fb_types
                .get(name)
                .cloned()
                .or_else(|| builtin_fb_layout(name))
                .unwrap_or_else(|| panic!("unknown FB type {name}"));
            (sub.size, sub.align)
        } else {
            (v.ty.size(), v.ty.align())
        };
        align = align.max(a);
        off = (off + a - 1) & !(a - 1);
        members.insert(v.name.clone(), (off, v.ty.clone()));
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
    for v in &program.iface {
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
        buckets[region as usize].push((v, v.name.clone()));
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
    for v in &program.iface {
        if v.kind != VarKind::External || fields.contains_key(&v.name) {
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
                fields.insert(v.name.clone(), tf.clone());
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
