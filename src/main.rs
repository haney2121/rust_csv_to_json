//tutorial-read-serde-03.rs
use std::process;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::{collections::HashMap, io, error::Error};

// This introduces a type alias so that we can conveniently reference our
// record type.
type Record = HashMap<String, String>;

fn run() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut record_refs: Vec<Record> = [].to_vec();
    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);

        record_refs.push(record);
    }
    let file = File::create("entitles.json")?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &record_refs)?;
    writer.flush()?;

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}