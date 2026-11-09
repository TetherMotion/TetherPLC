//! TPLB v1 loadable blob: converts a relocatable ELF object
//! (`tplcc --target` output) into a position-independent image plus
//! a fixup table a bare-metal loader applies at RAM-placement time —
//! no linker runs on the target.
//!
//! Layout (all little-endian u32):
//!
//! ```text
//! 0x00  magic 'TPLB', version, header_size, n_sections, n_syms,
//!       n_imports, n_fixups, flags
//! hdr   section records {name_off, load_off, size, align, flags, rsv}
//!       symbol records  {name_off, value_off, size, info, pad[3]}
//!       import records  {name_off, rsv}
//!       fixup records   {place_off, type, value, addend}
//!       string table (NUL-separated names, deduped)
//! data  section contents at load_off; the whole blob is memcpy'd
//!       to the load base so load_off == offset from base
//! ```
//!
//! Section record flags: bit0 writable, bit1 executable (alloc is
//! implied — only alloc sections are carried).
//!
//! Fixup types: the loader computes `target = base + value +
//! addend` for defined symbols, or `target = import_addr + addend`
//! when `value` has bit31 set (`value & 0x7fffffff` = import index):
//!
//! - `ABS32`/`ABS64`: store target as a pointer-width word
//! - `MOVW`/`MOVT`: store the low/high 16 bits of target inside the
//!   instruction's imm4:imm12 field
//! - `THM_CALL`: rewrite a Thumb BL pair so it reaches target
//!   (imm32 = (target - place - 4) & ~1, split per Thumb encoding)
//! - `REL32`: write (target - place) as a literal-pool word
//!
//! PC-relative relocations against *defined* symbols are resolved
//! at emit time — the blob base cancels, so the field is patched as
//! if the blob were linked at base 0. `.ARM.exidx` and other unwind
//! metadata are not carried: bare-metal blob targets never unwind.

use std::collections::BTreeMap;

pub const TPLB_MAGIC: u32 = u32::from_le_bytes(*b"TPLB");
pub const TPLB_VERSION: u32 = 1;
pub const TPLB_HEADER_SIZE: usize = 32;
const SREC_SIZE: usize = 24;
const YREC_SIZE: usize = 16;
const IREC_SIZE: usize = 8;
const FREC_SIZE: usize = 16;

/* blob fixup types (also the wire contract for loaders) */
pub const F_ABS32: u32 = 1;
pub const F_ABS64: u32 = 2;
pub const F_MOVW: u32 = 3;
pub const F_MOVT: u32 = 4;
pub const F_THM_CALL: u32 = 5;
pub const F_REL32: u32 = 6;
/// fixup value flag: low 31 bits index the import table
pub const F_IMPORT: u32 = 0x8000_0000;

/* ELF constants */
const ET_REL: u16 = 1;
const EM_ARM: u16 = 40;
const SHT_PROGBITS: u32 = 1;
const SHT_SYMTAB: u32 = 2;
const SHT_RELA: u32 = 4;
const SHT_NOBITS: u32 = 8;
const SHT_REL: u32 = 9;
const SHT_ARM_EXIDX: u32 = 0x7000_0001;
const SHT_ARM_ATTRIBUTES: u32 = 0x7000_0003;
const SHF_WRITE: u64 = 0x1;
const SHF_ALLOC: u64 = 0x2;
const SHF_EXECINSTR: u64 = 0x4;
const SHN_UNDEF: u16 = 0;
const STB_GLOBAL: u8 = 1;
const STB_WEAK: u8 = 2;
const STT_SECTION: u8 = 3;
const STT_FILE: u8 = 4;

/* ARM relocation types (EM_ARM) */
const R_ARM_ABS32: u32 = 2;
const R_ARM_REL32: u32 = 3;
const R_ARM_GLOB_DAT: u32 = 21;
const R_ARM_JUMP_SLOT: u32 = 22;
const R_ARM_RELATIVE: u32 = 23;
const R_ARM_THM_CALL: u32 = 10;
const R_ARM_CALL: u32 = 28;
const R_ARM_JUMP24: u32 = 29;
const R_ARM_MOVW_ABS_NC: u32 = 47;
const R_ARM_MOVT_ABS_NC: u32 = 48;

