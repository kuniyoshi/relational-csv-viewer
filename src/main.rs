use csv::Reader;
use std::error::Error;
use std::fs::File;

#[derive(Debug, serde::Deserialize)]
struct CharacterRecord {
    id: usize,
    name: String,
}

#[derive(Debug, serde::Deserialize)]
struct SkillRecord {
    id: usize,
    character_id: usize,
    name: String,
    detail_id: usize,
}

#[derive(Debug, serde::Deserialize)]
struct DetailRecord {
    id: usize,
    description: String,
}

struct Database {
    characters: Vec<CharacterRecord>,
    skills: Vec<SkillRecord>,
    details: Vec<DetailRecord>,
}

fn read_characters(file_path: &str) -> Result<Vec<CharacterRecord>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut reader = Reader::from_reader(file);
    let mut result= Vec::new();

    for read in reader.deserialize() {
        let record: CharacterRecord = read?;
        result.push(record);
    }

    Ok(result)
}

fn main() -> Result<(), Box<dyn Error>> {
    let characters = read_characters("data/characters.csv");
    eprintln!("{:?}", characters);
    Ok(())
}
