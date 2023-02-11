use async_trait::async_trait;
use clap::ArgMatches;
use eyre::{eyre, Result};
use std::ffi::OsStr;
use std::process::{Command, Output};
use tokio::task;

use crate::command::build::prepare_command_build;
use crate::command::create::prepare_command_create;
use crate::command::down::prepare_command_down;
use crate::command::events::prepare_command_events;
use crate::command::exec::prepare_command_exec;
use crate::command::images::prepare_command_images;
use crate::command::kill::prepare_command_kill;
use crate::command::logs::prepare_command_logs;
use crate::command::ls::prepare_command_ls;
use crate::command::pause::prepare_command_pause;
use crate::command::ps::prepare_command_ps;
use crate::command::pull::prepare_command_pull;
use crate::command::push::prepare_command_push;
use crate::command::restart::prepare_command_restart;
use crate::command::rm::prepare_command_rm;
use crate::command::run::prepare_command_run;
use crate::command::start::prepare_command_start;
use crate::command::stop::prepare_command_stop;
use crate::command::top::prepare_command_top;
use crate::command::unpause::prepare_command_unpause;
use crate::command::up::prepare_command_up;

use super::system::builder;

#[derive(Debug)]
pub enum CommandType {
    Build,
    Create,
    Down,
    Exec,
    Events,
    Images,
    Kill,
    Ls,
    Logs,
    Pause,
    Pull,
    Push,
    Ps,
    Restart,
    Rm,
    Run,
    Start,
    Stop,
    Top,
    Unpause,
    Up,
}

pub enum CommandOuput {
    Status,
    Output,
}

pub struct Docker {
    bin_path: String,
}

#[async_trait]
pub trait Container {
    fn init(bin_path: String) -> Self
    where
        Self: Sized;
    async fn compose(
        &self,
        command_type: CommandType,
        config_args: &Vec<&OsStr>,
        default_command_args: &Vec<&OsStr>,
        match_args:&ArgMatches,
        command_output: Option<CommandOuput>,
    ) -> Result<Output>;
}

#[async_trait]
impl Container for Docker {
    fn init(bin_path: String) -> Self
    where
        Self: Sized,
    {
        Docker { bin_path }
    }

    async fn compose(
        &self,
        command: CommandType,
        config_args: &Vec<&OsStr>,
        default_command_args: &Vec<&OsStr>,
        match_args: &ArgMatches,
        command_output: Option<CommandOuput>,
    ) -> Result<Output> {
        let output = Self::execute_command(self, command, config_args, default_command_args, match_args, command_output).await?;
        Ok(output)
    }
}

impl Docker {
    async fn execute_command(
        &self,
        command_type: CommandType,
        config_args: &Vec<&OsStr>,
        default_command_args: &Vec<&OsStr>,
        match_args: &ArgMatches,
        command_output: Option<CommandOuput>,
    ) -> Result<Output> {
        let output = if let Some(output) = command_output {
            output
        } else {
            CommandOuput::Status
        };     

        // Build command arguments from matches args & mix with dctl_args
        let mut args = match command_type {
            CommandType::Build => prepare_command_build(match_args)?,
            CommandType::Create => prepare_command_create(match_args)?,
            CommandType::Down => prepare_command_down(match_args)?,
            CommandType::Exec => prepare_command_exec(match_args)?,
            CommandType::Events => prepare_command_events(match_args)?,
            CommandType::Images => prepare_command_images(match_args)?,
            CommandType::Kill => prepare_command_kill(match_args)?,
            CommandType::Ls => prepare_command_ls(match_args)?,
            CommandType::Logs => prepare_command_logs(match_args)?,
            CommandType::Pause => prepare_command_pause(match_args)?,
            CommandType::Pull => prepare_command_pull(match_args)?,
            CommandType::Push => prepare_command_push(match_args)?,
            CommandType::Ps => prepare_command_ps(match_args)?,
            CommandType::Restart => prepare_command_restart(match_args)?,
            CommandType::Rm => prepare_command_rm(match_args)?,
            CommandType::Run => prepare_command_run(match_args)?,
            CommandType::Start => prepare_command_start(match_args)?,
            CommandType::Stop => prepare_command_stop(match_args)?,
            CommandType::Top => prepare_command_top(match_args)?,
            CommandType::Unpause => prepare_command_unpause(match_args)?,
            CommandType::Up => prepare_command_up(match_args)?,
        };

        let mut docker_commmand_arg = vec![OsStr::new("compose")];
        docker_commmand_arg.append(&mut config_args.to_owned());
        docker_commmand_arg.append(&mut args);
        docker_commmand_arg.append(&mut default_command_args.to_owned());


        // Build command
        let mut cmd: Command = builder(self.bin_path.to_owned(), docker_commmand_arg);

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
