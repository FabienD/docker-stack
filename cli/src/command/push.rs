use clap::{Arg, ArgAction, ArgMatches, Command};
use eyre::Result;
use std::ffi::OsStr;

pub fn compose_push() -> Command {
    Command::new("push")
        .about("Push services")
        .arg(
            Arg::new("PROJECT")
                .help("The name of the docker-compose file alias")
                .required(true),
        )
        .arg(
            Arg::new("SERVICE")
                .help("The name of the service(s) to push")
                .num_args(0..20)
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("IGNORE_PUSH_FAILURES")
                .help("Push what it can and ignores images with push failures")
                .long("ignore-push-failures")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("INCLUDE_DEPENDENCIES")
                .help("Also push images of services declared as dependencies")
                .long("include-deps")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("QUIET")
                .help("Push without printing progress information")
                .long("quiet")
                .short('q')
                .action(ArgAction::SetTrue),
        )
}

pub fn prepare_command_push<'a>(
    args_matches: &'a ArgMatches,
) -> Result<Vec<&'a OsStr>> {
    let mut args: Vec<&OsStr> = vec![];

    args.push(OsStr::new("push"));

    if args_matches.get_flag("IGNORE_PUSH_FAILURES") {
        args.push(OsStr::new("--ignore-push-failures"));
    }
    if args_matches.get_flag("INCLUDE_DEPENDENCIES") {
        args.push(OsStr::new("--include-deps"));
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
