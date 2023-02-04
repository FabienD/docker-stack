use clap::{Arg, ArgAction, ArgMatches, Command};
use eyre::Result;
use std::ffi::OsStr;

pub fn compose_pause() -> Command {
    Command::new("pause")
        .about("Pause services")
        .arg(
            Arg::new("PROJECT")
                .help("The name of the docker-compose file alias")
                .required(true),
        )
        .arg(
            Arg::new("SERVICE")
                .help("The name of the service(s) to pause")
                .num_args(0..20)
                .action(ArgAction::Append),
        )
}

pub fn prepare_command_pause<'a>(
    args_matches: &'a ArgMatches,
    config_args: &'a mut Vec<&'a OsStr>,
) -> Result<Vec<&'a OsStr>> {
    let mut args: Vec<&OsStr> = vec![];

    args.append(config_args);
    args.push(OsStr::new("pause"));

    if let Some(services) = args_matches.get_occurrences::<String>("SERVICE") {
        for service in services {
            for s in service {
                args.push(OsStr::new(s));
            }
        }
    }

    Ok(args)
}
