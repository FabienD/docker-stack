use anyhow::{anyhow, Result};
use mockall::automock;
use std::ffi::OsString;
use std::process::Output;
use tokio::process::Command;

#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;
#[cfg(windows)]
use std::os::windows::process::ExitStatusExt;

use super::docker::CommandOutput;

#[derive(PartialEq, Eq)]
pub struct System {}

#[automock]
impl System {
    pub fn builder(bin_command: String, sorted_args: Vec<OsString>) -> Command {
        // Build a command with the given arguments
        let mut cmd = Command::new(bin_command);

        sorted_args.into_iter().for_each(|arg| {
            cmd.arg(arg);
        });

        cmd
    }

    pub async fn execute(
        bin_command_path: String,
        command_arg: &[OsString],
        output: &CommandOutput,
    ) -> Result<Output> {
        // Build command
        let mut cmd: Command = System::builder(bin_command_path, command_arg.to_vec());

        // Execute command asynchronously using tokio::process::Command
        match output {
            CommandOutput::Status => {
                let status = cmd.status().await?;

                if status.success() {
                    #[cfg(unix)]
                    let exit_status = std::process::ExitStatus::from_raw(status.code().unwrap_or(0));
                    #[cfg(windows)]
                    let exit_status = std::process::ExitStatus::from_raw(status.code().unwrap_or(0) as u32);

                    Ok(Output {
                        status: exit_status,
                        stdout: vec![],
                        stderr: vec![],
                    })
                } else {
                    Err(anyhow!("Command failed with status: {:?}", status.code()))
                }
            }
            CommandOutput::Output => {
                let output = cmd.output().await?;

                if output.status.success() {
                    Ok(output)
                } else {
                    Err(anyhow!("Command failed with status: {:?}", output.status.code()))
                }
            }
        }
    }
}
