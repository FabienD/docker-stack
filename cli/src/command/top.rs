use clap::{Arg, ArgMatches, Command};
use eyre::Result;
use std::ffi::OsStr;

pub fn compose_top() -> Command {
    Command::new("top")
        .about("Top on all containers for a project or only on selected service(s) of the project.")
        .arg(
            Arg::new("PROJECT")
                .help("The name of the docker-compose file alias")
                .required(true),
        )
        .arg(
            Arg::new("SERVICE")
                .help("The name of the service(s) to show top activity for")
                .num_args(0..20),
        )
}

pub fn prepare_command_top<'a>(args_matches: &'a ArgMatches) -> Result<Vec<&'a OsStr>> {
    let mut args: Vec<&OsStr> = vec![];

    args.push(OsStr::new("top"));

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
    fn it_returns_a_complete_vec_of_osstr_for_command_top() {
        let args_matches =
            compose_top().get_matches_from(vec!["top", "PROJECT", "service1", "service2"]);
        let args = prepare_command_top(&args_matches).unwrap();
        assert_eq!(args, vec!["top", "service1", "service2"]);
    }
}
