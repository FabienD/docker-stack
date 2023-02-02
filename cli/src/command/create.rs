use clap::{Arg, Command, ArgAction, ArgMatches};
use std::ffi::OsStr;
use eyre::Result;

pub fn compose_create() -> Command {
    Command::new("create")
        .about("Creates containers for a service of the project.")
        .arg(
            Arg::new("PROJECT")
                .help("The name of the docker-compose file alias")
                .required(true),
        )
        .arg(
            Arg::new("SERVICE")
                .help("The name of the service(s) to create")
                .num_args(0..20)
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("BUILD")
                .help("Build images before starting containers")
                .long("build")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("FORCE_RECREATE")
                .help("Recreate containers even if their configuration and image haven't changed")
                .long("force-recreate")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("NO_RECREATE")
                .help("If containers already exist, don't recreate them. Incompatible with --force-recreate")
                .long("no-recreate")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("PULL")
                .help("Pull images before starting containers")
                .long("pull")
                .default_value("missing")
                .value_parser(["missing", "always", "never"])
        )
}

pub fn prepare_command_create<'a>(
    args_matches: &'a ArgMatches,
    config_args: &'a mut Vec<&'a OsStr>,
) -> Result<Vec<&'a OsStr>> {
    let mut args: Vec<&OsStr> = vec![];

    args.append(config_args);
    args.push(OsStr::new("create"));

    if args_matches.get_flag("BUILD") {
        args.push(OsStr::new("--build"));
    }
    if args_matches.get_flag("FORCE_RECREATE") {
        args.push(OsStr::new("--force-recreate"));
    }
    if args_matches.get_flag("NO_RECREATE") {
        args.push(OsStr::new("--no-recreate"));
    }
    if let Some(pull) = args_matches.get_one::<String>("PULL") {
        args.push(OsStr::new("--pull"));
        args.push(OsStr::new(pull));
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