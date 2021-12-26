use std::{collections::HashMap, io};

use log::debug;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SahihConfig {
    pub output: SahihOutputConfig,
    pub input: SahihInputConfig,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SahihOutputConfig {
    pub target: String,
    #[serde(default)]
    pub overwrite: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SahihInputConfig {
    pub target: String,
    #[serde(default)]
    pub from_url: bool,
}

pub trait ConfigManagement {
    fn create_config_file(&self) -> io::Result<()>;
    fn exists_config_file(&self) -> bool;
}

pub struct ConfigManager {
    pub config: HashMap<String, SahihConfig>,
}

impl ConfigManager {
    pub fn from(path: &str) -> Self {
        let raw_config = std::fs::read_to_string(path).expect("Could not read file");

        let deser: HashMap<String, SahihConfig> = serde_json::from_str(&raw_config).unwrap();
        debug!("Deser config to {:#?}", deser);

        Self { config: deser }
    }
}

mod tests {
    #[test]
    fn it_deser_from_example_config() {
        let file_path = "./assets/sahih.json";

        let config_manager = crate::config::ConfigManager::from(file_path);
        let target_output = "./assets/generated/model";
        let overwrite = false;
        let target_input = "./assets/api-schema.json";

        let alphaproject = config_manager.config.get("schemaalpha").unwrap();

        assert_eq!(alphaproject.output.target, target_output);
        assert_eq!(alphaproject.input.target, target_input);
        assert_eq!(alphaproject.output.overwrite, overwrite);
    }
}
