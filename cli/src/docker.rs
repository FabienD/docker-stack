use crate::parser::ComposeItem;
use eyre::{Context, Result};
use std::process::Command;

enum DockerCommand {
    Start,
    Stop,
    Restart,
}

pub fn start(item: ComposeItem) -> Result<()> {
    execute_compose_command(item, DockerCommand::Start).wrap_err("Docker start error")?;
    Ok(())
}

pub fn stop(item: ComposeItem) -> Result<()> {
    execute_compose_command(item, DockerCommand::Stop).wrap_err("Docker stop error")?;
    Ok(())
}

pub fn restart(item: ComposeItem) -> Result<()> {
    execute_compose_command(item, DockerCommand::Restart).wrap_err("Docker restart error")?;
    Ok(())
}

fn execute_compose_command(item: ComposeItem, command: DockerCommand) -> Result<()> {
    let mut dc_cmd = Command::new("docker");

    dc_cmd
        .arg("compose")
        .arg("-p")
        .arg(item.alias)
        .arg("--env-file")
        .arg(item.enviroment_file);

    for compose_file in item.compose_files {
        dc_cmd.arg("-f").arg(compose_file);
    }

    match command {
        DockerCommand::Start => dc_cmd.arg("up").arg("-d"),
        DockerCommand::Stop => dc_cmd.arg("stop"),
        DockerCommand::Restart => dc_cmd.arg("restart"),
    };

    dc_cmd
        .status()
        .expect("Failed to run docker compose command");

    Ok(())
}
