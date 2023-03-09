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
            Arg::new("BUILD")
                .help("Build image before starting container")
                .long("build")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("DETACH")
                .help("Detached mode: Run command in the background")
                .long("detach")
                .short('d')
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("ENTRYPOINT")
                .help("Override the entrypoint of the image")
                .long("entrypoint"),
        )
        .arg(
            Arg::new("ENV")
                .help("Set environment variables")
                .long("env")
                .short('e')
                .num_args(0..20)
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("INTERACTIVE")
                .help("Keep STDIN open even if not attached")
                .long("interactive")
                .short('i')
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("LABEL")
                .help("Add or override a label")
                .short('l')
                .long("label"),
        )
        .arg(
            Arg::new("NAME")
                .help("Assign a name to the container")
                .long("name"),
        )
        .arg(
            Arg::new("NO_TTY")
                .help(
                    "Disable pseudo-TTY allocation. By default docker compose exec allocates a TTY",
                )
                .long("no_TTY")
                .short('T')
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("NO_DEPS")
                .help("Don't start linked services")
                .long("no-deps")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("PUBLISH")
                .help("Publish a container's port(s) to the host")
                .long("publish")
                .short('p'),
        )
        .arg(
            Arg::new("QUIET_PULL")
                .help("Pull without printing progress information")
                .long("quiet-pull")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("RM")
                .help("Remove container after run. Ignored in detached mode")
                .long("rm")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("SERVICE_PORTS")
                .help("Run command with the service's ports enabled and mapped to the host")
                .long("service-ports")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("USE_ALIASES")
                .help(
                    "Use the service's network aliases in the network(s) the container connects to",
                )
                .long("use-aliases")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("USER")
                .help("Run as specified username or uid")
                .long("user"),
        )
        .arg(
            Arg::new("VOLUME")
                .help("Bind mount a volume")
                .long("volume")
                .short('v'),
        )
        .arg(
            Arg::new("WORKDIR")
                .help("Working directory inside the container")
                .long("workdir"),
        )
}

pub fn prepare_command_run(args_matches: &ArgMatches) -> Result<Vec<&OsStr>> {
    let mut args: Vec<&OsStr> = vec![];

    args.push(OsStr::new("run"));

    if args_matches.get_flag("BUILD") {
        args.push(OsStr::new("--build"));
    }
    if args_matches.get_flag("DETACH") {
        args.push(OsStr::new("--detach"));
    }
    if let Some(entrypoint) = args_matches.get_one::<String>("ENTRYPOINT") {
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
    if args_matches.get_flag("INTERACTIVE") {
        args.push(OsStr::new("--interactive"));
    }
    if let Some(label) = args_matches.get_one::<String>("LABEL") {
        args.push(OsStr::new("--label"));
        args.push(OsStr::new(label));
    }
    if let Some(name) = args_matches.get_one::<String>("NAME") {
        args.push(OsStr::new("--name"));
        args.push(OsStr::new(name));
    }
    if args_matches.get_flag("NO_TTY") {
        args.push(OsStr::new("--no_TTY"));
    }
    if args_matches.get_flag("NO_DEPS") {
        args.push(OsStr::new("--no-deps"));
    }
    if let Some(publish) = args_matches.get_one::<String>("PUBLISH") {
        args.push(OsStr::new("--publish"));
        args.push(OsStr::new(publish));
    }
    if args_matches.get_flag("QUIET_PULL") {
        args.push(OsStr::new("--quiet-pull"));
    }
    if args_matches.get_flag("RM") {
        args.push(OsStr::new("--rm"));
    }
    if args_matches.get_flag("SERVICE_PORTS") {
        args.push(OsStr::new("--service-ports"));
    }
    if args_matches.get_flag("USE_ALIASES") {
        args.push(OsStr::new("--use-aliases"));
    }
    if let Some(user) = args_matches.get_one::<String>("USER") {
        args.push(OsStr::new("--user"));
        args.push(OsStr::new(user));
    }
    if let Some(volume) = args_matches.get_occurrences::<String>("VOLUME") {
        for v in volume {
            for s in v {
                args.push(OsStr::new("--volume"));
                args.push(OsStr::new(s));
            }
        }
    }
    if let Some(workdir) = args_matches.get_one::<String>("WORKDIR") {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_a_complete_vec_of_osstr_for_command_run() {
        let args_matches = compose_run().get_matches_from(vec![
            "run",
            "--build",
            "--detach",
            "--entrypoint",
            "entrypoint",
            "--env",
            "env1",
            "env2",
            "--interactive",
            "--label",
            "label",
            "--name",
            "name",
            "--no_TTY",
            "--no-deps",
            "--publish",
            "8080:80",
            "--quiet-pull",
            "--rm",
            "--service-ports",
            "--use-aliases",
            "PROJECT",
            "service1",
            "bash",
        ]);
        let args = prepare_command_run(&args_matches).unwrap();
        assert_eq!(
            args,
            vec![
                "run",
                "--build",
                "--detach",
                "--entrypoint",
                "entrypoint",
                "--env",
                "env1",
                "--env",
                "env2",
                "--interactive",
                "--label",
                "label",
                "--name",
                "name",
                "--no_TTY",
                "--no-deps",
                "--publish",
                "8080:80",
                "--quiet-pull",
                "--rm",
                "--service-ports",
                "--use-aliases",
                "service1",
                "bash"
            ]
        );
    }
}
