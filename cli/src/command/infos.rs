use std::str::from_utf8;

use clap::Command;
use anyhow::{Context, Result};
use futures::future::join_all;
use tabled::{Table, settings::{Margin, Style}};

use crate::command::{definitions::ps_def, config::check_item_config};
use crate::parser::config::{CliConfig, ComposeItem};
use crate::utils::docker::{CommandOutput, CommandType, Container};

pub fn projects_infos() -> Command {
    Command::new("infos").about("Describe all projects with their status")
}

/// Check the status of a single project (running vs total containers)
async fn check_project_status(
    item: &ComposeItem,
    container: &dyn Container,
) -> Result<(isize, isize)> {
    // Check config first
    let config_check = check_item_config(item)?;
    if !config_check.is_empty() {
        return Ok((-1, -1)); // Config error
    }

    let config_args = ComposeItem::to_args(item);
    let ps_command = ps_def().to_clap_command();

    // Get all containers for this project
    let args_all = ps_command
        .clone()
        .try_get_matches_from(vec!["ps", "-a", "-q", &item.alias])?;
    let ps_all = container
        .compose(
            CommandType::Ps,
            &config_args,
            &vec![],
            &args_all,
            Some(CommandOutput::Output),
        )
        .await?;
    let output_all = from_utf8(&ps_all.stdout).context("Invalid UTF-8 in ps output")?;
    let all_containers_count = output_all.lines().count();

    // Get running containers for this project
    let args_run = ps_command.try_get_matches_from(vec!["ps", "-q", &item.alias])?;
    let ps_run = container
        .compose(
            CommandType::Ps,
            &config_args,
            &vec![],
            &args_run,
            Some(CommandOutput::Output),
        )
        .await?;
    let output_running = from_utf8(&ps_run.stdout).context("Invalid UTF-8 in ps output")?;
    let running_containers_count = output_running.lines().count();

    Ok((
        running_containers_count.try_into().unwrap_or(0),
        all_containers_count.try_into().unwrap_or(0),
    ))
}

pub async fn exec_projects_infos(
    config: &mut dyn CliConfig,
    container: &dyn Container,
) -> Result<()> {
    let mut items = config.get_all_compose_items();

    // Create futures for all project status checks
    let futures: Vec<_> = items
        .iter()
        .map(|item| check_project_status(item, container))
        .collect();

    // Execute all checks in parallel
    let results = join_all(futures).await;

    // Update items with results
    for (item, result) in items.iter_mut().zip(results.into_iter()) {
        match result {
            Ok((running, total)) => item.set_status(running, total),
            Err(_) => item.set_status(-1, -1), // Mark as error on failure
        }
    }

    // Print all projects with their status
    println!(
        "{}",
        Table::new(items)
            .with(Style::modern())
            .with(Margin::new(0, 0, 1, 1))
    );

    Ok(())
}