use super::filesystem;

use std::{env, fs};
use console::style;

pub fn run_use(version: String) {
    let directory = filesystem::get_directory_name();
    let dir = match directory {
        Ok(_) => directory.unwrap(),
        Err(error) => panic!("Uh oh! I was unable to find the directory: {}.\nPlease create an issue at: {}/issues", error, env!("CARGO_PKG_REPOSITORY")) 
    };

    link_binary(&dir, "haxe");
    link_binary(&dir, "haxelib");

    println!("🎉 You are now on Haxe {}", style(&version).yellow());
}

fn link_binary(directory: &str, name: &str) {
    let _ = fs::remove_file(format!("{directory}/{name}"));
    let link = std::os::unix::fs::symlink(format!("{directory}/bin/haxe_20230901120757_a6ac3ae/{name}"), format!("{directory}/{name}"));
    match link {
        Ok(_) => {},
        Err(error) => panic!("Uh oh! I was unable to create a symlink: {}.\nPlease create an issue at: {}/issues", error, env!("CARGO_PKG_REPOSITORY"))
    }
}
