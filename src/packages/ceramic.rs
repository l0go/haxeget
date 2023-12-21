// Used to install the Ceramic graphics library
// Ceramic packages it's own version of Haxe, so it is included here
use super::common;
use crate::cache_directory::Cache;
use crate::github_schema;
use color_eyre::eyre::{eyre, Result};
use console::style;

/*
 * Gets the latest release of Ceramic
 */
pub async fn download(cache: &Cache) -> Result<String> {
    let client = reqwest::Client::new();
    let json: github_schema::Root = client
        .get("https://api.github.com/repos/ceramic-engine/ceramic/releases")
        .header("User-Agent", "haxeget (https://github.com/l0go/haxeget)")
        .send()
        .await
        .expect("Was unable to connect to Github API")
        .json()
        .await
        .expect("Was unable to parse release JSON");

    let release = &json[0];

    println!("Downloading Ceramic {}", style(&release.tag_name).yellow());

    let file_name = get_ceramic_archive().expect("Unable to infer the file name of the tar file");

    // Now we can find the url that matches that file name
    let binary_url = &release
        .assets
        .iter()
        .find(|&asset| asset.name == file_name)
        .expect("There was not a valid asset for that version and target...")
        .browser_download_url;

    let path = format!("{}/bin/{file_name}", cache.location);
    common::download_file(&client, binary_url, &path)
        .await
        .unwrap();

    Ok(file_name)
}

pub fn link_ceramic(cache: &Cache) -> Result<()> {
    // Check if not installed
    let tar_version = cache
        .find_version(&"ceramic".to_string())
        .ok_or_else(|| eyre!("Ceramic is not installed. Try running `haxeget install ceramic`"))?;

    if cfg!(target_os = "windows") {
        common::link(cache, &tar_version, "tools/ceramic.bat", "ceramic.bat")?;
    } else {
        common::link(cache, &tar_version, "tools/ceramic", "ceramic")?;
    }

    if cfg!(target_os = "windows") {
        println!("Note: You will need to run `setx /M HAXEPATH {}` and add `%HAXEPATH%` to your PATH vars to use this version of Haxe!", Cache::get_path().unwrap() + "\\haxe");
    } else if std::env::var("HAXE_STD_PATH").is_err() {
        println!("Note: You will need to add `export HAXE_STD_PATH={}/std/` to your shell config (i.e ~/.bashrc or ~/.zshrc)", Cache::get_path().unwrap());
    }

    Ok(())
}

fn get_ceramic_archive() -> Result<String> {
    if cfg!(target_os = "linux") && cfg!(target_arch = "x86_64") {
        Ok("ceramic-linux.zip".to_owned())
    } else if cfg!(target_os = "macos") && cfg!(target_arch = "x86_64") {
        Ok("ceramic-mac.zip".to_owned())
    } else if cfg!(target_os = "windows") && cfg!(target_arch = "x86_64") {
        Ok("ceramic-windows.zip".to_owned())
    } else {
        Err(eyre!(
            "Your operating system and/or architecture is unsupported".to_owned()
        ))
    }
}
