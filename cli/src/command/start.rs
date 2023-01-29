use clap::{Arg, Command, ArgAction, ArgMatches};
use std::ffi::OsStr;
use eyre::Result;

pub fn compose_start() -> Command {
    Command::new("start")
        .about("Start all containers for a project or only selected service(s) of the project.")
        .arg(
            Arg::new("PROJECT")
                .help("The name of the docker-compose file alias")
                .required(true),
        )
        .arg(
            Arg::new("SERVICE")
                .help("The name of the service(s) to start")
                .num_args(0..20),
        )
}

pub fn prepare_command_start<'a>(
    args_matches: &'a ArgMatches, 
    config_args: &'a mut Vec<&'a OsStr>
) -> Result<Vec<&'a OsStr>> {
    let mut args: Vec<&OsStr> = vec![];

    Ok(args)
}