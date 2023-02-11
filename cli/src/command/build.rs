use clap::{Arg, ArgAction, ArgMatches, Command};
use eyre::Result;
use std::ffi::OsStr;

pub fn compose_build() -> Command {
    Command::new("build")
        .about("Build all or selected service(s) for a project.")
        .arg(
            Arg::new("PROJECT")
                .help("The name of the docker-compose file alias")
                .required(true),
        )
        .arg(
            Arg::new("SERVICE")
                .help("The name of the service(s) to build")
                .num_args(0..20)
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("BUILD_ARG")
                .help("Set build-time variables for services")
                .long("build-arg")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("MEMORY")
                .help("Set memory limit for the build container. Not supported on buildkit yet")
                .long("memory")
                .short('m')
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("NO_CACHE")
                .help("Do not use cache when building the image")
                .long("no-cache")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("PROGRESS")
                .help("Only display IDs")
                .long("progress")
                .value_parser(["auto", "tty", "plain", "quiet"])
        )
        .arg(
            Arg::new("PULL")
                .help("Always attempt to pull a newer version of the image")
                .long("pull")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("QUIET")
                .help("Don't print anything to STDOUT")
                .long("quiet")
                .short('q')
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("SSH")
                .help("Set SSH authentications used when building service images. (use 'default' for using your default SSH Agent)")
                .long("ssh")
                .action(ArgAction::SetTrue)
        )
}

pub fn prepare_command_build<'a>(
    args_matches: &'a ArgMatches,
) -> Result<Vec<&'a OsStr>> {
    let mut args: Vec<&OsStr> = vec![];

    args.push(OsStr::new("build"));

    if args_matches.get_flag("BUILD_ARG") {
        args.push(OsStr::new("--build-arg"));
    }
    if args_matches.get_flag("MEMORY") {
        args.push(OsStr::new("--memory"));
    }
    if args_matches.get_flag("NO_CACHE") {
        args.push(OsStr::new("--no-cache"));
    }
    if let Some(progress) = args_matches.get_one::<String>("PROGRESS") {
        args.push(OsStr::new("--progress"));
        args.push(OsStr::new(progress));
    }
    if args_matches.get_flag("PULL") {
        args.push(OsStr::new("--pull"));
    }
    if args_matches.get_flag("QUIET") {
        args.push(OsStr::new("--quiet"));
    }
    if args_matches.get_flag("SSH") {
        args.push(OsStr::new("--ssh"));
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
