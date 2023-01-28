use clap::{Command, Arg};

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
                .help("The name of the service(s) to stop")
                .num_args(0..20),
        )
}