use std::{ffi::OsStr, process::Command};

pub(crate) fn builder(bin_command: String, sorted_args: Vec<&OsStr>) -> Command {
    let mut cmd = Command::new(bin_command);

    sorted_args.into_iter().for_each(|arg| {
        cmd.arg(arg);
    });

    return cmd;
}
