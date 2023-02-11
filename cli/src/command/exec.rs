use clap::{Arg, ArgAction, ArgMatches, Command};
use eyre::Result;
use std::ffi::OsStr;

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
                .help("Detached mode: Run command in the background")
                .long("detach")
                .short('d')
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("ENV")
                .help("Set environment variables")
                .long("env")
                .short('e')
                .num_args(0..20),
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

pub fn prepare_command_exec<'a>(
    args_matches: &'a ArgMatches,
) -> Result<Vec<&'a OsStr>> {
    let mut args: Vec<&OsStr> = vec![];

    args.push(OsStr::new("exec"));

    if args_matches.get_flag("DETACH") {
        args.push(OsStr::new("--detach"));
    }
    if args_matches.get_flag("PRIVILEGED") {
        args.push(OsStr::new("--privileged"));
    }
    if let Some(env) = args_matches.get_occurrences::<String>("ENV") {
        for e in env {
            for s in e {
                args.push(OsStr::new("--env"));
                args.push(OsStr::new(s));
            }
        }
    }
    if let Some(index) = args_matches.get_one::<String>("INDEX") {
        args.push(OsStr::new("--index"));
        args.push(OsStr::new(index));
    }
    if let Some(interactive) = args_matches.get_one::<String>("INTERACTIVE") {
        args.push(OsStr::new("--interactive"));
        args.push(OsStr::new(interactive));
    }
    if let Some(no_tty) = args_matches.get_one::<String>("NO_TTY") {
        args.push(OsStr::new("--no_TTY"));
        args.push(OsStr::new(no_tty));
    }
    if let Some(tty) = args_matches.get_one::<String>("TTY") {
        args.push(OsStr::new("--tty"));
        args.push(OsStr::new(tty));
    }
    if let Some(user) = args_matches.get_one::<String>("USER") {
        args.push(OsStr::new("--user"));
        args.push(OsStr::new(user));
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
