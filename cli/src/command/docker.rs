use crate::parser::config::{CliConfig, ComposeItem};
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
    fn up(&self, item: &ComposeItem) -> Result<()>;
    fn start(&self, item: &ComposeItem) -> Result<()>;
    fn stop(&self, item: &ComposeItem) -> Result<()>;
    fn down(&self, item: &ComposeItem) -> Result<()>;
    fn restart(&self, item: &ComposeItem) -> Result<()>;
    fn ps(&self, item: &ComposeItem) -> Result<()>;
    fn build(&self, item: &ComposeItem, service: Option<String>) -> Result<()>;
    fn logs(&self, item: &ComposeItem, service: Option<String>) -> Result<()>;
    fn exec(
        &self,
        item: &ComposeItem,
        service: Option<String>,
        subcommand: Option<String>,
    ) -> Result<()>;
    fn run(
        &self,
        item: &ComposeItem,
        service: Option<String>,
        subcommand: Option<String>,
    ) -> Result<()>;
    fn list(&self, config: &mut dyn CliConfig) -> Result<()>;
}

pub enum CommandType {
    Up,
    Start,
    Stop,
    Down,
    Restart,
    Build,
    Logs,
    List,
    Ps,
    Exec,
    Run,
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
        },
        CommandType::Start => args.push(OsStr::new("start")),
        CommandType::Stop => args.push(OsStr::new("stop")),
        CommandType::Down => args.push(OsStr::new("down")),
        CommandType::Restart => args.push(OsStr::new("restart")),
        CommandType::Build => args.push(OsStr::new("build")),
        CommandType::Logs => args.push(OsStr::new("logs")),
        CommandType::List => {
            args.push(OsStr::new("ls"));
            args.push(OsStr::new("--format"));
            args.push(OsStr::new("json"));
        }
        CommandType::Ps => args.push(OsStr::new("ps")),
        CommandType::Exec => args.push(OsStr::new("exec")),
        CommandType::Run => args.push(OsStr::new("run")),
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

#[automock]
impl Container for Docker {
    fn init(bin_path: String) -> Self
    where
        Self: Sized,
    {
        Docker { bin_path }
    }

    fn up(&self, item: &ComposeItem) -> Result<()> {
        let _ = self.execute_command(
            CommandType::Up,
            Some(item),
            None,
            None,
            CommandOuput::Status,
        )?;
        Ok(())
    }

    fn start(&self, item: &ComposeItem) -> Result<()> {
        let _ = self.execute_command(
            CommandType::Start,
            Some(item),
            None,
            None,
            CommandOuput::Status,
        )?;
        Ok(())
    }

    fn stop(&self, item: &ComposeItem) -> Result<()> {
        let _ = self.execute_command(
            CommandType::Stop,
            Some(item),
            None,
            None,
            CommandOuput::Status,
        )?;
        Ok(())
    }

    fn down(&self, item: &ComposeItem) -> Result<()> {
        let _ = self.execute_command(
            CommandType::Down,
            Some(item),
            None,
            None,
            CommandOuput::Status,
        )?;
        Ok(())
    }

    fn restart(&self, item: &ComposeItem) -> Result<()> {
        let _ = self.execute_command(
            CommandType::Restart,
            Some(item),
            None,
            None,
            CommandOuput::Status,
        )?;
        Ok(())
    }

    fn build(&self, item: &ComposeItem, service: Option<String>) -> Result<()> {
        let _ = self.execute_command(
            CommandType::Build,
            Some(item),
            service,
            None,
            CommandOuput::Status,
        )?;
        Ok(())
    }

    fn logs(&self, item: &ComposeItem, service: Option<String>) -> Result<()> {
        let _ = self.execute_command(
            CommandType::Logs,
            Some(item),
            service,
            None,
            CommandOuput::Status,
        )?;
        Ok(())
    }

    fn ps(&self, item: &ComposeItem) -> Result<()> {
        let _ = self.execute_command(
            CommandType::Ps,
            Some(item),
            None,
            None,
            CommandOuput::Status,
        )?;
        Ok(())
    }

    fn exec(
        &self,
        item: &ComposeItem,
        service: Option<String>,
        subcommand: Option<String>,
    ) -> Result<()> {
        let _ = self.execute_command(
            CommandType::Exec,
            Some(item),
            service,
            subcommand,
            CommandOuput::Status,
        )?;
        Ok(())
    }

    fn run(
        &self,
        item: &ComposeItem,
        service: Option<String>,
        subcommand: Option<String>,
    ) -> Result<()> {
        let _ = self.execute_command(
            CommandType::Run,
            Some(item),
            service,
            subcommand,
            CommandOuput::Status,
        )?;
        Ok(())
    }

    fn list(&self, config: &mut dyn CliConfig) -> Result<()> {
        let cmd_output = self
            .execute_command(CommandType::List, None, None, None, CommandOuput::Output)
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
}
