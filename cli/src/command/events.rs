use clap::{Arg, Command, ArgAction, ArgMatches};
use std::ffi::OsStr;
use eyre::Result;

pub fn compose_events() -> Command {
    Command::new("events")
        .about("Receive real time events from containers.")
        .arg(
            Arg::new("PROJECT")
                .help("The name of the docker-compose file alias")
                .required(true),
        )
        .arg(
            Arg::new("SERVICE")
                .help("The name of the service(s) to listen for events")
                .num_args(0..20),
        )
        .arg(
            Arg::new("JSON")
                .help("Output events as a stream of json objects")
                .long("json")
                .action(ArgAction::SetTrue)
        )
}

pub fn prepare_command_events<'a>(
    args_matches: &'a ArgMatches,
    config_args: &'a mut Vec<&'a OsStr>,
) -> Result<Vec<&'a OsStr>> {
    let mut args: Vec<&OsStr> = vec![];

    args.append(config_args);
    args.push(OsStr::new("events"));

    if args_matches.get_flag("JSON") {
        args.push(OsStr::new("--json"));
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