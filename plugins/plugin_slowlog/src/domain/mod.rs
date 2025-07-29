pub mod fields;
pub mod table;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SlowLogScanResult {
    pub matched_files: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportStatus {
    pub status: String,
}
