fn main() {
    let dir = std::env::args().nth(1).unwrap();
    for e in std::fs::read_dir(&dir).unwrap() {
        let p = e.unwrap().path();
        if p.extension().map(|x| x == "xml").unwrap_or(false) {
            let src = std::fs::read_to_string(&p).unwrap();
            match plcopen_io::import(&src) {
                Ok(rep) => {
                    let prj = &rep.project;
                    let blocks: usize = prj
                        .pous
                        .iter()
                        .flat_map(|p| &p.networks)
                        .map(|n| n.blocks.len())
                        .sum();
                    let wires: usize = prj
                        .pous
                        .iter()
                        .flat_map(|p| &p.networks)
                        .map(|n| n.wires.len())
                        .sum();
                    let nets: usize = prj.pous.iter().flat_map(|p| &p.networks).count();
                    println!(
                        "{}: {} pous {} nets {} blocks {} wires, {} notes",
                        p.file_name().unwrap().to_string_lossy(),
                        prj.pous.len(),
                        nets,
                        blocks,
                        wires,
                        rep.notes.len()
                    );
                    for n in &rep.notes {
                        println!("   note: {}", n.what);
                    }
                }
                Err(e) => println!(
                    "{}: IMPORT FAIL {e}",
                    p.file_name().unwrap().to_string_lossy()
                ),
            }
        }
    }
}
