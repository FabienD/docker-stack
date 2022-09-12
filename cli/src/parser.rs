use std::fs;
use serde::Deserialize;
use toml::Spanned;


#[derive(Debug, Clone, Deserialize)]
pub struct ComposeItem {
    pub alias: String,
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
    pub fn load(config_path_file: String) -> Self {
        // Load config file
        let full_config_path = shellexpand::tilde(&config_path_file).to_string();
        let config_content = fs::read_to_string(full_config_path).expect("Unable to open config file");
        let config: DctlConfig = toml::from_str(config_content.as_str()).expect("Unable to parse config file");
        
        config
    }

    pub fn get_compose_item(&self, alias: String) -> Option<ComposeItem> {
        let mut result: Option<ComposeItem> = None;
        for item in &self.collections {
            if item.alias == alias {
                result = Some(item.clone());
                break;
            }
        }
        result
    }
}