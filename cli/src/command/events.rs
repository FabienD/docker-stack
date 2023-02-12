use clap::{Arg, ArgAction, ArgMatches, Command};
use eyre::Result;
use std::ffi::OsStr;

pub fn compose_events() -> Command {
    Command::new("events")
        .about("Receive real time events from containers.")
        .arg(
            Arg::new("PROJECT")
                .help("The name of the docker-compose file alias")
                .required(true),
        )
        .arg(
            Arg::new("SERVICE")
                .help("The name of the service(s) to listen for events")
                .num_args(0..20),
        )
        .arg(
            Arg::new("JSON")
                .help("Output events as a stream of json objects")
                .long("json")
                .action(ArgAction::SetTrue),
        )
}

pub fn prepare_command_events<'a>(args_matches: &'a ArgMatches) -> Result<Vec<&'a OsStr>> {
    let mut args: Vec<&OsStr> = vec![];

    args.push(OsStr::new("events"));

    if args_matches.get_flag("JSON") {
        args.push(OsStr::new("--json"));
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
    fn it_returns_a_complete_vec_of_osstr_for_command_events() {
        let args_matches = compose_events()
            .get_matches_from(vec!["events", "--json", "PROJECT", "service1", "service2"]);
        let args = prepare_command_events(&args_matches).unwrap();
        assert_eq!(args, vec!["events", "--json", "service1", "service2"]);
    }
}
