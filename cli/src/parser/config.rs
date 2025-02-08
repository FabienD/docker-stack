use eyre::{Context, Result};
use serde::Deserialize;
use std::{ffi::OsStr, fs};
use tabled::Tabled;

#[derive(Debug, Clone, Deserialize, Tabled, PartialEq, Eq)]
pub enum ComposeStatus {
    Running,
    PartialRunning,
    Stopped,
    ConfigError,
}

#[derive(Debug, Clone, Deserialize, Tabled, PartialEq)]
pub struct ComposeItem {
    #[tabled(rename = " 🐋 Alias", display  = "display_alias")]
    pub alias: String,
    #[tabled(rename = " 📃 Description", display  = "display_description")]
    pub description: Option<String>,
    #[tabled(rename = "⚡Status", display  = "display_status")]
    pub status: Option<ComposeStatus>,
    #[tabled(skip)]
    pub use_project_name: Option<bool>,
    #[tabled(skip)]
    pub enviroment_file: Option<String>,
    #[tabled(skip)]
    pub compose_files: Vec<String>,
}

pub trait CliConfig {
    fn get_container_bin_path(&self) -> Result<String>;
    fn get_default_command_args(&self, command_name: &str) -> Option<DefaultCommandArgs>;
    fn load(config_path_file: String) -> Result<Self>
    where
        Self: Sized;
    fn get_compose_item_by_alias(&self, alias: String) -> Option<ComposeItem>;
    fn get_all_compose_items(&self) -> Vec<ComposeItem>;
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub docker_bin: String,
    pub default_command_args: Option<Vec<DefaultCommandArgs>>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct DefaultCommandArgs {
    pub command_name: String,
    pub command_args: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct DctlConfig {
    pub main: Config,
    pub collections: Vec<ComposeItem>,
}

fn display_alias(alias: &String) -> String {
    alias.to_string()
}

fn display_status(status: &Option<ComposeStatus>) -> String {
    match status {
        Some(s) => {
            if *s == ComposeStatus::Running {
                "🟢 Running".to_string()
            } else if *s == ComposeStatus::PartialRunning {
                "🟠 Partially running".to_string()
            } else if *s == ComposeStatus::Stopped {
                "🔴 Stopped".to_string()
            } else {
                "🟣 Error, check your config".to_string()
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
    pub fn set_status(&mut self, running_container: isize, all_container: isize) {
        let status = if (running_container == all_container) && (all_container > 0) {
            ComposeStatus::Running
        } else if  running_container == 0 {
            ComposeStatus::Stopped
        } else if (running_container == all_container) && (all_container == -1)  {
            ComposeStatus::ConfigError
        } else {
            ComposeStatus::PartialRunning
        };
        self.status = Some(status);
    }

    pub fn to_args(compose_item: &ComposeItem) -> Vec<&OsStr> {
        let mut item_args: Vec<&OsStr> = Vec::new();
        if compose_item.use_project_name.unwrap_or(true) {
            item_args.push(OsStr::new("-p"));
            item_args.push(OsStr::new(&compose_item.alias));
        }

        match &compose_item.enviroment_file {
            Some(env_file) => {
                item_args.push(OsStr::new("--env-file"));
                item_args.push(OsStr::new(env_file));
            }
            None => {}
        };

        compose_item.compose_files.iter().for_each(|compose_file| {
            item_args.push(OsStr::new("-f"));
            item_args.push(OsStr::new(compose_file));
        });

        item_args
    }
}

impl DefaultCommandArgs {
    pub fn default(command_name: &str) -> DefaultCommandArgs {
        DefaultCommandArgs {
            command_name: command_name.to_string(),
            command_args: Vec::new(),
        }
    }

    pub fn to_args(default_command_args: &DefaultCommandArgs) -> Vec<&OsStr> {
        let mut default_arg: Vec<&OsStr> = Vec::new();
        if !default_command_args.command_args.is_empty() {
            default_command_args.command_args.iter().for_each(|arg| {
                default_arg.push(OsStr::new(arg));
            })
        }
        default_arg
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

impl CliConfig for DctlConfig {
    fn get_container_bin_path(&self) -> Result<String> {
        Ok(self.main.docker_bin.to_string())
    }

    fn get_default_command_args(&self, command_name: &str) -> Option<DefaultCommandArgs> {
        let mut result: Option<DefaultCommandArgs> = None;
        if let Some(default_command_args) = &self.main.default_command_args {
            for default_command_arg in default_command_args {
                if default_command_arg.command_name == *command_name {
                    result = Some(default_command_arg.clone());
                    break;
                }
            }
        }

        result
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
    fn it_returns_the_project_alias() {
        let alias = String::from("test");
        assert_eq!(display_alias(&alias), "test");
    }

    #[test]
    fn it_displays_the_project_description() {
        let description = Some(String::from("description"));
        assert_eq!(display_description(&description), "description");

        let description = None;
        assert_eq!(display_description(&description), "");
    }

    #[test]
    fn it_displays_the_project_status() {
        let status = Some(ComposeStatus::Running);
        assert_eq!(display_status(&status), "🟢 Running");

        let status = Some(ComposeStatus::PartialRunning);
        assert_eq!(display_status(&status), "🟠 Partially running");

        let status = Some(ComposeStatus::Stopped);
        assert_eq!(display_status(&status), "🔴 Stopped");

        let status = Some(ComposeStatus::ConfigError);
        assert_eq!(display_status(&status), "🟣 Error, check your config");

        let status = None;
        assert_eq!(display_status(&status), "🔴 Stopped");
    }

    #[test]
    fn it_returns_the_docker_bin_path() {
        let config = DctlConfig {
            main: Config {
                docker_bin: String::from("/usr/bin/docker"),
                default_command_args: None,
            },
            collections: Vec::new(),
        };

        assert_eq!(config.get_container_bin_path().unwrap(), "/usr/bin/docker");
    }

    #[test]
    fn it_sets_the_compose_item_status() {
        let mut compose_item = ComposeItem {
            alias: String::from("test"),
            use_project_name: Some(false),
            description: Some(String::from("description")),
            compose_files: vec![
                String::from("docker-compose.yml"),
                String::from("docker-compose.override.yml"),
            ],
            enviroment_file: Some(String::from("test.env")),
            status: None,
        };

        compose_item.set_status(0, 0);
        assert_eq!(compose_item.status, Some(ComposeStatus::Stopped));

        compose_item.set_status(0, 6);
        assert_eq!(compose_item.status, Some(ComposeStatus::Stopped));

        compose_item.set_status(2, 3);
        assert_eq!(compose_item.status, Some(ComposeStatus::PartialRunning));

        compose_item.set_status(3, 3);
        assert_eq!(compose_item.status, Some(ComposeStatus::Running));

        compose_item.set_status(-1, -1);
        assert_eq!(compose_item.status, Some(ComposeStatus::ConfigError));
    }

    #[test]
    fn it_returns_args_as_str_from_a_complete_compose_item() {
        let compose_item = ComposeItem {
            alias: String::from("test"),
            use_project_name: Some(false),
            description: Some(String::from("description")),
            compose_files: vec![
                String::from("docker-compose.yml"),
                String::from("docker-compose.override.yml"),
            ],
            enviroment_file: Some(String::from("test.env")),
            status: None,
        };

        let args = ComposeItem::to_args(&compose_item);

        assert!(args.len() == 6);
        assert!(args[0] == OsStr::new("--env-file"));
        assert!(args[1] == OsStr::new("test.env"));
        assert!(args[2] == OsStr::new("-f"));
        assert!(args[3] == OsStr::new("docker-compose.yml"));
        assert!(args[4] == OsStr::new("-f"));
        assert!(args[5] == OsStr::new("docker-compose.override.yml"));
    }

    #[test]
    fn it_returns_args_as_str_from_a_minimal_compose_item() {
        let compose_item = ComposeItem {
            alias: String::from("test"),
            use_project_name: None,
            description: Some(String::from("description")),
            compose_files: vec![String::from("docker-compose.yml")],
            enviroment_file: None,
            status: None,
        };

        let args = ComposeItem::to_args(&compose_item);

        assert!(args.len() == 4);
        assert!(args[0] == OsStr::new("-p"));
        assert!(args[1] == OsStr::new("test"));
        assert!(args[2] == OsStr::new("-f"));
        assert!(args[3] == OsStr::new("docker-compose.yml"));
    }

    #[test]
    fn it_build_a_undefined_default_command_args() {
        let command_args = DefaultCommandArgs::default("down");

        assert!(command_args.command_name == "down");
        assert!(command_args.command_args.len() == 0);
    }

    #[test]
    fn it_returns_args_as_str_from_an_undefined_default_command_args() {
        let command_args = DefaultCommandArgs::default("down");
        let args = DefaultCommandArgs::to_args(&command_args);

        assert!(args.len() == 0);
    }

    #[test]
    fn it_returns_args_as_str_from_a_defined_default_command_args() {
        let command_args = DefaultCommandArgs {
            command_name: String::from("down"),
            command_args: vec![String::from("--volumes"), String::from("--remove-orphans")],
        };

        let args = DefaultCommandArgs::to_args(&command_args);

        assert!(args.len() == 2);
        assert!(args[0] == OsStr::new("--volumes"));
        assert!(args[1] == OsStr::new("--remove-orphans"));
    }
}
