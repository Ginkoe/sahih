#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::io;

use config::ConfigManager;
use log::{debug, info};

use crate::{
    codegen::{InterfaceGenerator, ValidationGenerator},
    reader::consume_schemas,
};

pub mod codegen;
pub mod config;
pub mod reader;

pub struct Sahih {
    config: ConfigManager,
}

impl Sahih {
    pub fn new(config: ConfigManager) -> Self {
        Sahih { config }
    }

    pub fn generate(self) -> io::Result<()> {
        debug!("Running with options: {:#?}", self.config);

        for (project_name, project_config) in self.config.projects {
            info!("Starting generation of project {}", project_name);
            debug!("W/ config :\n{:#?}", project_config);
            let schemas = consume_schemas(&project_config.input.target);

            for model in &schemas {
                let generator = InterfaceGenerator::from(model);
                let serialized = generator.build();
                debug!("Serialized:\n {}", serialized);
    
                let generator = ValidationGenerator::from(model);
                info!("Serialized:\n {:#?}", generator);
            }
        }

        

        

        Ok(())
    }
}
