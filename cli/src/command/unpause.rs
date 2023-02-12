use clap::{Arg, ArgAction, ArgMatches, Command};
use eyre::Result;
use std::ffi::OsStr;

pub fn compose_unpause() -> Command {
    Command::new("unpause")
        .about("Unpause services")
        .arg(
            Arg::new("PROJECT")
                .help("The name of the docker-compose file alias")
                .required(true),
        )
        .arg(
            Arg::new("SERVICE")
                .help("The name of the service(s) to unpause")
                .num_args(0..20)
                .action(ArgAction::Append),
        )
}

pub fn prepare_command_unpause(args_matches: &ArgMatches) -> Result<Vec<&OsStr>> {
    let mut args: Vec<&OsStr> = vec![];

    args.push(OsStr::new("unpause"));

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
    fn it_returns_a_complete_vec_of_osstr_for_command_unpause() {
        let args_matches =
            compose_unpause().get_matches_from(vec!["unpause", "PROJECT", "service1", "service2"]);
        let args = prepare_command_unpause(&args_matches).unwrap();
        assert_eq!(args, vec!["unpause", "service1", "service2"]);
    }
}
