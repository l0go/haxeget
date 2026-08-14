// Used to install haxeget itself
use super::common;
use crate::cache_directory::{Cache, Version};
use crate::github_schema;
use color_eyre::eyre::{Result, eyre};
use console::style;

/*
 * Gets the latest release of Haxeget
 */
pub fn download(cache: &Cache) -> Result<Version> {
    let json =
        github_schema::from_release_url("https://api.github.com/repos/l0go/haxeget/releases")?;

    let release = &json[0];

    println!("Downloading Haxeget {}", style(&release.tag_name).yellow());

    let file_name = get_haxeget_archive().expect("Unable to infer the file name of the tar file");

    // Now we can find the url that matches that file name
    let binary_url = &release
        .assets
        .iter()
        .find(|&asset| asset.name == file_name)
        .expect("There was not a valid asset for that version and target...")
        .browser_download_url;

    let path = format!("{}/bin/{file_name}", cache.location);
    common::download_file(binary_url, &path).unwrap();

    Ok(Version {
        version: "haxeget".to_string(),
        archive_name: file_name.clone(),
        directory: file_name,
    })
}

fn get_haxeget_archive() -> Result<String> {
    if cfg!(target_os = "linux") && cfg!(target_arch = "x86_64") {
        Ok("haxeget-x86_64-unknown-linux-gnu.tar.gz".to_owned())
    } else if cfg!(target_os = "macos") && cfg!(target_arch = "x86_64") {
        Ok("haxeget-x86_64-apple-darwin.tar.gz".to_owned())
    } else {
        Err(eyre!(
            "Your operating system and/or architecture is unsupported".to_owned()
        ))
    }
}
