use clap::{Arg, ArgAction, ArgMatches, Command};
use eyre::Result;
use std::ffi::OsStr;

pub fn compose_watch() -> Command {
    Command::new("watch")
    .about("Watch build context for service and rebuild/refresh containers when files are updated for the selected project")
    .arg(
        Arg::new("PROJECT")
            .help("The name of the docker-compose file alias")
            .required(true),
    )
    .arg(
        Arg::new("SERVICE")
            .help("The name of the service(s) to watch")
            .required(false)
            .num_args(0..20),
    )
    .arg(
        Arg::new("NO_UP")
            .help("Do not build & start services before watching")
            .long("no-up")
            .action(ArgAction::SetTrue),
    )
    .arg(
        Arg::new("QUIET")
            .help("hide build output")
            .long("quiet")
            .action(ArgAction::SetTrue),
    )
    .arg(
        Arg::new("DRY_RUN")
            .help("Execute command in dry run mode")
            .long("dry-run")
            .action(ArgAction::SetTrue)
    )
}

pub fn prepare_command_watch(args_matches: &ArgMatches) -> Result<Vec<&OsStr>> {
    let mut args: Vec<&OsStr> = vec![];

    args.push(OsStr::new("watch"));

    if args_matches.get_flag("NO_UP") {
        args.push(OsStr::new("--no-up"));
    }
    if args_matches.get_flag("QUIET") {
        args.push(OsStr::new("--quiet"));
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
    fn it_returns_a_complete_vec_of_osstr_for_command_watch() {
        let args_matches = compose_watch().get_matches_from(vec![
            "watch",
            "--no-up",
            "--quiet",
            "PROJECT_NAME",
        ]);
        let args = prepare_command_watch(&args_matches).unwrap();

        assert_eq!(
            args,
            vec![
                OsStr::new("watch"),
                OsStr::new("--no-up"),
                OsStr::new("--quiet"),
            ]
        );
    }
}
