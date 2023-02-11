use clap::{Arg, ArgAction, ArgMatches, Command};
use eyre::Result;
use std::ffi::OsStr;

pub fn compose_logs() -> Command {
    Command::new("logs")
        .about("View logs output from all containers or from selected services of the project.")
        .arg(
            Arg::new("PROJECT")
                .help("The name of the docker-compose file alias")
                .required(true),
        )
        .arg(
            Arg::new("SERVICE")
                .help("The name of the service(s) to show logs for")
                .required(false)
                .num_args(0..20),
        )
        .arg(
            Arg::new("FOLLOW")
                .help("Follow log output")
                .long("follow")
                .short('f')
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("NO_COLOR")
                .help("Produce monochrome output")
                .long("no-color")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("NO_LOG_PREFIX")
                .help("Don't print prefix in logs")
                .long("no-log-prefix")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("SINCE")
                .help("Show logs since timestamp (e.g. 2013-01-02T13:23:37Z) or relative (e.g. 42m for 42 minutes)")
                .long("since")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("TAIL")
                .help("Number of lines to show from the end of the logs for each container")
                .long("tail")
        )
}

pub fn prepare_command_logs<'a>(
    args_matches: &'a ArgMatches,
) -> Result<Vec<&'a OsStr>> {
    let mut args: Vec<&OsStr> = vec![];

    args.push(OsStr::new("logs"));

    if args_matches.get_flag("FOLLOW") {
        args.push(OsStr::new("--follow"));
    }
    if args_matches.get_flag("NO_COLOR") {
        args.push(OsStr::new("--no-color"));
    }
    if args_matches.get_flag("NO_LOG_PREFIX") {
        args.push(OsStr::new("--no-log-prefix"));
    }
    if args_matches.get_flag("SINCE") {
        args.push(OsStr::new("--since"));
    }
    if let Some(tail) = args_matches.get_one::<String>("TAIL") {
        args.push(OsStr::new("--tail"));
        args.push(OsStr::new(tail));
    }

    if let Some(services) = args_matches.get_occurrences::<String>("SERVICE") {
        for service in services {
            for s in service {
                args.push(OsStr::new(s));
            }
        }
    }

    Ok(args)
}
