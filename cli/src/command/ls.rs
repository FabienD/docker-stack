use clap::{Arg, ArgAction, ArgMatches, Command};
use eyre::Result;
use std::ffi::OsStr;

pub fn compose_ls() -> Command {
    Command::new("ls")
        .about("List running compose projects")
        .arg(
            Arg::new("PROJECT")
                .help("The name of the docker-compose file alias")
                .required(true),
        )
        .arg(
            Arg::new("ALL")
                .help("Show all stopped Compose projects")
                .short('a')
                .long("all")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("FILTER")
                .help("Filter output based on conditions provided")
                .long("filter")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("FORMAT")
                .help("Pretty-print services using a Go template")
                .short('f')
                .long("format")
                .value_parser(["table", "json"])
                .default_value("table"),
        )
        .arg(
            Arg::new("QUIET")
                .help("Only display IDs")
                .short('q')
                .long("quiet")
                .action(ArgAction::SetTrue),
        )
}

pub fn prepare_command_ls<'a>(
    args_matches: &'a ArgMatches,
    config_args: &'a mut Vec<&'a OsStr>,
) -> Result<Vec<&'a OsStr>> {
    let mut args: Vec<&OsStr> = vec![];

    if args_matches.get_flag("ALL") {
        args.push(OsStr::new("--all"));
    }
    if args_matches.get_flag("FILTER") {
        args.push(OsStr::new("--filter"));
    }
    if let Some(format) = args_matches.get_one::<String>("FORMAT") {
        args.push(OsStr::new("--format"));
        args.push(OsStr::new(format));
    }
    if args_matches.get_flag("QUIET") {
        args.push(OsStr::new("--quiet"));
    }

    args.append(config_args);
    args.push(OsStr::new("ls"));

    Ok(args)
}
