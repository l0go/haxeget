use super::filesystem;
use std::fs;

pub fn installed() {
    for version in filesystem::get_installed().unwrap().flatten() {
        let version = version.split_whitespace().next().unwrap();
        println!("{version}");
    }
}

pub fn current() {
    let directory = filesystem::get_directory_name();
    let dir = match directory {
        Ok(_) => directory.unwrap(),
        Err(error) => panic!(
            "Uh oh! I was unable to find the directory: {}.\nPlease create an issue at: {}/issues",
            error,
            env!("CARGO_PKG_REPOSITORY")
        ),
    };
    let version = fs::read_to_string(dir + "/_current/haxe_version").unwrap();
    let version = version.split_whitespace().next().unwrap();
    println!("Haxe {}", version);
}
