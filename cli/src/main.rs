use clap::{Parser, Subcommand};
use dotenv::dotenv;
use eyre::{eyre, Result};
use std::env;

pub mod command;
pub mod parser;

use command::docker::{Container, Docker};
use command::system::System;
use parser::config::{CliConfig, DctlConfig};

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

#[derive(Subcommand, Debug)]
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
    config: &mut dyn CliConfig,
    container: &dyn Container,
    command: &Commands,
    name: Option<String>,
    service: Option<String>,
    subcommand: Option<String>,
) -> Result<()> {
    match name {
        Some(name) => match config.get_compose_item_by_alias(name.to_string()) {
            Some(item) => match command {
                Commands::Start { .. } => container.start(&item),
                Commands::Stop { .. } => container.stop(&item),
                Commands::Down { .. } => container.down(&item),
                Commands::Restart { .. } => container.restart(&item),
                Commands::Build { .. } => container.build(&item, service),
                Commands::Logs { .. } => container.logs(&item, service),
                Commands::Ps { .. } => container.ps(&item),
                Commands::Exec { .. } => container.exec(&item, service, subcommand),
                Commands::Cd { .. } => {
                    let system = System::init();
                    println!("{}", system.cd(&item).unwrap());
                    Ok(())
                }
                _ => Err(eyre!("Should not happen, unknown command")),
            },
            None => Err(eyre!("Compose item {name} not found")),
        },
        None => match command {
            Commands::List => container.list(config),
            _ => Err(eyre!("Should not happen, no item, but not list command")),
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
    let mut config: DctlConfig = match CliConfig::load(config_file_path) {
        Ok(config) => config,
        Err(err) => {
            println!("Load config error: {}", err);
            std::process::exit(1);
        }
    };

    let cli = Cli::parse();
    let docker: Docker = Container::init(config.get_container_bin_path().unwrap());

    let cmd = match &cli.command {
        Commands::List => {
            execute_compose_command(&mut config, &docker, &cli.command, None, None, None)
        }
        Commands::Cd { name } => execute_compose_command(
            &mut config,
            &docker,
            &cli.command,
            Some(name.to_string()),
            None,
            None,
        ),
        Commands::Start { name } => execute_compose_command(
            &mut config,
            &docker,
            &cli.command,
            Some(name.to_string()),
            None,
            None,
        ),
        Commands::Stop { name } => execute_compose_command(
            &mut config,
            &docker,
            &cli.command,
            Some(name.to_string()),
            None,
            None,
        ),
        Commands::Down { name } => execute_compose_command(
            &mut config,
            &docker,
            &cli.command,
            Some(name.to_string()),
            None,
            None,
        ),
        Commands::Restart { name } => execute_compose_command(
            &mut config,
            &docker,
            &cli.command,
            Some(name.to_string()),
            None,
            None,
        ),
        Commands::Ps { name } => execute_compose_command(
            &mut config,
            &docker,
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
            &docker,
            &cli.command,
            Some(name.to_string()),
            Some(service.to_string()),
            Some(subcommand.to_string()),
        ),
        Commands::Build { name, service } => execute_compose_command(
            &mut config,
            &docker,
            &cli.command,
            Some(name.to_owned()),
            service.to_owned(),
            None,
        ),
        Commands::Logs { name, service } => execute_compose_command(
            &mut config,
            &docker,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::config::ComposeItem;
    use crate::{command::docker::MockDocker, parser::config::MockDctlConfig};

    fn get_mocked_config() -> MockDctlConfig {
        // Mock config
        let mut mock_config = MockDctlConfig::default();
        mock_config
            .expect_get_container_bin_path()
            .returning(|| Ok(String::from("path/to/docker")));

        mock_config
            .expect_get_compose_item_by_alias()
            .returning(|_| {
                Some(ComposeItem {
                    alias: String::from("test"),
                    use_project_name: None,
                    status: None,
                    description: None,
                    compose_files: ["/home/user/test".to_string()].to_vec(),
                    enviroment_file: None,
                })
            });

        mock_config.expect_get_all_compose_items().returning(|| {
            vec![ComposeItem {
                alias: String::from("test"),
                use_project_name: None,
                status: None,
                description: None,
                compose_files: ["/home/user/test".to_string()].to_vec(),
                enviroment_file: None,
            }]
        });

        mock_config
    }

    #[test]
    fn get_test_execute_start_cmd() {
        // Mocked config
        let mut mock_config = get_mocked_config();
        // Mock docker
        let mut mock_docker = MockDocker::default();
        mock_docker.expect_start().returning(|_| Ok(()));

        let command = Commands::Start {
            name: String::from("test"),
        };

        let result = execute_compose_command(
            &mut mock_config,
            &mock_docker,
            &command,
            Some(String::from("test")),
            None,
            None,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn get_test_execute_stop_cmd() {
        // Mocked config
        let mut mock_config = get_mocked_config();
        // Mock docker
        let mut mock_docker = MockDocker::default();
        mock_docker.expect_stop().returning(|_| Ok(()));

        let command = Commands::Stop {
            name: String::from("test"),
        };

        let result = execute_compose_command(
            &mut mock_config,
            &mock_docker,
            &command,
            Some(String::from("test")),
            None,
            None,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn get_test_execute_down_cmd() {
        // Mocked config
        let mut mock_config = get_mocked_config();
        // Mock docker
        let mut mock_docker = MockDocker::default();
        mock_docker.expect_down().returning(|_| Ok(()));

        let command = Commands::Down {
            name: String::from("test"),
        };

        let result = execute_compose_command(
            &mut mock_config,
            &mock_docker,
            &command,
            Some(String::from("test")),
            None,
            None,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn get_test_execute_restart_cmd() {
        // Mocked config
        let mut mock_config = get_mocked_config();
        // Mock docker
        let mut mock_docker = MockDocker::default();
        mock_docker.expect_restart().returning(|_| Ok(()));

        let command = Commands::Restart {
            name: String::from("test"),
        };

        let result = execute_compose_command(
            &mut mock_config,
            &mock_docker,
            &command,
            Some(String::from("test")),
            None,
            None,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn get_test_execute_ps_cmd() {
        // Mocked config
        let mut mock_config = get_mocked_config();
        // Mock docker
        let mut mock_docker = MockDocker::default();
        mock_docker.expect_ps().returning(|_| Ok(()));

        let command = Commands::Ps {
            name: String::from("test"),
        };

        let result = execute_compose_command(
            &mut mock_config,
            &mock_docker,
            &command,
            Some(String::from("test")),
            None,
            None,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn get_test_execute_logs_cmd() {
        // Mocked config
        let mut mock_config = get_mocked_config();
        // Mock docker
        let mut mock_docker = MockDocker::default();
        mock_docker.expect_logs().returning(|_, _| Ok(()));

        let command = Commands::Logs {
            name: String::from("test"),
            service: Some(String::from("service")),
        };

        let result = execute_compose_command(
            &mut mock_config,
            &mock_docker,
            &command,
            Some(String::from("test")),
            Some(String::from("service")),
            None,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn get_test_execute_build_cmd() {
        // Mocked config
        let mut mock_config = get_mocked_config();
        // Mock docker
        let mut mock_docker = MockDocker::default();
        mock_docker.expect_build().returning(|_, _| Ok(()));

        let command = Commands::Build {
            name: String::from("test"),
            service: Some(String::from("service")),
        };

        let result = execute_compose_command(
            &mut mock_config,
            &mock_docker,
            &command,
            Some(String::from("test")),
            Some(String::from("service")),
            None,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn get_test_execute_exec_cmd() {
        // Mocked config
        let mut mock_config = get_mocked_config();
        // Mock docker
        let mut mock_docker = MockDocker::default();
        mock_docker.expect_exec().returning(|_, _, _| Ok(()));

        let command = Commands::Exec {
            name: String::from("test"),
            service: String::from("service"),
            subcommand: String::from("subcommand"),
        };

        let result = execute_compose_command(
            &mut mock_config,
            &mock_docker,
            &command,
            Some(String::from("test")),
            Some(String::from("service")),
            Some(String::from("subcommand")),
        );

        assert!(result.is_ok());
    }

    #[test]
    fn get_test_execute_list_cmd() {
        // Mocked config
        let mut mock_config = get_mocked_config();
        // Mock docker
        let mut mock_docker = MockDocker::default();
        mock_docker.expect_list().returning(|_| Ok(()));

        let command = Commands::List {};

        let result = execute_compose_command(
            &mut mock_config,
            &mock_docker,
            &command,
            None,
           None,
            None,
        );

        assert!(result.is_ok());
    }
}
