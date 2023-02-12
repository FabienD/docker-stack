use clap::Command;
use eyre::{eyre, Result};
use std::ffi::OsStr;
use std::process::exit;

use crate::parser::config::{CliConfig, DefaultCommandArgs, ComposeItem};
use crate::utils::docker::{CommandType, Container};

use crate::command::build::compose_build;
use crate::command::cd::{cd_project, exec_cd_project};
use crate::command::completion::{exec_shell_completion, shell_completion};
use crate::command::create::compose_create;
use crate::command::down::compose_down;
use crate::command::events::compose_events;
use crate::command::exec::compose_exec;
use crate::command::images::compose_images;
use crate::command::infos::{exec_projects_infos, projects_infos};
use crate::command::kill::compose_kill;
use crate::command::logs::compose_logs;
use crate::command::ls::compose_ls;
use crate::command::pause::compose_pause;
use crate::command::ps::compose_ps;
use crate::command::pull::compose_pull;
use crate::command::push::compose_push;
use crate::command::restart::compose_restart;
use crate::command::rm::compose_rm;
use crate::command::run::compose_run;
use crate::command::start::compose_start;
use crate::command::stop::compose_stop;
use crate::command::top::compose_top;
use crate::command::unpause::compose_unpause;
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
        .subcommand(compose_create())
        .subcommand(compose_down())
        .subcommand(compose_exec())
        .subcommand(compose_events())
        .subcommand(compose_images())
        .subcommand(compose_kill())
        .subcommand(compose_logs())
        .subcommand(compose_ls())
        .subcommand(compose_ps())
        .subcommand(compose_pause())
        .subcommand(compose_pull())
        .subcommand(compose_push())
        .subcommand(compose_restart())
        .subcommand(compose_rm())
        .subcommand(compose_run())
        .subcommand(compose_start())
        .subcommand(compose_stop())
        .subcommand(compose_top())
        .subcommand(compose_unpause())
        .subcommand(compose_up())
        .subcommand(shell_completion())
        .subcommand(cd_project())
        .subcommand(projects_infos())
}

