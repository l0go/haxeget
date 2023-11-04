use super::filesystem;

use console::style;
use std::{env, fs};

pub fn run_use(version: String) {
    let directory = filesystem::get_directory_name();
    
    // Check if installed already
    let tar_version = filesystem::get_installed(&version);
    if tar_version.is_none() {
        println!("This version is not installed. Try running {}", style(format!("haxeget install {}", version)).yellow());
        return;
    }
    let tar_version = tar_version.unwrap();

    let dir = match directory {
        Ok(_) => directory.unwrap(),
        Err(error) => panic!(
            "Uh oh! I was unable to find the directory: {}.\nPlease create an issue at: {}/issues",
            error,
            env!("CARGO_PKG_REPOSITORY")
        ),
    };

    link_binary(&tar_version, &dir, "haxe");
    link_binary(&tar_version, &dir, "haxelib");

    println!("🎉 You are now on Haxe {}", style(&version).yellow());
}

fn link_binary(version: &str, directory: &str, name: &str) {
    let _ = fs::remove_file(format!("{directory}/{name}"));
    let link = std::os::unix::fs::symlink(
        format!("{directory}/bin/{version}/{name}"),
        format!("{directory}/{name}"),
    );
    match link {
        Ok(_) => {}
        Err(error) => panic!(
            "Uh oh! I was unable to create a symlink: {}.\nPlease create an issue at: {}/issues",
            error,
            env!("CARGO_PKG_REPOSITORY")
        ),
    }
}
