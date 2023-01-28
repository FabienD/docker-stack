use clap::{Command, Arg};

pub fn compose_stop() -> Command {
    Command::new("stop")
        .about("Stop all containers for a project or only selected service(s) of the project.")
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
        .arg(
            Arg::new("TIMEOUT")
                .help("Specify a shutdown timeout in seconds")
                .short('t')
                .long("timeout")
        )
}