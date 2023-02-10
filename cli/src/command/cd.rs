use clap::{Arg, Command};
use eyre::Result;

use crate::{parser::config::ComposeItem, utils::system::System};

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
    let system = System::init();
    println!("{}", system.cd(compose_item).unwrap());
    Ok(())
}
