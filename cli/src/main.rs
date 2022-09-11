use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about="A docker-compose missing feature.", long_about="Register docker-compose files, then, play with them whereever you are in the terminal.")]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

pub mod docker;
pub mod parser;

#[derive(Subcommand)]
enum Commands {
    /// List all the available docker-compose files in config file
    List,
    /// Start a docker-compose file
    Start {
        /// The name of the docker-compose file alias
        #[clap(value_parser)]
        name: String,
    },
    /// Stop a docker-compose file
    Stop {
        /// The name of the docker-compose file alias
        #[clap(value_parser)]
        name: String,
    },
     /// Restart a docker-compose file
     Restart {
        /// The name of the docker-compose file alias
        #[clap(value_parser)]
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::List => {
            println!("List docker-compose files under config file");
        }
        Commands::Start { name } => {
            println!("Start all containers in the {named} docker-compose file", named = name);
        },
        Commands::Stop { name } => {
            println!("Stop all containers in the  {named} docker-compose file", named = name);
        },
        Commands::Restart { name } => {
            println!("Stop all containers in the {named} docker-compose file", named = name);
        },
    }
}