pub async fn run(
    container: &dyn Container, 
    config: &mut dyn CliConfig,
) -> Result<()> {
    // Get the command name and args
    let matches = cli().get_matches();
    let (command_name, args) = matches.subcommand().unwrap();
    let default_command_args = config.get_default_command_args(command_name);
    
    match command_name {
        "infos" => exec_projects_infos(config, container).await?,
        "completion" => exec_shell_completion(&mut cli(), args)?,
        _ => {}
    }

    // For next commands, we need a project
    if let Err(..) = args.try_contains_id("PROJECT") {
        exit(1)
    }

    // Get the compose item for the project
    let compose_item = match args.get_one::<String>("PROJECT") {
        Some(name) => match config.get_compose_item_by_alias(name.to_string()) {
            Some(item) => item,
            None => return Err(eyre!("No project found with alias: {}", name)),
        },
        None => exit(1),
    };

    if command_name == "cd" {
        exec_cd_project(&compose_item)?;
        exit(0);
    }

    // Run Docker compose command
    let mut default_arg: Vec<&OsStr> = vec![];
    // Configuration args
    default_arg.append(&mut ComposeItem::to_args(&compose_item));
    // Global command args
    let command_args = match default_command_args {
        Some(command_args) => command_args,
        None => DefaultCommandArgs::default(&command_name),
    };
    let default_command_arg = DefaultCommandArgs::to_args(&command_args);

    match command_name {
        "build" => {
            container
                .compose(CommandType::Build, &default_arg, &default_command_arg, args, None)
                .await?
        }
        "create" => {
            container
                .compose(CommandType::Create, &default_arg, &default_command_arg, args, None)
                .await?
        }
        "down" => {
            container
                .compose(CommandType::Down, &default_arg, &default_command_arg,args, None)
                .await?
        }
        "exec" => {
            container
                .compose(CommandType::Exec, &default_arg, &default_command_arg,args, None)
                .await?
        }
        "events" => {
            container
                .compose(CommandType::Events, &default_arg, &default_command_arg,args, None)
                .await?
        }
        "images" => {
            container
                .compose(CommandType::Images, &default_arg, &default_command_arg,args, None)
                .await?
        }
        "kill" => {
            container
                .compose(CommandType::Kill, &default_arg, &default_command_arg,args, None)
                .await?
        }
        "logs" => {
            container
                .compose(CommandType::Logs, &default_arg, &default_command_arg,args, None)
                .await?
        }
        "ls" => {
            container
                .compose(CommandType::Ls, &default_arg, &default_command_arg,args, None)
                .await?
        }
        "pause" => {
            container
                .compose(CommandType::Pause, &default_arg, &default_command_arg,args, None)
                .await?
        }
        "pull" => {
            container
                .compose(CommandType::Pull, &default_arg, &default_command_arg,args, None)
                .await?
        }
        "push" => {
            container
                .compose(CommandType::Push, &default_arg, &default_command_arg,args, None)
                .await?
        }
        "ps" => {
            container
                .compose(CommandType::Ps, &default_arg, &default_command_arg,args, None)
                .await?
        }
        "restart" => {
            container
                .compose(CommandType::Restart, &default_arg, &default_command_arg,args, None)
                .await?
        }
        "rm" => {
            container
                .compose(CommandType::Rm, &default_arg, &default_command_arg,args, None)
                .await?
        }
        "run" => {
            container
                .compose(CommandType::Run, &default_arg, &default_command_arg,args, None)
                .await?
        }
        "start" => {
            container
                .compose(CommandType::Start, &default_arg, &default_command_arg,args, None)
                .await?
        }
        "stop" => {
            container
                .compose(CommandType::Stop, &default_arg, &default_command_arg,args, None)
                .await?
        }
        "top" => {
            container
                .compose(CommandType::Top, &default_arg, &default_command_arg,args, None)
                .await?
        }
        "unpause" => {
            container
                .compose(CommandType::Unpause, &default_arg, &default_command_arg,args, None)
                .await?
        }
        "up" => {
            container
                .compose(CommandType::Up, &default_arg, &default_command_arg,args, None)
                .await?
        }
        _ => return Err(eyre!("Not yet implemented")),
    };

    Ok(())
}

#[cfg(test)]
mod tests {
     use super::*;

    #[test]
    fn it_verifies_the_cli() {
        cli().debug_assert();
    }

    // use crate::parser::config::ComposeItem;
    // use crate::{parser::config::MockDctlConfig, utils::docker::MockDocker};

    // fn get_mocked_config() -> MockDctlConfig {
    //     // Mock config
    //     let mut mock_config = MockDctlConfig::default();
    //     mock_config
    //         .expect_get_container_bin_path()
    //         .returning(|| Ok(String::from("path/to/docker")));

    //     mock_config
    //         .expect_get_compose_item_by_alias()
    //         .returning(|_| {
    //             Some(ComposeItem {
    //                 alias: String::from("test"),
    //                 use_project_name: None,
    //                 status: None,
    //                 description: None,
    //                 compose_files: ["/home/user/test".to_string()].to_vec(),
    //                 enviroment_file: None,
    //             })
    //         });

    //     mock_config.expect_get_all_compose_items().returning(|| {
    //         vec![ComposeItem {
    //             alias: String::from("test"),
    //             use_project_name: None,
    //             status: None,
    //             description: None,
    //             compose_files: ["/home/user/test".to_string()].to_vec(),
    //             enviroment_file: None,
    //         }]
    //     });

    //     mock_config
    // }


    // #[test]
    // fn get_test_execute_up_cmd() {
    //     // Mocked config
    //     let mut mock_config = get_mocked_config();
    //     // Mock docker
    //     let mut mock_docker = MockDocker::default();
    //     mock_docker.expect_up().returning(|_| Ok(()));

