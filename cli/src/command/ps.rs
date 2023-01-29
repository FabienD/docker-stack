use clap::{Arg, Command, ArgAction, ArgMatches};
use std::ffi::OsStr;
use eyre::Result;

pub fn compose_ps() -> Command {
    Command::new("ps")
        .about("List containers for a project or only selected service(s) of the project.")
        .arg(
            Arg::new("PROJECT")
                .help("The name of the docker-compose file alias")
                .required(true),
        )
        .arg(
            Arg::new("ALL")
                .help("Show all stopped containers (including those created by the run command)")
                .long("all")
                .short('a')
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("FILTER")
                .help("Filter services by a property (supported filters: status).")
                .long("filter"),
        )
        .arg(
            Arg::new("FORMAT")
                .help("format the output.")
                .long("format")
                .value_parser(["table", "json"]),
        )
        .arg(
            Arg::new("QUIET")
                .help("Only display IDs")
                .long("quiet")
                .short('q')
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("SERVICES")
                .help("Display services")
                .long("services")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("STATUS")
                .help("Filter services by status.")
                .long("status")
                .value_parser(["paused", "restarting", "removing", "running", "dead", "created", "exited"]),
        )
}

pub fn prepare_command_ps<'a>(
    args_matches: &'a ArgMatches, 
    config_args: &'a mut Vec<&'a OsStr>
) -> Result<Vec<&'a OsStr>> {
    let mut args: Vec<&OsStr> = vec![];

    Ok(args)
}