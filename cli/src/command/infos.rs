use clap::{Command};
use eyre::Result;

use crate::parser::config::CliConfig;

pub fn projects_infos() -> Command {
    Command::new("infos")
        .about("Describe all projects with their status")
}

pub fn exec_projects_infos(config: &mut dyn CliConfig) -> Result<()> {
    // Compare with our Dctl config.
    let mut items = config.get_all_compose_items();

    for item in &mut items {
        println!("{:?}", item);
    }
    
    Ok(())
}


    // fn list(&self, config: &mut dyn CliConfig) -> Result<()> {
    //     let cmd_output = self
    //         .execute_command(CommandType::List, None, None, None, CommandOuput::Output)
    //         .unwrap();
    //     let result: Value =
    //         serde_json::from_str(String::from_utf8(cmd_output.stdout).unwrap().as_str())?;

    //     // Compare with our Dctl config.
    //     let mut items = config.get_all_compose_items();

    //     for item in &mut items {
    //         result.as_array().unwrap().iter().for_each(|project| {
    //             // Run compose ps -a -q to get the number of containers
    //             let cmd_all_containers = self
    //                 .execute_command(
    //                     CommandType::Ps,
    //                     Some(item),
    //                     None,
    //                     None,
    //                     CommandOuput::Output,
    //                 )
    //                 .unwrap();
    //             print!("{:?}", cmd_all_containers);
    //             //let all_containers = cmd_all_containers.stdout.len();

    //             // Run compose ps -q --status running to get the number of running containers
    //             /*
    //             let cmd_running_containers = self
    //                 .execute_command(
    //                     CommandType::Ps,
    //                     Some(item),
    //                     None,
    //                     Some(String::from("-q --status running --format json")),
    //                     CommandOuput::Output,
    //                 )
    //                 .unwrap();
    //             let running_containers = cmd_running_containers.stdout.len();
    //             */
    //             // Relies on at least one compose file full path
    //             /*
    //             if project["ConfigFiles"]
    //                 .as_str()
    //                 .unwrap()
    //                 .split(',')
    //                 .any(|x| x == item.compose_files[0].as_str())
    //             {
    //                 //item.set_status(running_containers, all_containers);
    //                 item.set_status(1, 1);
    //             }
    //              */
    //         });
    //     }

    //     println!(
    //         "{}",
    //         Table::new(items)
    //             .with(Style::modern())
    //             .with(Margin::new(0, 0, 1, 1))
    //     );
    //     Ok(())
    // }