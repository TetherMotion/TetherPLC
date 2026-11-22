//! State-blob layout algorithm (doc 07 §3).
//!
//! Per program instance: collect fields into regions
//! `[global | fb_instances | task_local | retain | internal]`, order
//! each region by qualified name, assign aligned offsets, emit the
//! `layout_desc` (LAYD) field table.

use crate::ir::*;
use std::collections::{BTreeMap, BTreeSet};
use tplc_pkg::sections::{Layd, LaydField, LaydRegion};

type Members = BTreeMap<String, (u64, Type)>;

mod builtin_timers;
mod builtin_io_svc;
mod builtin_modbus_sacn;
mod builtin_bacnet_s7;
mod builtin_file_mail;
mod builtin_iec;
mod builtin_motion;
mod builtin_ua_eip;
mod builtin_mqtt_ec;
mod builtin_misc_fb;
mod builtin_tail_a;
mod builtin_tail_b;

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
    /// GVL names — their `NAME.var` fields are global storage,
    /// resolvable by qualified ref from any program scope.
    pub gvls: BTreeSet<String>,
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
            gvls: BTreeSet::new(),
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
    let mut off = 0u64;
    match name {
        "TON" | "TOF" | "TP" => off = builtin_timers::arm_ton(off, &mut m),
        "R_TRIG" | "F_TRIG" => off = builtin_timers::arm_r_trig(off, &mut m),
        "CTU" | "CTD" | "CTUD" => off = builtin_timers::arm_ctu(off, &mut m),
        "RS" | "SR" => off = builtin_timers::arm_rs(off, &mut m),
        "FB_Socket" => off = builtin_io_svc::arm_fb_socket(off, &mut m),
        "FB_Serial" => off = builtin_io_svc::arm_fb_serial(off, &mut m),
        "FB_Ads" => off = builtin_io_svc::arm_fb_ads(off, &mut m),
        "FB_RtMon" => off = builtin_io_svc::arm_fb_rt_mon(off, &mut m),
        "FB_Modbus" => off = builtin_modbus_sacn::arm_fb_modbus(off, &mut m),
        "FB_ModbusRtu" => off = builtin_modbus_sacn::arm_fb_modbus_rtu(off, &mut m),
        "FB_SacnSource" => off = builtin_modbus_sacn::arm_fb_sacn_source(off, &mut m),
        "FB_SacnReceiver" => off = builtin_modbus_sacn::arm_fb_sacn_receiver(off, &mut m),
        "FB_Bacnet" => off = builtin_bacnet_s7::arm_fb_bacnet(off, &mut m),
        "FB_Pjlink" => off = builtin_bacnet_s7::arm_fb_pjlink(off, &mut m),
        "FB_S7Conn" => off = builtin_bacnet_s7::arm_fb_s7_conn(off, &mut m),
        "FB_S7Io" => off = builtin_bacnet_s7::arm_fb_s7_io(off, &mut m),
        "FB_S5Conn" => off = builtin_bacnet_s7::arm_fb_s5_conn(off, &mut m),
        "FB_S5Io" => off = builtin_bacnet_s7::arm_fb_s5_io(off, &mut m),
        "FB_FtpConn" => off = builtin_file_mail::arm_fb_ftp_conn(off, &mut m),
        "FB_FtpCmd" => off = builtin_file_mail::arm_fb_ftp_cmd(off, &mut m),
        "FB_FtpXfer" => off = builtin_file_mail::arm_fb_ftp_xfer(off, &mut m),
        "FB_SmtpSend" => off = builtin_file_mail::arm_fb_smtp_send(off, &mut m),
        "FB_Iec870Conn" => off = builtin_iec::arm_fb_iec870_conn(off, &mut m),
        "FB_Iec870Tx" => off = builtin_iec::arm_fb_iec870_tx(off, &mut m),
        "FB_Iec870Rx" => off = builtin_iec::arm_fb_iec870_rx(off, &mut m),
        "FB_MotionAxis" => off = builtin_motion::arm_fb_motion_axis(off, &mut m),
        "FB_NciChan" => off = builtin_motion::arm_fb_nci_chan(off, &mut m),
        "FB_Fifo" => off = builtin_motion::arm_fb_fifo(off, &mut m),
        "FB_Kin" => off = builtin_motion::arm_fb_kin(off, &mut m),
        "FB_McGroup" => off = builtin_motion::arm_fb_mc_group(off, &mut m),
        "FB_Cam" => off = builtin_motion::arm_fb_cam(off, &mut m),
        "FB_Path" => off = builtin_motion::arm_fb_path(off, &mut m),
        "FB_UaClient" => off = builtin_ua_eip::arm_fb_ua_client(off, &mut m),
        "FB_Eip" => off = builtin_ua_eip::arm_fb_eip(off, &mut m),
        "FB_Iec61850" => off = builtin_iec::arm_fb_iec61850(off, &mut m),
        "FB_Mqtt" => off = builtin_mqtt_ec::arm_fb_mqtt(off, &mut m),
        "FB_EcSync" => off = builtin_mqtt_ec::arm_fb_ec_sync(off, &mut m),
        "FB_EcDiagRead" => off = builtin_mqtt_ec::arm_fb_ec_diag_read(off, &mut m),
        "FB_CanOpen" => off = builtin_misc_fb::arm_fb_can_open(off, &mut m),
        "FB_IoLink" => off = builtin_misc_fb::arm_fb_io_link(off, &mut m),
        "FB_Profinet" => off = builtin_misc_fb::arm_fb_profinet(off, &mut m),
        "FB_Soe" => off = builtin_misc_fb::arm_fb_soe(off, &mut m),
        "FB_Ocpp" => off = builtin_misc_fb::arm_fb_ocpp(off, &mut m),
        "FB_Aes70" => off = builtin_misc_fb::arm_fb_aes70(off, &mut m),
        "FB_Dmx" => off = builtin_tail_a::arm_fb_dmx(off, &mut m),
        "FB_EnOcean" => off = builtin_tail_a::arm_fb_en_ocean(off, &mut m),
        "FB_Genibus" => off = builtin_tail_a::arm_fb_genibus(off, &mut m),
        "FB_Mpbus" => off = builtin_tail_a::arm_fb_mpbus(off, &mut m),
        "FB_DynMem" => off = builtin_tail_a::arm_fb_dyn_mem(off, &mut m),
        "FB_EventLog" => off = builtin_tail_a::arm_fb_event_log(off, &mut m),
        "FB_Db" => off = builtin_tail_a::arm_fb_db(off, &mut m),
        "FB_XmlSrv" => off = builtin_tail_b::arm_fb_xml_srv(off, &mut m),
        "FB_Sym" => off = builtin_tail_b::arm_fb_sym(off, &mut m),
        "FB_Iof" => off = builtin_tail_b::arm_fb_iof(off, &mut m),
        "FB_Mbus" => off = builtin_tail_b::arm_fb_mbus(off, &mut m),
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
        gvls: module
            .gvls
            .iter()
            .map(|g| g.name.clone())
            .collect(),
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
