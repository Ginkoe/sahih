use std::io;

pub trait ConfigManagement {
    fn create_config_file(&self) -> io::Result<()>;
    fn exists_config_file(&self) -> bool;
}

pub struct ConfigManager;

impl Default for ConfigManager {
    fn default() -> Self {
        Self {}
    }
}

