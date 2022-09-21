use crate::parser::ComposeItem;
use eyre::{eyre, Context, Result};
use std::process::Command;

enum DockerCommand {
    Start,
    Stop,
    Down,
    Restart,
    Build,
}

pub struct Docker {
    bin_path: String,
}

impl Docker {
    pub fn init(bin_path: String) -> Self {
        Docker { bin_path }
    }

    pub fn start(&self, item: &ComposeItem) -> Result<()> {
        self.execute(DockerCommand::Start, item, None)
    }

    pub fn stop(&self, item: &ComposeItem) -> Result<()> {
        self.execute(DockerCommand::Stop, item, None)
    }

    pub fn down(&self, item: &ComposeItem) -> Result<()> {
        self.execute(DockerCommand::Down, item, None)
    }

    pub fn restart(&self, item: &ComposeItem) -> Result<()> {
        self.execute(DockerCommand::Restart, item, None)
    }

    pub fn build(&self, item: &ComposeItem, service: Option<String>) -> Result<()> {
        self.execute(DockerCommand::Build, item, service)
    }

    fn execute(
        &self,
        command: DockerCommand,
        item: &ComposeItem,
        service: Option<String>,
    ) -> Result<()> {
        let mut cmd = Command::new(&self.bin_path);

        cmd.arg("compose").arg("-p").arg(item.alias.clone());

        let env_file = item.enviroment_file.clone().unwrap();

        if !env_file.is_empty() {
            cmd.arg("--env-file").arg(env_file);
        }

        // Compose file(s)
        for compose_file in item.compose_files.clone() {
            cmd.arg("-f").arg(compose_file);
        }

        match command {
            DockerCommand::Start => cmd.arg("up").arg("-d"),
            DockerCommand::Stop => cmd.arg("stop"),
            DockerCommand::Down => cmd.arg("down"),
            DockerCommand::Restart => cmd.arg("restart"),
            DockerCommand::Build => cmd.arg("build"),
        };

        if let Some(service) = service {
            cmd.arg(service);
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