struct Section {
    name: String,
    sh_type: u32,
    flags: u64,
    offset: u64,
    size: u64,
    link: u32,
    info: u32,
    align: u64,
    entsize: u64,
}

struct Sym {
    name: String,
    value: u64,
    size: u64,
    info: u8,
    shndx: u16,
}

struct Reloc {
    place_sec: usize,
    place_off: u64,
    sym: usize,
    r#type: u32,
    addend: i64, // RELA addend; REL decoded per type during scan
    is_rela: bool,
}

struct Elf {
    machine: u16,
    secs: Vec<Section>,
    syms: Vec<Sym>,
    relocs: Vec<Reloc>,
}

fn ru16(d: &[u8], o: usize) -> u16 {
    u16::from_le_bytes(d[o..o + 2].try_into().unwrap())
}
fn ru32(d: &[u8], o: usize) -> u32 {
    u32::from_le_bytes(d[o..o + 4].try_into().unwrap())
}
fn ru64(d: &[u8], o: usize) -> u64 {
    u64::from_le_bytes(d[o..o + 8].try_into().unwrap())
}

/// Minimal ELF32/ELF64 little-endian ET_REL parser — enough for
/// LLVM-emitted objects: section headers, symtab, strtabs, REL/RELA.
fn parse_elf(d: &[u8]) -> Result<Elf, String> {
    if d.len() < 52 || d[0..4] != [0x7f, b'E', b'L', b'F'] {
        return Err("not an ELF object".into());
    }
    if d[5] != 1 {
        return Err("only little-endian ELF supported".into());
    }
    let is64 = d[4] == 2;
    if d[4] != 1 && !is64 {
        return Err("bad ELF class".into());
    }
    if d.len() < if is64 { 64 } else { 52 } {
        return Err("truncated ELF header".into());
    }
    let (et, machine, shoff, shentsize, shnum, shstrndx) = if is64 {
        (
            ru16(d, 16),
            ru16(d, 18),
            ru64(d, 40),
            ru16(d, 58) as usize,
            ru16(d, 60) as usize,
            ru16(d, 62) as usize,
        )
    } else {
        (
            ru16(d, 16),
            ru16(d, 18),
            ru32(d, 32) as u64,
            ru16(d, 46) as usize,
            ru16(d, 48) as usize,
            ru16(d, 50) as usize,
        )
    };
    if et != ET_REL {
        return Err(format!("expected ET_REL, got e_type {et}"));
    }
    if shoff == 0 || shnum == 0 {
        return Err("no section headers".into());
    }
    /* section header tuple: name_off,type,flags,addr,offset,size,
    link,info,align,entsize */
    let mut raw: Vec<[u64; 10]> = Vec::with_capacity(shnum);
    for i in 0..shnum {
        let o = shoff as usize + i * shentsize;
        if o + shentsize > d.len() {
            return Err("section header out of bounds".into());
        }
        raw.push(if is64 {
            [
                ru32(d, o) as u64,
                ru32(d, o + 4) as u64,
                ru64(d, o + 8),
                ru64(d, o + 16),
                ru64(d, o + 24),
                ru64(d, o + 32),
                ru32(d, o + 40) as u64,
                ru32(d, o + 44) as u64,
                ru64(d, o + 48),
                ru64(d, o + 56),
            ]
        } else {
            [
                ru32(d, o) as u64,
                ru32(d, o + 4) as u64,
                ru32(d, o + 8) as u64,
                ru32(d, o + 12) as u64,
                ru32(d, o + 16) as u64,
                ru32(d, o + 20) as u64,
                ru32(d, o + 24) as u64,
                ru32(d, o + 28) as u64,
                ru32(d, o + 32) as u64,
                ru32(d, o + 36) as u64,
            ]
        });
    }
    let cstr = |sec: &[u64; 10], name_off: u64| -> Result<String, String> {
        let o = sec[4] as usize + name_off as usize;
        if o >= d.len() {
            return Err("string offset out of bounds".into());
        }
        let end = d[o..]
            .iter()
            .position(|&b| b == 0)
            .map(|p| o + p)
            .ok_or("unterminated string")?;
        Ok(String::from_utf8_lossy(&d[o..end]).into_owned())
    };
    if shstrndx >= raw.len() {
        return Err("bad shstrndx".into());
    }
    let shstr = &raw[shstrndx];
    let mut secs = Vec::with_capacity(shnum);
    for r in &raw {
        secs.push(Section {
            name: cstr(shstr, r[0])?,
            sh_type: r[1] as u32,
            flags: r[2],
            offset: r[4],
            size: r[5],
            link: r[6] as u32,
            info: r[7] as u32,
            align: r[8],
            entsize: r[9],
        });
    }
    /* symbols */
    let mut syms = Vec::new();
    if let Some(si) = secs.iter().position(|s| s.sh_type == SHT_SYMTAB) {
        let st = &secs[si];
        if st.link as usize >= secs.len() {
            return Err("symtab link out of bounds".into());
        }
        let strsec = &raw[st.link as usize];
        let entsz = if is64 { 24usize } else { 16 };
        let n = (st.size / st.entsize.max(entsz as u64)) as usize;
        for k in 0..n {
            let o = st.offset as usize + k * entsz;
            if o + entsz > d.len() {
                return Err("symbol out of bounds".into());
            }
            let (name_off, info, shndx, value, size) = if is64 {
                (
                    ru32(d, o),
                    d[o + 4],
                    ru16(d, o + 6),
                    ru64(d, o + 8),
                    ru64(d, o + 16),
                )
            } else {
                (
                    ru32(d, o),
                    d[o + 12],
                    ru16(d, o + 14),
                    ru32(d, o + 4) as u64,
                    ru32(d, o + 8) as u64,
                )
            };
            let name = if name_off == 0 {
                String::new()
            } else {
                cstr(strsec, name_off as u64)?
            };
            syms.push(Sym {
                name,
                value,
                size,
                info,
                shndx,
            });
        }
    }
    /* relocations: SHT_REL/SHT_RELA; sh_info = target section */
    let mut relocs = Vec::new();
    for s in &secs {
        if s.sh_type != SHT_REL && s.sh_type != SHT_RELA {
            continue;
        }
        let is_rela = s.sh_type == SHT_RELA;
        let entsz = if is64 {
            if is_rela {
                24usize
            } else {
                16
            }
        } else if is_rela {
            12
        } else {
            8
        };
        let n = (s.size / s.entsize.max(entsz as u64)) as usize;
        for k in 0..n {
            let o = s.offset as usize + k * entsz;
            if o + entsz > d.len() {
                return Err("reloc out of bounds".into());
            }
            let (r_offset, r_sym, r_type, r_addend) = if is64 {
                let info = ru64(d, o + 8);
                (
                    ru64(d, o),
                    (info >> 32) as usize,
                    (info & 0xff) as u32,
                    if is_rela { ru64(d, o + 16) as i64 } else { 0 },
                )
            } else {
                let info = ru32(d, o + 4);
                (
                    ru32(d, o) as u64,
                    (info >> 8) as usize,
                    info & 0xff,
                    if is_rela {
                        ru32(d, o + 8) as i32 as i64
                    } else {
                        0
                    },
                )
            };
            relocs.push(Reloc {
                place_sec: s.info as usize,
                place_off: r_offset,
                sym: r_sym,
                r#type: r_type,
                addend: r_addend,
                is_rela,
            });
        }
    }
    Ok(Elf {
        machine,
        secs,
        syms,
        relocs,
    })
}

