use crate::utils::docker::Container;
use crate::parser::config::CliConfig;
use clap::{Command, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Generator, Shell};
use eyre::{eyre, Result};
use std::io;

use crate::command::build::compose_build;
use crate::command::down::compose_down;
use crate::command::exec::compose_exec;
use crate::command::logs::compose_logs;
use crate::command::ps::compose_ps;
use crate::command::restart::compose_restart;
use crate::command::run::compose_run;
use crate::command::start::compose_start;
use crate::command::stop::compose_stop;
use crate::command::top::compose_top;
use crate::command::up::compose_up;
use crate::command::cd::cd_project;
use crate::command::completion::shell_completion;
use crate::command::list::list_projects;


pub fn cli() -> Command {
    Command::new("dctl")
        .about("A docker-compose missing feature.")
        .long_about("Register docker-compose files, then, play with them whereever you are in the terminal")
        .version(version!())
        .author("Fabien D. <fabien@myprod.net>")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(compose_build())
        .subcommand(compose_down())
        .subcommand(compose_exec())
        .subcommand(compose_ps())
        .subcommand(compose_logs())
        .subcommand(compose_restart())
        .subcommand(compose_run())
        .subcommand(compose_start())
        .subcommand(compose_stop())
        .subcommand(compose_top())
        .subcommand(compose_up())
        .subcommand(shell_completion())
        .subcommand(cd_project())
        .subcommand(list_projects())
}



// fn execute_compose_command(
//     config: &mut dyn CliConfig,
//     container: &dyn Container,
//     project: &String,
//     command: &Commands,
// ) -> Result<()> {
//     println!("Project {:?}", project);
//     // println!("Args {:?}", &command.contains(&"project".to_string()));

//     Ok(())
//     /*
//     match command.project {
//         Some(name) => match config.get_compose_item_by_alias(name.to_string()) {
//             Some(item) => match command {
//                 Commands::Up { .. } => container.up(&item),
//                 Commands::Start { .. } => container.start(&item),
//                 Commands::Stop { .. } => container.stop(&item),
//                 Commands::Down { .. } => container.down(&item),
//                 Commands::Restart { .. } => container.restart(&item),
//                 Commands::Build { .. } => container.build(&item, service),
//                 Commands::Logs { .. } => container.logs(&item, service),
//                 Commands::Ps { .. } => container.ps(&item),
//                 Commands::Exec { .. } => container.exec(&item, service, subcommand),
//                 Commands::Run { .. } => container.exec(&item, service, subcommand),
//                 Commands::Cd { .. } => {
//                     let system = System::init();
//                     println!("{}", system.cd(&item).unwrap());
//                     Ok(())
//                 }
//                 _ => Err(eyre!("Should not happen, unknown command")),
//             },
//             None => Err(eyre!("Compose item {name} not found")),
//         },
//         None => match command {
//             Commands::List => container.list(config),
//             _ => Err(eyre!("Should not happen, no item, but not list command")),
//         },
//     }
//      */
// }

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}


// impl Cli {
//     pub fn run(container: &dyn Container, config: &mut dyn CliConfig) -> Result<()> {
//         let cli = Cli::parse();
        
//         match &cli.command {
//             // Generate shell completions
//             Commands::Completion { generator } => {
//                 let mut cmd = Cli::command();
//                 eprintln!("Generating completion file for {:?}...", &generator);
//                 print_completions(generator.unwrap(), &mut cmd);
//                 std::process::exit(1);
//             }
//             Commands::List => {
//                 // Todo
//                 print!("Not implemented yet");
//                 Ok(())
//             }
//             Commands::Cd { project } => {
//                 // Todo
//                 println!("Not implemented yet {:?}", project);
//                 Ok(())
//             }
//             Commands::Build(args) => {
//                 execute_compose_command(config, container, &args.project, &cli.command)
//             }
//             Commands::Down(args) => {
//                 execute_compose_command(config, container, &args.project, &cli.command)
//             }
//             Commands::Exec(args) => {
//                 execute_compose_command(config, container, &args.project, &cli.command)
//             }
//             Commands::Ps(args) => {
//                 execute_compose_command(config, container, &args.project, &cli.command)
//             }
//             Commands::Logs(args) => {
//                 execute_compose_command(config, container, &args.project, &cli.command)
//             }
//             Commands::Restart(args) => {
//                 execute_compose_command(config, container, &args.project, &cli.command)
//             }
//             Commands::Start(args) => {
//                 execute_compose_command(config, container, &args.project, &cli.command)
//             }
//             Commands::Stop(args) => {
//                 execute_compose_command(config, container, &args.project, &cli.command)
//             }
//             Commands::Top(args) => {
//                 execute_compose_command(config, container, &args.project, &cli.command)
//             }
//             Commands::Run(args) => {
//                 execute_compose_command(config, container, &args.project, &cli.command)
//             }
//             Commands::Up(args) => {
//                 execute_compose_command(config, container, &args.project, &cli.command)
//             }
//             _ => Err(eyre!("Should not happen, unknown command")),
//         }
//     }
// }

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
