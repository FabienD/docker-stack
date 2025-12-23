use std::path::Path;
use std::str::from_utf8;

use clap::{Arg, ArgMatches, Command};
use anyhow::{Context, Result};

use crate::command::definitions::config_def;
use crate::parser::config::{CliConfig, ComposeItem};
use crate::utils::docker::{CommandOutput, CommandType, Container};

pub fn check_config() -> Command {
    Command::new("check-config")
        .about("Check configuration files existence and optionally validate syntax")
        .arg(
            Arg::new("VALIDATE")
                .long("validate")
                .short('v')
                .help("Also validate docker-compose syntax using 'docker compose config'")
                .action(clap::ArgAction::SetTrue),
        )
}

pub async fn exec_check_config(
    config: &mut dyn CliConfig,
    container: &dyn Container,
    args: &ArgMatches,
) -> Result<()> {
    let validate_syntax = args.get_flag("VALIDATE");

    // Check docker bin path
    let config_docker_bin_path = config.get_container_bin_path()?;
    let mut has_error = false;

    if !check_docker_bin_path(&config_docker_bin_path)? {
        println!("\nConfiguration :\n");
        println!("❌ - Docker bin path: {}", config_docker_bin_path);
        has_error = true;
    }

    // Check files in compose items
    let compose_items = config.get_all_compose_items();

    for item in compose_items {
        let compose_item_errors = check_item_config(&item)?;
        let mut item_has_errors = false;

        if !compose_item_errors.is_empty() {
            println!("\nProject : {:?} ", item.alias);
            for error in compose_item_errors {
                println!("{}", error);
            }
            item_has_errors = true;
            has_error = true;
        }

        // Validate syntax if requested and files exist
        if validate_syntax && !item_has_errors {
            let syntax_errors = validate_compose_syntax(&item, container).await?;
            if !syntax_errors.is_empty() {
                if !item_has_errors {
                    println!("\nProject : {:?} ", item.alias);
                }
                for error in syntax_errors {
                    println!("{}", error);
                }
                has_error = true;
            }
        }
    }

    if !has_error {
        if validate_syntax {
            println!("✅ - No errors found (files exist and syntax is valid)");
        } else {
            println!("✅ - No errors found");
        }
    }

    Ok(())
}

/// Validate docker-compose file syntax using `docker compose config --quiet`
async fn validate_compose_syntax(
    item: &ComposeItem,
    container: &dyn Container,
) -> Result<Vec<String>> {
    let mut error_list: Vec<String> = Vec::new();

    let config_args = ComposeItem::to_args(item);

    // Get the config command definition
    let config_command = config_def().to_clap_command();

    // Run docker compose config --quiet to validate syntax
    let args = config_command.try_get_matches_from(vec!["config", "--quiet", &item.alias])?;

    let result = container
        .compose(
            CommandType::Config,
            &config_args,
            &vec![],
            &args,
            Some(CommandOutput::Output),
        )
        .await;

    match result {
        Ok(output) => {
            // Check if there's anything on stderr (warnings or errors)
            let stderr = from_utf8(&output.stderr).context("Invalid UTF-8 in stderr")?;
            if !stderr.trim().is_empty() {
                error_list.push(format!("⚠️  - Compose warning:\n{}", stderr.trim()));
            }
        }
        Err(e) => {
            error_list.push(format!("❌ - Compose syntax error: {}", e));
        }
    }

    Ok(error_list)
}

pub fn check_item_config(item: &ComposeItem) -> Result<Vec<String>> {
    let mut error_list: Vec<String> = Vec::new();

    if let Some(env_file) = &item.enviroment_file {
        let file_path = Path::new(&env_file);
        if !file_path.exists() {
            error_list.push(format!("❌ - env file: {:?}", file_path));
        }
    }

    for file in &item.compose_files {
        let file_path = Path::new(&file);
        if !file_path.exists() {
            error_list.push(format!("❌ - Compose file: {:?}", file_path));
        }
    }

    Ok(error_list)
}

fn check_docker_bin_path(config_docker_bin_path: &str) -> Result<bool> {
    let docker_bin_path = Path::new(&config_docker_bin_path);

    Ok(docker_bin_path.exists())
}

#[cfg(test)]
mod tests {
    use crate::parser::config::ComposeStatus;

    use super::*;

    #[test]
    fn test_check_config_command_has_validate_flag() {
        let cmd = check_config();

        // Verify command is built correctly
        assert_eq!(cmd.get_name(), "check-config");

        // Test without flag
        let result = cmd.clone().try_get_matches_from(vec!["check-config"]);
        assert!(result.is_ok());
        assert!(!result.unwrap().get_flag("VALIDATE"));

        // Test with flag
        let result = cmd.clone().try_get_matches_from(vec!["check-config", "--validate"]);
        assert!(result.is_ok());
        assert!(result.unwrap().get_flag("VALIDATE"));

        // Test with short flag
        let result = cmd.try_get_matches_from(vec!["check-config", "-v"]);
        assert!(result.is_ok());
        assert!(result.unwrap().get_flag("VALIDATE"));
    }

    #[test]
    fn test_check_item_config_valid_compose_file() {
        let item = ComposeItem {
            alias: "test".to_string(),
            enviroment_file: None,
            compose_files: vec!["tests/docker-compose.test.yml".to_string()],
            description: Some("test".to_string()),
            status: Some(ComposeStatus::Running),
            use_project_name: Some(false),
        };

        let errors = check_item_config(&item).unwrap();
        assert!(errors.is_empty());
    }

    #[test]
    fn test_check_item_config_missing_files() {
        let item = ComposeItem {
            alias: "test".to_string(),
            enviroment_file: Some("tests/.env".to_string()),
            compose_files: vec!["tests/docker-compose.yml".to_string()],
            description: Some("test".to_string()),
            status: Some(ComposeStatus::Running),
            use_project_name: Some(false),
        };

        let errors = check_item_config(&item).unwrap();

        assert_eq!(errors.len(), 2);
        assert!(errors.iter().any(|e| e.contains("env file")));
        assert!(errors.iter().any(|e| e.contains("Compose file")));
    }

    #[test]
    fn test_check_item_config_missing_compose_only() {
        let item = ComposeItem {
            alias: "test".to_string(),
            enviroment_file: None,
            compose_files: vec!["nonexistent/docker-compose.yml".to_string()],
            description: None,
            status: None,
            use_project_name: None,
        };

        let errors = check_item_config(&item).unwrap();

        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("Compose file"));
    }

    #[test]
    fn test_check_item_config_multiple_compose_files() {
        let item = ComposeItem {
            alias: "test".to_string(),
            enviroment_file: None,
            compose_files: vec![
                "nonexistent1.yml".to_string(),
                "nonexistent2.yml".to_string(),
            ],
            description: None,
            status: None,
            use_project_name: None,
        };

        let errors = check_item_config(&item).unwrap();

        assert_eq!(errors.len(), 2);
    }

    #[test]
    fn test_check_docker_bin_path_exists() {
        // /usr/bin or /bin should exist on Unix systems
        #[cfg(unix)]
        {
            assert!(check_docker_bin_path("/usr/bin").unwrap());
        }
    }

    #[test]
    fn test_check_docker_bin_path_not_exists() {
        assert!(!check_docker_bin_path("/nonexistent/path/docker").unwrap());
    }
}