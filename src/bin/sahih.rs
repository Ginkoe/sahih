extern crate clap;
extern crate pretty_env_logger;
extern crate termcolor;

use clap::{App, AppSettings, Arg};
use log::{debug, error};
use sahih::{config::ConfigManager, printer::Printer, Sahih};

fn main() {
    pretty_env_logger::init();
    let generate_command = App::new("generate").about("Generates hooks from OpenAPI schema");

    let cli = App::new("sahih")
        .global_setting(AppSettings::AllArgsOverrideSelf)
        .global_setting(AppSettings::DeriveDisplayOrder)
        .global_setting(AppSettings::AllowNegativeNumbers)
        .about("|TODO: ??|")
        .subcommand(generate_command)
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Path to config file")
                .takes_value(true),
        )
        .get_matches();

    debug!("{:?}", cli.subcommand());

    let config_path = cli.value_of("config").unwrap_or("sahih.config.json");
    let config_manager = ConfigManager::from(config_path);
    let std_output = termcolor::StandardStream::stdout(termcolor::ColorChoice::Always);
    let sahih = Sahih::new(config_manager, Printer::new(std_output));

    match cli.subcommand() {
        Some(("generate", _)) => match sahih.generate() {
            Ok(_) => {
                debug!("Succes");
            }
            Err(e) => error!("Could not generate validation. \n {:?}", e),
        },
        _ => unreachable!(),
    }
}
