use anyhow::{Context, Result};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use tracing::info;
use zip::ZipArchive;
use serde_json::Value;

/// Extracts a ZIP file to the specified output directory.
/// 
/// # Arguments
/// * `zip_path` - Path to the ZIP file to extract
/// * `output_dir` - Directory where the ZIP contents will be extracted
pub fn extract_zip_file(zip_path: &Path, output_dir: &Path) -> Result<()> {
    let zip_file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(zip_file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = output_dir.join(file.name());

        if let Some(parent) = outpath.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }

    info!("Extracted ZIP file to: {}", output_dir.display());
    Ok(())
}

/// Removes a specific file from the filesystem.
/// 
/// # Arguments
/// * `file_path` - Path to the file to remove
pub fn remove_file(file_path: &Path) -> Result<()> {
    if file_path.exists() {
        fs::remove_file(file_path)
            .with_context(|| format!("Failed to remove file: {}", file_path.display()))?;
        info!("Removed file: {}", file_path.display());
    }
    Ok(())
}

/// Removes a directory and all its contents recursively.
/// 
/// # Arguments
/// * `dir_path` - Path to the directory to remove
pub fn remove_dir_recursive(dir_path: &Path) -> Result<()> {
    if dir_path.exists() {
        fs::remove_dir_all(dir_path)
            .with_context(|| format!("Failed to remove directory: {}", dir_path.display()))?;
        info!("Removed directory and contents: {}", dir_path.display());
    }
    Ok(())
}


/// Ensures that a directory exists, creating it and its parents if necessary.
/// 
/// # Arguments
/// * `dir_path` - Path to the directory to ensure exists
pub fn ensure_dir_exists(dir_path: &Path) -> Result<()> {
    if !dir_path.exists() {
        fs::create_dir_all(dir_path)
            .with_context(|| format!("Failed to create directory: {}", dir_path.display()))
    } else {
        Ok(())
    }
}

/// Saves binary data to a file, creating parent directories if necessary.
/// 
/// # Arguments
/// * `data` - The binary data to save
/// * `file_path` - Path where the file should be saved
pub fn save_binary_data(data: &[u8], file_path: &Path) -> Result<()> {
    if let Some(parent) = file_path.parent() {
        ensure_dir_exists(parent)?;
    }

    let mut file = File::create(file_path)
        .with_context(|| format!("Failed to create file: {}", file_path.display()))?;

    file.write_all(data)
        .with_context(|| format!("Failed to write data to file: {}", file_path.display()))?;

    Ok(())
}



pub fn save_json_to_file(json: &Value, file_path: &str) -> Result<()> {
    let json_string = serde_json::to_string_pretty(json).context("Failed to serialize JSON")?;
    std::fs::write(file_path, json_string).context("Failed to write JSON to file")?;
    Ok(())
}
