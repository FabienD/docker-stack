//! Declarative argument system for docker compose commands
//!
//! This module provides a type-safe, declarative way to define command arguments
//! that automatically generates both the clap definition and the argument preparation.

use clap::{Arg, ArgAction, ArgMatches, Command};
use std::ffi::OsString;

/// Represents different types of command arguments
#[derive(Clone)]
pub enum ArgDef {
    /// Boolean flag (--flag)
    Flag {
        id: &'static str,
        long: &'static str,
        short: Option<char>,
        help: &'static str,
    },
    /// String value (--option value)
    Value {
        id: &'static str,
        long: &'static str,
        short: Option<char>,
        help: &'static str,
    },
    /// Value with predefined choices (--option choice1|choice2)
    Choice {
        id: &'static str,
        long: &'static str,
        short: Option<char>,
        help: &'static str,
        choices: &'static [&'static str],
    },
    /// Numeric value with validation (--timeout 10)
    Number {
        id: &'static str,
        long: &'static str,
        short: Option<char>,
        help: &'static str,
    },
    /// Multiple services (service1 service2 ...)
    Services,
    /// Container name
    Container,
    /// Service name followed by command and its arguments (for exec/run)
    /// Format: SERVICE COMMAND [ARGS...]
    ServiceWithCommand,
}

impl ArgDef {
    /// Convert to clap Arg
    pub fn to_clap_arg(&self) -> Arg {
        match self {
            ArgDef::Flag { id, long, short, help } => {
                let mut arg = Arg::new(*id)
                    .long(*long)
                    .help(*help)
                    .action(ArgAction::SetTrue);
                if let Some(s) = short {
                    arg = arg.short(*s);
                }
                arg
            }
            ArgDef::Value { id, long, short, help } => {
                let mut arg = Arg::new(*id)
                    .long(*long)
                    .help(*help);
                if let Some(s) = short {
                    arg = arg.short(*s);
                }
                arg
            }
            ArgDef::Choice { id, long, short, help, choices } => {
                let mut arg = Arg::new(*id)
                    .long(*long)
                    .help(*help)
                    .value_parser(choices.to_vec());
                if let Some(s) = short {
                    arg = arg.short(*s);
                }
                arg
            }
            ArgDef::Number { id, long, short, help } => {
                let mut arg = Arg::new(*id)
                    .long(*long)
                    .help(*help)
                    .value_parser(clap::value_parser!(i64).range(0..));
                if let Some(s) = short {
                    arg = arg.short(*s);
                }
                arg
            }
            ArgDef::Services => {
                Arg::new("SERVICE")
                    .help("The name of the service(s)")
                    .num_args(0..20)
                    .action(ArgAction::Append)
            }
            ArgDef::Container => {
                Arg::new("CONTAINER")
                    .help("The name of the container")
                    .required(true)
            }
            ArgDef::ServiceWithCommand => {
                Arg::new("COMMAND_ARGS")
                    .help("Service name followed by command and arguments")
                    .required(true)
                    .num_args(1..)
                    .trailing_var_arg(true)
                    .allow_hyphen_values(true)
            }
        }
    }

    /// Extract argument value and add to args vector (owned strings)
    pub fn extract_to_args(&self, matches: &ArgMatches, args: &mut Vec<OsString>) {
        match self {
            ArgDef::Flag { id, long, .. } => {
                if matches.get_flag(id) {
                    args.push(OsString::from(format!("--{}", long)));
                }
            }
            ArgDef::Value { id, long, .. } | ArgDef::Choice { id, long, .. } => {
                if let Some(value) = matches.get_one::<String>(id) {
                    args.push(OsString::from(format!("--{}", long)));
                    args.push(OsString::from(value));
                }
            }
            ArgDef::Number { id, long, .. } => {
                if let Some(value) = matches.get_one::<i64>(id) {
                    args.push(OsString::from(format!("--{}", long)));
                    args.push(OsString::from(value.to_string()));
                }
            }
            ArgDef::Services => {
                if let Some(services) = matches.get_occurrences::<String>("SERVICE") {
                    for service in services {
                        for s in service {
                            args.push(OsString::from(s));
                        }
                    }
                }
            }
            ArgDef::Container => {
                if let Some(container) = matches.get_one::<String>("CONTAINER") {
                    args.push(OsString::from(container));
                }
            }
            ArgDef::ServiceWithCommand => {
                if let Some(cmd_args) = matches.get_many::<String>("COMMAND_ARGS") {
                    for arg in cmd_args {
                        args.push(OsString::from(arg));
                    }
                }
            }
        }
    }
}

/// Command definition with arguments
pub struct CommandDef {
    pub name: &'static str,
    pub about: &'static str,
    pub args: Vec<ArgDef>,
    /// Whether this command requires a PROJECT argument
    pub needs_project: bool,
}

impl CommandDef {
    /// Build the clap Command
    pub fn to_clap_command(&self) -> Command {
        let mut cmd = Command::new(self.name).about(self.about);

        // Add PROJECT arg if needed
        if self.needs_project {
            cmd = cmd.arg(
                Arg::new("PROJECT")
                    .help("The name of the docker-compose file alias")
                    .required(true),
            );
        }

        // Add all other args
        for arg_def in &self.args {
            cmd = cmd.arg(arg_def.to_clap_arg());
        }

        cmd
    }

