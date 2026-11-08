//! Blob-migration property fuzz (doc 14 §5): random old/new layout
//! pairs through `migrate::diff` + `plan_to_emit`.
//!
//! Invariants checked every iteration:
//! - verdict matches the rule table: deny <=> a Deny rule fired
//!   (reason 6 demoted to a report under `allow_retain_loss`)
//! - every new leaf gets exactly one primary rule
//!   (Copy | Transform | DefaultInit) unless the plan denied
//! - every old leaf is consumed: copied, transformed, dropped, or
//!   denied as a removed member
//! - a type-changed match never emits Copy — only Transform (allowed
//!   table) or Deny
//! - emitted ops never read/write outside their blobs, and the
//!   copied/widened bytes are exactly what the rules promise

use tplc_mir::migrate::{self, Leaf, MigOp, Rule, Side, Transform, TAG_ARRAY, TAG_STRING};

/* ---------- deterministic PRNG (xorshift64*), no deps ---------- */

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
    fn chance(&mut self, pct: u64) -> bool {
        self.below(100) < pct
    }
    fn pick<'a, T>(&mut self, v: &'a [T]) -> &'a T {
        &v[self.below(v.len() as u64) as usize]
    }
}

/* ---------- leaf construction ---------- */

/// scalar tag → storage size
fn tag_size(tag: u8) -> u64 {
    match tag {
        0 | 1 | 5 => 1,
        2 | 6 => 2,
        3 | 7 | 9 => 4,
        4 | 8 | 10 | 11 => 8,
        _ => 0,
    }
}

const SCALARS: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

fn sig_of(tag: u8, size: u64, extra: &[u8]) -> Vec<u8> {
    let mut s = vec![tag];
    s.extend_from_slice(&(size as u32).to_le_bytes());
    s.extend_from_slice(extra);
    s
}

fn mk_leaf(
    name: String,
    tag: u8,
    size: u64,
    off: u64,
    retain: bool,
    kind: u8,
    field_idx: u32,
) -> Leaf {
    let (elem_sig, elem_size, lo, hi, sig) = if tag == TAG_ARRAY {
        // elements are always INT (tag 2, 2 B) in the fuzz model
        let esig = sig_of(2, 2, &[]);
        let n = size / 2;
        let ex = [
            &1i32.to_le_bytes()[..],
            &(n as i32).to_le_bytes()[..],
            &esig[..],
        ]
        .concat();
        (esig, 2, 1, n as i32, sig_of(tag, size, &ex))
    } else {
        (vec![], 0, 0, 0, sig_of(tag, size, &[]))
    };
    Leaf {
        name,
        abs_off: off,
        size,
        tag,
        elem_sig,
        elem_size,
        lo,
        hi,
        sig,
        field_idx,
        retain,
        kind,
    }
}

/// random leaf: name taken so old/new overlap, sequential offsets
fn rand_leaf(rng: &mut Rng, name: String, off: &mut u64, idx: u32) -> Leaf {
    let r = rng.below(100);
    let (tag, size) = if r < 70 {
        let t = *rng.pick(SCALARS);
        (t, tag_size(t))
    } else if r < 85 {
        (TAG_STRING, 4 + rng.below(60))
    } else {
        (TAG_ARRAY, 2 * (1 + rng.below(8)))
    };
    let l = mk_leaf(
        name,
        tag,
        size.max(1),
        *off,
        rng.chance(25),
        if rng.chance(10) { 7 } else { 1 },
        idx,
    );
    *off += l.size;
    l
}

/// mutate an old leaf into its new-side counterpart (or drop it)
fn mutate(rng: &mut Rng, ol: &Leaf, off: &mut u64, idx: u32) -> Option<Leaf> {
    if rng.chance(18) {
        return None; // dropped
    }
    let mut nl = ol.clone();
    nl.field_idx = idx;
    nl.retain = rng.chance(25);
    let r = rng.below(100);
    if r < 55 {
        // unchanged or plain re-size within same tag
        if ol.tag == TAG_STRING && rng.chance(50) {
            nl.size = 4 + rng.below(60);
            nl.sig = sig_of(TAG_STRING, nl.size, &[]);
        } else if ol.tag == TAG_ARRAY && rng.chance(50) {
            nl.size = 2 * (1 + rng.below(10));
            nl.hi = (nl.size / 2) as i32;
            nl.sig = sig_of(
                TAG_ARRAY,
                nl.size,
                &[
                    &nl.lo.to_le_bytes()[..],
                    &nl.hi.to_le_bytes()[..],
                    &ol.elem_sig[..],
                ]
                .concat(),
            );
        }
    } else if r < 75 {
        // retag: sometimes a legal widen, sometimes not
        nl.tag = *rng.pick(SCALARS);
        nl.size = tag_size(nl.tag);
        nl.sig = sig_of(nl.tag, nl.size, &[]);
        nl.elem_sig = vec![];
        nl.elem_size = 0;
        nl.lo = 0;
        nl.hi = 0;
        if rng.chance(15) {
            nl.kind = 7;
        }
    }
    nl.abs_off = *off;
    *off += nl.size.max(1);
    Some(nl)
}

