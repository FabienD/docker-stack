use clap::{Arg, ArgMatches, Command};
use clap_complete::{generate, Shell};
use eyre::{eyre, Result};
use std::io;

pub fn shell_completion() -> Command {
    Command::new("completion")
        .about("Geneate shell completion (bash, fish, zsh, powershell, elvish)")
        .arg(
            Arg::new("generator")
                .help("The shell to generate completion for")
                .value_parser(["bash", "fish", "zsh", "powershell", "elvish"])
                .required(true),
        )
}

pub fn exec_shell_completion(command: &mut Command, args: &ArgMatches) -> Result<()> {
    let generator = args.get_one::<String>("generator").unwrap();
    let shell = match generator.as_str() {
        "bash" => Shell::Bash,
        "fish" => Shell::Fish,
        "zsh" => Shell::Zsh,
        "powershell" => Shell::PowerShell,
        "elvish" => Shell::Elvish,
        _ => return Err(eyre!("Shell not supported")),
    };
    generate(shell, command, "dctl", &mut io::stdout());
    Ok(())
}
