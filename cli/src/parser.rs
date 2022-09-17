use std::fs;
use serde::Deserialize;
use eyre::{Result, Context};

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