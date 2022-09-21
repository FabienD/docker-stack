use clap::{Parser, Subcommand};
use dotenv::dotenv;
use eyre::{eyre, Result};
use std::env;
use tabled::{Margin, Style, Table};

#[derive(Parser)]
#[clap(
    author,
    version,
    about = "A docker-compose missing feature.",
    long_about = "Register docker-compose files, then, play with them whereever you are in the terminal."
)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

pub mod docker;
pub mod parser;

use docker::Docker;

#[derive(Subcommand)]
enum Commands {
    /// List all the available docker-compose files in config file
    List,
    /// Start a docker-compose file
    Start {
        /// The name of the docker-compose file alias
        #[clap(value_parser)]
        name: String,
    },
    /// Stop a docker-compose file
    Stop {
        /// The name of the docker-compose file alias
        #[clap(value_parser)]
        name: String,
    },
    /// Down a docker-compose file
    Down {
        /// The name of the docker-compose file alias
        #[clap(value_parser)]
        name: String,
    },
    /// Restart a docker-compose file
    Restart {
        /// The name of the docker-compose file alias
        #[clap(value_parser)]
        name: String,
    },
    /// Build all or one service from a docker-compose file
    Build {
        /// The name of the docker-compose file alias
        #[clap(value_parser)]
        name: String,
        /// The name of the service to build (optional)
        #[clap(value_parser)]
        service: Option<String>,
    },
}

fn execute_compose_command(
    config: parser::DctlConfig,
    command: &Commands,
    name: String,
    service: Option<String>,
) -> Result<()> {
    let docker = Docker::init(config.main.docker_bin.clone());

    match config.get_compose_item_by_alias(name.to_string()) {
        Some(item) => match command {
            Commands::Start { .. } => docker.start(&item),
            Commands::Stop { .. } => docker.stop(&item),
            Commands::Down { .. } => docker.down(&item),
            Commands::Restart { .. } => docker.restart(&item),
            Commands::Build { .. } => docker.build(&item, service),
            Commands::List => Err(eyre!("List command should not be here")),
        },
        None => Err(eyre!("Compose item {name} not found")),
    }
}

fn main() {
    // Load .env file
    dotenv().ok();
    // Get the custom config file path from env
    let config_file_path = env::var("DCTL_CONFIG_FILE_PATH")
        .unwrap_or_else(|_| String::from("~/.config/dctl/config.toml"));
    // Load config file
    let config = match parser::DctlConfig::load(config_file_path) {
        Ok(config) => config,
        Err(err) => {
            println!("Load config error: {}", err);
            std::process::exit(1);
        }
    };

    let cli = Cli::parse();

    let cmd = match &cli.command {
        Commands::List => {
            let items = config.get_all_compose_items();
            println!(
                "{}",
                Table::new(items)
                    .with(Style::modern())
                    .with(Margin::new(0, 0, 1, 1))
            );
            Ok(())
        }
        Commands::Start { name } => {
            execute_compose_command(config, &cli.command, name.to_string(), None)
        }
        Commands::Stop { name } => {
            execute_compose_command(config, &cli.command, name.to_string(), None)
        }
        Commands::Down { name } => {
            execute_compose_command(config, &cli.command, name.to_string(), None)
        }
        Commands::Restart { name } => {
            execute_compose_command(config, &cli.command, name.to_string(), None)
        }
        Commands::Build { name, service } => execute_compose_command(
            config,
            &cli.command,
            name.to_string(),
            service.to_owned(),
        ),
    };

    match cmd {
        Ok(_) => {}
        Err(err) => {
            println!("Command error: {}", err);
            std::process::exit(1);
        }
    }
}
