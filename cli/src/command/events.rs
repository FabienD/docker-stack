use clap::{Arg, ArgAction, ArgMatches, Command};
use eyre::Result;
use std::ffi::OsStr;

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
                .action(ArgAction::SetTrue),
        )
}

pub fn prepare_command_events<'a>(
    args_matches: &'a ArgMatches,
) -> Result<Vec<&'a OsStr>> {
    let mut args: Vec<&OsStr> = vec![];

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
