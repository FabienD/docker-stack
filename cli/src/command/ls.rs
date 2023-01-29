use clap::{Command, Arg, ArgAction};

pub fn compose_ls() -> Command {
    Command::new("ls")
        .about("List running compose projects")
        .arg(
            Arg::new("PROJECT")
                .help("The name of the docker-compose file alias")
                .required(true),
        )
        .arg(
            Arg::new("ALL")
                .help("Show all stopped Compose projects")
                .short('a')
                .long("all")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("FILTER")
                .help("Filter output based on conditions provided.")
                .long("filter")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("FORMAT")
                .help("Pretty-print services using a Go template")
                .short('f')
                .long("format")
                .value_parser(["table", "json"])
                .default_value("table"),
        )
        .arg(
            Arg::new("QUIET")
                .help("Only display IDs")
                .short('q')
                .long("quiet")
                .action(ArgAction::SetTrue),
        )
}