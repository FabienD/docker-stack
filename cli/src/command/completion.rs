use clap::{Command, Arg};

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