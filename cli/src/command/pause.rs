use clap::{Arg, ArgAction, ArgMatches, Command};
use eyre::Result;
use std::ffi::OsStr;

pub fn compose_pause() -> Command {
    Command::new("pause")
        .about("Pause services")
        .arg(
            Arg::new("PROJECT")
                .help("The name of the docker-compose file alias")
                .required(true),
        )
        .arg(
            Arg::new("SERVICE")
                .help("The name of the service(s) to pause")
                .num_args(0..20)
                .action(ArgAction::Append),
        )
}

pub fn prepare_command_pause<'a>(args_matches: &'a ArgMatches) -> Result<Vec<&'a OsStr>> {
    let mut args: Vec<&OsStr> = vec![];

    args.push(OsStr::new("pause"));

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
    fn it_returns_a_complete_vec_of_osstr_for_command_pause() {
        let args_matches =
            compose_pause().get_matches_from(vec!["pause", "PROJECT", "service1", "service2"]);
        let args = prepare_command_pause(&args_matches).unwrap();
        assert_eq!(args, vec!["pause", "service1", "service2"]);
    }
}
