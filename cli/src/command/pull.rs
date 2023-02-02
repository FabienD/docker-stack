use clap::{Arg, Command, ArgAction, ArgMatches};
use std::ffi::OsStr;
use eyre::Result;

pub fn compose_pull() -> Command {
    Command::new("pull")
        .about("Pull service images")
        .arg(
            Arg::new("PROJECT")
                .help("The name of the docker-compose file alias")
                .required(true),
        )
        .arg(
            Arg::new("SERVICE")
                .help("The name of the service(s) to pull")
                .num_args(0..20)
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("IGNORE_BUILDABLE")
                .help("Ignore images that can be built")
                .long("ignore-buildable")
                .action(ArgAction::SetTrue),
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

pub fn prepare_command_pull<'a>(
    args_matches: &'a ArgMatches,
    config_args: &'a mut Vec<&'a OsStr>,
) -> Result<Vec<&'a OsStr>> {
    let mut args: Vec<&OsStr> = vec![];

    args.append(config_args);
    args.push(OsStr::new("pull"));

    if args_matches.get_flag("IGNORE_BUILDABLE") {
        args.push(OsStr::new("--ignore-buildable"));
    }
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