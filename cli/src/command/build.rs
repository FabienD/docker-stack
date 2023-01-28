use clap::{Arg, Command, ArgAction};

pub fn compose_build() -> Command {
    Command::new("build")
        .about("Build all or selected service(s) for a project.")
        .arg(
            Arg::new("PROJECT")
                .help("The name of the docker-compose file alias")
                .required(true),
        )
        .arg(
            Arg::new("SERVICE")
                .help("The name of the service(s) to build")
                .num_args(0..20),
        )
        .arg(
            Arg::new("BUILD_ARG")
                .help("Set build-time variables for services.")
                .long("build-arg")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("MEMORY")
                .help("Set memory limit for the build container. Not supported on buildkit yet.")
                .long("memory")
                .short('m')
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("NO_CACHE")
                .help("Do not use cache when building the image")
                .long("no-cache")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("PROGRESS")
                .help("Only display IDs")
                .long("progress")
                .short('q')
                .value_parser(["auto", "tty", "plain", "quiet"])
        )
        .arg(
            Arg::new("PULL")
                .help("Always attempt to pull a newer version of the image.")
                .long("pull")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("QUIET")
                .help("Don't print anything to STDOUT")
                .long("quiet")
                .short('q')
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("SSH")
                .help("Set SSH authentications used when building service images. (use 'default' for using your default SSH Agent)")
                .long("ssh")
                .action(ArgAction::SetTrue)
        )
}