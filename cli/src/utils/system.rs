use eyre::{eyre, Result};
use mockall::automock;
use std::{
    ffi::OsStr,
    process::{Command, Output},
};
use tokio::task;

use super::docker::CommandOuput;

#[derive(PartialEq, Eq)]
pub struct System {}

#[automock]
impl System {
    pub fn builder<'a>(bin_command: String, sorted_args: Vec<&'a OsStr>) -> Command {
        // Build a command with the given arguments
        let mut cmd = Command::new(bin_command);

        sorted_args.into_iter().for_each(|arg| {
            cmd.arg(arg);
        });

        cmd
    }

    pub async fn execute<'a>(
        bin_command_path: String,
        commmand_arg: &Vec<&'a OsStr>,
        output: &CommandOuput,
    ) -> Result<Output> {
        // Build command
        let mut cmd: Command = System::builder(bin_command_path, commmand_arg.clone());

        // Execute command
        match output {
            CommandOuput::Status => {
                let status = task::spawn(async move { cmd.status() });
                let status = status.await??;

                if status.success() {
                    Ok(Output {
                        status,
                        stdout: vec![],
                        stderr: vec![],
                    })
                } else {
                    Err(eyre!("Command failed"))
                }
            }
            CommandOuput::Output => {
                let output = task::spawn(async move { cmd.output() });
                let output = output.await??;

                if output.status.success() {
                    Ok(output)
                } else {
                    Err(eyre!("Command failed"))
                }
            }
        }
    }
}
