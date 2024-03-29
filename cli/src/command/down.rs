use clap::{Arg, ArgAction, ArgMatches, Command};
use eyre::Result;
use std::ffi::OsStr;

pub fn compose_down() -> Command {
    Command::new("down")
        .about("Stop and remove containers, networks, images, and volumes for a project.")
        .arg(
            Arg::new("PROJECT")
                .help("The name of the docker-compose file alias")
                .required(true),
        )
        .arg(
            Arg::new("REMOVE_ORPHANS")
                .help("Remove containers for services not defined in the Compose file")
                .long("remove-orphans")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("RMI")
                .help("Remove images used by services. \"local\" remove only images that don't have a custom tag")
                .long("rmi")
                .value_parser(["local", "all"])
        )
        .arg(
            Arg::new("TIMEOUT")
                .help("Specify a shutdown timeout in seconds")
                .short('t')
                .long("timeout")
        )
        .arg(
            Arg::new("VOLUMES")
                .help("Remove named volumes declared in the volumes section of the Compose file and anonymous volumes attached to containers")
                .short('v')
                .long("volumes")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("DRY_RUN")
                .help("Execute command in dry run mode")
                .long("dry-run")
                .action(ArgAction::SetTrue)
        )
}

pub fn prepare_command_down(args_matches: &ArgMatches) -> Result<Vec<&OsStr>> {
    let mut args: Vec<&OsStr> = vec![];

    args.push(OsStr::new("down"));

    if args_matches.get_flag("REMOVE_ORPHANS") {
        args.push(OsStr::new("--remove-orphans"));
    }
    if let Some(rmi) = args_matches.get_one::<String>("RMI") {
        args.push(OsStr::new("--rmi"));
        args.push(OsStr::new(rmi));
    }
    if let Some(timeout) = args_matches.get_one::<String>("TIMEOUT") {
        args.push(OsStr::new("--timeout"));
        args.push(OsStr::new(timeout));
    }
    if args_matches.get_flag("VOLUMES") {
        args.push(OsStr::new("--volumes"));
    }
    if args_matches.get_flag("DRY_RUN") {
        args.push(OsStr::new("--dry-run"));
    }
    Ok(args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_a_complete_vec_of_osstr_for_command_down() {
        let args_matches = compose_down().get_matches_from(vec![
            "down",
            "test",
            "--remove-orphans",
            "--rmi",
            "local",
            "--timeout",
            "10",
            "--volumes",
        ]);
        let args = prepare_command_down(&args_matches).unwrap();

        assert_eq!(
            args,
            vec![
                OsStr::new("down"),
                OsStr::new("--remove-orphans"),
                OsStr::new("--rmi"),
                OsStr::new("local"),
                OsStr::new("--timeout"),
                OsStr::new("10"),
                OsStr::new("--volumes"),
            ]
        );
    }
}
