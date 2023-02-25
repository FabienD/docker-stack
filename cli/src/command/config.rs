use crate::parser::config::CliConfig;
use clap::Command;
use eyre::Result;
use std::path::Path;

pub fn check_config() -> Command {
    Command::new("check-config").about("Check configuration files existance")
}

pub fn exec_check_config(config: &mut dyn CliConfig) -> Result<()> {
    // Check docker bin path
    let config_docker_bin_path = config.get_container_bin_path()?;
    let docker_bin_path = Path::new(&config_docker_bin_path);

    println!("\nConfiguration :\n");
    println!(
        "{} - Docker bin path:  {:?}",
        return_check_step(docker_bin_path.exists()),
        docker_bin_path
    );

    // Check files in compose items
    let compose_items = config.get_all_compose_items();

    for item in compose_items {
        println!("\nProject : {:?}\n", item.alias);

        match &item.enviroment_file {
            Some(env_file) => {
                let file_path = Path::new(&env_file);
                println!(
                    "{} - env file: {:?}",
                    return_check_step(file_path.exists()),
                    file_path
                );
            }
            None => {}
        }

        for file in item.compose_files {
            let file_path = Path::new(&file);
            println!(
                "{} - Compose file: {:?}",
                return_check_step(file_path.exists()),
                file_path
            );
        }
    }

    Ok(())
}

fn return_check_step(condition: bool) -> char {
    if condition {
        '✅'
    } else {
        '❌'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn it_returns_the_check_step() {
        assert!(return_check_step(true) == '✅');
        assert!(return_check_step(false) == '❌');
    }
}
