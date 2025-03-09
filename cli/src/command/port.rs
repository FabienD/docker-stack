use clap::{Arg, ArgAction, ArgMatches, Command};
use eyre::Result;
use std::ffi::OsStr;

pub fn compose_port() -> Command {
    Command::new("port")
        .about("Print the public port for a port binding for a service of the project")
        .arg(
            Arg::new("PROJECT")
                .help("The name of the docker-compose file alias")
                .required(true)
        )        
        .arg(
            Arg::new("SERVICE")
                .help("The name of the service(s) to display public port")
        )
        .arg(
            Arg::new("PRIVATE_PORT")
                .help("Private port")
        )
        .arg(
            Arg::new("PROTOCOL")
                .help("Service protocol.")
                .long("protocol")
                .value_parser(["tcp", "udp"])
        )
        .arg(
            Arg::new("INDEX")
                .help("Index of the container if service has multiple replicas.")
                .long("index")
                .action(ArgAction::SetTrue)
        )
}

pub fn prepare_command_port(args_matches: &ArgMatches) -> Result<Vec<&OsStr>> {
    let mut args: Vec<&OsStr> = vec![];

    args.push(OsStr::new("port"));

    if let Some(services) = args_matches.get_occurrences::<String>("SERVICE") {
        for service in services {
            for s in service {
                args.push(OsStr::new(s));
            }
        }
    }

    if args_matches.get_flag("INDEX") {
        args.push(OsStr::new("--index"));
    }

    if let Some(protocol) = args_matches.get_one::<String>("PROTOCOL") {
        args.push(OsStr::new("--protocol"));
        args.push(OsStr::new(protocol));
    }
    
    if let Some(port) = args_matches.get_one::<String>("PRIVATE_PORT") {
        args.push(OsStr::new(port));
    }

    Ok(args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_a_complete_vec_of_osstr_for_command_port() {
        let args_matches =
            compose_port().get_matches_from(vec!["port", "PROJECT", "service1", "--index", "--protocol", "tcp", "8080"]);
        let args = prepare_command_port(&args_matches).unwrap();
        assert_eq!(args, vec!["port", "service1", "--index", "--protocol", "tcp", "8080"]);
    }
}