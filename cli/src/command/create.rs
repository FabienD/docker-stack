use clap::{Arg, ArgAction, ArgMatches, Command};
use eyre::Result;
use std::ffi::OsStr;

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
                .value_parser(["missing", "always", "never"])
        )
}

pub fn prepare_command_create(args_matches: &ArgMatches) -> Result<Vec<&OsStr>> {
    let mut args: Vec<&OsStr> = vec![];

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_a_complete_vec_of_osstr_for_compose_create() {
        let args_matches = compose_create().get_matches_from(vec![
            "create",
            "--build",
            "--force-recreate",
            "--no-recreate",
            "--pull",
            "missing",
            "PROJECT",
            "service1",
            "service2",
        ]);
        let args = prepare_command_create(&args_matches).unwrap();
        assert_eq!(
            args,
            vec![
                OsStr::new("create"),
                OsStr::new("--build"),
                OsStr::new("--force-recreate"),
                OsStr::new("--no-recreate"),
                OsStr::new("--pull"),
                OsStr::new("missing"),
                OsStr::new("service1"),
                OsStr::new("service2"),
            ]
        );
    }
}
