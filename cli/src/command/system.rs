use eyre::Result;

use crate::parser::config::ComposeItem;
use std::{ffi::OsStr, path::Path, process::Command};

#[derive(PartialEq, Eq)]
pub struct System {}

pub fn builder(bin_command: String, sorted_args: Vec<&OsStr>) -> Command {
    // Build a command with the given arguments
    let mut cmd = Command::new(bin_command);

    sorted_args.into_iter().for_each(|arg| {
        cmd.arg(arg);
    });

    cmd
}

impl System {
    pub fn init() -> Self {
        System {}
    }

    pub fn cd<'a>(&'a self, item: &'a ComposeItem) -> Result<&str> {
        // Get path from a compose item
        let path = Path::new(OsStr::new(&item.compose_files[0]))
            .parent()
            .unwrap();

        let path_str = path.to_str().unwrap();

        Ok(path_str)
    }
}
