use clap::{Arg, Command, ArgAction, ArgMatches};
use std::ffi::OsStr;
use eyre::Result;

pub fn compose_down() -> Command {
    Command::new("down")
        .about("Stop and remove containers, networks, images, and volumes for a project.")
        .arg(
            Arg::new("PROJECT")
                .help("The name of the docker-compose file alias")
                .required(true),
        )
        .arg(
            Arg::new("REMOVE_ORPHANS")
                .help("Remove containers for services not defined in the Compose file.")
                .long("remove-orphans")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("RMI")
                .help("Remove images used by services. \"local\" remove only images that don't have a custom tag.")
                .long("rmi")
                .value_parser(["local", "all"])
        )
        .arg(
            Arg::new("TIMEOUT")
                .help("Specify a shutdown timeout in seconds")
                .short('t')
                .long("timeout")
        )
        .arg(
            Arg::new("VOLUMES")
                .help("Remove named volumes declared in the volumes section of the Compose file and anonymous volumes attached to containers.")
                .short('v')
                .long("volumes")
        )
}

pub fn prepare_command_down<'a>(
    args_matches: &'a ArgMatches, 
    config_args: &'a mut Vec<&'a OsStr>
) -> Result<Vec<&'a OsStr>> {
    let mut args: Vec<&OsStr> = vec![];

    Ok(args)
}