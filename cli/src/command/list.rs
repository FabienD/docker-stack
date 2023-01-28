use clap::{Command};
use eyre::Result;

use crate::parser::config::CliConfig;

pub fn list_projects() -> Command {
    Command::new("list")
        .about("List all projects")
}

pub fn exec_list_projects(config: &mut dyn CliConfig) -> Result<()> {
    // Compare with our Dctl config.
    let mut items = config.get_all_compose_items();

    for item in &mut items {
        println!("{:?}", item);
    }
    
    Ok(())
}