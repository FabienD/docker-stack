use std::env;
use std::fs;
use std::path::Path;

use clap::{Arg, ArgMatches, Command};
use anyhow::{anyhow, Context, Result};
use toml_edit::{Array, DocumentMut, Item, Table, Value};

use crate::parser::config::CliConfig;

pub fn register_project() -> Command {
    Command::new("register")
        .about("Register a new project in the configuration")
        .arg(
            Arg::new("ALIAS")
                .help("The alias for the project")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("COMPOSE_FILES")
                .help("Path(s) to the docker-compose.yml file(s)")
                .required(true)
                .num_args(1..)
                .index(2),
        )
        .arg(
            Arg::new("ENV_FILE")
                .long("env-file")
                .short('e')
                .help("Path to the environment file"),
        )
        .arg(
            Arg::new("DESCRIPTION")
                .long("description")
                .short('d')
                .help("Description of the project"),
        )
}

fn get_config_path() -> String {
    env::var("DCTL_CONFIG_FILE_PATH")
        .unwrap_or_else(|_| String::from("~/.config/dctl/config.toml"))
}

fn expand_path(path: &str) -> String {
    shellexpand::tilde(path).to_string()
}

pub fn exec_register_project(config: &dyn CliConfig, args: &ArgMatches) -> Result<()> {
    let alias = args.get_one::<String>("ALIAS").unwrap();
    let compose_files_args: Vec<&String> = args
        .get_many::<String>("COMPOSE_FILES")
        .unwrap()
        .collect();
    let env_file = args.get_one::<String>("ENV_FILE");
    let description = args.get_one::<String>("DESCRIPTION");

    // Check if alias already exists
    if config.get_compose_item_by_alias(alias.clone()).is_some() {
        return Err(anyhow!("Project with alias '{}' already exists", alias));
    }

    // Validate all compose files exist
    for compose_file in &compose_files_args {
        let compose_path = expand_path(compose_file);
        if !Path::new(&compose_path).exists() {
            return Err(anyhow!("Compose file does not exist: {}", compose_file));
        }
    }

    // Validate env file if provided
    if let Some(env) = env_file {
        let env_path = expand_path(env);
        if !Path::new(&env_path).exists() {
            return Err(anyhow!("Environment file does not exist: {}", env));
        }
    }

    // Read and modify config file
    let config_path = expand_path(&get_config_path());
    let config_content = fs::read_to_string(&config_path)
        .context(format!("Failed to read config file: {}", config_path))?;

    let mut doc = config_content
        .parse::<DocumentMut>()
        .context("Failed to parse config file")?;

    // Create new collection entry
    let mut new_collection = Table::new();
    new_collection.insert("alias", Value::from(alias.as_str()).into());

    if let Some(desc) = description {
        new_collection.insert("description", Value::from(desc.as_str()).into());
    }

    if let Some(env) = env_file {
        new_collection.insert("enviroment_file", Value::from(env.as_str()).into());
    }

    let mut compose_files = Array::new();
    for file in &compose_files_args {
        compose_files.push(file.as_str());
    }
    new_collection.insert("compose_files", Item::Value(Value::Array(compose_files)));

    // Add to collections array
    if let Some(collections) = doc.get_mut("collections") {
        if let Some(arr) = collections.as_array_of_tables_mut() {
            arr.push(new_collection);
        } else {
            return Err(anyhow!("Invalid config format: 'collections' is not an array of tables"));
        }
    } else {
        return Err(anyhow!("Invalid config format: missing 'collections' section"));
    }

    // Write back to file
    fs::write(&config_path, doc.to_string())
        .context(format!("Failed to write config file: {}", config_path))?;

    println!("Project '{}' registered successfully", alias);
    println!("  Compose file(s):");
    for file in &compose_files_args {
        println!("    - {}", file);
    }
    if let Some(env) = env_file {
        println!("  Environment file: {}", env);
    }
    if let Some(desc) = description {
        println!("  Description: {}", desc);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_command_has_required_args() {
        let cmd = register_project();

        // Verify command is built correctly
        assert_eq!(cmd.get_name(), "register");

        // Test that it requires ALIAS and COMPOSE_FILES
        let result = cmd.clone().try_get_matches_from(vec!["register"]);
        assert!(result.is_err());

        let result = cmd.clone().try_get_matches_from(vec!["register", "myproject"]);
        assert!(result.is_err());

        let result = cmd.clone().try_get_matches_from(vec![
            "register",
            "myproject",
            "/path/to/docker-compose.yml",
        ]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_register_command_with_multiple_compose_files() {
        let cmd = register_project();

        let result = cmd.try_get_matches_from(vec![
            "register",
            "myproject",
            "/path/to/docker-compose.yml",
            "/path/to/docker-compose.override.yml",
        ]);

        assert!(result.is_ok());
        let matches = result.unwrap();

        let files: Vec<&String> = matches
            .get_many::<String>("COMPOSE_FILES")
            .unwrap()
            .collect();

        assert_eq!(files.len(), 2);
        assert_eq!(files[0], "/path/to/docker-compose.yml");
        assert_eq!(files[1], "/path/to/docker-compose.override.yml");
    }

    #[test]
    fn test_register_command_with_options() {
        let cmd = register_project();

        let result = cmd.try_get_matches_from(vec![
            "register",
            "myproject",
            "/path/to/docker-compose.yml",
            "--env-file",
            "/path/to/.env",
            "--description",
            "My test project",
        ]);

        assert!(result.is_ok());
        let matches = result.unwrap();
        assert_eq!(
            matches.get_one::<String>("ALIAS").unwrap(),
            "myproject"
        );

        let files: Vec<&String> = matches
            .get_many::<String>("COMPOSE_FILES")
            .unwrap()
            .collect();
        assert_eq!(files[0], "/path/to/docker-compose.yml");

        assert_eq!(
            matches.get_one::<String>("ENV_FILE").unwrap(),
            "/path/to/.env"
        );
        assert_eq!(
            matches.get_one::<String>("DESCRIPTION").unwrap(),
            "My test project"
        );
    }

    #[test]
    fn test_expand_path() {
        let path = expand_path("/absolute/path");
        assert_eq!(path, "/absolute/path");

        // Tilde expansion should work
        let home_path = expand_path("~/test");
        assert!(home_path.starts_with('/'));
        assert!(home_path.ends_with("/test"));
    }
}
