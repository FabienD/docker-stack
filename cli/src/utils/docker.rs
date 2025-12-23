use async_trait::async_trait;
use clap::ArgMatches;
use anyhow::{anyhow, Result};
use std::ffi::{OsStr, OsString};
use std::process::Output;

use crate::command::registry::get_command_by_name;
use super::system::System;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    Port,
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
    Watch
}

impl CommandType {
    /// Returns the command name as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            CommandType::Build => "build",
            CommandType::Create => "create",
            CommandType::Down => "down",
            CommandType::Exec => "exec",
            CommandType::Events => "events",
            CommandType::Images => "images",
            CommandType::Kill => "kill",
            CommandType::Ls => "ls",
            CommandType::Logs => "logs",
            CommandType::Pause => "pause",
            CommandType::Port => "port",
            CommandType::Pull => "pull",
            CommandType::Push => "push",
            CommandType::Ps => "ps",
            CommandType::Restart => "restart",
            CommandType::Rm => "rm",
            CommandType::Run => "run",
            CommandType::Start => "start",
            CommandType::Stop => "stop",
            CommandType::Top => "top",
            CommandType::Unpause => "unpause",
            CommandType::Up => "up",
            CommandType::Watch => "watch",
        }
    }
}

pub enum CommandOutput {
    Status,
    Output,
}

#[derive(Debug, PartialEq)]
pub struct Docker {
    pub bin_path: String,
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
        match_args: &ArgMatches,
        command_output: Option<CommandOutput>,
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
        command_output: Option<CommandOutput>,
    ) -> Result<Output> {
        let output = if let Some(output) = command_output {
            output
        } else {
            CommandOutput::Status
        };

        let cmd_args =
            Self::prepare_command(self, command, config_args, default_command_args, match_args)?;

        let cmd_output = System::execute(self.bin_path.to_owned(), &cmd_args, &output).await?;

        Ok(cmd_output)
    }
}