/* ----------------------- Thumb BL pair codec -------------------- */

fn thm_call_imm(hw1: u16, hw2: u16) -> i32 {
    let s = ((hw1 >> 10) & 1) as i32;
    let imm10 = (hw1 & 0x3ff) as i32;
    let j1 = ((hw2 >> 13) & 1) as i32;
    let j2 = ((hw2 >> 11) & 1) as i32;
    let imm11 = (hw2 & 0x7ff) as i32;
    let i1 = 1 - (j1 ^ s);
    let i2 = 1 - (j2 ^ s);
    let imm = (s << 24) | (i1 << 23) | (i2 << 22) | (imm10 << 12) | (imm11 << 1);
    if imm & (1 << 24) != 0 {
        imm - (1 << 25)
    } else {
        imm
    }
}

fn thm_call_pack(imm: i32) -> (u16, u16) {
    let imm = imm & 0x1ff_ffff;
    let s = (imm >> 24) & 1;
    let i1 = (imm >> 23) & 1;
    let i2 = (imm >> 22) & 1;
    let j1 = 1 - (i1 ^ s);
    let j2 = 1 - (i2 ^ s);
    let hw1 = 0xf000 | ((s as u16) << 10) | (((imm >> 12) & 0x3ff) as u16);
    let hw2 = 0xd000 | ((j1 as u16) << 13) | ((j2 as u16) << 11) | ((imm >> 1) & 0x7ff) as u16;
    (hw1, hw2)
}

