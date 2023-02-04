use clap::{Arg, ArgAction, ArgMatches, Command};
use eyre::Result;
use std::ffi::OsStr;

pub fn compose_kill() -> Command {
    Command::new("kill")
        .about("Kill containers")
        .arg(
            Arg::new("PROJECT")
                .help("The name of the docker-compose file alias")
                .required(true),
        )
        .arg(
            Arg::new("SERVICE")
                .help("The name of the service(s) to kill")
                .num_args(0..20),
        )
        .arg(
            Arg::new("REMOVE_ORPHANS")
                .help("Remove containers for services not defined in the Compose file")
                .long("remove-orphans")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("signal")
                .short('s')
                .long("signal")
                .help("SIGNAL to send to the container")
                .default_value("SIGKILL")
                .value_parser(["SIGKILL", "SIGTERM", "SIGINT"]),
        )
}

pub fn prepare_command_kill<'a>(
    args_matches: &'a ArgMatches,
    config_args: &'a mut Vec<&'a OsStr>,
) -> Result<Vec<&'a OsStr>> {
    let mut args: Vec<&OsStr> = vec![];

    args.append(config_args);
    args.push(OsStr::new("kill"));

    if args_matches.get_flag("REMOVE_ORPHANS") {
        args.push(OsStr::new("--remove-orphans"));
    }
    if let Some(signal) = args_matches.get_one::<String>("signal") {
        args.push(OsStr::new("--signal"));
        args.push(OsStr::new(signal));
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