impl Docker {
    fn prepare_command(
        &self,
        command_type: CommandType,
        config_args: &Vec<&OsStr>,
        default_command_args: &Vec<&OsStr>,
        match_args: &ArgMatches,
    ) -> Result<Vec<OsString>> {
        // Get the command handler from the registry
        let handler = get_command_by_name(command_type.as_str())
            .ok_or_else(|| anyhow!("Unknown command: {}", command_type.as_str()))?;

        // Build command arguments from matches args & mix with dctl_args
        let mut args = handler.prepare(match_args);

        // Build the full docker compose command
        let mut docker_command_arg: Vec<OsString> = vec![OsString::from("compose")];
        let mut only_args = args.split_off(1); // Remove first arg (command name)

        // Add config args (like -f docker-compose.yml)
        for arg in config_args {
            docker_command_arg.push(OsString::from(arg));
        }

        // Add command name
        docker_command_arg.append(&mut args);

        // Add default command args
        for arg in default_command_args {
            docker_command_arg.push(OsString::from(arg));
        }

        // Add the rest of the args
        docker_command_arg.append(&mut only_args);

        Ok(docker_command_arg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsStr;
    use crate::command::definitions::*;

    #[test]
    fn it_prepares_docker_compose_down() {
        let bin_path = "docker".to_string();
        let docker = Docker::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![];

        let def = down_def();
        let matches = def.to_clap_command().get_matches_from(vec!["down", "PROJECT_NAME"]);

        let cmd_args = docker.prepare_command(
            CommandType::Down,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args: Vec<OsString> = vec![
            OsString::from("compose"),
            OsString::from("-f"),
            OsString::from("docker-compose.yml"),
            OsString::from("down"),
        ];

        assert_eq!(cmd_args.unwrap(), expected_args);
    }

    #[test]
    fn it_prepare_docker_compose_build() {
        let bin_path = "docker".to_string();
        let docker = Docker::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![];

        let def = build_def();
        let matches = def.to_clap_command().get_matches_from(vec!["build", "PROJECT_NAME"]);

        let cmd_args = docker.prepare_command(
            CommandType::Build,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args: Vec<OsString> = vec![
            OsString::from("compose"),
            OsString::from("-f"),
            OsString::from("docker-compose.yml"),
            OsString::from("build"),
        ];

        assert_eq!(cmd_args.unwrap(), expected_args);
    }

    #[test]
    fn it_prepare_docker_compose_create() {
        let bin_path = "docker".to_string();
        let docker = Docker::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![];

        let def = create_def();
        let matches = def.to_clap_command().get_matches_from(vec!["create", "PROJECT_NAME"]);

        let cmd_args = docker.prepare_command(
            CommandType::Create,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args: Vec<OsString> = vec![
            OsString::from("compose"),
            OsString::from("-f"),
            OsString::from("docker-compose.yml"),
            OsString::from("create"),
        ];

        assert_eq!(cmd_args.unwrap(), expected_args);
    }

    #[test]
    fn it_prepare_docker_compose_exec() {
        let bin_path = "docker".to_string();
        let docker: Docker = Container::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![];

        let def = exec_def();
        let matches = def.to_clap_command().get_matches_from(vec!["exec", "PROJECT_NAME", "SERVICE"]);

        let cmd_args = docker.prepare_command(
            CommandType::Exec,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args: Vec<OsString> = vec![
            OsString::from("compose"),
            OsString::from("-f"),
            OsString::from("docker-compose.yml"),
            OsString::from("exec"),
            OsString::from("SERVICE"),
        ];

        assert_eq!(cmd_args.unwrap(), expected_args);
    }

    #[test]
    fn it_prepare_docker_compose_events() {
        let bin_path = "docker".to_string();
        let docker: Docker = Container::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![];

        let def = events_def();
        let matches = def.to_clap_command().get_matches_from(vec!["events", "PROJECT_NAME"]);

        let cmd_args = docker.prepare_command(
            CommandType::Events,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args: Vec<OsString> = vec![
            OsString::from("compose"),
            OsString::from("-f"),
            OsString::from("docker-compose.yml"),
            OsString::from("events"),
        ];

        assert_eq!(cmd_args.unwrap(), expected_args);
    }

    #[test]
    fn it_prepare_docker_compose_kill() {
        let bin_path = "docker".to_string();
        let docker: Docker = Container::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![];

        let def = kill_def();
        let matches = def.to_clap_command().get_matches_from(vec!["kill", "PROJECT_NAME"]);

        let cmd_args = docker.prepare_command(
            CommandType::Kill,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args: Vec<OsString> = vec![
            OsString::from("compose"),
            OsString::from("-f"),
            OsString::from("docker-compose.yml"),
            OsString::from("kill"),
        ];

        assert_eq!(cmd_args.unwrap(), expected_args);
    }

    #[test]
    fn it_prepare_docker_compose_images() {
        let bin_path = "docker".to_string();
        let docker: Docker = Container::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![];

        let def = images_def();
        let matches = def.to_clap_command().get_matches_from(vec!["images", "PROJECT_NAME"]);

        let cmd_args = docker.prepare_command(
            CommandType::Images,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args: Vec<OsString> = vec![
            OsString::from("compose"),
            OsString::from("-f"),
            OsString::from("docker-compose.yml"),
            OsString::from("images"),
        ];

        assert_eq!(cmd_args.unwrap(), expected_args);
    }

    #[test]
    fn it_prepare_docker_compose_logs() {
        let bin_path = "docker".to_string();
        let docker: Docker = Container::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![];

        let def = logs_def();
        let matches = def.to_clap_command().get_matches_from(vec!["logs", "PROJECT_NAME"]);

        let cmd_args = docker.prepare_command(
            CommandType::Logs,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args: Vec<OsString> = vec![
            OsString::from("compose"),
            OsString::from("-f"),
            OsString::from("docker-compose.yml"),
            OsString::from("logs"),
        ];

        assert_eq!(cmd_args.unwrap(), expected_args);
    }

    #[test]
    fn it_prepare_docker_compose_ls() {
        let bin_path = "docker".to_string();
        let docker: Docker = Container::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![];

        let def = ls_def();
        let matches = def.to_clap_command().get_matches_from(vec!["ls"]);

        let cmd_args = docker.prepare_command(
            CommandType::Ls,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args: Vec<OsString> = vec![
            OsString::from("compose"),
            OsString::from("-f"),
            OsString::from("docker-compose.yml"),
            OsString::from("ls"),
        ];

        assert_eq!(cmd_args.unwrap(), expected_args);
    }

    #[test]
    fn it_prepare_docker_compose_pause() {
        let bin_path = "docker".to_string();
        let docker: Docker = Container::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![];

        let def = pause_def();
        let matches = def.to_clap_command().get_matches_from(vec!["pause", "PROJECT_NAME"]);

        let cmd_args = docker.prepare_command(
            CommandType::Pause,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args: Vec<OsString> = vec![
            OsString::from("compose"),
            OsString::from("-f"),
            OsString::from("docker-compose.yml"),
            OsString::from("pause"),
        ];

        assert_eq!(cmd_args.unwrap(), expected_args);
    }

    #[test]
    fn it_prepare_docker_compose_port() {
        let bin_path = "docker".to_string();
        let docker: Docker = Container::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![];

        let def = port_def();
        let matches = def.to_clap_command().get_matches_from(vec!["port", "PROJECT_NAME", "SERVICE"]);

        let cmd_args = docker.prepare_command(
            CommandType::Port,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args: Vec<OsString> = vec![
            OsString::from("compose"),
            OsString::from("-f"),
            OsString::from("docker-compose.yml"),
            OsString::from("port"),
            OsString::from("SERVICE"),
        ];

        assert_eq!(cmd_args.unwrap(), expected_args);
    }

    #[test]
    fn it_prepare_docker_compose_ps() {
        let bin_path = "docker".to_string();
        let docker: Docker = Container::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![];

        let def = ps_def();
        let matches = def.to_clap_command().get_matches_from(vec!["ps", "PROJECT_NAME"]);

        let cmd_args = docker.prepare_command(
            CommandType::Ps,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args: Vec<OsString> = vec![
            OsString::from("compose"),
            OsString::from("-f"),
            OsString::from("docker-compose.yml"),
            OsString::from("ps"),
        ];

        assert_eq!(cmd_args.unwrap(), expected_args);
    }

    #[test]
    fn it_prepare_docker_compose_pull() {
        let bin_path = "docker".to_string();
        let docker: Docker = Container::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![];

        let def = pull_def();
        let matches = def.to_clap_command().get_matches_from(vec!["pull", "PROJECT_NAME"]);

        let cmd_args = docker.prepare_command(
            CommandType::Pull,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args: Vec<OsString> = vec![
            OsString::from("compose"),
            OsString::from("-f"),
            OsString::from("docker-compose.yml"),
            OsString::from("pull"),
        ];

        assert_eq!(cmd_args.unwrap(), expected_args);
    }

    #[test]
    fn it_prepare_docker_compose_push() {
        let bin_path = "docker".to_string();
        let docker: Docker = Container::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![];

        let def = push_def();
        let matches = def.to_clap_command().get_matches_from(vec!["push", "PROJECT_NAME"]);

        let cmd_args = docker.prepare_command(
            CommandType::Push,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args: Vec<OsString> = vec![
            OsString::from("compose"),
            OsString::from("-f"),
            OsString::from("docker-compose.yml"),
            OsString::from("push"),
        ];

        assert_eq!(cmd_args.unwrap(), expected_args);
    }

    #[test]
    fn it_prepare_docker_compose_restart() {
        let bin_path = "docker".to_string();
        let docker: Docker = Container::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![];

        let def = restart_def();
        let matches = def.to_clap_command().get_matches_from(vec!["restart", "PROJECT_NAME"]);

        let cmd_args = docker.prepare_command(
            CommandType::Restart,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args: Vec<OsString> = vec![
            OsString::from("compose"),
            OsString::from("-f"),
            OsString::from("docker-compose.yml"),
            OsString::from("restart"),
        ];

        assert_eq!(cmd_args.unwrap(), expected_args);
    }

    #[test]
    fn it_prepare_docker_compose_rm() {
        let bin_path = "docker".to_string();
        let docker: Docker = Container::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![];

        let def = rm_def();
        let matches = def.to_clap_command().get_matches_from(vec!["rm", "PROJECT_NAME"]);

        let cmd_args = docker.prepare_command(
            CommandType::Rm,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args: Vec<OsString> = vec![
            OsString::from("compose"),
            OsString::from("-f"),
            OsString::from("docker-compose.yml"),
            OsString::from("rm"),
        ];

        assert_eq!(cmd_args.unwrap(), expected_args);
    }

    #[test]
    fn it_prepare_docker_compose_run() {
        let bin_path = "docker".to_string();
        let docker: Docker = Container::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![OsStr::new("-i"), OsStr::new("--rm")];

        let def = run_def();
        let matches = def.to_clap_command().get_matches_from(vec!["run", "PROJECT_NAME", "SERVICE"]);

        let cmd_args = docker.prepare_command(
            CommandType::Run,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args: Vec<OsString> = vec![
            OsString::from("compose"),
            OsString::from("-f"),
            OsString::from("docker-compose.yml"),
            OsString::from("run"),
            OsString::from("-i"),
            OsString::from("--rm"),
            OsString::from("SERVICE"),
        ];

        assert_eq!(cmd_args.unwrap(), expected_args);
    }

    #[test]
    fn it_prepare_docker_compose_start() {
        let bin_path = "docker".to_string();
        let docker: Docker = Container::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![];

        let def = start_def();
        let matches = def.to_clap_command().get_matches_from(vec!["start", "PROJECT_NAME"]);

        let cmd_args = docker.prepare_command(
            CommandType::Start,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args: Vec<OsString> = vec![
            OsString::from("compose"),
            OsString::from("-f"),
            OsString::from("docker-compose.yml"),
            OsString::from("start"),
        ];

        assert_eq!(cmd_args.unwrap(), expected_args);
    }

    #[test]
    fn it_prepare_docker_compose_stop() {
        let bin_path = "docker".to_string();
        let docker: Docker = Container::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![];

        let def = stop_def();
        let matches = def.to_clap_command().get_matches_from(vec!["stop", "PROJECT_NAME"]);

        let cmd_args = docker.prepare_command(
            CommandType::Stop,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args: Vec<OsString> = vec![
            OsString::from("compose"),
            OsString::from("-f"),
            OsString::from("docker-compose.yml"),
            OsString::from("stop"),
        ];

        assert_eq!(cmd_args.unwrap(), expected_args);
    }

    #[test]
    fn it_prepare_docker_compose_top() {
        let bin_path = "docker".to_string();
        let docker: Docker = Container::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![];

        let def = top_def();
        let matches = def.to_clap_command().get_matches_from(vec!["top", "PROJECT_NAME"]);

        let cmd_args = docker.prepare_command(
            CommandType::Top,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args: Vec<OsString> = vec![
            OsString::from("compose"),
            OsString::from("-f"),
            OsString::from("docker-compose.yml"),
            OsString::from("top"),
        ];

        assert_eq!(cmd_args.unwrap(), expected_args);
    }

    #[test]
    fn it_prepare_docker_compose_unpause() {
        let bin_path = "docker".to_string();
        let docker: Docker = Container::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![];

        let def = unpause_def();
        let matches = def.to_clap_command().get_matches_from(vec!["unpause", "PROJECT_NAME"]);

        let cmd_args = docker.prepare_command(
            CommandType::Unpause,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args: Vec<OsString> = vec![
            OsString::from("compose"),
            OsString::from("-f"),
            OsString::from("docker-compose.yml"),
            OsString::from("unpause"),
        ];

        assert_eq!(cmd_args.unwrap(), expected_args);
    }

    #[test]
    fn it_prepares_docker_compose_up() {
        let bin_path = "docker".to_string();
        let docker: Docker = Container::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![OsStr::new("-d")];

        let def = up_def();
        let matches = def.to_clap_command().get_matches_from(vec!["up", "PROJECT_NAME"]);

        let cmd_args = docker.prepare_command(
            CommandType::Up,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args: Vec<OsString> = vec![
            OsString::from("compose"),
            OsString::from("-f"),
            OsString::from("docker-compose.yml"),
            OsString::from("up"),
            OsString::from("-d"),
        ];

        assert_eq!(cmd_args.unwrap(), expected_args);
    }

    #[test]
    fn it_prepares_docker_compose_watch() {
        let bin_path = "docker".to_string();
        let docker: Docker = Container::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![];

        let def = watch_def();
        let matches = def.to_clap_command().get_matches_from(vec!["watch", "PROJECT_NAME"]);

        let cmd_args = docker.prepare_command(
            CommandType::Watch,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args: Vec<OsString> = vec![
            OsString::from("compose"),
            OsString::from("-f"),
            OsString::from("docker-compose.yml"),
            OsString::from("watch"),
        ];

        assert_eq!(cmd_args.unwrap(), expected_args);
    }
}