/* --------------------------- blob model ------------------------- */

struct BlobSec {
    name: String,
    load_off: u32,
    size: u32,
    align: u32,
    flags: u32, // bit0 write, bit1 exec
    content: Vec<u8>,
}

struct BlobSym {
    name_off: u32,
    value_off: u32,
    size: u32,
    info: u8,
}

struct Fixup {
    place_off: u32,
    r#type: u32,
    value: u32,
    addend: i32,
}

fn align_up(v: u64, a: u64) -> u64 {
    if a <= 1 {
        v
    } else {
        (v + a - 1) & !(a - 1)
    }
}

/// What a relocation resolves to — computed in the scan pass,
/// valued in the layout pass.
enum Action {
    Skip,
    Fix {
        r#type: u32,
        sym: usize,
        addend: i64,
    },
}

/// Build a TPLB blob from an ELF ET_REL object. ARM (EM_ARM)
/// objects only for v1; other machines are rejected rather than
/// silently mis-emitted.
pub fn build_blob(obj: &[u8]) -> Result<Vec<u8>, String> {
    let elf = parse_elf(obj)?;
    if elf.machine != EM_ARM {
        return Err(format!(
            "tplb v1 supports EM_ARM objects only (e_machine={})",
            elf.machine
        ));
    }
    /* content sections: allocatable PROGBITS/NOBITS minus unwind
    metadata (.ARM.exidx is alloc but dead weight on blob targets —
    its PREL31/personality relocs need no runtime fixup) */
    let mut secs: Vec<BlobSec> = Vec::new();
    let mut elf_to_blob: BTreeMap<usize, usize> = BTreeMap::new();
    // (kept for clarity; blob offsets assigned in the layout pass)
    for (i, s) in elf.secs.iter().enumerate() {
        let keep = s.flags & SHF_ALLOC != 0
            && (s.sh_type == SHT_PROGBITS || s.sh_type == SHT_NOBITS)
            && s.sh_type != SHT_ARM_EXIDX
            && s.sh_type != SHT_ARM_ATTRIBUTES;
        if !keep {
            continue;
        }
        let sz = s.size as usize;
        let mut content = vec![0u8; sz];
        if s.sh_type == SHT_PROGBITS {
            let o = s.offset as usize;
            if o + sz > obj.len() {
                return Err(format!("section {} past EOF", s.name));
            }
            content.copy_from_slice(&obj[o..o + sz]);
        }
        elf_to_blob.insert(i, secs.len());
        secs.push(BlobSec {
            name: s.name.clone(),
            load_off: 0,
            size: s.size as u32,
            align: s.align.max(1) as u32,
            flags: (if s.flags & SHF_WRITE != 0 { 1 } else { 0 })
                | (if s.flags & SHF_EXECINSTR != 0 { 2 } else { 0 }),
            content,
        });
    }
    /* ---- scan pass: classify relocs, collect import names ---- */
    let mut actions: Vec<Action> = Vec::with_capacity(elf.relocs.len());
    let mut import_names: Vec<String> = Vec::new();
    let mut seen_imports: BTreeMap<String, ()> = BTreeMap::new();
    let mut n_fixups = 0u32;

    /* helper: read 4 bytes at a reloc place inside section content */
    let read_field = |r: &Reloc| -> u32 {
        let Some(&bi) = elf_to_blob.get(&r.place_sec) else {
            return 0;
        };
        let o = r.place_off as usize;
        secs[bi].content[o..o + 4]
            .try_into()
            .map(u32::from_le_bytes)
            .unwrap_or(0)
    };

    for r in &elf.relocs {
        if !elf_to_blob.contains_key(&r.place_sec) {
            actions.push(Action::Skip);
            continue;
        }
        let sy = elf
            .syms
            .get(r.sym)
            .ok_or_else(|| format!("reloc sym index {} out of bounds", r.sym))?;
        let undef = sy.shndx == SHN_UNDEF;
        /* addend: RELA carries it; REL reads the in-place field
        (sign-extended — fields can hold negative offsets) */
        let a_plain = if r.is_rela {
            r.addend
        } else {
            read_field(r) as i32 as i64
        };
        match r.r#type {
            0 => actions.push(Action::Skip), // R_ARM_NONE
            R_ARM_ABS32 | R_ARM_GLOB_DAT | R_ARM_JUMP_SLOT | R_ARM_RELATIVE => {
                if undef && seen_imports.insert(sy.name.clone(), ()).is_none() {
                    import_names.push(sy.name.clone());
                }
                n_fixups += 1;
                actions.push(Action::Fix {
                    r#type: F_ABS32,
                    sym: r.sym,
                    addend: a_plain,
                });
            }
            R_ARM_REL32 => {
                if undef {
                    if seen_imports.insert(sy.name.clone(), ()).is_none() {
                        import_names.push(sy.name.clone());
                    }
                    n_fixups += 1;
                    actions.push(Action::Fix {
                        r#type: F_REL32,
                        sym: r.sym,
                        addend: a_plain,
                    });
                } else {
                    actions.push(Action::Fix {
                        r#type: F_REL32,
                        sym: r.sym,
                        addend: a_plain,
                    });
                }
            }
            R_ARM_THM_CALL => {
                let bi = elf_to_blob[&r.place_sec];
                let o = r.place_off as usize;
                if o + 4 > secs[bi].content.len() {
                    return Err("THM_CALL reloc out of section bounds".into());
                }
                let hw1 =
                    u16::from_le_bytes(secs[bi].content[o..o + 2].try_into().unwrap_or([0; 2]));
                let hw2 =
                    u16::from_le_bytes(secs[bi].content[o + 2..o + 4].try_into().unwrap_or([0; 2]));
                let a = thm_call_imm(hw1, hw2) as i64;
                if undef {
                    if seen_imports.insert(sy.name.clone(), ()).is_none() {
                        import_names.push(sy.name.clone());
                    }
                    n_fixups += 1;
                    actions.push(Action::Fix {
                        r#type: F_THM_CALL,
                        sym: r.sym,
                        addend: a,
                    });
                } else {
                    actions.push(Action::Fix {
                        r#type: F_THM_CALL,
                        sym: r.sym,
                        addend: a,
                    });
                }
            }
            R_ARM_MOVW_ABS_NC | R_ARM_MOVT_ABS_NC => {
                /* Thumb-2 MOVW/MOVT: imm16 = imm4:i:imm3:imm8 split
                across two halfwords; the in-place field is the
                addend (zero-extended — high addend bits are not
                representable, fine for the small offsets LLVM
                emits). Reject ARM-state or malformed encodings. */
                let ft = if r.r#type == R_ARM_MOVW_ABS_NC {
                    F_MOVW
                } else {
                    F_MOVT
                };
                let want = if r.r#type == R_ARM_MOVW_ABS_NC {
                    0xf240u16
                } else {
                    0xf2c0u16
                };
                let content = &secs[elf_to_blob[&r.place_sec]].content;
                let po = r.place_off as usize;
                if po + 4 > content.len() {
                    return Err("MOVW/MOVT reloc out of section bounds".into());
                }
                let hw1 = u16::from_le_bytes([content[po], content[po + 1]]);
                let hw2 = u16::from_le_bytes([content[po + 2], content[po + 3]]);
                if hw1 & 0xfbf0 != want || hw2 & 0x8000 != 0 {
                    return Err(format!(
                        "MOVW/MOVT reloc at section {}+{po:#x} is not Thumb-2 encoding",
                        elf.secs[r.place_sec].name
                    ));
                }
                /* REL: in-place imm16 is the addend; RELA: explicit */
                let imm16 = if r.is_rela {
                    (r.addend as u32) & 0xffff
                } else {
                    (((hw1 & 0xf) as u32) << 12)
                        | (((hw1 >> 10) & 1) as u32) << 11
                        | (((hw2 >> 12) & 7) as u32) << 8
                        | (hw2 & 0xff) as u32
                };
                if undef && seen_imports.insert(sy.name.clone(), ()).is_none() {
                    import_names.push(sy.name.clone());
                }
                n_fixups += 1;
                actions.push(Action::Fix {
                    r#type: ft,
                    sym: r.sym,
                    addend: imm16 as i64,
                });
            }
            R_ARM_CALL | R_ARM_JUMP24 => {
                return Err(format!(
                    "ARM-state call reloc type {} unsupported in tplb v1",
                    r.r#type
                ));
            }
            t => {
                return Err(format!("unsupported ARM reloc type {t}"));
            }
        }
    }

    /* ---- defined globals → blob symtab ---- */
    let mut blob_syms: Vec<BlobSym> = Vec::new();
    let mut sym_value_sec: Vec<Option<(usize, u64)>> = vec![None; elf.syms.len()];
    for (i, sy) in elf.syms.iter().enumerate() {
        let bind = sy.info >> 4;
        let exported = matches!(bind, STB_GLOBAL | STB_WEAK)
            && sy.shndx != SHN_UNDEF
            && (sy.shndx as usize) < elf.secs.len()
            && elf_to_blob.contains_key(&(sy.shndx as usize))
            && sy.info & 0xf != STT_SECTION
            && sy.info & 0xf != STT_FILE;
        if !exported {
            continue;
        }
        sym_value_sec[i] = Some((sy.shndx as usize, sy.value));
        blob_syms.push(BlobSym {
            name_off: 0, // interned below
            value_off: 0,
            size: sy.size as u32,
            info: sy.info,
        });
    }

    /* ---- string table ---- */
    let mut strtab: BTreeMap<String, u32> = BTreeMap::new();
    let mut stroff = 1u32;
    let intern = |strtab: &mut BTreeMap<String, u32>, stroff: &mut u32, n: &str| -> u32 {
        if let Some(&o) = strtab.get(n) {
            return o;
        }
        let o = *stroff;
        *stroff += n.len() as u32 + 1;
        strtab.insert(n.to_string(), o);
        o
    };
    for s in &secs {
        intern(&mut strtab, &mut stroff, &s.name);
    }
    /* map elf sym idx → blob sym idx (same order as blob_syms) */
    let mut sym_to_blob: BTreeMap<usize, usize> = BTreeMap::new();
    let mut bi = 0usize;
    for (i, v) in sym_value_sec.iter().enumerate() {
        if v.is_some() {
            sym_to_blob.insert(i, bi);
            bi += 1;
        }
    }
    for (i, sy) in elf.syms.iter().enumerate() {
        if let Some(&bi) = sym_to_blob.get(&i) {
            blob_syms[bi].name_off = intern(&mut strtab, &mut stroff, &sy.name);
        }
    }
    for n in &import_names {
        intern(&mut strtab, &mut stroff, n);
    }
    let nimports = import_names.len() as u32;

    /* ---- layout ---- */
    let nsecs = secs.len() as u32;
    let nsyms = blob_syms.len() as u32;
    let records_end = TPLB_HEADER_SIZE
        + nsecs as usize * SREC_SIZE
        + nsyms as usize * YREC_SIZE
        + nimports as usize * IREC_SIZE
        + n_fixups as usize * FREC_SIZE;
    let mut cursor = align_up(records_end as u64 + stroff as u64, 16);
    for s in secs.iter_mut() {
        cursor = align_up(cursor, s.align as u64);
        s.load_off = cursor as u32;
        cursor += s.size as u64;
    }
    let blob_len = cursor as usize;

    /* section content base offsets (pre-reloc values) */
    let blob_sec_base: Vec<u64> = secs.iter().map(|s| s.load_off as u64).collect();
    let elf_sec_base = |si: usize| -> u64 {
        elf_to_blob
            .get(&si)
            .map(|&bi| blob_sec_base[bi])
            .unwrap_or(0)
    };

    /* resolve symtab value_off */
    for (i, (sec, val)) in sym_value_sec
        .iter()
        .enumerate()
        .filter_map(|(i, x)| x.map(|v| (i, v)))
    {
        if let Some(&bi) = sym_to_blob.get(&i) {
            blob_syms[bi].value_off = (elf_sec_base(sec) + val) as u32;
        }
    }

    /* ---- pass 2: value relocs, emit patches ---- */
    let mut fixups: Vec<Fixup> = Vec::new();
    let mut patches: Vec<(usize, usize, [u8; 4])> = Vec::new(); // (blob_sec, off_in_sec, bytes)
    let imp_index =
        |name: &str| -> u32 { import_names.iter().position(|n| n == name).unwrap() as u32 };

    for (r, act) in elf.relocs.iter().zip(actions.iter()) {
        let Action::Fix {
            r#type,
            sym,
            addend,
        } = act
        else {
            continue;
        };
        let sy = &elf.syms[*sym];
        let undef = sy.shndx == SHN_UNDEF;
        let place_blob = elf_sec_base(r.place_sec) + r.place_off;
        let ftype = *r#type;
        /* target blob offset for defined syms */
        let s_off = if undef {
            0
        } else if sy.shndx as usize >= elf.secs.len() {
            return Err(format!("reloc sym {} has bad shndx", sy.name));
        } else if !elf_to_blob.contains_key(&(sy.shndx as usize)) {
            return Err(format!("reloc sym {} lives in a dropped section", sy.name));
        } else {
            elf_sec_base(sy.shndx as usize) + sy.value
        };
        match ftype {
            F_ABS32 => {
                let (value, add) = if undef {
                    (F_IMPORT | imp_index(&sy.name), *addend as i32)
                } else {
                    (s_off as u32, *addend as i32)
                };
                fixups.push(Fixup {
                    place_off: place_blob as u32,
                    r#type: F_ABS32,
                    value,
                    addend: add,
                });
            }
            F_REL32 => {
                if undef {
                    fixups.push(Fixup {
                        place_off: place_blob as u32,
                        r#type: F_REL32,
                        value: F_IMPORT | imp_index(&sy.name),
                        addend: *addend as i32,
                    });
                } else {
                    /* field = S + A - P (base cancels) */
                    let v = (s_off as i64 + *addend - place_blob as i64) as u32;
                    let bi = elf_to_blob[&r.place_sec];
                    patches.push((bi, r.place_off as usize, v.to_le_bytes()));
                }
            }
            F_THM_CALL => {
                if undef {
                    fixups.push(Fixup {
                        place_off: place_blob as u32,
                        r#type: F_THM_CALL,
                        value: F_IMPORT | imp_index(&sy.name),
                        addend: *addend as i32,
                    });
                } else {
                    /* imm32 = (S + A - P) & ~1 — identical math to
                    the linker since both sides are blob offsets.
                    Thumb BL imm32 is 25-bit signed: ±16 MiB. */
                    let d = s_off as i64 + *addend - place_blob as i64;
                    if !(-0x100_0000..0x100_0000).contains(&d) {
                        return Err(format!(
                            "THM_CALL target out of ±16MiB range at blob offset {place_blob}"
                        ));
                    }
                    let imm = (d & !1) as i32;
                    let (h1, h2) = thm_call_pack(imm);
                    let mut b4 = [0u8; 4];
                    b1copy(&mut b4, h1, h2);
                    let bi = elf_to_blob[&r.place_sec];
                    patches.push((bi, r.place_off as usize, b4));
                }
            }
            F_MOVW | F_MOVT => {
                let (value, add) = if undef {
                    (F_IMPORT | imp_index(&sy.name), *addend as i32)
                } else {
                    (s_off as u32, *addend as i32)
                };
                fixups.push(Fixup {
                    place_off: place_blob as u32,
                    r#type: ftype,
                    value,
                    addend: add,
                });
            }
            _ => unreachable!("action type {ftype}"),
        }
    }

    /* ---- apply patches to section contents ---- */
    for (bi, off, bytes) in &patches {
        let s = &mut secs[*bi];
        let o = *off;
        if o + 4 > s.content.len() {
            return Err("emit patch outside section".into());
        }
        s.content[o..o + 4].copy_from_slice(bytes);
    }

    /* ---- serialize ---- */
    let strbytes = {
        let mut v = vec![0u8; stroff as usize];
        for (name, &o) in &strtab {
            v[o as usize..o as usize + name.len()].copy_from_slice(name.as_bytes());
        }
        v
    };
    let mut out = Vec::with_capacity(blob_len);
    out.extend_from_slice(&TPLB_MAGIC.to_le_bytes());
    out.extend_from_slice(&TPLB_VERSION.to_le_bytes());
    out.extend_from_slice(&(TPLB_HEADER_SIZE as u32).to_le_bytes());
    out.extend_from_slice(&nsecs.to_le_bytes());
    out.extend_from_slice(&nsyms.to_le_bytes());
    out.extend_from_slice(&nimports.to_le_bytes());
    out.extend_from_slice(&(fixups.len() as u32).to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes()); // flags
    debug_assert_eq!(out.len(), TPLB_HEADER_SIZE);
    for s in &secs {
        out.extend_from_slice(&strtab[&s.name].to_le_bytes());
        out.extend_from_slice(&s.load_off.to_le_bytes());
        out.extend_from_slice(&s.size.to_le_bytes());
        out.extend_from_slice(&s.align.to_le_bytes());
        out.extend_from_slice(&s.flags.to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes());
    }
    for s in &blob_syms {
        out.extend_from_slice(&s.name_off.to_le_bytes());
        out.extend_from_slice(&s.value_off.to_le_bytes());
        out.extend_from_slice(&s.size.to_le_bytes());
        out.push(s.info);
        out.extend_from_slice(&[0; 3]);
    }
    for n in &import_names {
        out.extend_from_slice(&strtab[n].to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes());
    }
    for f in &fixups {
        out.extend_from_slice(&f.place_off.to_le_bytes());
        out.extend_from_slice(&f.r#type.to_le_bytes());
        out.extend_from_slice(&f.value.to_le_bytes());
        out.extend_from_slice(&f.addend.to_le_bytes());
    }
    out.extend_from_slice(&strbytes);
    /* pad to first content offset, then place each section */
    let first = secs
        .iter()
        .map(|s| s.load_off as usize)
        .min()
        .unwrap_or(out.len());
    if out.len() < first {
        out.resize(first, 0);
    }
    for s in &secs {
        let end = s.load_off as usize + s.content.len();
        if out.len() < end {
            out.resize(end, 0);
        }
        out[s.load_off as usize..end].copy_from_slice(&s.content);
    }
    out.truncate(blob_len);
    Ok(out)
}

