use crate::parser::config::ComposeItem;
use eyre::{eyre, Context, Result};
use std::{ffi::OsStr, process::Command};

use crate::command::builder;

pub enum DockerCommand {
    Start,
    Stop,
    Down,
    Restart,
    Build,
    Logs,
    Ps,
    Exec,
}

pub struct Docker {
    bin_path: String,
}

pub fn prepare_command(
    bin_path: String,
    command: DockerCommand,
    item: &ComposeItem,
    service: Option<String>,
    subcommand: Option<String>,
) -> Result<Command> {
    let mut args: Vec<&OsStr> = vec![];

    args.push(OsStr::new("compose"));
    args.push(OsStr::new("-p"));
    args.push(OsStr::new(&item.alias));

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

    match command {
        DockerCommand::Start => {
            args.push(OsStr::new("up"));
            args.push(OsStr::new("-d"));
            args.push(OsStr::new("--remove-orphans"));
        }
        DockerCommand::Stop => args.push(OsStr::new("stop")),
        DockerCommand::Down => args.push(OsStr::new("down")),
        DockerCommand::Restart => args.push(OsStr::new("restart")),
        DockerCommand::Build => args.push(OsStr::new("build")),
        DockerCommand::Logs => args.push(OsStr::new("logs")),
        DockerCommand::Ps => args.push(OsStr::new("ps")),
        DockerCommand::Exec => args.push(OsStr::new("exec")),
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
    pub fn init(bin_path: String) -> Self {
        Docker { bin_path }
    }

    pub fn start(&self, item: &ComposeItem) -> Result<()> {
        self.execute_command(DockerCommand::Start, item, None, None)
    }

    pub fn stop(&self, item: &ComposeItem) -> Result<()> {
        self.execute_command(DockerCommand::Stop, item, None, None)
    }

    pub fn down(&self, item: &ComposeItem) -> Result<()> {
        self.execute_command(DockerCommand::Down, item, None, None)
    }

    pub fn restart(&self, item: &ComposeItem) -> Result<()> {
        self.execute_command(DockerCommand::Restart, item, None, None)
    }

    pub fn build(&self, item: &ComposeItem, service: Option<String>) -> Result<()> {
        self.execute_command(DockerCommand::Build, item, service, None)
    }

    pub fn logs(&self, item: &ComposeItem, service: Option<String>) -> Result<()> {
        self.execute_command(DockerCommand::Logs, item, service, None)
    }

    pub fn ps(&self, item: &ComposeItem) -> Result<()> {
        self.execute_command(DockerCommand::Ps, item, None, None)
    }

    pub fn exec(
        &self,
        item: &ComposeItem,
        service: Option<String>,
        subcommand: Option<String>,
    ) -> Result<()> {
        self.execute_command(DockerCommand::Exec, item, service, subcommand)
    }

    fn execute_command(
        &self,
        command: DockerCommand,
        item: &ComposeItem,
        service: Option<String>,
        subcommand: Option<String>,
    ) -> Result<()> {
        let mut cmd =
            prepare_command(self.bin_path.to_owned(), command, item, service, subcommand)?;

        let status = cmd
            .status()
            .context("Failed to execute docker-compose command")?;

        if status.success() {
            Ok(())
        } else {
            Err(eyre!("Docker-compose command failed"))
        }
    }
}
