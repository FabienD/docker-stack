use eyre::{Context, Result};
use serde::Deserialize;
use std::fs;
use tabled::Tabled;

#[derive(Debug, Clone, Deserialize, Tabled)]
pub struct ComposeItem {
    #[tabled(rename = " 🐋 Alias", display_with = "display_alias")]
    pub alias: String,
    #[tabled(rename = " 📃 Description", display_with = "display_description")]
    pub description: Option<String>,
    #[tabled(skip)]
    pub enviroment_file: Option<String>,
    #[tabled(skip)]
    pub compose_files: Vec<String>,
}

fn display_alias(alias: &String) -> String {
    alias.to_string()
}

fn display_description(o: &Option<String>) -> String {
    match o {
        Some(s) => s.to_string(),
        None => String::new(),
    }
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
        // Read the config file
        let config_content = DctlConfig::load_config_file(config_path_file)?;
        // Parse the config file
        let config: DctlConfig = DctlConfig::parse_config_file(config_content)?;

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

    fn load_config_file(config_path_file: String) -> Result<String> {
        // Load config file
        let full_config_path = shellexpand::tilde(&config_path_file).to_string();
        // Read the config file
        let config_content = fs::read_to_string(&full_config_path)
            .wrap_err(format!("config file not found in {full_config_path}"))?;

        Ok(config_content)
    }

    fn parse_config_file(config_content: String) -> Result<Self> {
        // Parse the config file
        let config: DctlConfig = toml::from_str(config_content.as_str())
            .wrap_err("TOML parse error, check your config file structure.")?;

        Ok(config)
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
