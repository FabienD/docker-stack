use async_trait::async_trait;
use clap::ArgMatches;
use eyre::Result;
use std::ffi::OsStr;
use std::process::Output;

use crate::command::build::prepare_command_build;
use crate::command::create::prepare_command_create;
use crate::command::down::prepare_command_down;
use crate::command::events::prepare_command_events;
use crate::command::exec::prepare_command_exec;
use crate::command::images::prepare_command_images;
use crate::command::kill::prepare_command_kill;
use crate::command::logs::prepare_command_logs;
use crate::command::ls::prepare_command_ls;
use crate::command::pause::prepare_command_pause;
use crate::command::ps::prepare_command_ps;
use crate::command::pull::prepare_command_pull;
use crate::command::push::prepare_command_push;
use crate::command::restart::prepare_command_restart;
use crate::command::rm::prepare_command_rm;
use crate::command::run::prepare_command_run;
use crate::command::start::prepare_command_start;
use crate::command::stop::prepare_command_stop;
use crate::command::top::prepare_command_top;
use crate::command::unpause::prepare_command_unpause;
use crate::command::up::prepare_command_up;

use super::system::System;

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
        command_output: Option<CommandOuput>,
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
        command_output: Option<CommandOuput>,
    ) -> Result<Output> {
        let output = if let Some(output) = command_output {
            output
        } else {
            CommandOuput::Status
        };

        let cmd_args =
            Self::prepare_command(self, command, config_args, default_command_args, match_args);

        let cmd_ouput = System::execute(self.bin_path.to_owned(), &cmd_args, &output).await?;

        Ok(cmd_ouput)
    }
}

