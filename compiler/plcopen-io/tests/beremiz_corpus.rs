//! Beremiz corpus import coverage (doc 06 §13.2, doc 14 §3.1).
//! Real Beremiz-exported TC6 XML in tools/oracle-beremiz/corpus is
//! imported and ≥95% of FBD constructs (nodes + wire edges) must
//! land in the model — gaps report as notes, never silent drops.

use plcopen_io::import;

/// Count XML elements with `tag` inside each `<FBD>` body of `src`.
/// Delimiter-suffixed so `<inOutVariable` can't match `<inOutVariables>`.
fn fbd_constructs(src: &str, tag: &str) -> usize {
    let open = format!("<{tag} ");
    let bare = format!("<{tag}>");
    src.split("<FBD>")
        .skip(1)
        .filter_map(|rest| rest.split_once("</FBD>"))
        .map(|(body, _)| body.matches(&open).count() + body.matches(&bare).count())
        .sum()
}

fn model_counts(p: &fbd_model::FbdProject) -> (usize, usize) {
    let mut nodes = 0;
    let mut edges = 0;
    for net in p.pous.iter().flat_map(|p| &p.networks) {
        nodes += net.blocks.len()
            + net.comments.len()
            + net.labels.len()
            + net.jumps.len()
            + net.returns.len();
        edges += net.wires.iter().map(|w| w.dsts.len()).sum::<usize>();
    }
    (nodes, edges)
}

#[test]
fn beremiz_corpus_imports() {
    let dir =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tools/oracle-beremiz/corpus");
    let mut files: Vec<_> = std::fs::read_dir(&dir)
        .expect("corpus dir missing")
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().map(|x| x == "xml").unwrap_or(false))
        .collect();
    files.sort();
    assert!(files.len() >= 5, "corpus too small: {}", files.len());

    let mut tot_nodes = 0;
    let mut tot_edges = 0;
    let mut got_nodes = 0;
    let mut got_edges = 0;
    for f in &files {
        let src = std::fs::read_to_string(f).unwrap();
        let rep = import(&src).unwrap_or_else(|e| panic!("{}: {e}", f.display()));
        let xml_nodes = [
            "inVariable",
            "outVariable",
            "inOutVariable",
            "block",
            "comment",
        ]
        .iter()
        .map(|t| fbd_constructs(&src, t))
        .sum::<usize>();
        let xml_edges = fbd_constructs(&src, "connection");
        let (mn, me) = model_counts(&rep.project);
        println!(
            "{}: nodes {mn}/{xml_nodes} edges {me}/{xml_edges} notes {}",
            f.file_name().unwrap().to_string_lossy(),
            rep.notes.len()
        );
        tot_nodes += xml_nodes;
        tot_edges += xml_edges;
        got_nodes += mn;
        got_edges += me;
    }
    let node_cov = got_nodes as f64 / tot_nodes as f64;
    let edge_cov = got_edges as f64 / tot_edges as f64;
    println!(
        "coverage: nodes {got_nodes}/{tot_nodes} ({:.1}%) edges {got_edges}/{tot_edges} ({:.1}%)",
        node_cov * 100.0,
        edge_cov * 100.0
    );
    assert!(node_cov >= 0.95, "node coverage {node_cov:.3} < 95%");
    assert!(edge_cov >= 0.95, "edge coverage {edge_cov:.3} < 95%");
}
