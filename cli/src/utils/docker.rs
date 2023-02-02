use crate::parser::config::ComposeItem;
use clap::ArgMatches;
use eyre::{eyre, Result};
use mockall::automock;
use std::ffi::OsStr;
use std::process::{Command, Output};

use crate::command::build::prepare_command_build;
use crate::command::create::prepare_command_create;
use crate::command::down::prepare_command_down;
use crate::command::exec::prepare_command_exec;
use crate::command::events::prepare_command_events;
use crate::command::images::prepare_command_images;
use crate::command::kill::prepare_command_kill;
use crate::command::logs::prepare_command_logs;
use crate::command::ls::prepare_command_ls;
use crate::command::pause::prepare_command_pause;
use crate::command::pull::prepare_command_pull;
use crate::command::push::prepare_command_push;
use crate::command::ps::prepare_command_ps;
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

pub trait Container {
    fn init(bin_path: String) -> Self
    where
        Self: Sized;
    fn compose(
        &self,
        command_type: CommandType,
        item: &ComposeItem,
        args: &ArgMatches,
    ) -> Result<()>;
}

#[automock]
impl Container for Docker {
    fn init(bin_path: String) -> Self
    where
        Self: Sized,
    {
        Docker { bin_path }
    }

    fn compose(&self, command: CommandType, item: &ComposeItem, args: &ArgMatches) -> Result<()> {
        Self::execute_command(&self, command, item, args)?;
        Ok(())
    }
}

impl Docker {
    fn execute_command(
        &self,
        command_type: CommandType,
        item: &ComposeItem,
        args: &ArgMatches,
    ) -> Result<Output> {
        let output = CommandOuput::Status;
        let mut docker_commmand_arg = vec![OsStr::new("compose")];

        // Build additional arguments from dctl config file (path, env_file, etc.)
        let mut dctl_args: Vec<&OsStr> = vec![];

        if item.use_project_name.unwrap_or(true) {
            dctl_args.push(OsStr::new("-p"));
            dctl_args.push(OsStr::new(&item.alias));
        }

        match &item.enviroment_file {
            Some(env_file) => {
                dctl_args.push(OsStr::new("--env-file"));
                dctl_args.push(OsStr::new(env_file));
            }
            None => {}
        };

        item.compose_files.iter().for_each(|compose_file| {
            dctl_args.push(OsStr::new("-f"));
            dctl_args.push(OsStr::new(compose_file));
        });

        // Build command arguments from matches args & mix with dctl_args
        let mut args = match command_type {
            CommandType::Build => prepare_command_build(args, &mut dctl_args)?,
            CommandType::Create => prepare_command_create(args, &mut dctl_args)?,
            CommandType::Down => prepare_command_down(args, &mut dctl_args)?,
            CommandType::Exec => prepare_command_exec(args, &mut dctl_args)?,
            CommandType::Events => prepare_command_events(args, &mut dctl_args)?,
            CommandType::Images => prepare_command_images(args, &mut dctl_args)?,
            CommandType::Kill => prepare_command_kill(args, &mut dctl_args)?,
            CommandType::Ls => prepare_command_ls(args, &mut dctl_args)?,
            CommandType::Logs => prepare_command_logs(args, &mut dctl_args)?,
            CommandType::Pause => prepare_command_pause(args, &mut dctl_args)?,
            CommandType::Pull => prepare_command_pull(args, &mut dctl_args)?,
            CommandType::Push => prepare_command_push(args, &mut dctl_args)?,
            CommandType::Ps => prepare_command_ps(args, &mut dctl_args)?,
            CommandType::Restart => prepare_command_restart(args, &mut dctl_args)?,
            CommandType::Rm => prepare_command_rm(args, &mut dctl_args)?,
            CommandType::Run => prepare_command_run(args, &mut dctl_args)?,
            CommandType::Start => prepare_command_start(args, &mut dctl_args)?,
            CommandType::Stop => prepare_command_stop(args, &mut dctl_args)?,
            CommandType::Top => prepare_command_top(args, &mut dctl_args)?,
            CommandType::Unpause => prepare_command_unpause(args, &mut dctl_args)?,
            CommandType::Up => prepare_command_up(args, &mut dctl_args)?,
        };
        
        docker_commmand_arg.append(&mut args);

        // Build command
        let mut cmd: Command = builder(self.bin_path.to_owned(), docker_commmand_arg);

        // Execute command
        match output {
            CommandOuput::Status => {
                let status = cmd.status()?;
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
                let output = cmd.output()?;
                if output.status.success() {
                    Ok(output)
                } else {
                    Err(eyre!("Command failed"))
                }
            }
        }
    }
}
