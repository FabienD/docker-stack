use crate::parser::ComposeItem;
use eyre::{eyre, Context, Result};
use std::process::Command;

enum DockerCommand {
    Start,
    Stop,
    Restart,
}

pub struct Docker {
    bin_path: String,
}

impl Docker {
    pub fn init(bin_path: String) -> Self {
        Docker { bin_path }
    }

    pub fn start(&self, item: &ComposeItem) -> Result<()> {
        self.execute(DockerCommand::Start, item)
    }

    pub fn stop(&self, item: &ComposeItem) -> Result<()> {
        self.execute(DockerCommand::Stop, item)
    }

    pub fn restart(&self, item: &ComposeItem) -> Result<()> {
        self.execute(DockerCommand::Restart, item)
    }

    fn execute(&self, command: DockerCommand, item: &ComposeItem) -> Result<()> {
        let mut cmd = Command::new(&self.bin_path);

        cmd.arg("compose")
            .arg("-p")
            .arg(item.alias.clone())
            .arg("--env-file")
            .arg(item.enviroment_file.clone());

        // Compose file(s)
        for compose_file in item.compose_files.clone() {
            cmd.arg("-f").arg(compose_file);
        }

        match command {
            DockerCommand::Start => cmd.arg("up").arg("-d"),
            DockerCommand::Stop => cmd.arg("stop"),
            DockerCommand::Restart => cmd.arg("restart"),
        };

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
