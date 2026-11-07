//! Print the JSON Schema for `fbd.v1` (build with --features schema).
#[cfg(feature = "schema")]
fn main() {
    let s = schemars::schema_for!(fbd_model::FbdProject);
    println!("{}", serde_json::to_string_pretty(&s).unwrap());
}

#[cfg(not(feature = "schema"))]
fn main() {
    eprintln!("rebuild with --features schema");
    std::process::exit(2);
}
