use std::str::from_utf8;

use clap::Command;
use eyre::Result;
use tabled::{Margin, Style, Table};

use crate::command::ps::compose_ps;
use crate::parser::config::CliConfig;
use crate::utils::docker::{CommandOuput, CommandType, Container};

pub fn projects_infos() -> Command {
    Command::new("infos").about("Describe all projects with their status")
}

pub fn exec_projects_infos(config: &mut dyn CliConfig, container: &dyn Container) -> Result<()> {
    // Compare with our Dctl config.
    let mut items = config.get_all_compose_items();

    for item in &mut items {
        // Get all containers for this project
        let args_all = compose_ps().try_get_matches_from(vec!["ps", "-a", "-q", &item.alias])?;
        let ps_all =
            container.compose(CommandType::Ps, item, &args_all, Some(CommandOuput::Output))?;
        let output_all = from_utf8(&ps_all.stdout).unwrap();
        let all_containers_count = output_all.lines().count();

        // Get running containers for this project
        let args_run = compose_ps().try_get_matches_from(vec!["ps", "-q", &item.alias])?;
        let ps_run =
            container.compose(CommandType::Ps, item, &args_run, Some(CommandOuput::Output))?;
        let output_running = from_utf8(&ps_run.stdout).unwrap();
        let running_containers_count = output_running.lines().count();

        item.set_status(running_containers_count, all_containers_count)
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
