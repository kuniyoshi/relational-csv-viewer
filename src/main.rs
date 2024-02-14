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

#[derive(Debug)]
struct Database {
    characters: Vec<CharacterRecord>,
    skills: Vec<SkillRecord>,
    details: Vec<DetailRecord>,
}

fn read_csv<T>(file_path: &str) -> Result<Vec<T>, Box<dyn Error>>
where
    T: for<'de> serde::Deserialize<'de>, // `T`は`Deserialize`トレイトを実装している必要がある
{
    let file = File::open(file_path)?;
    let mut reader = Reader::from_reader(file);
    let mut result = Vec::new();

    for read in reader.deserialize() {
        let record: T = read?;
        result.push(record);
    }

    Ok(result)
}

fn main() -> Result<(), Box<dyn Error>> {
    let characters: Vec<CharacterRecord> = read_csv("data/characters.csv")?;
    let skills: Vec<SkillRecord> = read_csv("data/skills.csv")?;
    let details: Vec<DetailRecord> = read_csv("data/details.csv")?;
    let database = Database {
        characters,
        skills,
        details,
    };
    eprintln!("{:?}", database);

    Ok(())
}
