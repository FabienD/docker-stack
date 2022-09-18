use eyre::{Context, Result};
use serde::Deserialize;
use std::fs;

#[derive(Debug, Clone, Deserialize)]
pub struct ComposeItem {
    pub alias: String,
    pub description: Option<String>,
    pub enviroment_file: String,
    pub compose_files: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub docker_bin: String,
}

#[derive(Debug, Deserialize)]
pub struct DctlConfig {
    pub main: Config,
    pub collections: Vec<ComposeItem>,
}

impl DctlConfig {
    pub fn load(config_path_file: String) -> Result<Self> {
        // Load config file
        let full_config_path = shellexpand::tilde(&config_path_file).to_string();
        // Read the config file
        let config_content = fs::read_to_string(&full_config_path)
            .wrap_err(format!("config file not found in {full_config_path}"))?;
        // Parse the config file
        let config: DctlConfig = toml::from_str(config_content.as_str())
            .wrap_err("TOML parse error, check your config file structure.")?;

        Ok(config)
    }

    pub fn get_compose_item_by_alias(&self, alias: String) -> Option<ComposeItem> {
        let mut result: Option<ComposeItem> = None;
        for item in &self.collections {
            if item.alias == alias {
                result = Some(item.clone());
                break;
            }
        }

        result
    }

    pub fn get_all_compose_items(&self) -> Vec<ComposeItem> {
        self.collections.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::DctlConfig;

    fn get_valid_config() -> String {

        let config = r#"
        [main]
        docker_bin = "docker"
        [[collections]]
        alias = "test1"
        description = "test description"
        enviroment_file = "/home/test/test1/.env"
        compose_files = ["/home/test/test1/docker-compose.yml"]
        

        [[collections]]
        alias = "test2"
        description = "test description 2"
        enviroment_file = "/home/test/test2/.env"
        compose_files = [
            "/home/test/test2/docker-compose1.yml",
            "/home/test/test2/docker-compose2.yml",
        ]
        "#;

        config.to_string()
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
        assert!(2 == items.len());
    }

    #[test]
    fn get_item_attributes_values_for_test1() {
        let config: DctlConfig = toml::from_str(get_valid_config().as_str()).unwrap();
        let item = config.get_compose_item_by_alias(String::from("test1")).unwrap();
        assert!(item.alias == "test1");
        assert!(item.description.unwrap() == "test description");
        assert!(item.enviroment_file == "/home/test/test1/.env");
        assert!(item.compose_files.len() == 1);
        assert!(item.compose_files[0] == "/home/test/test1/docker-compose.yml");
    }

    #[test]
    fn get_item_attributes_values_for_test2() {
        let config: DctlConfig = toml::from_str(get_valid_config().as_str()).unwrap();
        let item = config.get_compose_item_by_alias(String::from("test2")).unwrap();
        assert!(item.alias == "test2");
        assert!(item.description.unwrap() == "test description 2");
        assert!(item.enviroment_file == "/home/test/test2/.env");
        assert!(item.compose_files.len() == 2);
        assert!(item.compose_files[0] == "/home/test/test2/docker-compose1.yml");
        assert!(item.compose_files[1] == "/home/test/test2/docker-compose2.yml");
    }
}