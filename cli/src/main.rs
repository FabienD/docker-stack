use dotenv::dotenv;
use eyre::Result;
use std::env;

#[macro_use]
extern crate version;

pub mod cli;
pub mod command;
pub mod parser;
pub mod utils;

use parser::config::{CliConfig, DctlConfig};
use utils::docker::{Container, Docker};

fn load_config_path() -> Result<String> {
    let config_file_path = env::var("DCTL_CONFIG_FILE_PATH")
        .unwrap_or_else(|_| String::from("~/.config/dctl/config.toml"));
    Ok(config_file_path)
}

#[tokio::main]
async fn main() {
    // Load .env file
    dotenv().ok();

    // Get the custom config file path from env
    let config_file_path = load_config_path().unwrap();

    // Get config file
    let mut config: DctlConfig = match CliConfig::load(config_file_path) {
        Ok(config) => config,
        Err(err) => {
            println!("Load config error: {}", err);
            std::process::exit(1);
        }
    };

    // Get container manager
    let docker: Docker = Container::init(config.get_container_bin_path().unwrap());

    // Execute cli command
    if let Err(err) = cli::run(&docker, &mut config).await {
        println!("Command exection error: {}", err);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use std::env::{remove_var, set_var};

    use super::*;

    #[test]
    fn get_default_load_config_path() {
        remove_var("DCTL_CONFIG_FILE_PATH");
        let config_file_path = load_config_path().unwrap();
        assert_eq!(config_file_path, "~/.config/dctl/config.toml");
    }

    #[test]
    fn get_env_load_config_path() {
        set_var("DCTL_CONFIG_FILE_PATH", "../tests/config.toml");
        let config_file_path = load_config_path().unwrap();
        assert_eq!(config_file_path, "../tests/config.toml");
    }
}
