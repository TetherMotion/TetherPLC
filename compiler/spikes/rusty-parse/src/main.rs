//! P0.5 spike S1–S3 — ruSTy-as-library consumability probe (doc 13 §7).
//!
//! S1: parse a representative ST corpus via `plc_compiler::parser`, walk
//!     every AST node, verify source locations — "AST fully traversable".
//! S2: classify every AstStatement kind as lowerable-to-tplc-mir today /
//!     needs-semantics / dialect-excluded — the ≥90% criterion.
//! S3: run their index + TypeAnnotator and report whether annotation is
//!     additive (map on the side) or mutates the AST (entanglement).

use plc_ast::ast::{AstNode, AstStatement, LinkageType};
use plc_ast::control_statements::AstControlStatement;
use plc_ast::provider::IdProvider;
use plc_source::source_location::SourceLocationFactory;
use plc_source::SourceCode;
use std::collections::BTreeMap;

/* Representative corpus: one file covering our v1 dialect surface
 * (doc 05 §3/§4): PROGRAM + FB decl, VAR blocks incl. retain/AT, calls,
 * control flow, ops, types, arrays/struct/enum, init values. */
const CORPUS: &str = r#"
TYPE E_Color : (RED, GREEN, BLUE) := GREEN; END_TYPE
TYPE S_Point : STRUCT x : REAL; y : REAL; END_STRUCT END_TYPE

FUNCTION_BLOCK FB_TonWrap
VAR_INPUT  IN : BOOL; PT : TIME; END_VAR
VAR_OUTPUT Q  : BOOL; ET : TIME; END_VAR
VAR        tmr : TON; cnt : INT := 0; END_VAR
    tmr(IN := IN, PT := PT);
    Q := tmr.Q;
    ET := tmr.ET;
    IF IN THEN cnt := cnt + 1; ELSE cnt := 0; END_IF;
END_FUNCTION_BLOCK

FUNCTION F_Add : INT
VAR_INPUT a : INT; b : INT; END_VAR
    F_Add := a + b * 2;
END_FUNCTION

PROGRAM MAIN
VAR
    ton1     : FB_TonWrap;
    running  AT %QX0.0 : BOOL;
    sensor   AT %IW0.0 : INT;
    arr      : ARRAY[0..9] OF DINT := [10(0)];
    pt       : S_Point := (x := 1.0, y := 2.0);
    color    : E_Color;
    strv     : STRING[16] := 'hi';
    ticks    : UDINT;
END_VAR
VAR RETAIN
    r        : DINT := 42;
