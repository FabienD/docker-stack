use crate::parser::config::{CliConfig, ComposeItem};
use clap::Command;
use eyre::Result;
use std::{path::Path};

pub fn check_config() -> Command {
    Command::new("check-config").about("Check configuration files existance")
}

pub fn exec_check_config(config: &mut dyn CliConfig) -> Result<()> {
    // Check docker bin path
    let config_docker_bin_path = config.get_container_bin_path()?;
    let mut has_error = false;
    
    if check_docker_bin_path(&config_docker_bin_path).expect("Docker bin path error") == false {
        println!("\nConfiguration :\n");
        println!(
            "❌ - Docker bin path: {}",
            config_docker_bin_path
        );
        has_error = true;
    }  

    // Check files in compose items
    let compose_items = config.get_all_compose_items();

    for item in compose_items {

        let compose_item_errors = check_item_config(&item).expect("Item error List");
        if compose_item_errors.len() > 0 {
            println!("\nProject : {:?} ", item.alias);
            for error in compose_item_errors {
                println!("{}", error);
            }
            has_error = true;
        }
    }

    if has_error == false {
        println!("✅ - No errors found");
    }

    Ok(())
}

pub fn check_item_config(item: &ComposeItem) -> Result<Vec<String>> {
    let mut error_list: Vec<String> = Vec::new();

    match &item.enviroment_file {
        Some(env_file) => {
            let file_path = Path::new(&env_file);
            if file_path.exists() == false {
                error_list.push(format!(
                    "❌ - env file: {:?}",
                    file_path
                ));
            }
        }
        None => {}
    }

    for file in &item.compose_files {
        let file_path = Path::new(&file);
        if file_path.exists() == false {
            error_list.push(format!(
                "❌ - Compose file: {:?}",
                file_path
            ));
        };
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
    pub fn it_returns_no_errors_when_the_item_compose_is_good() {
        let item = ComposeItem {
            alias: "test".to_string(),
            enviroment_file: None,
            compose_files: vec!["tests/docker-compose.test.yml".to_string()],
            description: Some("test".to_string()),
            status: Some(ComposeStatus::Running),
            use_project_name: Some(false),
        };
        
        
        assert!(true == (check_item_config(&item).expect("Error list").len() == 0));
    }

    #[test]
    pub fn it_returns_errors_when_the_item_compose_is_bad() {
        let item = ComposeItem {
            alias: "test".to_string(),
            enviroment_file: Some("tests/.env".to_string()), // This file does not exist
            compose_files: vec!["tests/docker-compose.yml".to_string()], // This file does not exist
            description: Some("test".to_string()),
            status: Some(ComposeStatus::Running),
            use_project_name: Some(false),
        };
        
        let error_list = check_item_config(&item).expect("Error list");

        assert!(true == (error_list.len() == 2)); // 2 errors
        assert!(true == error_list.contains(&"❌ - env file: \"tests/.env\"".to_string()));
        assert!(true == error_list.contains(&"❌ - Compose file: \"tests/docker-compose.yml\"".to_string()));
    }
}
