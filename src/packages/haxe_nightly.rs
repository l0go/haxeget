use super::common;
use crate::cache_directory::{Cache, Version};
use color_eyre::eyre::{Result, eyre};
use console::style;

pub fn download(cache: &Cache) -> Result<Version> {
    println!("Downloading Haxe {}", style("nightly").yellow());

    let file_name: String = common::get_haxe_archive("nightly")?;

    // Now we can find the url that matches that file name
    let binary_url = format!(
        "https://build.haxe.org/builds/haxe/{}/{file}",
        get_sys_name().unwrap(),
        file = file_name
    );

    let path = format!("{}/bin/{file_name}", cache.location);
    common::download_file(binary_url.as_str(), &path).unwrap();

    // TODO: Use real version from file
    Ok(Version {
        version: "nightly".to_string(),
        archive_name: file_name.clone(),
        directory: cache.get_haxe_dir_name(file_name.as_str())?,
    })
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
