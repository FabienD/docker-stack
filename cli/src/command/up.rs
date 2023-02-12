use clap::{Arg, ArgAction, ArgMatches, Command};
use eyre::Result;
use std::ffi::OsStr;

pub fn compose_up() -> Command {
    Command::new("up")
        .about("Create and start containers for a project.")
        .arg(
            Arg::new("PROJECT")
                .help("The name of the docker-compose file alias")
                .required(true),
        )
        .arg(
            Arg::new("ABORT_ON_CONTAINER_EXIT")
                .help("Stops all containers if any container was stopped. Incompatible with -d")
                .long("abort-on-container-exit")
                .action(ArgAction::SetTrue)
                .conflicts_with("DETACH")
        )
        .arg(
            Arg::new("ALWAYS_RECREATE_DEPS")
                .help("Recreate dependent containers. Incompatible with --no-recreate")
                .long("always-recreate-deps")
                .action(ArgAction::SetTrue)
                .conflicts_with("NO_RECREATE")
        )
        .arg(
            Arg::new("ATTACH")
                .help("Attach to service output")
                .long("attach")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("ATTACH_DEPENDENCIES")
                .help("Attach to dependent containers")
                .long("attach-dependencies")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("BUILD")
                .help("Build images before starting containers")
                .long("build")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("DETACH")
                .help("Detached mode: Run containers in the background")
                .long("detach")
                .short('d')
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("EXIT_CODE_FROM")
                .help("Return the exit code of the selected service container. Implies --abort-on-container-exit")
                .long("exit-code-from")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("FORCE_RECREATE")
                .help("Recreate containers even if their configuration and image haven't changed")
                .long("force-recreate")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("NO_ATTTACH")
                .help("Don't attach to specified service")
                .long("no-attach")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("NO_BUILD")
                .help("Don't build an image, even if it's missing")
                .long("no-build")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("NO_COLOR")
                .help("Produce monochrome output")
                .long("no-color")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("NO_DEPS")
                .help("Don't start linked services")
                .long("no-deps")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("NO_LOG_PREFIX")
                .help("Don't print prefix in logs")
                .long("no-log-prefix")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("NO_RECREATE")
                .help("If containers already exist, don't recreate them. Incompatible with --force-recreate and --always-recreate-deps")
                .long("no-recreate")
                .action(ArgAction::SetTrue)
                .conflicts_with("FORCE_RECREATE")
                .conflicts_with("ALWAYS_RECREATE_DEPS")
        )
        .arg(
            Arg::new("NO_START")
                .help("Don't start the services after creating them")
                .long("no-start")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("PULL")
                .help("Pull image before running")
                .long("pull")
                .value_parser(["always", "missing", "never"])
        )
        .arg(
            Arg::new("QUIET_PULL")
                .help("Pull without printing progress information")
                .long("quiet-pull")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("REMOVE_ORPHANS")
                .help("Remove containers for services not defined in the Compose file")
                .long("remove-orphans")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("RENEW_ANON_VOLUMES")
                .help("Recreate anonymous volumes instead of retrieving data from the previous containers")
                .long("renew-anon-volumes")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("SCALE")
                .help("Scale SERVICE to NUM instances. Overrides the scale setting in the Compose file if present")
                .long("scale")
                .value_names(["SERVICE", "NUM"])
        )
        .arg(
            Arg::new("TIMEOUT")
                .help("Use this timeout in seconds for container shutdown when attached or when containers are already running")
                .long("timeout")
                .short('t')
        )
        .arg(
            Arg::new("TIMESTAMPS")
                .help("Show timestamps")
                .long("timestamps")
        )
        .arg(
            Arg::new("WAIT")
                .help("Wait for services to be running|healthy. Implies detached mode")
                .long("wait")
                .action(ArgAction::SetTrue)
        )
}

