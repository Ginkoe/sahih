#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::io;

use log::debug;

use crate::{codegen::InterfaceGenerator, reader::consume_schemas};

pub mod codegen;
pub mod config;
pub mod reader;

#[derive(Default)]
pub struct Sahih {}

#[derive(Debug)]
pub struct SahihOptions<'a> {
    pub schema_path: &'a str,
}

impl Sahih {
    pub fn new() -> Self {
        Sahih {}
    }

    pub fn generate(self, opts: SahihOptions) -> io::Result<()> {
        debug!("Running with options: {:?}", &opts);
        let schemas = consume_schemas(opts.schema_path);

        for model in schemas {
            let generator = InterfaceGenerator::from(&model);
            let serialized = generator.build();
            debug!("Serialized:\n {}", serialized);
        }

        Ok(())
    }
}