impl Docker {
    fn prepare_command<'a>(
        &'a self,
        command_type: CommandType,
        config_args: &Vec<&'a OsStr>,
        default_command_args: &Vec<&'a OsStr>,
        match_args: &'a ArgMatches,
    ) -> Vec<&OsStr> {
        // Build command arguments from matches args & mix with dctl_args
        let mut args = match command_type {
            CommandType::Build => prepare_command_build(match_args).unwrap(),
            CommandType::Create => prepare_command_create(match_args).unwrap(),
            CommandType::Down => prepare_command_down(match_args).unwrap(),
            CommandType::Exec => prepare_command_exec(match_args).unwrap(),
            CommandType::Events => prepare_command_events(match_args).unwrap(),
            CommandType::Images => prepare_command_images(match_args).unwrap(),
            CommandType::Kill => prepare_command_kill(match_args).unwrap(),
            CommandType::Ls => prepare_command_ls(match_args).unwrap(),
            CommandType::Logs => prepare_command_logs(match_args).unwrap(),
            CommandType::Pause => prepare_command_pause(match_args).unwrap(),
            CommandType::Pull => prepare_command_pull(match_args).unwrap(),
            CommandType::Push => prepare_command_push(match_args).unwrap(),
            CommandType::Ps => prepare_command_ps(match_args).unwrap(),
            CommandType::Restart => prepare_command_restart(match_args).unwrap(),
            CommandType::Rm => prepare_command_rm(match_args).unwrap(),
            CommandType::Run => prepare_command_run(match_args).unwrap(),
            CommandType::Start => prepare_command_start(match_args).unwrap(),
            CommandType::Stop => prepare_command_stop(match_args).unwrap(),
            CommandType::Top => prepare_command_top(match_args).unwrap(),
            CommandType::Unpause => prepare_command_unpause(match_args).unwrap(),
            CommandType::Up => prepare_command_up(match_args).unwrap(),
        };

        let mut docker_commmand_arg = vec![OsStr::new("compose")];
        docker_commmand_arg.append(&mut config_args.to_owned());
        docker_commmand_arg.append(&mut args);
        docker_commmand_arg.append(&mut default_command_args.to_owned());

        docker_commmand_arg
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsStr;

    use crate::command::build::compose_build;
    use crate::command::create::compose_create;
    use crate::command::down::compose_down;
    use crate::command::events::compose_events;
    use crate::command::exec::compose_exec;
    use crate::command::images::compose_images;
    use crate::command::kill::compose_kill;
    use crate::command::logs::compose_logs;
    use crate::command::ls::compose_ls;
    use crate::command::pause::compose_pause;
    use crate::command::ps::compose_ps;
    use crate::command::pull::compose_pull;
    use crate::command::push::compose_push;
    use crate::command::restart::compose_restart;
    use crate::command::rm::compose_rm;
    use crate::command::run::compose_run;
    use crate::command::start::compose_start;
    use crate::command::stop::compose_stop;
    use crate::command::top::compose_top;
    use crate::command::unpause::compose_unpause;
    use crate::command::up::compose_up;

    #[test]
    fn it_prepares_docker_compose_down() {
        let bin_path = "docker".to_string();
        let docker = Docker::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![];

        let matches = compose_down().get_matches_from(vec!["down", "PROJECT_NAME"]);

        let cmd_args = docker.prepare_command(
            CommandType::Down,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args = vec![
            OsStr::new("compose"),
            OsStr::new("-f"),
            OsStr::new("docker-compose.yml"),
            OsStr::new("down"),
        ];

        assert_eq!(cmd_args, expected_args);
    }

    #[test]
    fn it_prepare_docker_compose_build() {
        let bin_path = "docker".to_string();
        let docker = Docker::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![];

        let matches = compose_build().get_matches_from(vec!["build", "PROJECT_NAME"]);

        let cmd_args = docker.prepare_command(
            CommandType::Build,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args = vec![
            OsStr::new("compose"),
            OsStr::new("-f"),
            OsStr::new("docker-compose.yml"),
            OsStr::new("build"),
        ];

        assert_eq!(cmd_args, expected_args);
    }
    #[test]
    fn it_prepare_docker_compose_create() {
        let bin_path = "docker".to_string();
        let docker = Docker::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![];

        let matches = compose_create().get_matches_from(vec!["create", "PROJECT_NAME"]);

        let cmd_args = docker.prepare_command(
            CommandType::Create,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args = vec![
            OsStr::new("compose"),
            OsStr::new("-f"),
            OsStr::new("docker-compose.yml"),
            OsStr::new("create"),
        ];

        assert_eq!(cmd_args, expected_args);
    }

    #[test]
    fn it_prepare_docker_compose_exec() {
        let bin_path = "docker".to_string();
        let docker: Docker = Container::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![];

        let matches =
            compose_exec().get_matches_from(vec!["exec", "PROJECT_NAME", "SERVICE", "COMMAND"]);

        let cmd_args = docker.prepare_command(
            CommandType::Exec,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args = vec![
            OsStr::new("compose"),
            OsStr::new("-f"),
            OsStr::new("docker-compose.yml"),
            OsStr::new("exec"),
            OsStr::new("SERVICE"),
            OsStr::new("COMMAND"),
        ];

        assert_eq!(cmd_args, expected_args);
    }

    #[test]
    fn it_prepare_docker_compose_events() {
        let bin_path = "docker".to_string();
        let docker: Docker = Container::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![];

        let matches = compose_events().get_matches_from(vec!["events", "PROJECT_NAME"]);

        let cmd_args = docker.prepare_command(
            CommandType::Events,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args = vec![
            OsStr::new("compose"),
            OsStr::new("-f"),
            OsStr::new("docker-compose.yml"),
            OsStr::new("events"),
        ];

        assert_eq!(cmd_args, expected_args);
    }

    #[test]
    fn it_prepare_docker_compose_kill() {
        let bin_path = "docker".to_string();
        let docker: Docker = Container::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![];

        let matches = compose_kill().get_matches_from(vec!["kill", "PROJECT_NAME"]);

        let cmd_args = docker.prepare_command(
            CommandType::Kill,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args = vec![
            OsStr::new("compose"),
            OsStr::new("-f"),
            OsStr::new("docker-compose.yml"),
            OsStr::new("kill"),
        ];

        assert_eq!(cmd_args, expected_args);
    }

    #[test]
    fn it_prepare_docker_compose_images() {
        let bin_path = "docker".to_string();
        let docker: Docker = Container::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![];

        let matches = compose_images().get_matches_from(vec!["images", "PROJECT_NAME"]);

        let cmd_args = docker.prepare_command(
            CommandType::Images,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args = vec![
            OsStr::new("compose"),
            OsStr::new("-f"),
            OsStr::new("docker-compose.yml"),
            OsStr::new("images"),
        ];

        assert_eq!(cmd_args, expected_args);
    }

    #[test]
    fn it_prepare_docker_compose_logs() {
        let bin_path = "docker".to_string();
        let docker: Docker = Container::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![];

        let matches = compose_logs().get_matches_from(vec!["logs", "PROJECT_NAME"]);

        let cmd_args = docker.prepare_command(
            CommandType::Logs,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args = vec![
            OsStr::new("compose"),
            OsStr::new("-f"),
            OsStr::new("docker-compose.yml"),
            OsStr::new("logs"),
        ];

        assert_eq!(cmd_args, expected_args);
    }

    #[test]
    fn it_prepare_docker_compose_ls() {
        let bin_path = "docker".to_string();
        let docker: Docker = Container::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![];

        let matches = compose_ls().get_matches_from(vec!["ls", "PROJECT_NAME"]);

        let cmd_args = docker.prepare_command(
            CommandType::Ls,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args = vec![
            OsStr::new("compose"),
            OsStr::new("-f"),
            OsStr::new("docker-compose.yml"),
            OsStr::new("ls"),
        ];

        assert_eq!(cmd_args, expected_args);
    }

    #[test]
    fn it_prepare_docker_compose_pause() {
        let bin_path = "docker".to_string();
        let docker: Docker = Container::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![];

        let matches = compose_pause().get_matches_from(vec!["pause", "PROJECT_NAME"]);

        let cmd_args = docker.prepare_command(
            CommandType::Pause,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args = vec![
            OsStr::new("compose"),
            OsStr::new("-f"),
            OsStr::new("docker-compose.yml"),
            OsStr::new("pause"),
        ];

        assert_eq!(cmd_args, expected_args);
    }

    #[test]
    fn it_prepare_docker_compose_ps() {
        let bin_path = "docker".to_string();
        let docker: Docker = Container::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![];

        let matches = compose_ps().get_matches_from(vec!["ps", "PROJECT_NAME"]);

        let cmd_args = docker.prepare_command(
            CommandType::Ps,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args = vec![
            OsStr::new("compose"),
            OsStr::new("-f"),
            OsStr::new("docker-compose.yml"),
            OsStr::new("ps"),
        ];

        assert_eq!(cmd_args, expected_args);
    }

    #[test]
    fn it_prepare_docker_compose_pull() {
        let bin_path = "docker".to_string();
        let docker: Docker = Container::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![];

        let matches = compose_pull().get_matches_from(vec!["pull", "PROJECT_NAME"]);

        let cmd_args = docker.prepare_command(
            CommandType::Pull,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args = vec![
            OsStr::new("compose"),
            OsStr::new("-f"),
            OsStr::new("docker-compose.yml"),
            OsStr::new("pull"),
        ];

        assert_eq!(cmd_args, expected_args);
    }

    #[test]
    fn it_prepare_docker_compose_push() {
        let bin_path = "docker".to_string();
        let docker: Docker = Container::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![];

        let matches = compose_push().get_matches_from(vec!["push", "PROJECT_NAME"]);

        let cmd_args = docker.prepare_command(
            CommandType::Push,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args = vec![
            OsStr::new("compose"),
            OsStr::new("-f"),
            OsStr::new("docker-compose.yml"),
            OsStr::new("push"),
        ];

        assert_eq!(cmd_args, expected_args);
    }

    #[test]
    fn it_prepare_docker_compose_restart() {
        let bin_path = "docker".to_string();
        let docker: Docker = Container::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![];

        let matches = compose_restart().get_matches_from(vec!["restart", "PROJECT_NAME"]);

        let cmd_args = docker.prepare_command(
            CommandType::Restart,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args = vec![
            OsStr::new("compose"),
            OsStr::new("-f"),
            OsStr::new("docker-compose.yml"),
            OsStr::new("restart"),
        ];

        assert_eq!(cmd_args, expected_args);
    }

    #[test]
    fn it_prepare_docker_compose_rm() {
        let bin_path = "docker".to_string();
        let docker: Docker = Container::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![];

        let matches = compose_rm().get_matches_from(vec!["rm", "PROJECT_NAME"]);

        let cmd_args = docker.prepare_command(
            CommandType::Rm,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args = vec![
            OsStr::new("compose"),
            OsStr::new("-f"),
            OsStr::new("docker-compose.yml"),
            OsStr::new("rm"),
        ];

        assert_eq!(cmd_args, expected_args);
    }

    #[test]
    fn it_prepare_docker_compose_run() {
        let bin_path = "docker".to_string();
        let docker: Docker = Container::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![];

        let matches =
            compose_run().get_matches_from(vec!["run", "PROJECT_NAME", "SERVICE", "COMMAND"]);

        let cmd_args = docker.prepare_command(
            CommandType::Run,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args = vec![
            OsStr::new("compose"),
            OsStr::new("-f"),
            OsStr::new("docker-compose.yml"),
            OsStr::new("run"),
            OsStr::new("SERVICE"),
            OsStr::new("COMMAND"),
        ];

        assert_eq!(cmd_args, expected_args);
    }

    #[test]
    fn it_prepare_docker_compose_start() {
        let bin_path = "docker".to_string();
        let docker: Docker = Container::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![];

        let matches = compose_start().get_matches_from(vec!["start", "PROJECT_NAME"]);

        let cmd_args = docker.prepare_command(
            CommandType::Start,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args = vec![
            OsStr::new("compose"),
            OsStr::new("-f"),
            OsStr::new("docker-compose.yml"),
            OsStr::new("start"),
        ];

        assert_eq!(cmd_args, expected_args);
    }

    #[test]
    fn it_prepare_docker_compose_stop() {
        let bin_path = "docker".to_string();
        let docker: Docker = Container::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![];

        let matches = compose_stop().get_matches_from(vec!["stop", "PROJECT_NAME"]);

        let cmd_args = docker.prepare_command(
            CommandType::Stop,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args = vec![
            OsStr::new("compose"),
            OsStr::new("-f"),
            OsStr::new("docker-compose.yml"),
            OsStr::new("stop"),
        ];

        assert_eq!(cmd_args, expected_args);
    }

    #[test]
    fn it_prepare_docker_compose_top() {
        let bin_path = "docker".to_string();
        let docker: Docker = Container::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![];

        let matches = compose_top().get_matches_from(vec!["top", "PROJECT_NAME"]);

        let cmd_args = docker.prepare_command(
            CommandType::Top,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args = vec![
            OsStr::new("compose"),
            OsStr::new("-f"),
            OsStr::new("docker-compose.yml"),
            OsStr::new("top"),
        ];

        assert_eq!(cmd_args, expected_args);
    }

    #[test]
    fn it_prepare_docker_compose_unpause() {
        let bin_path = "docker".to_string();
        let docker: Docker = Container::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![];

        let matches = compose_unpause().get_matches_from(vec!["unpause", "PROJECT_NAME"]);

        let cmd_args = docker.prepare_command(
            CommandType::Unpause,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args = vec![
            OsStr::new("compose"),
            OsStr::new("-f"),
            OsStr::new("docker-compose.yml"),
            OsStr::new("unpause"),
        ];

        assert_eq!(cmd_args, expected_args);
    }

    #[test]
    fn it_prepares_docker_compose_up() {
        let bin_path = "docker".to_string();
        let docker: Docker = Container::init(bin_path.to_owned());

        let config_args = vec![OsStr::new("-f"), OsStr::new("docker-compose.yml")];
        let default_command_args = vec![OsStr::new("-d")];

        let matches = compose_up().get_matches_from(vec!["up", "PROJECT_NAME"]);

        let cmd_args = docker.prepare_command(
            CommandType::Up,
            &config_args,
            &default_command_args,
            &matches,
        );

        let expected_args = vec![
            OsStr::new("compose"),
            OsStr::new("-f"),
            OsStr::new("docker-compose.yml"),
            OsStr::new("up"),
            OsStr::new("-d"),
        ];

        assert_eq!(cmd_args, expected_args);
    }
}
