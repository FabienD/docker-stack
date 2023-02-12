use clap::{Arg, ArgMatches, Command};
use eyre::Result;
use std::ffi::OsStr;

pub fn compose_start() -> Command {
    Command::new("start")
        .about("Start all containers for a project or only selected service(s) of the project")
        .arg(
            Arg::new("PROJECT")
                .help("The name of the docker-compose file alias")
                .required(true),
        )
        .arg(
            Arg::new("SERVICE")
                .help("The name of the service(s) to start")
                .num_args(0..20),
        )
}

pub fn prepare_command_start(args_matches: &ArgMatches) -> Result<Vec<&OsStr>> {
    let mut args: Vec<&OsStr> = vec![];

    args.push(OsStr::new("start"));

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
    fn it_returns_a_complete_vec_of_osstr_for_command_start() {
        let args_matches =
            compose_start().get_matches_from(vec!["start", "PROJECT", "service1", "service2"]);
        let args = prepare_command_start(&args_matches).unwrap();
        assert_eq!(args, vec!["start", "service1", "service2"]);
    }
}
