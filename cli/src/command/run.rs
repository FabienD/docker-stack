use clap::{Arg, Command, ArgAction};

pub fn compose_run() -> Command {
    Command::new("run")
        .about("Run a one-off command on a service.")
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
            Arg::new("build")
                .help("Build image before starting container.")
                .long("build")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("detach")
                .help("Detached mode: Run command in the background.")
                .long("detach")
                .short('d')
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("entrypoint")
                .help("Override the entrypoint of the image")
                .long("entrypoint")
        )
        .arg(
            Arg::new("env")
                .help("Set environment variables")
                .long("env")
                .short('e')
        )
        .arg(
            Arg::new("interactive")
                .help("Keep STDIN open even if not attached.")
                .long("interactive")
                .short('I')
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("label")
                .help("Add or override a label")
                .short('l')
                .long("label")
        )
        .arg(
            Arg::new("name")
                .help("Assign a name to the container")
                .long("name")
        )
        .arg(
            Arg::new("no_TTY")
                .help("Disable pseudo-TTY allocation. By default docker compose exec allocates a TTY.")
                .long("no_TTY")
                .short('T')
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("no-deps")
                .help("Don't start linked services.")
                .long("no-deps")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("publish")
                .help("Publish a container's port(s) to the host")
                .long("publish")
                .short('p')
        )
        .arg(
            Arg::new("quiet-pull")
                .help("Pull without printing progress information")
                .long("quiet-pull")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("rm")
                .help("Remove container after run. Ignored in detached mode.")
                .long("rm")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("service-ports")
                .help("Run command with the service's ports enabled and mapped to the host.")
                .long("service-ports")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("use-aliases")
                .help("Use the service's network aliases in the network(s) the container connects to.")
                .long("use-aliases")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("user")
                .help("Run as specified username or uid")
                .long("user")
        )
        .arg(
            Arg::new("volume")
                .help("Bind mount a volume")
                .long("volume")
                .short('v')
        )
        .arg(
            Arg::new("workdir")
                .help("Working directory inside the container")
                .long("workdir")
        )

}
