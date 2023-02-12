use clap::{Arg, ArgAction, ArgMatches, Command};
use eyre::Result;
use std::ffi::OsStr;

pub fn compose_images() -> Command {
    Command::new("images")
        .about("List images used by the created containers")
        .arg(
            Arg::new("PROJECT")
                .help("The name of the docker-compose file alias")
                .required(true),
        )
        .arg(
            Arg::new("SERVICE")
                .help("The name of the service(s) to show images")
                .num_args(0..20)
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .help("Only display IDs")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("format")
                .long("format")
                .help("Format the output.")
                .default_value("table")
                .value_parser(["table", "json"]),
        )
}

pub fn prepare_command_images(args_matches: &ArgMatches) -> Result<Vec<&OsStr>> {
    let mut args: Vec<&OsStr> = vec![];

    args.push(OsStr::new("images"));

    if args_matches.get_flag("quiet") {
        args.push(OsStr::new("--quiet"));
    }
    if let Some(format) = args_matches.get_one::<String>("format") {
        args.push(OsStr::new("--format"));
        args.push(OsStr::new(format));
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
    fn it_returns_a_complete_vec_of_osstr_for_command_images() {
        let args_matches = compose_images().get_matches_from(vec![
            "images", "--quiet", "--format", "json", "PROJECT", "service1", "service2",
        ]);
        let args = prepare_command_images(&args_matches).unwrap();

        assert_eq!(
            args,
            vec![
                OsStr::new("images"),
                OsStr::new("--quiet"),
                OsStr::new("--format"),
                OsStr::new("json"),
                OsStr::new("service1"),
                OsStr::new("service2"),
            ]
        );
    }
}
