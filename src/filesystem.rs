use std::env;
use std::fs::OpenOptions;
use std::io::{self, Write, BufRead};
use std::path::Path;

pub fn get_directory_name() -> Result<String, String> {
    let mut directory_path = String::new();
    let home_dir = env::var("HOME").unwrap();

    if cfg!(target_os = "linux") && cfg!(target_arch = "x86_64") {
        directory_path.push_str((env::var("XDG_BIN_HOME").unwrap_or((home_dir + "/.local/bin").to_owned()) + "/haxeget").as_str());
    } else if cfg!(target_os = "macos") {
        directory_path.push_str((home_dir + "/home/logo/.haxeget").as_str());
    } else {
        return Err("Your operating system and/or architecture is unsupported".to_owned());
    }

    Ok(directory_path)
}

pub fn get_file_name(version: &String) -> Result<String, String> {
    let mut file_name = String::from("haxe-") + version.as_str();

    if cfg!(target_os = "linux") && cfg!(target_arch = "x86_64") {
        file_name.push_str("-linux64.tar.gz");
    } else if cfg!(target_os = "macos") {
        file_name.push_str("-osx.tar.gz");
    } else {
        return Err("Your operating system and/or architecture is unsupported".to_owned());
    }

    Ok(file_name)
}

/*
 * Adds a version to the installed cache
 * This is just a list of all of the versions that are currently installed
 */
pub fn add_version_to_installed(version: &String, binary_directory: String) {
    let mut installed = OpenOptions::new()
        .append(true)
        .create(true)
        .open(get_directory_name().unwrap() + "/_current/installed")
        .expect("Cannot open installed cache");

    installed.write_fmt(format_args!("{} {}\n", version, binary_directory))
        .expect("Cannot write to installed cache");
}

pub fn get_installed(version: &String) -> Option<String> {
    if let Ok(lines) = read_lines(get_directory_name().unwrap() + "/_current/installed") {
        for line in lines {
            if let Ok(cv) = line {
                let mut cached_version = cv.split_whitespace();
                let ver = cached_version.nth(0).unwrap();
                let directory = cached_version.nth(0).unwrap();

                if &ver == version {
                    return Some(directory.to_owned());
                }
            }
        }
    }

    None
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<std::fs::File>>>
where P: AsRef<Path>, {
    let file = std::fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
