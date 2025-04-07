pub mod pprof;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileConfig {
    pub base_url: String,
    pub component: String,
    pub collection_type: String,
    pub seconds: u64,
    pub output_dir: PathBuf,
}
