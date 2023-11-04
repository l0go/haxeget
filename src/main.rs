pub mod download;
pub mod filesystem;
pub mod github_schema;
pub mod install_command;
pub mod list_command;
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
    #[command(alias("i"), about = "Installs the specified version")]
    Install { version: String },
    #[command(alias("switch"), about = "Selects the version of Haxe to use")]
    Use { version: String },
    #[command(alias("ls"), about = "Lists the installed versions")]
    List,
    #[command(about = "Outputs the currently used Haxe version")]
    Current,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Install { version } => install_command::run_install(version).await,
        Commands::Use { version } => use_command::run_use(version),
        Commands::List => list_command::installed(),
        Commands::Current => list_command::current(),
    }
}
