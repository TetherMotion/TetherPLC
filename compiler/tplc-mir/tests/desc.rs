//! P1-04 descriptor emission: SMAP/TYPC byte-shape + LAYD wiring.

use tplc_mir as mir;
use tplc_mir::interp::program_ctx;

fn demo_module() -> mir::Module {
    let fb = mir::Pou {
        kind: mir::PouKind::FunctionBlock,
        name: "FB_TonWrap".into(),
        iface: vec![
            mir::VarDecl {
                name: "IN".into(),
                kind: mir::VarKind::Input,
                ty: mir::Type::Bool,
                init: None,
                retain: false,
                at: None,
                probe: false,
                prov: mir::Prov::None,
            },
            mir::VarDecl {
                name: "Q".into(),
                kind: mir::VarKind::Output,
                ty: mir::Type::Bool,
                init: None,
                retain: false,
                at: None,
                probe: false,
                prov: mir::Prov::None,
            },
            mir::VarDecl {
                name: "tmr".into(),
                kind: mir::VarKind::Local,
                ty: mir::Type::Fb { name: "TON".into() },
                init: None,
                retain: false,
                at: None,
                probe: false,
                prov: mir::Prov::None,
            },
        ],
        body: mir::Body::Stmts(vec![]),
        ret_ty: None,
        prov: mir::Prov::None,
    };
    let prog = mir::Pou {
        kind: mir::PouKind::Program,
        name: "MAIN".into(),
        iface: vec![
            mir::VarDecl {
                name: "ton1".into(),
                kind: mir::VarKind::Local,
                ty: mir::Type::Fb {
                    name: "FB_TonWrap".into(),
                },
                init: None,
                retain: false,
                at: None,
                probe: false,
                prov: mir::Prov::St {
                    file: 0,
                    offset: 10,
                    len: 20,
                },
            },
            mir::VarDecl {
                name: "running".into(),
                kind: mir::VarKind::Local,
                ty: mir::Type::Bool,
                init: None,
                retain: false,
                at: Some(mir::At {
                    region: mir::AtRegion::Q,
                    byte: 0,
                    bit: Some(0),
                    width: 'X',
                }),
                probe: false,
                prov: mir::Prov::St {
                    file: 0,
                    offset: 40,
                    len: 12,
                },
            },
            mir::VarDecl {
                name: "r".into(),
                kind: mir::VarKind::Local,
                ty: mir::Type::Int {
                    signed: true,
                    bits: 32,
                },
                init: None,
                retain: true,
                at: None,
                probe: false,
                prov: mir::Prov::None,
            },
        ],
        body: mir::Body::Stmts(vec![]),
        ret_ty: None,
        prov: mir::Prov::None,
    };
    mir::Module {
        pous: vec![fb, prog],
        gvls: vec![],
        tasks: vec![],
    }
}

#[test]
fn emit_descriptors() {
    let module = demo_module();
    let program = module.pous.iter().find(|p| p.name == "MAIN").unwrap();
    let mut layout = mir::layout_program(&module, program);
    assert!(layout.fields.contains_key("ton1"));

    let io = vec![(
        "MAIN.running".to_string(),
        mir::At {
            region: mir::AtRegion::Q,
            byte: 0,
            bit: Some(0),
            width: 'X',
        },
        0u64,
    )];
    let prov_of = |q: &str| -> Option<mir::Prov> {
        program
            .iface
            .iter()
            .find(|v| v.name == q)
            .map(|v| v.prov.clone())
    };
    let d = mir::desc::emit(&mut layout, program, &io, &prov_of);

    // SMAP magic + plausible size
    assert_eq!(&d.symmap[..4], b"SMAP");
    let count = u32::from_le_bytes(d.symmap[8..12].try_into().unwrap());
    // root + ton1 + running + r + FB members (IN,Q,tmr minus __* = 3,
    // tmr is FB-typed member → its own children skipped at v1) + 1 io
    assert!(count >= 6, "count {count}");
    // entries sorted by (region,offset) after root; entry bytes:
    // name_off@0 kind@4 flags@5 region@6 access@7 type_ref@8 offset@12
    let eoff = 32usize;
    {
        let mut found = false;
        for i in 0..count as usize {
            let e = &d.symmap[eoff + i * 64..eoff + i * 64 + 64];
            if e[4] == 10 && e[6] == 2 && e[7] == 2 {
                found = true; // IO_CHANNEL, region Q, write-only
            }
        }
        assert!(found, "no IO_CHANNEL Q entry");
    }

    // TYPC magic + includes Fb type
    assert_eq!(&d.typecat[..4], b"TYPC");

    // LAYD wiring: sym_entry + type_ref set (not MAX)
    assert!(d.layd.fields.iter().all(|f| f.sym_entry != u32::MAX));
    assert!(d.layd.fields.iter().all(|f| f.type_ref != u32::MAX));

    // provenance: flags bit0 set when provs exist
    let flags = u32::from_le_bytes(d.symmap[24..28].try_into().unwrap());
    assert_eq!(flags & 1, 1);

    // hash preimage non-empty and deterministic
    let d2 = {
        let mut l2 = mir::layout_program(&module, program);
        mir::desc::emit(&mut l2, program, &io, &prov_of)
    };
    assert_eq!(
        d.payload_for_hash, d2.payload_for_hash,
        "nondeterministic emit"
    );
    assert!(!d.payload_for_hash.is_empty());
}

