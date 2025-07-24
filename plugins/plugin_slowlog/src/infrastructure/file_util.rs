use anyhow::{Context, Result};
use regex::Regex;
use std::fs;

/// Gets slow query log files that match the specified template
/// Returns a list of file paths that match the regex pattern in the specified directory
pub fn get_slowlog_files(slowlogdir: &str, logtemplate: &str) -> Result<Vec<String>> {
    let regex = Regex::new(logtemplate)
        .with_context(|| format!("Invalid regex pattern: {}", logtemplate))?;
    let path = std::path::Path::new(slowlogdir);
    if !path.exists() {
        return Err(anyhow::anyhow!("Directory does not exist: {}", slowlogdir));
    }
    if !path.is_dir() {
        return Err(anyhow::anyhow!("Path is not a directory: {}", slowlogdir));
    }
    let dir = fs::read_dir(slowlogdir)
        .with_context(|| format!("Failed to read directory: {}", slowlogdir))?;
    let mut files = Vec::new();
    for entry in dir {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                tracing::warn!("Failed to read directory entry: {}", e);
                continue;
            }
        };
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let file_name = match path.file_name().and_then(|n| n.to_str()) {
            Some(name) => name,
            None => {
                tracing::warn!("Invalid filename encoding: {:?}", path);
                continue;
            }
        };
        if regex.is_match(file_name) {
            files.push(path.to_string_lossy().into_owned());
        }
    }
    files.sort();
    match files.len() {
        0 => tracing::info!("No matching slow query log files found in {}", slowlogdir),
        1 => tracing::info!("Found 1 matching slow query log file"),
        n => tracing::info!("Found {} matching slow query log files", n),
    }
    for file in &files {
        tracing::debug!("Matched file: {}", file);
    }
    Ok(files)
}
