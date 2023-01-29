use crate::parser::config::{CliConfig, ComposeItem};
use clap::ArgMatches;
use eyre::{eyre, Result};
use mockall::automock;
use serde_json::Value;
use std::ffi::OsStr;
use std::process::{Command, Output};
use tabled::{Margin, Style, Table};

use super::system::builder;

pub trait Container {
    fn init(bin_path: String) -> Self
    where
        Self: Sized;
    fn compose(&self, command: CommandType, item: &ComposeItem, args: &ArgMatches) -> Result<()>;
}

#[derive(Debug)]
pub enum CommandType {
    Build,
    Down,
    Exec,
    Ls,
    Logs,
    Ps,
    Restart,
    Run,
    Start,
    Stop,
    Top,
    Up,
}

pub enum CommandOuput {
    Status,
    Output,
}

pub struct Docker {
    bin_path: String,
}

pub(crate) fn prepare_command(
    bin_path: String,
    command: CommandType,
    item: Option<&ComposeItem>,
    service: Option<String>,
    subcommand: Option<String>,
) -> Result<Command> {
    let mut args: Vec<&OsStr> = vec![];

    args.push(OsStr::new("compose"));

    if let Some(item) = item {
        let use_project_name = item.use_project_name.unwrap_or(true);

        // By default, use the project name
        if use_project_name {
            args.push(OsStr::new("-p"));
            args.push(OsStr::new(&item.alias));
        }

        match &item.enviroment_file {
            Some(env_file) => {
                args.push(OsStr::new("--env-file"));
                args.push(OsStr::new(env_file));
            }
            None => {}
        };

        item.compose_files.iter().for_each(|compose_file| {
            args.push(OsStr::new("-f"));
            args.push(OsStr::new(compose_file));
        });
    }

    match command {
        CommandType::Up => {
            args.push(OsStr::new("up"));
            args.push(OsStr::new("-d"));
            args.push(OsStr::new("--remove-orphans"));
        }
        CommandType::Start => args.push(OsStr::new("start")),
        CommandType::Stop => args.push(OsStr::new("stop")),
        CommandType::Down => args.push(OsStr::new("down")),
        CommandType::Restart => args.push(OsStr::new("restart")),
        CommandType::Build => args.push(OsStr::new("build")),
        CommandType::Logs => args.push(OsStr::new("logs")),
        CommandType::Ls => {
            args.push(OsStr::new("ls"));
            args.push(OsStr::new("--format"));
            args.push(OsStr::new("json"));
        }
        CommandType::Ps => args.push(OsStr::new("ps")),
        CommandType::Exec => args.push(OsStr::new("exec")),
        CommandType::Run => args.push(OsStr::new("run")),
        _ => {}
    };

    match &service {
        Some(service) => {
            args.push(OsStr::new(service));
        }
        None => {}
    };

    match &subcommand {
        Some(subcommand) => {
            args.push(OsStr::new(subcommand));
        }
        None => {}
    };

    Ok(builder(bin_path, args))
}

impl Docker {

    fn execute_command(
        &self,
        command: CommandType,
        item: &ComposeItem,
        args: &ArgMatches,
    ) -> Result<()> {
        
        println!("command: {:?}", command);
        println!("args: {:?}", args.ids());

        // let mut cmd =
        //     prepare_command(self.bin_path.to_owned(), command, item, service, subcommand)?;

        // match output {
        //     CommandOuput::Status => {
        //         let status = cmd.status()?;
        //         if status.success() {
        //             Ok(Output {
        //                 status,
        //                 stdout: vec![],
        //                 stderr: vec![],
        //             })
        //         } else {
        //             Err(eyre!("Command failed"))
        //         }
        //     }
        //     CommandOuput::Output => {
        //         let output = cmd.output()?;
        //         if output.status.success() {
        //             Ok(output)
        //         } else {
        //             Err(eyre!("Command failed"))
        //         }
        //     }
        // }
        Ok(())
    }
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
        Self::execute_command(&self, command,item,args)?;
        Ok(())
    }
}
