#[cfg(test)]
mod tests {
    use std::ffi::OsString;

    use crate::utils::system::System;

    #[test]
    fn it_builds_a_system_command_process() {
        let bin_command = "ls".to_string();
        let args = vec![OsString::from("-l"), OsString::from("-a")];

        let cmd = System::builder(bin_command.to_owned(), args.to_owned());
        // Use as_std() to access the underlying std::process::Command for testing
        let std_cmd = cmd.as_std();
        let cmd_args: Vec<&std::ffi::OsStr> = std_cmd.get_args().collect();

        assert_eq!(std_cmd.get_program(), std::ffi::OsStr::new(&bin_command));
        assert_eq!(cmd_args, vec![std::ffi::OsStr::new("-l"), std::ffi::OsStr::new("-a")]);
    }
}
