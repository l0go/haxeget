pub mod github_schema;
pub mod filesystem;
pub mod download;

use std::{env, fs};
use clap::{Parser, Subcommand};
use console::style;
use futures::executor;


//TODO: Consider removing tokio as a dependency
#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Install { version } => {
            println!("Downloading Haxe {}", style(&version).yellow());
            let download = executor::block_on(download::from_github(version));

            let _ = match download {
                Ok(dld) => download::extract_tarball(dld.directory, dld.file_name),
                Err(error) => panic!("Uh oh! Download failed: {}.\nPlease create an issue at: {}/issues", error, env!("CARGO_PKG_REPOSITORY"))
            };

            println!("Installation Complete!")
        },

        Commands::Use { version: _ } => {
            let directory = filesystem::get_directory_name();
            let dir = match directory {
                Ok(_) => directory.unwrap(),
                Err(error) => panic!("Uh oh! Was unable to find the directory: {}.\nPlease create an issue at: {}/issues", error, env!("CARGO_PKG_REPOSITORY")) 
            };


            let _ = fs::remove_file(format!("{dir}/haxe"));
            let haxe_link = std::os::unix::fs::symlink(format!("{dir}/bin/haxe_20230901120757_a6ac3ae/haxe"), format!("{dir}/haxe"));
            match haxe_link {
                Ok(_) => {},
                Err(error) => panic!("Uh oh! I was unable to create a symlink: {}", error)
            }
        }
    }
}

/*
 * CLI Arguments and Commands
 */
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
