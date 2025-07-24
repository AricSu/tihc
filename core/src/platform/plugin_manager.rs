pub struct PluginManager;

impl PluginManager {
    pub fn new() -> Self {
        PluginManager
    }

    pub fn load_plugins(&self) {
        println!("Loading plugins...");
    }
}
