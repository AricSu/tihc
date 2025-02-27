use anyhow::{Context, Result};
use serde_json::Value;

pub fn save_json_to_file(json: &Value, file_path: &str) -> Result<()> {
    let json_string = serde_json::to_string_pretty(json).context("Failed to serialize JSON")?;
    std::fs::write(file_path, json_string).context("Failed to write JSON to file")?;
    Ok(())
}
