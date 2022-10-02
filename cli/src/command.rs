use std::{ffi::OsStr, process::Command};

pub(crate) fn builder(bin_command: String, sorted_args: Vec<&OsStr>) -> Command {
    // Build a command with the given arguments
    let mut cmd = Command::new(bin_command);

    sorted_args.into_iter().for_each(|arg| {
        cmd.arg(arg);
    });

    return cmd;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder() {
        let bin_command = "ls".to_string();
        let args = vec![OsStr::new("-l"), OsStr::new("-a")];

        let cmd = builder(bin_command.to_owned(), args.to_owned());
        let cmd_args: Vec<&OsStr> = cmd.get_args().collect();

        assert_eq!(cmd.get_program(), OsStr::new(&bin_command));
        assert_eq!(cmd_args, args);
    }
}
