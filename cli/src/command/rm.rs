use clap::{Arg, ArgAction, ArgMatches, Command};
use eyre::Result;
use std::ffi::OsStr;

pub fn compose_rm() -> Command {
    Command::new("rm")
        .about("Removes stopped service containers")
        .arg(
            Arg::new("PROJECT")
                .help("The name of the docker-compose file alias")
                .required(true),
        )
        .arg(
            Arg::new("SERVICE")
                .help("The name of the service(s) to remove")
                .num_args(0..20)
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("force")
                .short('f')
                .long("force")
                .help("Don't ask to confirm removal")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("stop")
                .short('s')
                .long("stop")
                .help("Stop the containers, if required, before removing")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("volumes")
                .short('v')
                .long("volumes")
                .help("Remove any anonymous volumes attached to containers")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("DRY_RUN")
                .help("Execute command in dry run mode")
                .long("dry-run")
                .action(ArgAction::SetTrue)
        )
}

pub fn prepare_command_rm(args_matches: &ArgMatches) -> Result<Vec<&OsStr>> {
    let mut args: Vec<&OsStr> = vec![];

    args.push(OsStr::new("rm"));

    if args_matches.get_flag("force") {
        args.push(OsStr::new("--force"));
    }
    if args_matches.get_flag("stop") {
        args.push(OsStr::new("--stop"));
    }
    if args_matches.get_flag("volumes") {
        args.push(OsStr::new("--volumes"));
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
    fn it_returns_a_complete_vec_of_osstr_for_command_rm() {
        let args_matches = compose_rm().get_matches_from(vec![
            "rm",
            "--force",
            "--stop",
            "--volumes",
            "PROJECT",
            "service1",
            "service2",
        ]);
        let args = prepare_command_rm(&args_matches).unwrap();
        assert_eq!(
            args,
            vec![
                "rm",
                "--force",
                "--stop",
                "--volumes",
                "service1",
                "service2"
            ]
        );
    }
}