END_VAR
VAR CONSTANT KC : INT := 7; END_VAR
    ton1(IN := sensor > 0, PT := T#500ms);
    running := ton1.Q AND NOT running;
    ticks := ticks + 1;
    arr[0] := F_Add(INT#1, INT#2);
    CASE sensor OF
        0: ticks := 0;
        1..3: ticks := ticks * 2;
    ELSE
        ticks := 0;
    END_CASE;
    FOR i := 0 TO 9 BY 2 DO arr[i] := arr[i] + 1; END_FOR;
    WHILE ticks > 100 DO ticks := ticks - 1; END_WHILE;
    REPEAT ticks := ticks + 1; UNTIL ticks >= 200 END_REPEAT;
END_PROGRAM
"#;

/* S2 classification: which AST kinds map to tplc-mir directly today. */
#[derive(Clone, Copy, PartialEq)]
enum M {
    Direct,      // maps now (expr, assign, call, if/loop/case, refs)
    Semantics,   // lowerable but needs our sema (literals, direct access)
    Excluded,    // out of dialect (pointers, refs, super/this, jumps)
    Trivia,      // empty/default scaffolding
}

fn classify(n: &AstNode) -> M {
    match &n.stmt {
        AstStatement::EmptyStatement(_) | AstStatement::DefaultValue(_)
        | AstStatement::MultipliedStatement(_) => M::Trivia,
        AstStatement::Literal(_) | AstStatement::DirectAccess(_)
        | AstStatement::HardwareAccess(_) | AstStatement::RangeStatement(_)
        | AstStatement::VlaRangeStatement => M::Semantics,
        AstStatement::ReferenceExpr(_) | AstStatement::Identifier(_)
        | AstStatement::BinaryExpression(_) | AstStatement::UnaryExpression(_)
        | AstStatement::ExpressionList(_) | AstStatement::ParenExpression(_)
        | AstStatement::Assignment(_) | AstStatement::OutputAssignment(_)
        | AstStatement::CallStatement(_) | AstStatement::ControlStatement(_)
        | AstStatement::CaseCondition(_) | AstStatement::ExitStatement(_)
        | AstStatement::ContinueStatement(_) | AstStatement::ReturnStatement(_) => {
            M::Direct
        }
        AstStatement::RefAssignment(_) | AstStatement::Super(_)
        | AstStatement::This | AstStatement::JumpStatement(_)
        | AstStatement::LabelStatement(_) | AstStatement::AllocationStatement(_) => {
            M::Excluded
        }
        _ => M::Semantics, // anything newer counts as needs-semantics
    }
}

fn kind_name(n: &AstNode) -> &'static str {
    match &n.stmt {
        AstStatement::EmptyStatement(_) => "EmptyStatement",
        AstStatement::DefaultValue(_) => "DefaultValue",
        AstStatement::Literal(_) => "Literal",
        AstStatement::MultipliedStatement(_) => "MultipliedStatement",
        AstStatement::ReferenceExpr(_) => "ReferenceExpr",
        AstStatement::Identifier(_) => "Identifier",
        AstStatement::Super(_) => "Super",
        AstStatement::This => "This",
        AstStatement::DirectAccess(_) => "DirectAccess",
        AstStatement::HardwareAccess(_) => "HardwareAccess",
        AstStatement::BinaryExpression(_) => "BinaryExpression",
        AstStatement::UnaryExpression(_) => "UnaryExpression",
        AstStatement::ExpressionList(_) => "ExpressionList",
        AstStatement::ParenExpression(_) => "ParenExpression",
        AstStatement::RangeStatement(_) => "RangeStatement",
        AstStatement::VlaRangeStatement => "VlaRangeStatement",
        AstStatement::Assignment(_) => "Assignment",
        AstStatement::OutputAssignment(_) => "OutputAssignment",
        AstStatement::RefAssignment(_) => "RefAssignment",
        AstStatement::CallStatement(_) => "CallStatement",
        AstStatement::ControlStatement(_) => "ControlStatement",
        AstStatement::CaseCondition(_) => "CaseCondition",
        AstStatement::ExitStatement(_) => "Exit",
        AstStatement::ContinueStatement(_) => "Continue",
        AstStatement::ReturnStatement(_) => "Return",
        AstStatement::JumpStatement(_) => "Jump",
        AstStatement::LabelStatement(_) => "Label",
        AstStatement::AllocationStatement(_) => "Allocation",
        _ => "<other>",
    }
}

fn walk(n: &AstNode, counts: &mut BTreeMap<&'static str, usize>, stats: &mut Stats) {
    *counts.entry(kind_name(n)).or_default() += 1;
    match classify(n) {
        M::Direct => stats.direct += 1,
        M::Semantics => stats.sem += 1,
        M::Excluded => stats.excl += 1,
        M::Trivia => stats.trivia += 1,
    }
    if n.location.to_range().is_none() {
        stats.noloc += 1;
    } else {
        stats.loc += 1;
    }
    let mut kids = Vec::new();
    children(n, &mut kids);
    for c in kids {
        walk(c, counts, stats);
    }
}

fn children<'a>(n: &'a AstNode, out: &mut Vec<&'a AstNode>) {
    match &n.stmt {
        AstStatement::BinaryExpression(b) => {
            out.push(&b.left);
            out.push(&b.right);
        }
        AstStatement::UnaryExpression(u) => out.push(&u.value),
        AstStatement::ParenExpression(p) => out.push(p),
        AstStatement::ExpressionList(v) => out.extend(v.iter()),
        AstStatement::Assignment(a)
        | AstStatement::OutputAssignment(a)
        | AstStatement::RefAssignment(a) => {
            out.push(&a.left);
            out.push(&a.right);
        }
        AstStatement::CallStatement(c) => {
            out.push(&c.operator);
            if let Some(p) = &c.parameters {
                out.push(p);
            }
        }
        AstStatement::MultipliedStatement(m) => out.push(&m.element),
        AstStatement::ControlStatement(c) => control_children(c, out),
        AstStatement::CaseCondition(c) => out.push(c),
        AstStatement::ReferenceExpr(r) => {
            if let Some(b) = &r.base {
                out.push(b);
            }
        }
        _ => {}
    }
}

