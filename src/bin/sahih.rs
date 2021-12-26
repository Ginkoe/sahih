extern crate clap;
extern crate pretty_env_logger;
extern crate termcolor;

use clap::{arg, App, AppSettings};
use log::{debug, error};
use sahih::{Sahih, SahihOptions};

fn main() {
    pretty_env_logger::init();
    let generate_command = App::new("generate")
        .about("Generates hooks from OpenAPI schema")
        .arg(arg!([SCHEMA]));

    let cli = App::new("sahih")
        .global_setting(AppSettings::AllArgsOverrideSelf)
        .global_setting(AppSettings::DeriveDisplayOrder)
        .global_setting(AppSettings::AllowNegativeNumbers)
        .about("|TODO: ??|")
        .subcommand(generate_command)
        .get_matches();

    debug!("{:?}", cli.subcommand());

    match cli.subcommand() {
        Some(("generate", sub_matches)) => {
            if let Some(schema_arg) = sub_matches.value_of("SCHEMA") {
                let opts = SahihOptions {
                    schema_path: schema_arg,
                };
                let sahih = Sahih::new();
                match sahih.generate(opts) {
                    Ok(_) => {
                        debug!("Succes");
                    }
                    Err(e) => error!("Could not generate validation. \n {:?}", e),
                }
            } else {
                error!("No schema path provided")
            }
        }
        _ => unreachable!(),
    }
}
