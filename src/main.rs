use csv::Reader;
use std::error::Error;
use std::fs::File;

#[derive(Debug, serde::Deserialize)]
struct Record {
    id: usize,
    name: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "sample.csv";
    let file = File::open(file_path)?;
    let mut rdr = Reader::from_reader(file);

    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("(id, name): ({}, {})", record.id, record.name);
        println!("{:?}", record);
    }

    Ok(())
}
