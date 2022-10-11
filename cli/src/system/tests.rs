#[cfg(test)]
mod tests {
    use crate::{parser::config::ComposeItem, system::command::System};

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
        };

        let system = System::init();
        assert!(system.cd(&item).unwrap() == "/home/test/test");
    }
}
