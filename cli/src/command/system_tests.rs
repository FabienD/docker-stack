#[cfg(test)]
mod tests {
    use std::ffi::OsStr;

    use crate::{
        command::system::{builder, System},
        parser::config::ComposeItem,
    };

    #[test]
    pub fn test_init() {
        let system = System::init();
        assert!(system == System {});
    }

    #[test]
    pub fn test_cd() {
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
    fn test_builder() {
        let bin_command = "ls".to_string();
        let args = vec![OsStr::new("-l"), OsStr::new("-a")];

        let cmd = builder(bin_command.to_owned(), args.to_owned());
        let cmd_args: Vec<&OsStr> = cmd.get_args().collect();

        assert_eq!(cmd.get_program(), OsStr::new(&bin_command));
        assert_eq!(cmd_args, args);
    }
}
