//! Command registry using the declarative definitions system
//!
//! This module registers all docker compose commands using the definitions
//! from definitions.rs, eliminating the need for individual command files.

use clap::{ArgMatches, Command};
use std::ffi::OsString;

use crate::utils::docker::CommandType;
use super::CommandHandler;
use super::definitions::*;

// Macro to generate command handlers from definitions
macro_rules! define_command_from_def {
    ($struct_name:ident, $command_type:ident, $def_fn:ident) => {
        pub struct $struct_name;

        impl CommandHandler for $struct_name {
            fn name(&self) -> &'static str {
                $def_fn().name
            }

            fn cli(&self) -> Command {
                $def_fn().to_clap_command()
            }

            fn command_type(&self) -> CommandType {
                CommandType::$command_type
            }

            fn prepare(&self, args: &ArgMatches) -> Vec<OsString> {
                $def_fn().prepare_args(args)
            }
        }
    };
}

// Define all command handlers using the new definitions
define_command_from_def!(BuildCommand, Build, build_def);
define_command_from_def!(CreateCommand, Create, create_def);
define_command_from_def!(DownCommand, Down, down_def);
define_command_from_def!(EventsCommand, Events, events_def);
define_command_from_def!(ExecCommand, Exec, exec_def);
define_command_from_def!(ImagesCommand, Images, images_def);
define_command_from_def!(KillCommand, Kill, kill_def);
define_command_from_def!(LogsCommand, Logs, logs_def);
define_command_from_def!(LsCommand, Ls, ls_def);
define_command_from_def!(PauseCommand, Pause, pause_def);
define_command_from_def!(PortCommand, Port, port_def);
define_command_from_def!(PsCommand, Ps, ps_def);
define_command_from_def!(PullCommand, Pull, pull_def);
define_command_from_def!(PushCommand, Push, push_def);
define_command_from_def!(RestartCommand, Restart, restart_def);
define_command_from_def!(RmCommand, Rm, rm_def);
define_command_from_def!(RunCommand, Run, run_def);
define_command_from_def!(StartCommand, Start, start_def);
define_command_from_def!(StopCommand, Stop, stop_def);
define_command_from_def!(TopCommand, Top, top_def);
define_command_from_def!(UnpauseCommand, Unpause, unpause_def);
define_command_from_def!(UpCommand, Up, up_def);
define_command_from_def!(WatchCommand, Watch, watch_def);

/// Returns all docker compose command handlers
pub fn get_compose_commands() -> Vec<Box<dyn CommandHandler>> {
    vec![
        Box::new(BuildCommand),
        Box::new(CreateCommand),
        Box::new(DownCommand),
        Box::new(EventsCommand),
        Box::new(ExecCommand),
        Box::new(ImagesCommand),
        Box::new(KillCommand),
        Box::new(LogsCommand),
        Box::new(LsCommand),
        Box::new(PauseCommand),
        Box::new(PortCommand),
        Box::new(PsCommand),
        Box::new(PullCommand),
        Box::new(PushCommand),
        Box::new(RestartCommand),
        Box::new(RmCommand),
        Box::new(RunCommand),
        Box::new(StartCommand),
        Box::new(StopCommand),
        Box::new(TopCommand),
        Box::new(UnpauseCommand),
        Box::new(UpCommand),
        Box::new(WatchCommand),
    ]
}

/// Find a command handler by name
pub fn get_command_by_name(name: &str) -> Option<Box<dyn CommandHandler>> {
    get_compose_commands().into_iter().find(|cmd| cmd.name() == name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_all_compose_commands() {
        let commands = get_compose_commands();
        assert_eq!(commands.len(), 23);
    }

    #[test]
    fn it_finds_command_by_name() {
        let cmd = get_command_by_name("build");
        assert!(cmd.is_some());
        assert_eq!(cmd.unwrap().name(), "build");
    }

    #[test]
    fn it_returns_none_for_unknown_command() {
        let cmd = get_command_by_name("unknown");
        assert!(cmd.is_none());
    }

    #[test]
    fn it_prepares_build_args_correctly() {
        let cmd = get_command_by_name("build").unwrap();
        let matches = cmd.cli().get_matches_from(vec!["build", "--no-cache", "myproject"]);
        let args = cmd.prepare(&matches);

        assert_eq!(args[0], OsString::from("build"));
        assert_eq!(args[1], OsString::from("--no-cache"));
    }

    #[test]
    fn it_prepares_up_args_with_flags_and_choices() {
        let cmd = get_command_by_name("up").unwrap();
        let matches = cmd.cli().get_matches_from(vec![
            "up", "-d", "--pull", "always", "myproject"
        ]);
        let args = cmd.prepare(&matches);

        assert!(args.contains(&OsString::from("up")));
        assert!(args.contains(&OsString::from("--detach")));
        assert!(args.contains(&OsString::from("--pull")));
        assert!(args.contains(&OsString::from("always")));
    }
}
