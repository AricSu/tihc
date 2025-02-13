use std::fs::File;
use std::io::Write;
use serde_json::Value;
use std::error::Error;

pub fn save_json_to_file(json: &Value, file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(file_path)?;
    file.write_all(serde_json::to_string_pretty(json)?.as_bytes())?;
    Ok(())
}
