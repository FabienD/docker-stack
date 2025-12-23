use std::env;
use std::fs;

use clap::{Arg, ArgMatches, Command};
use anyhow::{anyhow, Context, Result};
use toml_edit::DocumentMut;

use crate::parser::config::CliConfig;

pub fn unregister_project() -> Command {
    Command::new("unregister")
        .about("Unregister a project from the configuration")
        .arg(
            Arg::new("ALIAS")
                .help("The alias of the project to remove")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("FORCE")
                .long("force")
                .short('f')
                .help("Skip confirmation prompt")
                .action(clap::ArgAction::SetTrue),
        )
}

fn get_config_path() -> String {
    env::var("DCTL_CONFIG_FILE_PATH")
        .unwrap_or_else(|_| String::from("~/.config/dctl/config.toml"))
}

fn expand_path(path: &str) -> String {
    shellexpand::tilde(path).to_string()
}

pub fn exec_unregister_project(config: &dyn CliConfig, args: &ArgMatches) -> Result<()> {
    let alias = args.get_one::<String>("ALIAS").unwrap();
    let force = args.get_flag("FORCE");

    // Check if alias exists
    if config.get_compose_item_by_alias(alias.clone()).is_none() {
        return Err(anyhow!("Project with alias '{}' does not exist", alias));
    }

    // Confirmation (skip if --force)
    if !force {
        println!("Are you sure you want to unregister project '{}'? (y/N)", alias);
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .context("Failed to read user input")?;
        let input = input.trim().to_lowercase();
        if input != "y" && input != "yes" {
            println!("Operation cancelled");
            return Ok(());
        }
    }

    // Read and modify config file
    let config_path = expand_path(&get_config_path());
    let config_content = fs::read_to_string(&config_path)
        .context(format!("Failed to read config file: {}", config_path))?;

    let mut doc = config_content
        .parse::<DocumentMut>()
        .context("Failed to parse config file")?;

    // Find and remove the collection with matching alias
    if let Some(collections) = doc.get_mut("collections") {
        if let Some(arr) = collections.as_array_of_tables_mut() {
            // Find the index of the collection to remove
            let mut index_to_remove: Option<usize> = None;
            for (i, table) in arr.iter().enumerate() {
                if let Some(item_alias) = table.get("alias") {
                    if let Some(alias_str) = item_alias.as_str() {
                        if alias_str == alias {
                            index_to_remove = Some(i);
                            break;
                        }
                    }
                }
            }

            if let Some(idx) = index_to_remove {
                arr.remove(idx);
            } else {
                return Err(anyhow!(
                    "Project '{}' not found in config (inconsistent state)",
                    alias
                ));
            }
        } else {
            return Err(anyhow!("Invalid config format: 'collections' is not an array of tables"));
        }
    } else {
        return Err(anyhow!("Invalid config format: missing 'collections' section"));
    }

    // Write back to file
    fs::write(&config_path, doc.to_string())
        .context(format!("Failed to write config file: {}", config_path))?;

    println!("Project '{}' unregistered successfully", alias);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unregister_command_has_required_args() {
        let cmd = unregister_project();

        // Verify command is built correctly
        assert_eq!(cmd.get_name(), "unregister");

        // Test that it requires ALIAS
        let result = cmd.clone().try_get_matches_from(vec!["unregister"]);
        assert!(result.is_err());

        let result = cmd.clone().try_get_matches_from(vec!["unregister", "myproject"]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_unregister_command_with_force() {
        let cmd = unregister_project();

        let result = cmd.try_get_matches_from(vec!["unregister", "myproject", "--force"]);

        assert!(result.is_ok());
        let matches = result.unwrap();
        assert_eq!(
            matches.get_one::<String>("ALIAS").unwrap(),
            "myproject"
        );
        assert!(matches.get_flag("FORCE"));
    }

    #[test]
    fn test_unregister_command_force_short() {
        let cmd = unregister_project();

        let result = cmd.try_get_matches_from(vec!["unregister", "-f", "myproject"]);

        assert!(result.is_ok());
        let matches = result.unwrap();
        assert!(matches.get_flag("FORCE"));
    }
}
