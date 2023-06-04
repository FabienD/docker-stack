use clap::{Arg, ArgMatches, Command, ArgAction};
use eyre::Result;
use std::ffi::OsStr;

pub fn compose_restart() -> Command {
    Command::new("restart")
        .about("Restart all containers for a project or only selected service(s) of the project.")
        .arg(
            Arg::new("PROJECT")
                .help("The name of the docker-compose file alias")
                .required(true),
        )
        .arg(
            Arg::new("SERVICE")
                .help("The name of the service(s) to restart")
                .num_args(0..10),
        )
        .arg(
            Arg::new("TIMEOUT")
                .help("Specify a shutdown timeout in seconds")
                .short('t')
                .long("timeout"),
        )
        .arg(
            Arg::new("DRY_RUN")
                .help("Execute command in dry run mode")
                .long("dry-run")
                .action(ArgAction::SetTrue)
        )
}

pub fn prepare_command_restart(args_matches: &ArgMatches) -> Result<Vec<&OsStr>> {
    let mut args: Vec<&OsStr> = vec![];

    args.push(OsStr::new("restart"));

    if let Some(timeout) = args_matches.get_one::<String>("TIMEOUT") {
        args.push(OsStr::new("--timeout"));
        args.push(OsStr::new(timeout));
    }
    if args_matches.get_flag("DRY_RUN") {
        args.push(OsStr::new("--dry-run"));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_a_complete_vec_of_osstr_for_command_restart() {
        let args_matches =
            compose_restart().get_matches_from(vec!["restart", "PROJECT", "service1", "service2"]);
        let args = prepare_command_restart(&args_matches).unwrap();
        assert_eq!(args, vec!["restart", "service1", "service2"]);
    }
}
