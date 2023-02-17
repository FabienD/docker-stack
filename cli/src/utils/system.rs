use crate::parser::config::ComposeItem;
use eyre::Result;
use std::{ffi::OsStr, path::Path, process::Command};

#[derive(PartialEq, Eq)]
pub struct System {}

impl System {
    pub fn builder<'a>(bin_command: String, sorted_args: Vec<&'a OsStr>) -> Command {
        // Build a command with the given arguments
        let mut cmd = Command::new(bin_command);

        sorted_args.into_iter().for_each(|arg| {
            cmd.arg(arg);
        });

        cmd
    }

    pub fn cd(item: &ComposeItem) -> Result<&str> {
        // Get path from a compose item
        let path = Path::new(OsStr::new(&item.compose_files[0]))
            .parent()
            .unwrap();

        let path_str = path.to_str().unwrap();

        Ok(path_str)
    }
}
