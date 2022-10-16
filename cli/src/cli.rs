use crate::{command::docker::Container, command::system::System, parser::config::CliConfig};
use clap::{Command, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Generator, Shell};
use eyre::{eyre, Result};
use std::io;

#[derive(Parser)]
#[clap(
    author,
    version,
    about = "A docker-compose missing feature.",
    long_about = "Register docker-compose files, then, play with them whereever you are in the terminal."
)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Geneate shell completion (bash, fish, zsh, powershell, elvish)
    Completion {
        #[clap(value_enum)]
        generator: Option<Shell>,
    },
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

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

impl Cli {
    pub fn run(container: &dyn Container, config: &mut dyn CliConfig) -> Result<()> {
        let cli = Cli::parse();

        match &cli.command {
            // Generate shell completions
            Commands::Completion { generator } => {
                let mut cmd = Cli::command();
                eprintln!("Generating completion file for {:?}...", &generator);
                print_completions(generator.unwrap(), &mut cmd);
                std::process::exit(1);
            },
            Commands::List => {
                execute_compose_command(config, container, &cli.command, None, None, None)
            }
            Commands::Cd { name } => execute_compose_command(
                config,
                container,
                &cli.command,
                Some(name.to_string()),
                None,
                None,
            ),
            Commands::Start { name } => execute_compose_command(
                config,
                container,
                &cli.command,
                Some(name.to_string()),
                None,
                None,
            ),
            Commands::Stop { name } => execute_compose_command(
                config,
                container,
                &cli.command,
                Some(name.to_string()),
                None,
                None,
            ),
            Commands::Down { name } => execute_compose_command(
                config,
                container,
                &cli.command,
                Some(name.to_string()),
                None,
                None,
            ),
            Commands::Restart { name } => execute_compose_command(
                config,
                container,
                &cli.command,
                Some(name.to_string()),
                None,
                None,
            ),
            Commands::Ps { name } => execute_compose_command(
                config,
                container,
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
                config,
                container,
                &cli.command,
                Some(name.to_string()),
                Some(service.to_string()),
                Some(subcommand.to_string()),
            ),
            Commands::Build { name, service } => execute_compose_command(
                config,
                container,
                &cli.command,
                Some(name.to_owned()),
                service.to_owned(),
                None,
            ),
            Commands::Logs { name, service } => execute_compose_command(
                config,
                container,
                &cli.command,
                Some(name.to_string()),
                service.to_owned(),
                None,
            ),
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
    fn verify_cli() {
        Cli::command().debug_assert();
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

        let result =
            execute_compose_command(&mut mock_config, &mock_docker, &command, None, None, None);

        assert!(result.is_ok());
    }

    #[test]
    fn get_test_execute_list_with_name_cmd() {
        // Mocked config
        let mut mock_config = get_mocked_config();
        // Mock docker
        let mut mock_docker = MockDocker::default();
        mock_docker.expect_list().returning(|_| Ok(()));

        let command = Commands::List {};

        let result =
            execute_compose_command(&mut mock_config, &mock_docker, &command, Some(String::from("None")), None, None);

        assert!(result.is_err());
    }

    #[test]
    fn get_test_execute_any_cmd_without_name_cmd() {
        // Mocked config
        let mut mock_config = get_mocked_config();
        // Mock docker
        let mut mock_docker = MockDocker::default();
        mock_docker.expect_start().returning(|_| Ok(()));

        let command = Commands::Start {
            name: String::from("test"),
        };

        let result =
            execute_compose_command(&mut mock_config, &mock_docker, &command, None, None, None);

        assert!(result.is_err());
    }
}