    /// Prepare command arguments from matches (returns owned OsStrings)
    pub fn prepare_args(&self, matches: &ArgMatches) -> Vec<OsString> {
        let mut args: Vec<OsString> = vec![OsString::from(self.name)];

        // Extract flags and values first (before services/positional args)
        for arg_def in &self.args {
            match arg_def {
                ArgDef::Services | ArgDef::Container | ArgDef::ServiceWithCommand => {}
                _ => arg_def.extract_to_args(matches, &mut args),
            }
        }

        // Extract positional args last
        for arg_def in &self.args {
            match arg_def {
                ArgDef::Services | ArgDef::Container | ArgDef::ServiceWithCommand => {
                    arg_def.extract_to_args(matches, &mut args);
                }
                _ => {}
            }
        }

        args
    }
}

// Helper macros for concise argument definitions
#[macro_export]
macro_rules! flag {
    ($id:literal, $long:literal, $help:literal) => {
        $crate::command::args::ArgDef::Flag {
            id: $id,
            long: $long,
            short: None,
            help: $help,
        }
    };
    ($id:literal, $long:literal, $short:literal, $help:literal) => {
        $crate::command::args::ArgDef::Flag {
            id: $id,
            long: $long,
            short: Some($short),
            help: $help,
        }
    };
}

#[macro_export]
macro_rules! value {
    ($id:literal, $long:literal, $help:literal) => {
        $crate::command::args::ArgDef::Value {
            id: $id,
            long: $long,
            short: None,
            help: $help,
        }
    };
    ($id:literal, $long:literal, $short:literal, $help:literal) => {
        $crate::command::args::ArgDef::Value {
            id: $id,
            long: $long,
            short: Some($short),
            help: $help,
        }
    };
}

#[macro_export]
macro_rules! choice {
    ($id:literal, $long:literal, $help:literal, [$($choice:literal),+]) => {
        $crate::command::args::ArgDef::Choice {
            id: $id,
            long: $long,
            short: None,
            help: $help,
            choices: &[$($choice),+],
        }
    };
}

#[macro_export]
macro_rules! number {
    ($id:literal, $long:literal, $help:literal) => {
        $crate::command::args::ArgDef::Number {
            id: $id,
            long: $long,
            short: None,
            help: $help,
        }
    };
    ($id:literal, $long:literal, $short:literal, $help:literal) => {
        $crate::command::args::ArgDef::Number {
            id: $id,
            long: $long,
            short: Some($short),
            help: $help,
        }
    };
}

#[macro_export]
macro_rules! services {
    () => {
        $crate::command::args::ArgDef::Services
    };
}

#[macro_export]
macro_rules! container {
    () => {
        $crate::command::args::ArgDef::Container
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flag_arg_definition() {
        let flag = ArgDef::Flag {
            id: "NO_CACHE",
            long: "no-cache",
            short: None,
            help: "Do not use cache",
        };

        let clap_arg = flag.to_clap_arg();
        assert_eq!(clap_arg.get_id().as_str(), "NO_CACHE");
    }

    #[test]
    fn test_choice_arg_definition() {
        let choice = ArgDef::Choice {
            id: "PROGRESS",
            long: "progress",
            short: None,
            help: "Set progress type",
            choices: &["auto", "plain", "tty"],
        };

        let clap_arg = choice.to_clap_arg();
        assert_eq!(clap_arg.get_id().as_str(), "PROGRESS");
    }

    #[test]
    fn test_command_def_builds_clap_command() {
        let cmd_def = CommandDef {
            name: "test",
            about: "Test command",
            needs_project: true,
            args: vec![
                ArgDef::Flag {
                    id: "VERBOSE",
                    long: "verbose",
                    short: Some('v'),
                    help: "Verbose output",
                },
            ],
        };

        let cmd = cmd_def.to_clap_command();
        assert_eq!(cmd.get_name(), "test");
    }

    #[test]
    fn test_prepare_args_with_flags() {
        let cmd_def = CommandDef {
            name: "build",
            about: "Build command",
            needs_project: true,
            args: vec![
                ArgDef::Flag {
                    id: "NO_CACHE",
                    long: "no-cache",
                    short: None,
                    help: "No cache",
                },
                ArgDef::Flag {
                    id: "PULL",
                    long: "pull",
                    short: None,
                    help: "Pull images",
                },
            ],
        };

        let matches = cmd_def.to_clap_command().get_matches_from(vec![
            "build",
            "--no-cache",
            "PROJECT",
        ]);

        let args = cmd_def.prepare_args(&matches);
        assert_eq!(args, vec![
            OsString::from("build"),
            OsString::from("--no-cache"),
        ]);
    }

    #[test]
    fn test_prepare_args_with_choice() {
        let cmd_def = CommandDef {
            name: "build",
            about: "Build command",
            needs_project: true,
            args: vec![
                ArgDef::Choice {
                    id: "PROGRESS",
                    long: "progress",
                    short: None,
                    help: "Progress type",
                    choices: &["auto", "plain", "tty"],
                },
            ],
        };

        let matches = cmd_def.to_clap_command().get_matches_from(vec![
            "build",
            "--progress", "plain",
            "PROJECT",
        ]);

        let args = cmd_def.prepare_args(&matches);
        assert_eq!(args, vec![
            OsString::from("build"),
            OsString::from("--progress"),
            OsString::from("plain"),
        ]);
    }

    #[test]
    fn test_flag_macro() {
        let flag = flag!("TEST", "test", "Test flag");
        match flag {
            ArgDef::Flag { id, long, .. } => {
                assert_eq!(id, "TEST");
                assert_eq!(long, "test");
            }
            _ => panic!("Expected Flag"),
        }
    }

    #[test]
    fn test_choice_macro() {
        let choice = choice!("MODE", "mode", "Select mode", ["fast", "slow"]);
        match choice {
            ArgDef::Choice { id, choices, .. } => {
                assert_eq!(id, "MODE");
                assert_eq!(choices, &["fast", "slow"]);
            }
            _ => panic!("Expected Choice"),
        }
    }
}