use protobuf::Message;
use scip::types::Index;
use std::{path::Path, time::Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("samples/rust-regex.scip");

    let start = Instant::now();
    let contents = std::fs::read(path)?;
    println!("Read {} bytes of contents", contents.len());

    // Deserializing
    let mut index = Index::default();
    index.merge_from_bytes(&contents)?;

    let end = Instant::now();
    // println!("We have an index!, {:?}", index);
    println!(
        "Elapsed time loading SCIP index: {:.2}ms",
        (end - start).as_millis()
    );

    Ok(())
}