    //     let command = Commands::Up {
    //         name: String::from("test"),
    //     };

    //     let result = execute_compose_command(
    //         &mut mock_config,
    //         &mock_docker,
    //         &command,
    //         Some(String::from("test")),
    //         None,
    //         None,
    //     );

    //     assert!(result.is_ok());
    // }

    // #[test]
    // fn get_test_execute_start_cmd() {
    //     // Mocked config
    //     let mut mock_config = get_mocked_config();
    //     // Mock docker
    //     let mut mock_docker = MockDocker::default();
    //     mock_docker.expect_start().returning(|_| Ok(()));

    //     let command = Commands::Start {
    //         name: String::from("test"),
    //     };

    //     let result = execute_compose_command(
    //         &mut mock_config,
    //         &mock_docker,
    //         &command,
    //         Some(String::from("test")),
    //         None,
    //         None,
    //     );

    //     assert!(result.is_ok());
    // }

    // #[test]
    // fn get_test_execute_stop_cmd() {
    //     // Mocked config
    //     let mut mock_config = get_mocked_config();
    //     // Mock docker
    //     let mut mock_docker = MockDocker::default();
    //     mock_docker.expect_stop().returning(|_| Ok(()));

    //     let command = Commands::Stop {
    //         name: String::from("test"),
    //     };

    //     let result = execute_compose_command(
    //         &mut mock_config,
    //         &mock_docker,
    //         &command,
    //         Some(String::from("test")),
    //         None,
    //         None,
    //     );

    //     assert!(result.is_ok());
    // }

    // #[test]
    // fn get_test_execute_down_cmd() {
    //     // Mocked config
    //     let mut mock_config = get_mocked_config();
    //     // Mock docker
    //     let mut mock_docker = MockDocker::default();
    //     mock_docker.expect_down().returning(|_| Ok(()));

    //     let command = Commands::Down {
    //         name: String::from("test"),
    //     };

    //     let result = execute_compose_command(
    //         &mut mock_config,
    //         &mock_docker,
    //         &command,
    //         Some(String::from("test")),
    //         None,
    //         None,
    //     );

    //     assert!(result.is_ok());
    // }

    // #[test]
    // fn get_test_execute_restart_cmd() {
    //     // Mocked config
    //     let mut mock_config = get_mocked_config();
    //     // Mock docker
    //     let mut mock_docker = MockDocker::default();
    //     mock_docker.expect_restart().returning(|_| Ok(()));

    //     let command = Commands::Restart {
    //         name: String::from("test"),
    //     };

    //     let result = execute_compose_command(
    //         &mut mock_config,
    //         &mock_docker,
    //         &command,
    //         Some(String::from("test")),
    //         None,
    //         None,
    //     );

    //     assert!(result.is_ok());
    // }

    // #[test]
    // fn get_test_execute_ps_cmd() {
    //     // Mocked config
    //     let mut mock_config = get_mocked_config();
    //     // Mock docker
    //     let mut mock_docker = MockDocker::default();
    //     mock_docker.expect_ps().returning(|_| Ok(()));

    //     let command = Commands::Ps {
    //         name: String::from("test"),
    //     };

    //     let result = execute_compose_command(
    //         &mut mock_config,
    //         &mock_docker,
    //         &command,
    //         Some(String::from("test")),
    //         None,
    //         None,
    //     );

    //     assert!(result.is_ok());
    // }

    // #[test]
    // fn get_test_execute_logs_cmd() {
    //     // Mocked config
    //     let mut mock_config = get_mocked_config();
    //     // Mock docker
    //     let mut mock_docker = MockDocker::default();
    //     mock_docker.expect_logs().returning(|_, _| Ok(()));

    //     let command = Commands::Logs {
    //         name: String::from("test"),
    //         service: Some(String::from("service")),
    //     };

