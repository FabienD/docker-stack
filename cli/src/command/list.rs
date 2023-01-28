use clap::{Command, Arg};

pub fn list_projects() -> Command {
    Command::new("list")
        .about("List all projects")
}