use eyre::{Context, Result};
use mockall::automock;
use serde::Deserialize;
use std::fs;
use tabled::Tabled;

#[derive(Debug, Clone, Deserialize, Tabled)]
pub struct ComposeItem {
    #[tabled(rename = " 🐋 Alias", display_with = "display_alias")]
    pub alias: String,
    #[tabled(rename = " 📃 Description", display_with = "display_description")]
    pub description: Option<String>,
    #[tabled(rename = "⚡Status", display_with = "display_status")]
    pub status: Option<bool>,
    #[tabled(skip)]
    pub use_project_name: Option<bool>,
    #[tabled(skip)]
    pub enviroment_file: Option<String>,
    #[tabled(skip)]
    pub compose_files: Vec<String>,
}

pub trait CliConfig {
    fn get_container_bin_path(&self) -> Result<String>;
    fn load(config_path_file: String) -> Result<Self>
    where
        Self: Sized;
    fn get_compose_item_by_alias(&self, alias: String) -> Option<ComposeItem>;
    fn get_all_compose_items(&self) -> Vec<ComposeItem>;
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

fn display_alias(alias: &String) -> String {
    alias.to_string()
}

fn display_status(status: &Option<bool>) -> String {
    match status {
        Some(s) => {
            if *s {
                "🟢 Running".to_string()
            } else {
                "🔴 Stopped".to_string()
            }
        }
        None => "🔴 Stopped".to_string(),
    }
}

fn display_description(o: &Option<String>) -> String {
    match o {
        Some(s) => s.to_string(),
        None => String::new(),
    }
}

impl ComposeItem {
    pub fn set_status(&mut self, status: bool) {
        self.status = Some(status);
    }
}

impl DctlConfig {
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

#[automock]
impl CliConfig for DctlConfig {
    fn get_container_bin_path(&self) -> Result<String> {
        Ok(self.main.docker_bin.to_string())
    }

    fn load(config_path_file: String) -> Result<Self> {
        // Read the config file
        let config_content = DctlConfig::load_config_file(config_path_file)?;
        // Parse the config file
        let config: DctlConfig = DctlConfig::parse_config_file(config_content)?;

        Ok(config)
    }

    fn get_compose_item_by_alias(&self, alias: String) -> Option<ComposeItem> {
        let mut result: Option<ComposeItem> = None;
        for item in &self.get_all_compose_items() {
            if item.alias == alias {
                result = Some(item.clone());
                break;
            }
        }

        result
    }

    fn get_all_compose_items(&self) -> Vec<ComposeItem> {
        self.collections.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_display_alias() {
        let alias = String::from("test");
        assert_eq!(display_alias(&alias), "test");
    }

    #[test]
    fn get_display_description() {
        let description = Some(String::from("description"));
        assert_eq!(display_description(&description), "description");

        let description = None;
        assert_eq!(display_description(&description), "");
    }

    #[test]
    fn get_display_status() {
        let status = Some(true);
        assert_eq!(display_status(&status), "🟢 Running");

        let status = Some(false);
        assert_eq!(display_status(&status), "🔴 Stopped");

        let status = None;
        assert_eq!(display_status(&status), "🔴 Stopped");
    }
}
