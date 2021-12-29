#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::{
    fs::OpenOptions,
    io::{self, Write},
    path::PathBuf,
};

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

            debug!("{}", project_config.output.target);
            std::fs::create_dir_all(&project_config.output.target).unwrap();

            let mut model_file_path = PathBuf::from(&project_config.output.target);
            model_file_path.push("models.ts");

            let mut output_file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(model_file_path)
                .unwrap();

            // TODO: Header file
            output_file.write_all(
                br#"// Generated with Sahih
import yup from "yup";

"#,
            )?;
            let schemas = consume_schemas(&project_config.input.target);

            for model in &schemas {
                let generator = InterfaceGenerator::from(model);
                let serialized = generator.build();
                debug!("Serialized:\n {}", serialized);
                output_file.write_all(format!("{}\n", serialized).as_bytes())?;

                let generator = ValidationGenerator::from(model);
                output_file.write_all(format!("{}\n\n\n", generator.build()).as_bytes())?;
                info!("Serialized:\n {:#?}", generator);
            }
        }

        Ok(())
    }
}
