use crate::parser::ComposeItem;
use eyre::{eyre, Context, Result};
use std::process::Command;

enum DockerCommand {
    Start,
    Stop,
    Down,
    Restart,
    Build,
    Ps,
    Exec,
}

pub struct Docker {
    bin_path: String,
}

impl Docker {
    pub fn init(bin_path: String) -> Self {
        Docker { bin_path }
    }

    pub fn start(&self, item: &ComposeItem) -> Result<()> {
        self.execute(DockerCommand::Start, item, None, None)
    }

    pub fn stop(&self, item: &ComposeItem) -> Result<()> {
        self.execute(DockerCommand::Stop, item, None, None)
    }

    pub fn down(&self, item: &ComposeItem) -> Result<()> {
        self.execute(DockerCommand::Down, item, None, None)
    }

    pub fn restart(&self, item: &ComposeItem) -> Result<()> {
        self.execute(DockerCommand::Restart, item, None, None)
    }

    pub fn build(&self, item: &ComposeItem, service: Option<String>) -> Result<()> {
        self.execute(DockerCommand::Build, item, service, None)
    }

    pub fn ps(&self, item: &ComposeItem) -> Result<()> {
        self.execute(DockerCommand::Ps, item, None, None)
    }

    pub fn exec(
        &self,
        item: &ComposeItem,
        service: Option<String>,
        subcommand: Option<String>,
    ) -> Result<()> {
        self.execute(DockerCommand::Exec, item, service, subcommand)
    }

    fn execute(
        &self,
        command: DockerCommand,
        item: &ComposeItem,
        service: Option<String>,
        subcommand: Option<String>,
    ) -> Result<()> {
        let mut cmd = Command::new(&self.bin_path);

        cmd.arg("compose").arg("-p").arg(item.alias.clone());

        match item.enviroment_file.clone() {
            Some(env_file) => {
                cmd.arg("--env-file").arg(env_file);
            }
            None => {}
        };

        // Compose file(s)
        for compose_file in item.compose_files.clone() {
            cmd.arg("-f").arg(compose_file);
        }

        match command {
            DockerCommand::Start => cmd.arg("up").arg("-d").arg("--remove-orphans"),
            DockerCommand::Stop => cmd.arg("stop"),
            DockerCommand::Down => cmd.arg("down"),
            DockerCommand::Restart => cmd.arg("restart"),
            DockerCommand::Build => cmd.arg("build"),
            DockerCommand::Ps => cmd.arg("ps"),
            DockerCommand::Exec => cmd.arg("exec"),
        };

        if let Some(service) = service {
            cmd.arg(service);
        }

        if let Some(subcommand) = subcommand {
            cmd.arg(subcommand);
        }

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
