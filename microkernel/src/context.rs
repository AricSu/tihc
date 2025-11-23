use std::sync::Arc;
use once_cell::sync::OnceCell;
use crate::config::KernelConfig;

static GLOBAL_CONFIG: OnceCell<Arc<KernelConfig>> = OnceCell::new();

pub fn set_global_config(config: Arc<KernelConfig>) {
    let _ = GLOBAL_CONFIG.set(config);
}

pub fn get_global_config() -> Arc<KernelConfig> {
    GLOBAL_CONFIG.get().expect("Global config not set").clone()
}