fn gen_case(rng: &mut Rng) -> (Side, Side, Vec<(String, String)>, bool) {
    // old side: mix of plain vars and fb-member paths
    let mut old_leaves = Vec::new();
    let mut off = 0u64;
    let n = rng.below(10) as usize + 2;
    for i in 0..n {
        let name = if rng.chance(25) {
            format!("fb{}.m{}", rng.below(3), rng.below(3))
        } else {
            format!("v{}", rng.below(12))
        };
        if old_leaves.iter().any(|l: &Leaf| l.name == name) {
            continue;
        }
        old_leaves.push(rand_leaf(rng, name, &mut off, i as u32));
    }
    if old_leaves.is_empty() {
        old_leaves.push(rand_leaf(rng, "v0".into(), &mut off, 0));
    }

    // new side: mutate survivors, then add fresh fields
    let mut new_leaves = Vec::new();
    let mut noff = 0u64;
    for (i, ol) in old_leaves.iter().enumerate() {
        if let Some(nl) = mutate(rng, ol, &mut noff, 0x1000 + i as u32) {
            new_leaves.push(nl);
        }
    }
    let added = rng.below(5) as usize;
    for j in 0..added {
        new_leaves.push(rand_leaf(
            rng,
            format!("n{j}"),
            &mut noff,
            0x1000 + (n + j) as u32,
        ));
    }

    // renames: pair a dropped old leaf with an added new leaf
    let new_names: Vec<String> = new_leaves.iter().map(|l| l.name.clone()).collect();
    let mut renames = Vec::new();
    for ol in &old_leaves {
        if new_names.iter().any(|x| x == &ol.name) {
            continue;
        }
        if let Some(nn) = new_names.iter().find(|x| {
            !old_leaves.iter().any(|o| &o.name == *x) && !renames.iter().any(|(_, b)| b == *x)
        }) {
            if rng.chance(40) {
                renames.push((ol.name.clone(), nn.clone()));
            }
        }
    }

    let old_retain_used: u64 = old_leaves.iter().filter(|l| l.retain).map(|l| l.size).sum();
    let new_retain = match rng.below(4) {
        0 => old_retain_used.saturating_sub(1 + rng.below(4)),
        1 => old_retain_used,
        _ => old_retain_used + rng.below(16),
    };

    (
        Side {
            leaves: old_leaves,
            retain_region: (0, old_retain_used.max(64)),
        },
        Side {
            leaves: new_leaves,
            retain_region: (0, new_retain),
        },
        renames,
        rng.chance(50),
    )
}

/* ---------- emit executor: apply ops to byte blobs ---------- */

fn load_i64(b: &[u8], off: u64, tag: u8) -> i64 {
    let o = off as usize;
    match tag {
        1 => b[o] as i8 as i64,
        2 => i16::from_le_bytes(b[o..o + 2].try_into().unwrap()) as i64,
        3 => i32::from_le_bytes(b[o..o + 4].try_into().unwrap()) as i64,
        4 | 11 => i64::from_le_bytes(b[o..o + 8].try_into().unwrap()),
        _ => panic!("signed tag {tag}"),
    }
}

fn load_u64(b: &[u8], off: u64, tag: u8) -> u64 {
    let o = off as usize;
    match tag {
        0 | 5 => b[o] as u64,
        6 => u16::from_le_bytes(b[o..o + 2].try_into().unwrap()) as u64,
        7 => u32::from_le_bytes(b[o..o + 4].try_into().unwrap()) as u64,
        8 => u64::from_le_bytes(b[o..o + 8].try_into().unwrap()),
        _ => panic!("unsigned tag {tag}"),
    }
}

fn store_i64(b: &mut [u8], off: u64, tag: u8, v: i64) {
    let o = off as usize;
    match tag {
        1 => b[o] = v as i8 as u8,
        2 => b[o..o + 2].copy_from_slice(&(v as i16).to_le_bytes()),
        3 => b[o..o + 4].copy_from_slice(&(v as i32).to_le_bytes()),
        4 | 11 => b[o..o + 8].copy_from_slice(&v.to_le_bytes()),
        _ => panic!("signed tag {tag}"),
    }
}

