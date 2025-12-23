use std::path::Path;

use clap::{Arg, Command};
use anyhow::{Context, Result};

use crate::parser::config::ComposeItem;

pub fn cd_project() -> Command {
    Command::new("cd")
        .about("Change directory to the project directory")
        .arg(
            Arg::new("PROJECT")
                .help("The name of the docker-compose file alias")
                .required(true),
        )
}

pub fn exec_cd_project(compose_item: &ComposeItem) -> Result<()> {
    println!("{}", extract_path_from_cd_command(compose_item)?);
    Ok(())
}

fn extract_path_from_cd_command(compose_item: &ComposeItem) -> Result<String> {
    let first_file = compose_item
        .compose_files
        .first()
        .context("No compose files configured for this project")?;

    let path = Path::new(first_file)
        .parent()
        .context("Compose file path has no parent directory")?;

    path.to_str()
        .context("Path contains invalid UTF-8 characters")
        .map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn it_returns_the_path_from_cd_command() {
        let item = ComposeItem {
            alias: String::from("test"),
            description: None,
            compose_files: vec![String::from("/home/test/test/docker-compose.yml")],
            enviroment_file: None,
            use_project_name: None,
            status: None,
        };

        assert_eq!(
            extract_path_from_cd_command(&item).unwrap(),
            "/home/test/test"
        );
    }

    #[test]
    fn test_extract_path_with_multiple_compose_files() {
        let item = ComposeItem {
            alias: String::from("test"),
            description: None,
            compose_files: vec![
                String::from("/first/path/docker-compose.yml"),
                String::from("/second/path/docker-compose.yml"),
            ],
            enviroment_file: None,
            use_project_name: None,
            status: None,
        };

        // Should return the first file's path
        assert_eq!(
            extract_path_from_cd_command(&item).unwrap(),
            "/first/path"
        );
    }

    #[test]
    fn test_extract_path_empty_compose_files_returns_error() {
        let item = ComposeItem {
            alias: String::from("test"),
            description: None,
            compose_files: vec![],
            enviroment_file: None,
            use_project_name: None,
            status: None,
        };

        let result = extract_path_from_cd_command(&item);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("No compose files configured"));
    }

    #[test]
    fn test_extract_path_relative_path() {
        let item = ComposeItem {
            alias: String::from("test"),
            description: None,
            compose_files: vec![String::from("relative/path/docker-compose.yml")],
            enviroment_file: None,
            use_project_name: None,
            status: None,
        };

        assert_eq!(
            extract_path_from_cd_command(&item).unwrap(),
            "relative/path"
        );
    }
}
