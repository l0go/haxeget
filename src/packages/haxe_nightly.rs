use super::common;
use crate::cache_directory::Cache;
use color_eyre::eyre::{eyre, Result};
use console::style;

pub async fn download(cache: &Cache) -> Result<String> {
    let client = reqwest::Client::new();

    println!("Downloading Haxe {}", style("nightly").yellow());

    let file_name: String =
        common::get_archive_name("nightly").expect("Unable to infer the file name of the tar file");

    // Now we can find the url that matches that file name
    let binary_url = format!(
        "https://build.haxe.org/builds/haxe/{}/{file}",
        get_sys_name().unwrap(),
        file = file_name
    );

    let path = format!("{}/bin/{file_name}", cache.location);
    common::download_file(&client, binary_url.as_str(), &path)
        .await
        .unwrap();

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
