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
        .arg(
            Arg::new("DRY_RUN")
                .help("Execute command in dry run mode")
                .long("dry-run")
                .action(ArgAction::SetTrue)
        )
}

pub fn prepare_command_logs(args_matches: &ArgMatches) -> Result<Vec<&OsStr>> {
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
    fn it_returns_a_complete_vec_of_osstr_for_command_logs() {
        let args_matches = compose_logs().get_matches_from(vec![
            "logs",
            "--follow",
            "--no-color",
            "--no-log-prefix",
            "--since",
            "--tail",
            "5",
            "PROJECT",
            "service1",
        ]);

        let args = prepare_command_logs(&args_matches).unwrap();

        assert_eq!(
            args,
            vec![
                OsStr::new("logs"),
                OsStr::new("--follow"),
                OsStr::new("--no-color"),
                OsStr::new("--no-log-prefix"),
                OsStr::new("--since"),
                OsStr::new("--tail"),
                OsStr::new("5"),
                OsStr::new("service1"),
            ]
        );
    }
}
