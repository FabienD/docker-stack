use dotenv::dotenv;
use eyre::Result;
use std::env;

pub mod cli;
pub mod command;
pub mod parser;

use cli::Cli;

use command::docker::{Container, Docker};

use parser::config::{CliConfig, DctlConfig};

fn load_config_path() -> Result<String> {
    let config_file_path = env::var("DCTL_CONFIG_FILE_PATH")
        .unwrap_or_else(|_| String::from("~/.config/dctl/config.toml"));
    Ok(config_file_path)
}

fn main() {
    // Load .env file
    dotenv().ok();
    // Get the custom config file path from env
    let config_file_path = load_config_path().unwrap();

    // Load config file
    let mut config: DctlConfig = match CliConfig::load(config_file_path) {
        Ok(config) => config,
        Err(err) => {
            println!("Load config error: {}", err);
            std::process::exit(1);
        }
    };
    // Load container bin path
    let docker: Docker = Container::init(config.get_container_bin_path().unwrap());

    // Execute cli command
    match Cli::run(&docker, &mut config) {
        Ok(_) => {}
        Err(err) => {
            println!("Command exection error: {}", err);
            std::process::exit(1);
        }
    }
}
