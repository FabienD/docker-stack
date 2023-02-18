#[cfg(test)]
mod tests {
    use std::ffi::OsStr;

    use crate::utils::system::System;

    #[test]
    fn it_builds_a_system_command_process() {
        let bin_command = "ls".to_string();
        let args = vec![OsStr::new("-l"), OsStr::new("-a")];

        let cmd = System::builder(bin_command.to_owned(), args.to_owned());
        let cmd_args: Vec<&OsStr> = cmd.get_args().collect();

        assert_eq!(cmd.get_program(), OsStr::new(&bin_command));
        assert_eq!(cmd_args, args);
    }
}
