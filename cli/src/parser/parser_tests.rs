#[cfg(test)]
mod tests {
    use crate::parser::parser::DctlConfig;

    fn get_valid_config() -> String {
        let config = r#"
        [main]
        docker_bin = "docker"
        [[collections]]
        alias = "test1"
        description = "description 1"
        enviroment_file = "/home/test/test1/.env"
        compose_files = ["/home/test/test1/docker-compose.yml"]
        

        [[collections]]
        alias = "test2"
        enviroment_file = "/home/test/test2/.env"
        compose_files = [
            "/home/test/test2/docker-compose1.yml",
            "/home/test/test2/docker-compose2.yml",
        ]

        [[collections]]
        alias = "test3"
        description = "description 3"
        compose_files = [
            "/home/test/test3/docker-compose.yml"
        ]
        "#;

        config.to_string()
    }

    #[test]
    fn load_a_valid_config() {
        let config = DctlConfig::load("tests/valid_config.toml".to_string());
        assert!(config.is_ok());
    }

    #[test]
    fn load_a_unvalid_config() {
        let config = DctlConfig::load("tests/bad_config.toml".to_string());
        assert!(config.is_err());
    }

    #[test]
    fn get_a_valid_alias_item() {
        let config: DctlConfig = toml::from_str(get_valid_config().as_str()).unwrap();
        let item = config.get_compose_item_by_alias(String::from("test1"));
        assert!(item.is_some());
    }

    #[test]
    fn get_a_unvalid_alias_item() {
        let config: DctlConfig = toml::from_str(get_valid_config().as_str()).unwrap();
        let item = config.get_compose_item_by_alias(String::from("test"));
        assert!(item.is_none());
    }

    #[test]
    fn get_compose_items() {
        let config: DctlConfig = toml::from_str(get_valid_config().as_str()).unwrap();
        let items = config.get_all_compose_items();
        assert!(3 == items.len());
    }

    #[test]
    fn get_item_attributes_values_for_test1() {
        let config: DctlConfig = toml::from_str(get_valid_config().as_str()).unwrap();
        let item = config
            .get_compose_item_by_alias(String::from("test1"))
            .unwrap();
        assert!(item.alias == "test1");
        assert!(item.description.unwrap() == "description 1");
        assert!(item.enviroment_file.unwrap() == "/home/test/test1/.env");
        assert!(item.compose_files.len() == 1);
        assert!(item.compose_files[0] == "/home/test/test1/docker-compose.yml");
    }

    #[test]
    fn get_item_attributes_values_for_test2() {
        let config: DctlConfig = toml::from_str(get_valid_config().as_str()).unwrap();
        let item = config
            .get_compose_item_by_alias(String::from("test2"))
            .unwrap();
        assert!(item.alias == "test2");
        assert!(item.description.is_none());
        assert!(item.enviroment_file.unwrap() == "/home/test/test2/.env");
        assert!(item.compose_files.len() == 2);
        assert!(item.compose_files[0] == "/home/test/test2/docker-compose1.yml");
        assert!(item.compose_files[1] == "/home/test/test2/docker-compose2.yml");
    }

    #[test]
    fn get_item_attributes_values_for_test3() {
        let config: DctlConfig = toml::from_str(get_valid_config().as_str()).unwrap();
        let item = config
            .get_compose_item_by_alias(String::from("test3"))
            .unwrap();
        assert!(item.alias == "test3");
        assert!(item.description.unwrap() == "description 3");
        assert!(item.enviroment_file.is_none());
        assert!(item.compose_files.len() == 1);
        assert!(item.compose_files[0] == "/home/test/test3/docker-compose.yml");
    }
}