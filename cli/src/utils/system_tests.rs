#[cfg(test)]
mod tests {
    use std::ffi::OsStr;

    use crate::{
        parser::config::ComposeItem,
        utils::system::{builder, System},
    };

    #[test]
    pub fn it_inits_system() {
        let system = System::init();
        assert!(system == System {});
    }

    #[test]
    pub fn it_returns_the_path_from_cd_command() {
        let item = ComposeItem {
            alias: String::from("test"),
            description: None,
            compose_files: vec![String::from("/home/test/test/docker-compose.yml")],
            enviroment_file: None,
            use_project_name: None,
            status: None,
        };

        let system = System::init();
        assert!(system.cd(&item).unwrap() == "/home/test/test");
    }

    #[test]
    fn it_builds_a_system_command_process() {
        let bin_command = "ls".to_string();
        let args = vec![OsStr::new("-l"), OsStr::new("-a")];

        let cmd = builder(bin_command.to_owned(), args.to_owned());
        let cmd_args: Vec<&OsStr> = cmd.get_args().collect();

        assert_eq!(cmd.get_program(), OsStr::new(&bin_command));
        assert_eq!(cmd_args, args);
    }
}
