#[cfg(test)]
mod tests {
    use crate::command::docker;
    use crate::parser::config::ComposeItem;
    use std::ffi::OsStr;

    fn get_compose_item(use_project_name: bool) -> ComposeItem {
        let item = ComposeItem {
            alias: String::from("test"),
            description: Some(String::from("description")),
            enviroment_file: Some(String::from("/home/test/.env")),
            compose_files: vec![
                String::from("/home/test/docker-compose.yml"),
                String::from("/home/test/docker-compose2.yml"),
            ],
            use_project_name: Some(use_project_name),
            status: None,
        };

        item
    }

    #[test]
    fn prepare_up_command() {
        let item = get_compose_item(true);
        let command = docker::prepare_command(
            String::from("docker"),
            docker::CommandType::Up,
            Some(&item),
            None,
            None,
        );

        match command {
            Ok(command) => {
                let mut args: Vec<&OsStr> = vec![];
                args.push(OsStr::new("compose"));
                args.push(OsStr::new("-p"));
                args.push(OsStr::new(&item.alias));
                args.push(OsStr::new("--env-file"));
                match &item.enviroment_file {
                    Some(env_file) => {
                        args.push(OsStr::new(env_file));
                    }
                    None => {}
                };
                item.compose_files.iter().for_each(|compose_file| {
                    args.push(OsStr::new("-f"));
                    args.push(OsStr::new(compose_file));
                });
                args.push(OsStr::new("up"));
                args.push(OsStr::new("-d"));
                args.push(OsStr::new("--remove-orphans"));
                let cmd_args: Vec<&OsStr> = command.get_args().collect();

                assert_eq!(command.get_program(), OsStr::new("docker"));
                assert_eq!(cmd_args, args);
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn prepare_up_command_without_project_name() {
        let item = get_compose_item(false);
        let command = docker::prepare_command(
            String::from("docker"),
            docker::CommandType::Up,
            Some(&item),
            None,
            None,
        );

        match command {
            Ok(command) => {
                let mut args: Vec<&OsStr> = vec![];
                args.push(OsStr::new("compose"));
                args.push(OsStr::new("--env-file"));
                match &item.enviroment_file {
                    Some(env_file) => {
                        args.push(OsStr::new(env_file));
                    }
                    None => {}
                };
                item.compose_files.iter().for_each(|compose_file| {
                    args.push(OsStr::new("-f"));
                    args.push(OsStr::new(compose_file));
                });
                args.push(OsStr::new("up"));
                args.push(OsStr::new("-d"));
                args.push(OsStr::new("--remove-orphans"));
                let cmd_args: Vec<&OsStr> = command.get_args().collect();

                assert_eq!(command.get_program(), OsStr::new("docker"));
                assert_eq!(cmd_args, args);
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn prepare_start_command() {
        let item = get_compose_item(true);
        let command = docker::prepare_command(
            String::from("docker"),
            docker::CommandType::Start,
            Some(&item),
            None,
            None,
        );

        match command {
            Ok(command) => {
                let mut args: Vec<&OsStr> = vec![];
                args.push(OsStr::new("compose"));
                args.push(OsStr::new("-p"));
                args.push(OsStr::new(&item.alias));
                args.push(OsStr::new("--env-file"));
                match &item.enviroment_file {
                    Some(env_file) => {
                        args.push(OsStr::new(env_file));
                    }
                    None => {}
                };
                item.compose_files.iter().for_each(|compose_file| {
                    args.push(OsStr::new("-f"));
                    args.push(OsStr::new(compose_file));
                });
                args.push(OsStr::new("start"));
                let cmd_args: Vec<&OsStr> = command.get_args().collect();

                assert_eq!(command.get_program(), OsStr::new("docker"));
                assert_eq!(cmd_args, args);
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn prepare_stop_command() {
        let item = get_compose_item(true);
        let command = docker::prepare_command(
            String::from("docker"),
            docker::CommandType::Stop,
            Some(&item),
            None,
            None,
        );

        match command {
            Ok(command) => {
                let mut args: Vec<&OsStr> = vec![];
                args.push(OsStr::new("compose"));
                args.push(OsStr::new("-p"));
                args.push(OsStr::new(&item.alias));
                args.push(OsStr::new("--env-file"));
                match &item.enviroment_file {
                    Some(env_file) => {
                        args.push(OsStr::new(env_file));
                    }
                    None => {}
                };
                item.compose_files.iter().for_each(|compose_file| {
                    args.push(OsStr::new("-f"));
                    args.push(OsStr::new(compose_file));
                });
                args.push(OsStr::new("stop"));
                let cmd_args: Vec<&OsStr> = command.get_args().collect();

                assert_eq!(command.get_program(), OsStr::new("docker"));
                assert_eq!(cmd_args, args);
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn prepare_down_command() {
        let item = get_compose_item(true);
        let command = docker::prepare_command(
            String::from("docker"),
            docker::CommandType::Down,
            Some(&item),
            None,
            None,
        );

        match command {
            Ok(command) => {
                let mut args: Vec<&OsStr> = vec![];
                args.push(OsStr::new("compose"));
                args.push(OsStr::new("-p"));
                args.push(OsStr::new(&item.alias));
                args.push(OsStr::new("--env-file"));
                match &item.enviroment_file {
                    Some(env_file) => {
                        args.push(OsStr::new(env_file));
                    }
                    None => {}
                };
                item.compose_files.iter().for_each(|compose_file| {
                    args.push(OsStr::new("-f"));
                    args.push(OsStr::new(compose_file));
                });
                args.push(OsStr::new("down"));
                let cmd_args: Vec<&OsStr> = command.get_args().collect();

                assert_eq!(command.get_program(), OsStr::new("docker"));
                assert_eq!(cmd_args, args);
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn prepare_restart_command() {
        let item = get_compose_item(true);
        let command = docker::prepare_command(
            String::from("docker"),
            docker::CommandType::Restart,
            Some(&item),
            None,
            None,
        );

        match command {
            Ok(command) => {
                let mut args: Vec<&OsStr> = vec![];
                args.push(OsStr::new("compose"));
                args.push(OsStr::new("-p"));
                args.push(OsStr::new(&item.alias));
                args.push(OsStr::new("--env-file"));
                match &item.enviroment_file {
                    Some(env_file) => {
                        args.push(OsStr::new(env_file));
                    }
                    None => {}
                };
                item.compose_files.iter().for_each(|compose_file| {
                    args.push(OsStr::new("-f"));
                    args.push(OsStr::new(compose_file));
                });
                args.push(OsStr::new("restart"));
                let cmd_args: Vec<&OsStr> = command.get_args().collect();

                assert_eq!(command.get_program(), OsStr::new("docker"));
                assert_eq!(cmd_args, args);
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn prepare_ps_command() {
        let item = get_compose_item(true);
        let command = docker::prepare_command(
            String::from("docker"),
            docker::CommandType::Ps,
            Some(&item),
            None,
            None,
        );

        match command {
            Ok(command) => {
                let mut args: Vec<&OsStr> = vec![];
                args.push(OsStr::new("compose"));
                args.push(OsStr::new("-p"));
                args.push(OsStr::new(&item.alias));
                args.push(OsStr::new("--env-file"));
                match &item.enviroment_file {
                    Some(env_file) => {
                        args.push(OsStr::new(env_file));
                    }
                    None => {}
                };
                item.compose_files.iter().for_each(|compose_file| {
                    args.push(OsStr::new("-f"));
                    args.push(OsStr::new(compose_file));
                });
                args.push(OsStr::new("ps"));
                let cmd_args: Vec<&OsStr> = command.get_args().collect();

                assert_eq!(command.get_program(), OsStr::new("docker"));
                assert_eq!(cmd_args, args);
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn prepare_build_command() {
        let item = get_compose_item(true);
        let command = docker::prepare_command(
            String::from("docker"),
            docker::CommandType::Build,
            Some(&item),
            Some(String::from("my_service")),
            None,
        );

        match command {
            Ok(command) => {
                let mut args: Vec<&OsStr> = vec![];
                args.push(OsStr::new("compose"));
                args.push(OsStr::new("-p"));
                args.push(OsStr::new(&item.alias));
                args.push(OsStr::new("--env-file"));
                match &item.enviroment_file {
                    Some(env_file) => {
                        args.push(OsStr::new(env_file));
                    }
                    None => {}
                };
                item.compose_files.iter().for_each(|compose_file| {
                    args.push(OsStr::new("-f"));
                    args.push(OsStr::new(compose_file));
                });
                args.push(OsStr::new("build"));
                args.push(OsStr::new("my_service"));
                let cmd_args: Vec<&OsStr> = command.get_args().collect();

                assert_eq!(command.get_program(), OsStr::new("docker"));
                assert_eq!(cmd_args, args);
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn prepare_logs_command() {
        let item = get_compose_item(true);
        let command = docker::prepare_command(
            String::from("docker"),
            docker::CommandType::Logs,
            Some(&item),
            Some(String::from("my_service")),
            None,
        );

        match command {
            Ok(command) => {
                let mut args: Vec<&OsStr> = vec![];
                args.push(OsStr::new("compose"));
                args.push(OsStr::new("-p"));
                args.push(OsStr::new(&item.alias));
                args.push(OsStr::new("--env-file"));
                match &item.enviroment_file {
                    Some(env_file) => {
                        args.push(OsStr::new(env_file));
                    }
                    None => {}
                };
                item.compose_files.iter().for_each(|compose_file| {
                    args.push(OsStr::new("-f"));
                    args.push(OsStr::new(compose_file));
                });
                args.push(OsStr::new("logs"));
                args.push(OsStr::new("my_service"));
                let cmd_args: Vec<&OsStr> = command.get_args().collect();

                assert_eq!(command.get_program(), OsStr::new("docker"));
                assert_eq!(cmd_args, args);
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn prepare_exec_command() {
        let item = get_compose_item(true);
        let command = docker::prepare_command(
            String::from("docker"),
            docker::CommandType::Exec,
            Some(&item),
            Some(String::from("my_service")),
            Some(String::from("./bin/console doctrine:migrations:migrate")),
        );

        match command {
            Ok(command) => {
                let mut args: Vec<&OsStr> = vec![];
                args.push(OsStr::new("compose"));
                args.push(OsStr::new("-p"));
                args.push(OsStr::new(&item.alias));
                args.push(OsStr::new("--env-file"));
                match &item.enviroment_file {
                    Some(env_file) => {
                        args.push(OsStr::new(env_file));
                    }
                    None => {}
                };
                item.compose_files.iter().for_each(|compose_file| {
                    args.push(OsStr::new("-f"));
                    args.push(OsStr::new(compose_file));
                });
                args.push(OsStr::new("exec"));
                args.push(OsStr::new("my_service"));
                args.push(OsStr::new("./bin/console doctrine:migrations:migrate"));
                let cmd_args: Vec<&OsStr> = command.get_args().collect();

                assert_eq!(command.get_program(), OsStr::new("docker"));
                assert!(cmd_args == args);
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn prepare_run_command() {
        let item = get_compose_item(true);
        let command = docker::prepare_command(
            String::from("docker"),
            docker::CommandType::Run,
            Some(&item),
            Some(String::from("my_service")),
            Some(String::from("./bin/console doctrine:migrations:migrate")),
        );

        match command {
            Ok(command) => {
                let mut args: Vec<&OsStr> = vec![];
                args.push(OsStr::new("compose"));
                args.push(OsStr::new("-p"));
                args.push(OsStr::new(&item.alias));
                args.push(OsStr::new("--env-file"));
                match &item.enviroment_file {
                    Some(env_file) => {
                        args.push(OsStr::new(env_file));
                    }
                    None => {}
                };
                item.compose_files.iter().for_each(|compose_file| {
                    args.push(OsStr::new("-f"));
                    args.push(OsStr::new(compose_file));
                });
                args.push(OsStr::new("run"));
                args.push(OsStr::new("my_service"));
                args.push(OsStr::new("./bin/console doctrine:migrations:migrate"));
                let cmd_args: Vec<&OsStr> = command.get_args().collect();

                assert_eq!(command.get_program(), OsStr::new("docker"));
                assert!(cmd_args == args);
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn prepare_list_command() {
        let command = docker::prepare_command(
            String::from("docker"),
            docker::CommandType::List,
            None,
            None,
            None,
        );

        match command {
            Ok(command) => {
                let mut args: Vec<&OsStr> = vec![];
                args.push(OsStr::new("compose"));
                args.push(OsStr::new("ls"));
                args.push(OsStr::new("--format"));
                args.push(OsStr::new("json"));
                let cmd_args: Vec<&OsStr> = command.get_args().collect();

                assert_eq!(command.get_program(), OsStr::new("docker"));
                assert_eq!(cmd_args, args);
            }
            Err(_) => assert!(false),
        }
    }
}