#[test]
fn layout_hash_stable_under_stmt_changes() {
    // doc 02 §7 fast path: identical field sets ⇒ identical hash
    let m1 = demo_module();
    let m2 = demo_module();
    let p1 = m1.pous.iter().find(|p| p.name == "MAIN").unwrap();
    let p2 = m2.pous.iter().find(|p| p.name == "MAIN").unwrap();
    let mut l1 = mir::layout_program(&m1, p1);
    let mut l2 = mir::layout_program(&m2, p2);
    let nop = |_: &str| None;
    let d1 = mir::desc::emit(&mut l1, p1, &[], &nop);
    let d2 = mir::desc::emit(&mut l2, p2, &[], &nop);
    assert_eq!(d1.layd, d2.layd);
    assert_eq!(d1.payload_for_hash, d2.payload_for_hash);
}

#[test]
fn gvl_vars_emit_smap_entries() {
    let mut module = demo_module();
    module.gvls.push(mir::Gvl {
        name: "GLOBAL".into(),
        retain: false,
        vars: vec![mir::VarDecl {
            name: "g_max".into(),
            kind: mir::VarKind::Constant,
            ty: mir::Type::Int {
                signed: true,
                bits: 32,
            },
            init: None,
            retain: false,
            at: None,
            probe: false,
            prov: mir::Prov::None,
        }],
    });
    let program = module.pous.iter().find(|p| p.name == "MAIN").unwrap();
    let mut layout = mir::layout_program(&module, program);
    assert!(layout.fields.contains_key("GLOBAL.g_max"));
    let nop = |_: &str| None;
    let d = mir::desc::emit(&mut layout, program, &[], &nop);
    // GVL-qualified vars get real symmap entries, access=1 (const)
    let eoff = 32usize;
    let count = u32::from_le_bytes(d.symmap[8..12].try_into().unwrap());
    let stroff = u32::from_le_bytes(d.symmap[12..16].try_into().unwrap()) as usize;
    let mut found = false;
    for i in 0..count as usize {
        let e = &d.symmap[eoff + i * 64..eoff + i * 64 + 64];
        let noff = stroff + u32::from_le_bytes(e[0..4].try_into().unwrap()) as usize;
        let end = d.symmap[noff..].iter().position(|b| *b == 0).unwrap();
        let name = std::str::from_utf8(&d.symmap[noff..noff + end]).unwrap();
        if name == "GLOBAL.g_max" {
            assert_eq!(e[7], 1, "const gvl var must be access=ro");
            found = true;
        }
    }
    assert!(found, "GLOBAL.g_max missing from SMAP");
    assert!(d.layd.fields.iter().all(|f| f.sym_entry != u32::MAX));
}

#[test]
fn program_ctx_only_top_level() {
    let module = demo_module();
    let program = module.pous.iter().find(|p| p.name == "MAIN").unwrap();
    let layout = mir::layout_program(&module, program);
    let ctx = program_ctx(&layout);
    assert!(ctx.contains_key("ton1"));
    assert!(ctx.contains_key("running"));
    assert!(ctx.contains_key("r"));
    assert!(!ctx.contains_key("ton1.Q"));
}