fn b1copy(b: &mut [u8; 4], h1: u16, h2: u16) {
    b[..2].copy_from_slice(&h1.to_le_bytes());
    b[2..].copy_from_slice(&h2.to_le_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    /* Thumb BL codec roundtrip over the interesting immediates:
    zero, small ±, the ±16MiB edges, and the -4 addend LLVM emits
    for unresolved calls. */
    #[test]
    fn thm_call_codec_roundtrip() {
        for imm in [
            0,
            2,
            -2,
            4,
            -4,
            0x100,
            -0x100,
            0xfffe,
            -0xfffe,
            0xffffe,
            -0xffffe,
            0x1000000 - 2,
            -(0x1000000 - 2),
        ] {
            let (h1, h2) = thm_call_pack(imm);
            assert_eq!(thm_call_imm(h1, h2), imm, "imm {imm}");
        }
    }

    #[test]
    fn rejects_non_arm_and_non_rel() {
        assert!(build_blob(b"not an elf").is_err());
        /* 'ELF' + class64 + little-endian + ET_REL + EM_386 →
        parses far enough to hit the EM_ARM machine check */
        let mut e = vec![0u8; 128];
        e[0..4].copy_from_slice(&[0x7f, b'E', b'L', b'F']);
        e[4] = 2;
        e[5] = 1;
        e[16..18].copy_from_slice(&1u16.to_le_bytes()); // ET_REL
        e[18..20].copy_from_slice(&3u16.to_le_bytes()); // EM_386
        e[40..48].copy_from_slice(&64u64.to_le_bytes()); // shoff
        e[58..60].copy_from_slice(&64u16.to_le_bytes()); // shentsize
        e[60..62].copy_from_slice(&1u16.to_le_bytes()); // shnum
        e[62..64].copy_from_slice(&0u16.to_le_bytes()); // shstrndx=0
        assert!(build_blob(&e).is_err());
    }
}
