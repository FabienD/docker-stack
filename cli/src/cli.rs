use clap::Command;
use anyhow::{anyhow, Context, Result};
use std::ffi::OsStr;
use std::process::exit;

use crate::parser::config::{CliConfig, ComposeItem, DefaultCommandArgs};
use crate::utils::docker::Container;

use crate::command::cd::{cd_project, exec_cd_project};
use crate::command::completion::{exec_shell_completion, shell_completion};
use crate::command::config::{check_config, exec_check_config};
use crate::command::infos::{exec_projects_infos, projects_infos};
use crate::command::register::{exec_register_project, register_project};
use crate::command::registry::{get_compose_commands, get_command_by_name};
use crate::command::unregister::{exec_unregister_project, unregister_project};

fn cli() -> Command {
    let mut cmd = Command::new("dctl")
        .about("A docker-compose missing feature.")
        .long_about(
            "Register docker-compose files, then, play with them whereever you are in the terminal",
        )
        .version(version!())
        .author("Fabien D. <fabien@myprod.net>")
        .subcommand_required(true)
        .arg_required_else_help(true);

    // Add all docker compose commands from registry
    for handler in get_compose_commands() {
        cmd = cmd.subcommand(handler.cli());
    }

    // Add other commands
    cmd = cmd
        .subcommand(shell_completion())
        .subcommand(cd_project())
        .subcommand(check_config())
        .subcommand(projects_infos())
        .subcommand(register_project())
        .subcommand(unregister_project());

    cmd
}

pub async fn run(container: &dyn Container, config: &mut dyn CliConfig) -> Result<()> {
    // Get the command name and args
    let matches = cli().get_matches();
    let (command_name, args) = matches.subcommand().context("No subcommand provided")?;
    let default_command_args = config.get_default_command_args(command_name);

    // Handle special commands that don't need a project
    match command_name {
        "infos" => {
            exec_projects_infos(config, container).await?;
            return Ok(());
        }
        "check-config" => {
            exec_check_config(config, container, args).await?;
            return Ok(());
        }
        "completion" => {
            exec_shell_completion(&mut cli(), args)?;
            return Ok(());
        }
        "register" => {
            exec_register_project(config, args)?;
            return Ok(());
        }
        "unregister" => {
            exec_unregister_project(config, args)?;
            return Ok(());
        }
        _ => {}
    }

    // For next commands, we need a project
    if args.try_contains_id("PROJECT").is_err() {
        exit(1)
    }

    // Get the compose item for the project
    let compose_item = match args.get_one::<String>("PROJECT") {
        Some(name) => match config.get_compose_item_by_alias(name.to_string()) {
            Some(item) => item,
            None => return Err(anyhow!("No project found with alias: {}", name)),
        },
        None => exit(1),
    };

    // Handle cd command
    if command_name == "cd" {
        let _result = exec_cd_project(&compose_item);
        exit(0);
    }

    // Build configuration args
    let mut config_args: Vec<&OsStr> = vec![];
    config_args.append(&mut ComposeItem::to_args(&compose_item));

    // Get default command args
    let command_args = match default_command_args {
        Some(command_args) => command_args,
        None => DefaultCommandArgs::default(command_name),
    };
    let default_command_arg = DefaultCommandArgs::to_args(&command_args);

    // Execute docker compose command using registry
    if let Some(handler) = get_command_by_name(command_name) {
        container
            .compose(
                handler.command_type(),
                &config_args,
                &default_command_arg,
                args,
                None,
            )
            .await?;
    } else {
        return Err(anyhow!("Unknown command: {}", command_name));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_verifies_the_cli() {
        cli().debug_assert();
    }
}