fn store_u64(b: &mut [u8], off: u64, tag: u8, v: u64) {
    let o = off as usize;
    match tag {
        0 | 5 => b[o] = v as u8,
        6 => b[o..o + 2].copy_from_slice(&(v as u16).to_le_bytes()),
        7 => b[o..o + 4].copy_from_slice(&(v as u32).to_le_bytes()),
        8 => b[o..o + 8].copy_from_slice(&v.to_le_bytes()),
        _ => panic!("unsigned tag {tag}"),
    }
}

fn load_f64(b: &[u8], off: u64, tag: u8) -> f64 {
    let o = off as usize;
    match tag {
        9 => f32::from_le_bytes(b[o..o + 4].try_into().unwrap()) as f64,
        10 => f64::from_le_bytes(b[o..o + 8].try_into().unwrap()),
        _ => panic!("real tag {tag}"),
    }
}

fn store_f64(b: &mut [u8], off: u64, tag: u8, v: f64) {
    let o = off as usize;
    match tag {
        9 => b[o..o + 4].copy_from_slice(&(v as f32).to_le_bytes()),
        10 => b[o..o + 8].copy_from_slice(&v.to_le_bytes()),
        _ => panic!("real tag {tag}"),
    }
}

/// Reference executor for MigOp — panics on any out-of-bounds access
/// (an op escaping its blob is the "corruption" this fuzz hunts).
fn apply(ops: &[MigOp], old: &[u8], new: &mut [u8]) {
    for op in ops {
        match *op {
            MigOp::Copy {
                old_off,
                new_off,
                size,
            } => {
                let (o, n, s) = (old_off as usize, new_off as usize, size as usize);
                assert!(o + s <= old.len(), "Copy reads past old blob");
                assert!(n + s <= new.len(), "Copy writes past new blob");
                new[n..n + s].copy_from_slice(&old[o..o + s]);
            }
            MigOp::CopyMin {
                old_off,
                new_off,
                old_size,
                new_size,
            } => {
                let (o, n) = (old_off as usize, new_off as usize);
                let s = old_size.min(new_size) as usize;
                assert!(o + s <= old.len(), "CopyMin reads past old blob");
                assert!(n + s <= new.len(), "CopyMin writes past new blob");
                new[n..n + s].copy_from_slice(&old[o..o + s]);
            }
            MigOp::Widen {
                old_off,
                new_off,
                old_tag,
                new_tag,
            } => {
                assert!(
                    tag_size(new_tag) > tag_size(old_tag),
                    "widen narrows {old_tag}->{new_tag}"
                );
                if old_tag == 9 && new_tag == 10 {
                    let v = load_f64(old, old_off, old_tag);
                    store_f64(new, new_off, new_tag, v);
                } else if (1..=4).contains(&old_tag) && (1..=4).contains(&new_tag) {
                    let v = load_i64(old, old_off, old_tag);
                    store_i64(new, new_off, new_tag, v);
                } else if (5..=8).contains(&old_tag) && (5..=8).contains(&new_tag) {
                    let v = load_u64(old, old_off, old_tag);
                    store_u64(new, new_off, new_tag, v);
                } else {
                    panic!("widen outside transform table {old_tag}->{new_tag}");
                }
            }
        }
    }
}

/* ---------- invariants ---------- */

