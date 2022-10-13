use crate::parser::config::{ComposeItem, DctlConfig};
use eyre::{eyre, Result};
use serde_json::Value;
use std::ffi::OsStr;
use std::process::{Command, Output};
use tabled::{Margin, Style, Table};

use super::system::builder;

pub enum DockerCommand {
    Start,
    Stop,
    Down,
    Restart,
    Build,
    Logs,
    List,
    Ps,
    Exec,
}

pub enum CommandOuput {
    Status,
    Output,
}

pub struct Docker {
    bin_path: String,
}

/**
 * This command prepare the Rust standard Command and append all needed args
 */
pub fn prepare_command(
    bin_path: String,
    command: DockerCommand,
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
        DockerCommand::List => {
            args.push(OsStr::new("ls"));
            args.push(OsStr::new("--format"));
            args.push(OsStr::new("json"));
        }
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
        let _ = self.execute_command(
            DockerCommand::Start,
            Some(item),
            None,
            None,
            CommandOuput::Status,
        )?;
        Ok(())
    }

    pub fn stop(&self, item: &ComposeItem) -> Result<()> {
        let _ = self.execute_command(
            DockerCommand::Stop,
            Some(item),
            None,
            None,
            CommandOuput::Status,
        )?;
        Ok(())
    }

    pub fn down(&self, item: &ComposeItem) -> Result<()> {
        let _ = self.execute_command(
            DockerCommand::Down,
            Some(item),
            None,
            None,
            CommandOuput::Status,
        )?;
        Ok(())
    }

    pub fn restart(&self, item: &ComposeItem) -> Result<()> {
        let _ = self.execute_command(
            DockerCommand::Restart,
            Some(item),
            None,
            None,
            CommandOuput::Status,
        )?;
        Ok(())
    }

    pub fn build(&self, item: &ComposeItem, service: Option<String>) -> Result<()> {
        let _ = self.execute_command(
            DockerCommand::Build,
            Some(item),
            service,
            None,
            CommandOuput::Status,
        )?;
        Ok(())
    }

    pub fn logs(&self, item: &ComposeItem, service: Option<String>) -> Result<()> {
        let _ = self.execute_command(
            DockerCommand::Logs,
            Some(item),
            service,
            None,
            CommandOuput::Status,
        )?;
        Ok(())
    }

    pub fn ps(&self, item: &ComposeItem) -> Result<()> {
        let _ = self.execute_command(
            DockerCommand::Ps,
            Some(item),
            None,
            None,
            CommandOuput::Status,
        )?;
        Ok(())
    }

    pub fn exec(
        &self,
        item: &ComposeItem,
        service: Option<String>,
        subcommand: Option<String>,
    ) -> Result<()> {
        let _ = self.execute_command(
            DockerCommand::Exec,
            Some(item),
            service,
            subcommand,
            CommandOuput::Status,
        )?;
        Ok(())
    }

    pub fn list(&self, config: &mut DctlConfig) -> Result<()> {
        let cmd_output = self
            .execute_command(DockerCommand::List, None, None, None, CommandOuput::Output)
            .unwrap();
        let result: Value =
            serde_json::from_str(String::from_utf8(cmd_output.stdout).unwrap().as_str())?;

        // Compare with our Dctl config.
        let mut items = config.get_all_compose_items();

        for item in &mut items {
            result.as_array().unwrap().iter().for_each(|project| {
                // Relies on at least one compose file full path
                if project["ConfigFiles"]
                    .as_str()
                    .unwrap()
                    .split(',')
                    .any(|x| x == item.compose_files[0].as_str())
                {
                    item.set_status(true);
                }
            });
        }

        println!(
            "{}",
            Table::new(items)
                .with(Style::modern())
                .with(Margin::new(0, 0, 1, 1))
        );
        Ok(())
    }

    fn execute_command(
        &self,
        command: DockerCommand,
        item: Option<&ComposeItem>,
        service: Option<String>,
        subcommand: Option<String>,
        output: CommandOuput,
    ) -> Result<Output> {
        let mut cmd =
            prepare_command(self.bin_path.to_owned(), command, item, service, subcommand)?;

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