    //     let result = execute_compose_command(
    //         &mut mock_config,
    //         &mock_docker,
    //         &command,
    //         Some(String::from("test")),
    //         Some(String::from("service")),
    //         None,
    //     );

    //     assert!(result.is_ok());
    // }

    // #[test]
    // fn get_test_execute_build_cmd() {
    //     // Mocked config
    //     let mut mock_config = get_mocked_config();
    //     // Mock docker
    //     let mut mock_docker = MockDocker::default();
    //     mock_docker.expect_build().returning(|_, _| Ok(()));

    //     let command = Commands::Build {
    //         name: String::from("test"),
    //         service: Some(String::from("service")),
    //     };

    //     let result = execute_compose_command(
    //         &mut mock_config,
    //         &mock_docker,
    //         &command,
    //         Some(String::from("test")),
    //         Some(String::from("service")),
    //         None,
    //     );

    //     assert!(result.is_ok());
    // }

    // #[test]
    // fn get_test_execute_exec_cmd() {
    //     // Mocked config
    //     let mut mock_config = get_mocked_config();
    //     // Mock docker
    //     let mut mock_docker = MockDocker::default();
    //     mock_docker.expect_exec().returning(|_, _, _| Ok(()));

    //     let command = Commands::Exec {
    //         name: String::from("test"),
    //         service: String::from("service"),
    //         subcommand: String::from("subcommand"),
    //     };

    //     let result = execute_compose_command(
    //         &mut mock_config,
    //         &mock_docker,
    //         &command,
    //         Some(String::from("test")),
    //         Some(String::from("service")),
    //         Some(String::from("subcommand")),
    //     );

    //     assert!(result.is_ok());
    // }

    // #[test]
    // fn get_test_execute_run_cmd() {
    //     // Mocked config
    //     let mut mock_config = get_mocked_config();
    //     // Mock docker
    //     let mut mock_docker = MockDocker::default();
    //     mock_docker.expect_exec().returning(|_, _, _| Ok(()));

    //     let command = Commands::Run {
    //         name: String::from("test"),
    //         service: String::from("service"),
    //         subcommand: String::from("subcommand"),
    //     };

    //     let result = execute_compose_command(
    //         &mut mock_config,
    //         &mock_docker,
    //         &command,
    //         Some(String::from("test")),
    //         Some(String::from("service")),
    //         Some(String::from("subcommand")),
    //     );

    //     assert!(result.is_ok());
    // }

    // #[test]
    // fn get_test_execute_list_cmd() {
    //     // Mocked config
    //     let mut mock_config = get_mocked_config();
    //     // Mock docker
    //     let mut mock_docker = MockDocker::default();
    //     mock_docker.expect_list().returning(|_| Ok(()));

    //     let command = Commands::List {};

    //     let result =
    //         execute_compose_command(&mut mock_config, &mock_docker, &command, None, None, None);

    //     assert!(result.is_ok());
    // }

    // #[test]
    // fn get_test_execute_list_with_name_cmd() {
    //     // Mocked config
    //     let mut mock_config = get_mocked_config();
    //     // Mock docker
    //     let mut mock_docker = MockDocker::default();
    //     mock_docker.expect_list().returning(|_| Ok(()));

    //     let command = Commands::List {};

    //     let result = execute_compose_command(
    //         &mut mock_config,
    //         &mock_docker,
    //         &command,
    //         Some(String::from("None")),
    //         None,
    //         None,
    //     );

    //     assert!(result.is_err());
    // }

    // #[test]
    // fn get_test_execute_any_cmd_without_name_cmd() {
    //     // Mocked config
    //     let mut mock_config = get_mocked_config();
    //     // Mock docker
    //     let mut mock_docker = MockDocker::default();
    //     mock_docker.expect_start().returning(|_| Ok(()));

    //     let command = Commands::Start {
    //         name: String::from("test"),
    //     };

    //     let result =
    //         execute_compose_command(&mut mock_config, &mock_docker, &command, None, None, None);

    //     assert!(result.is_err());
    // }
}