pub fn prepare_command_up<'a>(args_matches: &'a ArgMatches) -> Result<Vec<&'a OsStr>> {
    let mut args: Vec<&OsStr> = vec![];

    args.push(OsStr::new("up"));

    if args_matches.get_flag("ABORT_ON_CONTAINER_EXIT") {
        args.push(OsStr::new("--abort-on-container-exit"));
    }
    if args_matches.get_flag("ALWAYS_RECREATE_DEPS") {
        args.push(OsStr::new("--always-recreate-deps"));
    }
    if args_matches.get_flag("ATTACH") {
        args.push(OsStr::new("--attach"));
    }
    if args_matches.get_flag("ATTACH_DEPENDENCIES") {
        args.push(OsStr::new("--attach-dependencies"));
    }
    if args_matches.get_flag("BUILD") {
        args.push(OsStr::new("--build"));
    }
    if args_matches.get_flag("DETACH") {
        args.push(OsStr::new("--detach"));
    }
    if args_matches.get_flag("EXIT_CODE_FROM") {
        args.push(OsStr::new("--exit-code-from"));
    }
    if args_matches.get_flag("FORCE_RECREATE") {
        args.push(OsStr::new("--force-recreate"));
    }
    if args_matches.get_flag("NO_ATTTACH") {
        args.push(OsStr::new("--no-attach"));
    }
    if args_matches.get_flag("NO_BUILD") {
        args.push(OsStr::new("--no-build"));
    }
    if args_matches.get_flag("NO_COLOR") {
        args.push(OsStr::new("--no-color"));
    }
    if *args_matches.get_one::<bool>("NO_DEPS").unwrap() {
        args.push(OsStr::new("--no-deps"));
    }
    if *args_matches.get_one::<bool>("NO_LOG_PREFIX").unwrap() {
        args.push(OsStr::new("--no-log-prefix"));
    }
    if *args_matches.get_one::<bool>("NO_RECREATE").unwrap() {
        args.push(OsStr::new("--no-recreate"));
    }
    if *args_matches.get_one::<bool>("NO_START").unwrap() {
        args.push(OsStr::new("--no-start"));
    }
    if let Some(pull) = args_matches.get_one::<String>("PULL") {
        args.push(OsStr::new("--pull"));
        args.push(OsStr::new(pull));
    }
    if *args_matches.get_one::<bool>("QUIET_PULL").unwrap() {
        args.push(OsStr::new("--quiet-pull"));
    }
    if *args_matches.get_one::<bool>("REMOVE_ORPHANS").unwrap() {
        args.push(OsStr::new("--remove-orphans"));
    }
    if *args_matches.get_one::<bool>("RENEW_ANON_VOLUMES").unwrap() {
        args.push(OsStr::new("--renew-anon-volumes"));
    }
    if let Some(scale) = args_matches.get_occurrences::<String>("SCALE") {
        args.push(OsStr::new("--scale"));
        scale.into_iter().for_each(|s| {
            s.into_iter().for_each(|s| {
                args.push(OsStr::new(s));
            });
        });
    }
    if let Some(timeout) = args_matches.get_one::<String>("TIMEOUT") {
        args.push(OsStr::new("--timeout"));
        args.push(OsStr::new(timeout));
    }
    if let Some(timestamps) = args_matches.get_one::<String>("TIMESTAMPS") {
        args.push(OsStr::new("--timestamps"));
        args.push(OsStr::new(timestamps));
    }
    if *args_matches.get_one::<bool>("WAIT").unwrap() {
        args.push(OsStr::new("--wait"));
    }

    Ok(args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_a_complete_vec_of_osstr_for_command_up() {
        let args_matches = compose_up().get_matches_from(vec![
            "up",
            "--always-recreate-deps",
            "--attach",
            "--attach-dependencies",
            "--build",
            "--detach",
            "--exit-code-from",
            "--force-recreate",
            "--no-attach",
            "--no-build",
            "--no-color",
            "--no-deps",
            "--no-log-prefix",
            "--no-start",
            "--pull",
            "always",
            "--quiet-pull",
            "--remove-orphans",
            "--renew-anon-volumes",
            "--scale",
            "service1",
            "2",
            "--timeout",
            "10",
            "--timestamps",
            "6540",
            "--wait",
            "PROJECT_NAME",
        ]);
        let args = prepare_command_up(&args_matches).unwrap();

        assert_eq!(
            args,
            vec![
                OsStr::new("up"),
                OsStr::new("--always-recreate-deps"),
                OsStr::new("--attach"),
                OsStr::new("--attach-dependencies"),
                OsStr::new("--build"),
                OsStr::new("--detach"),
                OsStr::new("--exit-code-from"),
                OsStr::new("--force-recreate"),
                OsStr::new("--no-attach"),
                OsStr::new("--no-build"),
                OsStr::new("--no-color"),
                OsStr::new("--no-deps"),
                OsStr::new("--no-log-prefix"),
                OsStr::new("--no-start"),
                OsStr::new("--pull"),
                OsStr::new("always"),
                OsStr::new("--quiet-pull"),
                OsStr::new("--remove-orphans"),
                OsStr::new("--renew-anon-volumes"),
                OsStr::new("--scale"),
                OsStr::new("service1"),
                OsStr::new("2"),
                OsStr::new("--timeout"),
                OsStr::new("10"),
                OsStr::new("--timestamps"),
                OsStr::new("6540"),
                OsStr::new("--wait"),
            ]
        );
    }
}
