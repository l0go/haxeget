pub mod github_schema;
pub mod filesystem;
pub mod download;
pub mod install_command;
pub mod use_command;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(alias("i"))]
    Install {
        version: String,
    },
    Use {
        version: String,
    }
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Install { version } => install_command::run_install(version).await,
        Commands::Use { version } => use_command::run_use(version),
    }
}

