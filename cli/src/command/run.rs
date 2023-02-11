use clap::{Arg, ArgAction, ArgMatches, Command};
use eyre::Result;
use std::ffi::OsStr;

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
                .help("Build image before starting container")
                .long("build")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("detach")
                .help("Detached mode: Run command in the background")
                .long("detach")
                .short('d')
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("entrypoint")
                .help("Override the entrypoint of the image")
                .long("entrypoint"),
        )
        .arg(
            Arg::new("env")
                .help("Set environment variables")
                .long("env")
                .short('e')
                .num_args(0..20),
        )
        .arg(
            Arg::new("interactive")
                .help("Keep STDIN open even if not attached")
                .long("interactive")
                .short('I')
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("label")
                .help("Add or override a label")
                .short('l')
                .long("label"),
        )
        .arg(
            Arg::new("name")
                .help("Assign a name to the container")
                .long("name"),
        )
        .arg(
            Arg::new("no_TTY")
                .help(
                    "Disable pseudo-TTY allocation. By default docker compose exec allocates a TTY",
                )
                .long("no_TTY")
                .short('T')
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("no-deps")
                .help("Don't start linked services")
                .long("no-deps")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("publish")
                .help("Publish a container's port(s) to the host")
                .long("publish")
                .short('p'),
        )
        .arg(
            Arg::new("quiet-pull")
                .help("Pull without printing progress information")
                .long("quiet-pull")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("rm")
                .help("Remove container after run. Ignored in detached mode")
                .long("rm")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("service-ports")
                .help("Run command with the service's ports enabled and mapped to the host")
                .long("service-ports")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("use-aliases")
                .help(
                    "Use the service's network aliases in the network(s) the container connects to",
                )
                .long("use-aliases")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("user")
                .help("Run as specified username or uid")
                .long("user"),
        )
        .arg(
            Arg::new("volume")
                .help("Bind mount a volume")
                .long("volume")
                .short('v'),
        )
        .arg(
            Arg::new("workdir")
                .help("Working directory inside the container")
                .long("workdir"),
        )
}

pub fn prepare_command_run<'a>(
    args_matches: &'a ArgMatches,
) -> Result<Vec<&'a OsStr>> {
    let mut args: Vec<&OsStr> = vec![];

    args.push(OsStr::new("run"));

    if args_matches.get_flag("build") {
        args.push(OsStr::new("--build"));
    }
    if args_matches.get_flag("detach") {
        args.push(OsStr::new("--detach"));
    }
    if let Some(entrypoint) = args_matches.get_one::<String>("entrypoint") {
        args.push(OsStr::new("--entrypoint"));
        args.push(OsStr::new(entrypoint));
    }
    if let Some(env) = args_matches.get_occurrences::<String>("ENV") {
        for e in env {
            for s in e {
                args.push(OsStr::new("--env"));
                args.push(OsStr::new(s));
            }
        }
    }
    if args_matches.get_flag("interactive") {
        args.push(OsStr::new("--interactive"));
    }
    if let Some(label) = args_matches.get_one::<String>("label") {
        args.push(OsStr::new("--label"));
        args.push(OsStr::new(label));
    }
    if let Some(name) = args_matches.get_one::<String>("name") {
        args.push(OsStr::new("--name"));
        args.push(OsStr::new(name));
    }
    if args_matches.get_flag("no_TTY") {
        args.push(OsStr::new("--no_TTY"));
    }
    if args_matches.get_flag("no-deps") {
        args.push(OsStr::new("--no-deps"));
    }
    if let Some(publish) = args_matches.get_one::<String>("publish") {
        args.push(OsStr::new("--publish"));
        args.push(OsStr::new(publish));
    }
    if args_matches.get_flag("quiet-pull") {
        args.push(OsStr::new("--quiet-pull"));
    }
    if args_matches.get_flag("rm") {
        args.push(OsStr::new("--rm"));
    }
    if args_matches.get_flag("service-ports") {
        args.push(OsStr::new("--service-ports"));
    }
    if args_matches.get_flag("use-aliases") {
        args.push(OsStr::new("--use-aliases"));
    }
    if let Some(user) = args_matches.get_one::<String>("user") {
        args.push(OsStr::new("--user"));
        args.push(OsStr::new(user));
    }
    if let Some(volume) = args_matches.get_occurrences::<String>("volume") {
        for v in volume {
            for s in v {
                args.push(OsStr::new("--volume"));
                args.push(OsStr::new(s));
            }
        }
    }
    if let Some(workdir) = args_matches.get_one::<String>("workdir") {
        args.push(OsStr::new("--workdir"));
        args.push(OsStr::new(workdir));
    }

    if let Some(service) = args_matches.get_one::<String>("SERVICE") {
        args.push(OsStr::new(service));
    }

    if let Some(command) = args_matches.get_one::<String>("COMMAND") {
        args.push(OsStr::new(command));
    }
    if let Some(command_args) = args_matches.get_occurrences::<String>("ARGS") {
        for a in command_args {
            for s in a {
                args.push(OsStr::new(s));
            }
        }
    }

    Ok(args)
}
