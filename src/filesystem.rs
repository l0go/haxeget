use flate2::read::GzDecoder;
use std::env;
use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io::{self, BufRead, Write};
use std::path::Path;
use tar::Archive;

pub fn get_directory_name() -> Result<String, String> {
    let mut directory_path = String::new();
    let home_dir = env::var("HOME").unwrap();

    if cfg!(target_os = "linux") && cfg!(target_arch = "x86_64") {
        directory_path.push_str(
            (env::var("XDG_BIN_HOME").unwrap_or((home_dir + "/.local/bin").to_owned())
                + "/haxeget")
                .as_str(),
        );
    } else if cfg!(target_os = "macos") {
        directory_path.push_str((home_dir + "/home/logo/.haxeget").as_str());
    } else {
        return Err("Your operating system and/or architecture is unsupported".to_owned());
    }

    Ok(directory_path)
}

pub fn get_file_name(version: &str) -> Result<String, String> {
    let mut file_name = String::from("haxe-") + version;

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

    installed
        .write_fmt(format_args!("{} {}\n", version, binary_directory))
        .expect("Cannot write to installed cache");
}

pub fn get_installed(version: &String) -> Option<String> {
    if let Ok(lines) = read_lines(get_directory_name().unwrap() + "/_current/installed") {
        for line in lines.flatten() {
            let mut cached_version = line.split_whitespace();
            let ver = cached_version.next().unwrap();
            let directory = cached_version.next().unwrap();

            if ver == version {
                return Some(directory.to_owned());
            }
        }
    }

    None
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<std::fs::File>>>
where
    P: AsRef<Path>,
{
    let file = std::fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Handles tarball
pub fn extract_tarball(directory: String, file_name: String) -> Result<(), Box<dyn Error>> {
    let tarball = fs::File::open(format!("{directory}/bin/{file_name}"))?;
    let tar = GzDecoder::new(tarball);
    let mut archive = Archive::new(tar);

    archive.unpack(format!("{directory}/bin/"))?;
    fs::remove_file(format!("{directory}/bin/{file_name}"))?;

    Ok(())
}

pub fn get_binary_directory(directory: &str, file_name: &str) -> Result<String, Box<dyn Error>> {
    let tarball = fs::File::open(format!("{directory}/bin/{file_name}"))?;
    let tar = GzDecoder::new(tarball);
    let mut archive = Archive::new(tar);

    // Get the name of the directory extracted
    let mut name = String::new();
    if let Some(file) = archive.entries().unwrap().next() {
        let file = file.unwrap();
        name.push_str(
            file.header()
                .path()
                .unwrap()
                .as_ref()
                .to_str()
                .expect("Unable to get extracted directory name"),
        );
        name.truncate(name.len() - 1);
    }

    Ok(name)
}