fn control_children<'a>(c: &'a AstControlStatement, out: &mut Vec<&'a AstNode>) {
    match c {
        AstControlStatement::If(i) => {
            for blk in &i.blocks {
                out.push(&blk.condition);
                out.extend(blk.body.iter());
            }
            out.extend(i.else_block.iter());
        }
        AstControlStatement::ForLoop(f) => {
            out.push(&f.counter);
            out.push(&f.start);
            out.push(&f.end);
            if let Some(b) = &f.by_step {
                out.push(b);
            }
            out.extend(f.body.iter());
        }
        AstControlStatement::WhileLoop(w) | AstControlStatement::RepeatLoop(w) => {
            out.push(&w.condition);
            out.extend(w.body.iter());
        }
        AstControlStatement::Case(c) => {
            out.push(&c.selector);
            for blk in &c.case_blocks {
                out.push(&blk.condition);
                out.extend(blk.body.iter());
            }
            out.extend(c.else_block.iter());
        }
        _ => {}
    }
}

#[derive(Default)]
struct Stats {
    loc: usize, noloc: usize,
    direct: usize, sem: usize, excl: usize, trivia: usize,
}

fn main() {
    let source = SourceCode::from(CORPUS);
    let idp = IdProvider::default();
    let factory = SourceLocationFactory::for_source(&source);
    let (mut unit, diags) = plc_compiler::parser::parse(
        plc_compiler::lexer::lex_with_ids(&source.source, idp.clone(), factory),
        LinkageType::Internal,
        "<corpus>",
    );
    println!("=== S1: parse ===");
    println!("parse diagnostics: {}", diags.len());
    for d in &diags {
        println!("  diag: {}", d.get_message());
    }
    plc_ast::ast::pre_process(&mut unit, idp.clone());

    println!("pous:");
    let mut counts = BTreeMap::new();
    let mut st = Stats::default();
    for pou in &unit.pous {
        println!(
            "  {:?} {}  var_blocks={} line {}",
            pou.kind,
            pou.name,
            pou.variable_blocks.len(),
            pou.name_location.get_line()
        );
    }
    for gv in &unit.global_vars {
        for v in &gv.variables {
            println!("  gvar {} (line {})", v.name, v.location.get_line());
        }
    }
    for imp in &unit.implementations {
        println!("impl {} ({} stmts)", imp.name, imp.statements.len());
        for s in &imp.statements {
            walk(s, &mut counts, &mut st);
        }
    }

    println!("\nnode-kind histogram:");
    for (k, n) in &counts {
        println!("  {:<22} {}", k, n);
    }
    println!("\nS1 verdict: {} nodes, {} with location, {} without",
             st.loc + st.noloc, st.loc, st.noloc);

    let total = st.direct + st.sem + st.excl;
    let pct = if total > 0 {
        100.0 * (st.direct + st.sem) as f64 / total as f64
    } else { 0.0 };
    println!("\n=== S2: tplc-mir boundary ===");
    println!("direct {}  needs-sema {}  dialect-excluded {}  (trivia {})",
             st.direct, st.sem, st.excl, st.trivia);
    println!("lowerable coverage: {:.0}% (criterion ≥90%)", pct);

    /* S3: index + annotation — is their sema additive or entangled? */
    println!("\n=== S3: sema entanglement ===");
    let mut index = plc_compiler::index::indexer::index(&unit);
    let builtins = plc_compiler::builtins::parse_built_ins(IdProvider::default());
    index.import(plc_compiler::index::indexer::index(&builtins));
    for t in plc_compiler::typesystem::get_builtin_types() {
        index.register_type(t);
    }
    let (ann, _deps, _strlits) = plc_compiler::resolver::TypeAnnotator::visit_unit(
        &index, &unit, IdProvider::default());
    println!("TypeAnnotator produced side-table annotations; AST untouched.");
    println!("probing annotated types on MAIN's statements:");
    for imp in &unit.implementations {
        if imp.name != "MAIN" { continue; }
        for s in &imp.statements {
            probe_ann(&ann, &index, s);
        }
    }
}

fn probe_ann(
    ann: &impl plc_compiler::resolver::AnnotationMap,
    index: &plc_compiler::index::Index,
    n: &AstNode,
) {
    if let Some(t) = ann.get_type(n, index) {
        println!("  line {:>3} {:<20} -> {}",
                 n.location.get_line(), kind_name(n), t.get_name());
    }
    let mut kids = Vec::new();
    children(n, &mut kids);
    for c in kids {
        probe_ann(ann, index, c);
    }
}