#[test]
fn migrate_diff_fuzz() {
    let mut rng = Rng(0x9E3779B97F4A7C15);
    let mut seen = [0u32; 6]; // rule-kind coverage
    for iter in 0..400 {
        let (old, new, renames, allow_loss) = gen_case(&mut rng);
        let old_blob_size = old
            .leaves
            .iter()
            .map(|l| l.abs_off + l.size)
            .max()
            .unwrap_or(0) as usize;
        let new_blob_size = new
            .leaves
            .iter()
            .map(|l| l.abs_off + l.size)
            .max()
            .unwrap_or(0) as usize;
        let plan = migrate::diff(&old, &new, &renames, allow_loss);

        // rule-kind coverage tally
        for r in &plan.rules {
            match r {
                Rule::Copy { .. } => seen[0] += 1,
                Rule::Transform { .. } => seen[1] += 1,
                Rule::DefaultInit { .. } => seen[2] += 1,
                Rule::Dropped { .. } => seen[3] += 1,
                Rule::Deny { .. } => seen[4] += 1,
                Rule::Alias { .. } => seen[5] += 1,
            }
        }

        let deny_rules: Vec<&Rule> = plan
            .rules
            .iter()
            .filter(|r| matches!(r, Rule::Deny { .. }))
            .collect();

        // verdict coherence: deny <=> Deny rule, except reason 6
        // demoted by allow_retain_loss (reported, not blocking)
        if plan.deny {
            assert!(!deny_rules.is_empty(), "iter {iter}: deny w/o rule");
        } else if allow_loss {
            for r in &deny_rules {
                if let Rule::Deny { reason, .. } = r {
                    assert_eq!(
                        *reason,
                        migrate::DENY_RETAIN_SHRINK,
                        "iter {iter}: non-DENY6 rule w/o deny"
                    );
                }
            }
        } else {
            assert!(deny_rules.is_empty(), "iter {iter}: rules but no deny");
        }

        if !plan.deny {
            // every new leaf: exactly one primary rule
            for nl in &new.leaves {
                let prim = plan
                    .rules
                    .iter()
                    .filter(|r| match r {
                        Rule::Copy { new_field, .. }
                        | Rule::Transform { new_field, .. }
                        | Rule::DefaultInit { new_field, .. } => *new_field == nl.field_idx,
                        _ => false,
                    })
                    .count();
                assert_eq!(prim, 1, "iter {iter}: {} has {prim} rules", nl.name);
            }
            // every old leaf: consumed exactly once by a work rule
            // (Alias rules are auxiliary evidence for renames, not
            // consumption — a renamed match still emits Copy/Transform)
            for ol in &old.leaves {
                let consumed = plan
                    .rules
                    .iter()
                    .filter(|r| match r {
                        Rule::Copy { old_field, .. }
                        | Rule::Transform { old_field, .. }
                        | Rule::Dropped { old_field, .. } => *old_field == ol.field_idx,
                        _ => false,
                    })
                    .count();
                assert_eq!(consumed, 1, "iter {iter}: {} consumed {consumed}x", ol.name);
            }
        }

        // a type-changed match never silently Copies
        let new_by_name: std::collections::BTreeMap<&str, &Leaf> =
            new.leaves.iter().map(|l| (l.name.as_str(), l)).collect();
        for ol in &old.leaves {
            let target = renames
                .iter()
                .find(|(o, _)| o == &ol.name)
                .map(|(_, n)| n.as_str())
                .unwrap_or(ol.name.as_str());
            let Some(nl) = new_by_name.get(target) else {
                continue;
            };
            if ol.sig != nl.sig || ol.size != nl.size {
                let copied = plan.rules.iter().any(
                    |r| matches!(r, Rule::Copy { old_field, .. } if *old_field == ol.field_idx),
                );
                assert!(
                    !copied,
                    "iter {iter}: type-changed {} copied verbatim",
                    nl.name
                );
            }
        }

        // emit: apply ops, verify bounds safety (in apply) + bytes
        let emit = migrate::plan_to_emit(&plan);
        assert_eq!(emit.degraded, plan.degraded);
        let mut old_blob = vec![0u8; old_blob_size.max(1)];
        for b in old_blob.iter_mut() {
            *b = rng.next() as u8;
        }
        let mut new_blob = vec![0xEEu8; new_blob_size.max(1)];
        apply(&emit.ops, &old_blob, &mut new_blob);

        for r in &plan.rules {
            match r {
                Rule::Copy {
                    old_off,
                    new_off,
                    size,
                    ..
                } => {
                    let (o, n, s) = (*old_off as usize, *new_off as usize, *size as usize);
                    assert_eq!(
                        &new_blob[n..n + s],
                        &old_blob[o..o + s],
                        "iter {iter}: copy bytes differ"
                    );
                }
                Rule::Transform {
                    id,
                    old_off,
                    new_off,
                    old_size,
                    new_size,
                    old_tag,
                    new_tag,
                    ..
                } => match id {
                    Transform::IntWiden => assert_eq!(
                        load_i64(&new_blob, *new_off, *new_tag),
                        load_i64(&old_blob, *old_off, *old_tag),
                        "iter {iter}: int widen value changed"
                    ),
                    Transform::UintWiden => assert_eq!(
                        load_u64(&new_blob, *new_off, *new_tag),
                        load_u64(&old_blob, *old_off, *old_tag),
                        "iter {iter}: uint widen value changed"
                    ),
                    Transform::RealWiden => assert_eq!(
                        load_f64(&new_blob, *new_off, *new_tag),
                        load_f64(&old_blob, *old_off, *old_tag),
                        "iter {iter}: real widen value changed"
                    ),
                    Transform::StringTrunc | Transform::ArrayGrow => {
                        let s = (*old_size).min(*new_size) as usize;
                        assert_eq!(
                            &new_blob[*new_off as usize..*new_off as usize + s],
                            &old_blob[*old_off as usize..*old_off as usize + s],
                            "iter {iter}: trunc/grow prefix differs"
                        );
                    }
                },
                _ => {}
            }
        }
    }

    // coverage: the corpus must have hit every rule kind
    for (i, c) in seen.iter().enumerate() {
        let kind = [
            "Copy",
            "Transform",
            "DefaultInit",
            "Dropped",
            "Deny",
            "Alias",
        ][i];
        assert!(*c > 0, "rule kind {kind} never generated");
    }
}
