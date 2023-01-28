use clap::{Arg, Command, ArgAction};

pub fn compose_exec() -> Command {
    Command::new("exec")
        .about("Execute a command in a running service of the project.")
        .arg(
            Arg::new("PROJECT")
                .help("The name of the docker-compose file alias")
                .required(true),
        )
        .arg(
            Arg::new("SERVICE")
                .help("The name of the service where the command will be executed")
                .required(true),
        )
        .arg(
            Arg::new("COMMAND")
                .help("The command to execute")
                .required(true),
        )
        .arg(
            Arg::new("ARGS")
                .help("The command arguments")
                .num_args(0..20),
        )
        .arg(
            Arg::new("DETACH")
                .help("Detached mode: Run command in the background.")
                .long("detach")
                .short('d')
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("ENV")
                .help("Set environment variables")
                .long("env")
                .short('e')
        )
        .arg(
            Arg::new("INDEX")
                .help("index of the container if there are multiple instances of a service")
                .long("index")
                .short('i')
                .default_value("1")
        )
        .arg(
            Arg::new("INTERACTIVE")
                .help("Keep STDIN open even if not attached.")
                .long("interactive")
                .short('I')
                .default_value("true")
        )
        .arg(
            Arg::new("NO_TTY")
                .help("Disable pseudo-TTY allocation. By default docker compose exec allocates a TTY.")
                .long("no_TTY")
                .short('T')
                .default_value("true")
        )
        .arg(
            Arg::new("PRIVILEGED")
                .help("Give extended privileges to the process.")
                .long("privileged")
                .short('P')
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("TTY")
                .help("Allocate a pseudo-TTY.")
                .long("tty")
                .short('t')
                .default_value("true")
        )
        .arg(
            Arg::new("USER")
                .help("Run the command as this user.")
                .long("user")
                .short('u')
        )
        .arg(
            Arg::new("WORKDIR")
                .help("Path to workdir directory for this command.")
                .long("workdir")
                .short('w')
        )
}