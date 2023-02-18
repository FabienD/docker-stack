use std::{ffi::OsStr, path::Path};

use clap::{Arg, Command};
use eyre::Result;

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
    println!("{}", extract_path_from_cd_command(&compose_item)?);
    Ok(())
}

fn extract_path_from_cd_command(compose_item: &ComposeItem) -> Result<String> {
    let path = Path::new(OsStr::new(&compose_item.compose_files[0]))
        .parent()
        .unwrap();

    let path_str = path.to_str().unwrap();
    Ok(path_str.to_string())
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

        assert!(extract_path_from_cd_command(&item).unwrap() == "/home/test/test");
    }
}
