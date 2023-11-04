use super::filesystem;

use std::{env, fs};
use console::style;

pub fn run_use(version: String) {
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


    println!("🎉 You are now on Haxe {}", style(&version).yellow());
}
