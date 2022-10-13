use clap::{Parser, Subcommand};
use dotenv::dotenv;
use eyre::{eyre, Result};
use std::env;

pub mod command;
pub mod parser;

use command::docker::Docker;
use command::system::System;
use parser::config::DctlConfig;

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

#[derive(Subcommand)]
enum Commands {
    /// List all the available docker-compose files in the config
    List,
    /// Print the directory of the docker-compose file.
    /// Uesful to use with shell cd command.
    Cd {
        /// The name of the docker-compose file alias
        #[clap(value_parser)]
        name: String,
    },
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
    /// Build all or one service of a docker-compose file
    Build {
        /// The name of the docker-compose file alias
        #[clap(value_parser)]
        name: String,
        /// The name of the service to build (optional)
        #[clap(value_parser)]
        service: Option<String>,
    },
    /// Display logs of all or one service of a docker-compose file
    Logs {
        /// The name of the docker-compose file alias
        #[clap(value_parser)]
        name: String,
        /// The name of the service to build (optional)
        #[clap(value_parser)]
        service: Option<String>,
    },
    /// Show running containers of a docker-compose file
    Ps {
        /// The name of the docker-compose file alias
        #[clap(value_parser)]
        name: String,
    },
    /// Execute a command in a container of a docker-compose file
    Exec {
        /// The name of the docker-compose file alias
        #[clap(value_parser)]
        name: String,
        /// The name of the service where to exexute the command
        #[clap(value_parser)]
        service: String,
        /// The command to execute
        #[clap(value_parser)]
        subcommand: String,
    },
}

fn execute_compose_command(
    config: &mut DctlConfig,
    command: &Commands,
    name: Option<String>,
    service: Option<String>,
    subcommand: Option<String>,
) -> Result<()> {
    let docker = Docker::init(config.main.docker_bin.clone());
    let system = System::init();

    match name {
        Some(name) => match config.get_compose_item_by_alias(name.to_string()) {
            Some(item) => match command {
                Commands::Start { .. } => docker.start(&item),
                Commands::Stop { .. } => docker.stop(&item),
                Commands::Down { .. } => docker.down(&item),
                Commands::Restart { .. } => docker.restart(&item),
                Commands::Build { .. } => docker.build(&item, service),
                Commands::Logs { .. } => docker.logs(&item, service),
                Commands::Ps { .. } => docker.ps(&item),
                Commands::Exec { .. } => docker.exec(&item, service, subcommand),
                Commands::Cd { .. } => {
                    println!("{}", system.cd(&item).unwrap());
                    Ok(())
                }
                _ => Err(eyre!("Should not happen")),
            },
            None => Err(eyre!("Compose item {name} not found")),
        },
        None => match command {
            Commands::List => docker.list(config),
            _ => Err(eyre!("Should not happen")),
        },
    }
}

fn main() {
    // Load .env file
    dotenv().ok();
    // Get the custom config file path from env
    let config_file_path = env::var("DCTL_CONFIG_FILE_PATH")
        .unwrap_or_else(|_| String::from("~/.config/dctl/config.toml"));
    // Load config file
    let mut config = match DctlConfig::load(config_file_path) {
        Ok(config) => config,
        Err(err) => {
            println!("Load config error: {}", err);
            std::process::exit(1);
        }
    };

    let cli = Cli::parse();

    let cmd = match &cli.command {
        Commands::List => execute_compose_command(&mut config, &cli.command, None, None, None),
        Commands::Cd { name } => execute_compose_command(
            &mut config,
            &cli.command,
            Some(name.to_string()),
            None,
            None,
        ),
        Commands::Start { name } => execute_compose_command(
            &mut config,
            &cli.command,
            Some(name.to_string()),
            None,
            None,
        ),
        Commands::Stop { name } => execute_compose_command(
            &mut config,
            &cli.command,
            Some(name.to_string()),
            None,
            None,
        ),
        Commands::Down { name } => execute_compose_command(
            &mut config,
            &cli.command,
            Some(name.to_string()),
            None,
            None,
        ),
        Commands::Restart { name } => execute_compose_command(
            &mut config,
            &cli.command,
            Some(name.to_string()),
            None,
            None,
        ),
        Commands::Ps { name } => execute_compose_command(
            &mut config,
            &cli.command,
            Some(name.to_string()),
            None,
            None,
        ),
        Commands::Exec {
            name,
            service,
            subcommand,
        } => execute_compose_command(
            &mut config,
            &cli.command,
            Some(name.to_string()),
            Some(service.to_string()),
            Some(subcommand.to_string()),
        ),
        Commands::Build { name, service } => execute_compose_command(
            &mut config,
            &cli.command,
            Some(name.to_owned()),
            service.to_owned(),
            None,
        ),
        Commands::Logs { name, service } => execute_compose_command(
            &mut config,
            &cli.command,
            Some(name.to_string()),
            service.to_owned(),
            None,
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
