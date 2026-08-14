use super::common;
use crate::cache_directory::Cache;
use crate::cache_directory::Version;
use color_eyre::eyre::{Result, eyre};
use flate2::read::GzDecoder;
use std::fs;
use std::io::Error;
use std::io::ErrorKind;
use tar::Archive;

pub fn download(cache: &Cache) -> Result<Version> {
    println!("Downloading latest Neko");

    let file_name: String = get_neko_archive()?;

    // Now we can find the url that matches that file name
    let binary_url = format!(
        "https://build.haxe.org/builds/neko/{}/{file}",
        get_sys_name().unwrap(),
        file = file_name
    );

    let path = format!("{}/bin/{file_name}", cache.location);
    common::download_file(binary_url.as_str(), &path).unwrap();

    Ok(Version {
        version: "neko".to_string(),
        archive_name: file_name.clone(),
        directory: get_neko_dir_name(cache, file_name.as_str())?,
    })
}

pub fn link_neko(cache: &Cache) -> Result<()> {
    // Check if not installed
    let directory = cache
        .find_version(&"neko".to_string())
        .ok_or_else(|| eyre!("Neko is not installed. Try running `haxeget install neko`"))?
        .version;

    common::link(cache, &directory, "neko", "neko")?;

    if cfg!(target_os = "windows") {
        println!(
            "Note: You will need to run `setx /M NEKO_INSTPATH {}` and add `%NEKO_INSTPATH%` to your PATH vars to use Neko!",
            Cache::get_path().unwrap() + "\\neko"
        );
    } /*else if std::env::var("HAXE_STD_PATH").is_err() { I don't know if there are similar variables for non windows systems
    println!("Note: You will need to add `export HAXE_STD_PATH={}/std/` to your shell config (i.e ~/.bashrc or ~/.zshrc)", Cache::get_path().unwrap());
    }*/

    Ok(())
}

pub fn get_neko_dir_name(cache: &Cache, file_name: &str) -> Result<String> {
    if cfg!(target_os = "windows") {
        get_extracted_dir_zip(cache)
    } else {
        get_extracted_dir_tar(cache, file_name)
    }
}

fn get_tarball(cache: &Cache, file_name: &str) -> Result<fs::File, Error> {
    let path = format!("{}/bin/neko/{file_name}", cache.location);

    match fs::File::open(path) {
        Ok(file) => Ok(file),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                let alt_path = format!("{}/bin/{file_name}", cache.location);
                fs::File::open(alt_path)
            }
            _ => Err(e),
        },
    }
}

fn get_extracted_dir_tar(cache: &Cache, file_name: &str) -> Result<String> {
    let tarball = get_tarball(cache, file_name)?;
    let tar = GzDecoder::new(tarball);
    let mut archive = Archive::new(tar);
    let mut name = String::from("neko/");

    // Get the name of the directory extracted
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
    };

    Ok(name)
}

fn get_extracted_dir_zip(cache: &Cache) -> Result<String> {
    let mut name = String::from("neko\\");

    let extracted_dir_path = format!("{}\\bin\\neko", cache.location);

    let mut extracted_dir = std::fs::read_dir(extracted_dir_path)?;

    // Get the name of the already extracted directory
    if let Some(dir) = extracted_dir.next() {
        let dir = dir.unwrap();
        name.push_str(
            dir.path()
                .file_name()
                .unwrap()
                .to_str()
                .expect("Unable to get extracted directory name"),
        );
    };

    Ok(name)
}

fn get_neko_archive() -> Result<String> {
    let mut file_name = String::new();

    file_name.push_str("neko_latest");
    if (cfg!(target_os = "linux") && cfg!(target_arch = "x86_64")) || cfg!(target_os = "macos") {
        file_name.push_str(".tar.gz");
    } else if cfg!(target_os = "windows") {
        file_name.push_str(".zip");
    } else {
        return Err(eyre!(
            "Your operating system and/or architecture is unsupported".to_owned()
        ));
    }

    Ok(file_name)
}

fn get_sys_name() -> Result<String> {
    let mut sys = String::new();
    if cfg!(target_os = "linux") && cfg!(target_arch = "x86_64") {
        sys.push_str("linux64");
    } else if cfg!(target_os = "macos") {
        sys.push_str("mac");
    } else if cfg!(target_os = "windows") {
        if cfg!(target_arch = "x86_64") {
            sys.push_str("windows64");
        } else {
            sys.push_str("windows");
        }
    } else {
        return Err(eyre!(
            "Your operating system and/or architecture is unsupported".to_owned()
        ));
    }

    Ok(sys)
}
