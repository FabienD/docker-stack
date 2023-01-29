use crate::parser::config::CliConfig;
use crate::utils::docker::{CommandType, Container};
use clap::Command;
use eyre::{eyre, Result};

use crate::command::build::compose_build;
use crate::command::cd::{cd_project, exec_cd_project};
use crate::command::completion::{exec_shell_completion, shell_completion};
use crate::command::down::compose_down;
use crate::command::exec::compose_exec;
use crate::command::infos::{exec_projects_infos, projects_infos};
use crate::command::logs::compose_logs;
use crate::command::ls::compose_ls;
use crate::command::ps::compose_ps;
use crate::command::restart::compose_restart;
use crate::command::run::compose_run;
use crate::command::start::compose_start;
use crate::command::stop::compose_stop;
use crate::command::top::compose_top;
use crate::command::up::compose_up;

fn cli() -> Command {
    Command::new("dctl")
        .about("A docker-compose missing feature.")
        .long_about(
            "Register docker-compose files, then, play with them whereever you are in the terminal",
        )
        .version(version!())
        .author("Fabien D. <fabien@myprod.net>")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(compose_build())
        .subcommand(compose_down())
        .subcommand(compose_exec())
        .subcommand(compose_ps())
        .subcommand(compose_logs())
        .subcommand(compose_ls())
        .subcommand(compose_restart())
        .subcommand(compose_run())
        .subcommand(compose_start())
        .subcommand(compose_stop())
        .subcommand(compose_top())
        .subcommand(compose_up())
        .subcommand(shell_completion())
        .subcommand(cd_project())
        .subcommand(projects_infos())
}

pub fn run(container: &dyn Container, config: &mut dyn CliConfig) -> Result<()> {
    // Get the command name and args
    let matches = cli().get_matches();
    let (command_name, args) = matches.subcommand().unwrap();
    // Get the compose item for the project
    let compose_item = match args.get_one::<String>("PROJECT") {
        Some(name) => match config.get_compose_item_by_alias(name.to_string()) {
            Some(item) => item,
            None => return Err(eyre!("No project found with alias: {}", name)),
        },
        None => return Err(eyre!("Not yet implemented")), // Should never happen
    };
    // Run the command
    match command_name {
        "completion" => exec_shell_completion(&mut cli(), args)?,
        "list" => exec_projects_infos(config)?,
        "cd" => exec_cd_project(&compose_item)?,
        "build" => container.compose(CommandType::Build, &compose_item, args)?,
        "down" => container.compose(CommandType::Down, &compose_item, args)?,
        "exec" => container.compose(CommandType::Exec, &compose_item, args)?,
        "logs" => container.compose(CommandType::Logs, &compose_item, args)?,
        "ps" => container.compose(CommandType::Ps, &compose_item, args)?,
        "restart" => container.compose(CommandType::Restart, &compose_item, args)?,
        "run" => container.compose(CommandType::Run, &compose_item, args)?,
        "start" => container.compose(CommandType::Start, &compose_item, args)?,
        "stop" => container.compose(CommandType::Stop, &compose_item, args)?,
        "top" => container.compose(CommandType::Top, &compose_item, args)?,
        "up" => container.compose(CommandType::Up, &compose_item, args)?,
        _ => return Err(eyre!("Not yet implemented")),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::config::ComposeItem;
    use crate::{parser::config::MockDctlConfig, utils::docker::MockDocker};

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
        cli::command().debug_assert();
    }

    #[test]
    fn get_test_execute_up_cmd() {
        // Mocked config
        let mut mock_config = get_mocked_config();
        // Mock docker
        let mut mock_docker = MockDocker::default();
        mock_docker.expect_up().returning(|_| Ok(()));

        let command = Commands::Up {
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
    fn get_test_execute_run_cmd() {
        // Mocked config
        let mut mock_config = get_mocked_config();
        // Mock docker
        let mut mock_docker = MockDocker::default();
        mock_docker.expect_exec().returning(|_, _, _| Ok(()));

        let command = Commands::Run {
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

        let result = execute_compose_command(
            &mut mock_config,
            &mock_docker,
            &command,
            Some(String::from("None")),
            None,
            None,
        );

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
