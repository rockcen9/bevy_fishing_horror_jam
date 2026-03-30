use serde::{Deserialize, Deserializer, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::Write;

fn deserialize_bool_from_str<'de, D: Deserializer<'de>>(d: D) -> Result<bool, D::Error> {
    let s = String::deserialize(d)?;
    match s.to_uppercase().as_str() {
        "TRUE" => Ok(true),
        _ => Ok(false),
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct ItemData {
    #[serde(default)]
    id: String,
    #[serde(default, deserialize_with = "deserialize_bool_from_str")]
    random_color: bool,
    #[serde(default)]
    file_path: String,
}

const BASE_URL: &str = "https://docs.google.com/spreadsheets/d/e/2PACX-1vS-dVhyv9qLlv53Pdqab4-mI_w2coXoKgqG0HL_zfegBF5ucpOqIZx4bM67J704IlgcTmmetUuaMH5Q/pub";

fn download_csv<T: for<'de> Deserialize<'de>>(gid: &str) -> Result<Vec<T>, Box<dyn Error>> {
    let url = format!("{BASE_URL}?gid={gid}&single=true&output=csv");
    let response_text = reqwest::blocking::get(&url)?.text()?;
    let mut reader = csv::ReaderBuilder::new()
        .flexible(true)
        .from_reader(response_text.as_bytes());
    let mut rows: Vec<T> = Vec::new();
    for result in reader.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}

fn save_json<T: Serialize>(data: &T, path: &str) -> Result<(), Box<dyn Error>> {
    let json_string = serde_json::to_string_pretty(data)?;
    let mut file = File::create(path)?;
    file.write_all(json_string.as_bytes())?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Downloading items data...");
    let items: Vec<ItemData> = download_csv("1717474158")?;
    println!("Parsed {} items", items.len());
    save_json(&items, "assets/balance/items.json")?;
    println!("Saved assets/balance/items.json");

    println!("Done!");
    Ok(())
}
