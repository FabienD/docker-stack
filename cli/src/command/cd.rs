use clap::{Command, Arg};

pub fn cd_project() -> Command {
    Command::new("cd")
        .about("Change directory to the project directory")
        .arg(
            Arg::new("PROJECT")
                .help("The name of the docker-compose file alias")
                .required(true),
        )
}