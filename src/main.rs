pub mod cache_directory;
pub mod github_schema;
pub mod install_command;
pub mod list_command;
pub mod packages;
pub mod uninstall_command;
pub mod update_command;
pub mod use_command;

use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;

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
    #[command(alias("remove"), about = "Uninstalls the specified version")]
    Uninstall { version: String },
    #[command(alias("switch"), about = "Selects the version of Haxe to use")]
    Use { version: String },
    #[command(alias("ls"), about = "Lists the installed versions")]
    List,
    #[command(about = "Updates haxeget to the latest version")]
    Update,
    #[command(about = "Outputs the currently used Haxe version")]
    Current,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    color_eyre::install()?;

    match args.command {
        Commands::Install { version } => install_command::run_install(version).await?,
        Commands::Uninstall { version } => uninstall_command::run_uninstall(version)?,
        Commands::Use { version } => use_command::run_use(version)?,
        Commands::List => list_command::installed(),
        Commands::Update => update_command::run_update().await?,
        Commands::Current => list_command::current(),
    }

    Ok(())
}
