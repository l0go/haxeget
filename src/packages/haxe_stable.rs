use super::common;
use crate::cache_directory::Version;
use crate::github_schema;
use crate::{cache_directory::Cache, github_schema::Release};
use color_eyre::eyre::{Result, eyre};
use console::style;

/*
 * Gets the Haxe archive from github
 */
pub fn download(cache: &Cache, version: &String) -> Result<Version> {
    let json = github_schema::from_release_url(
        "https://api.github.com/repos/HaxeFoundation/haxe/releases",
    )?;

    let release: Release = if version != "latest" {
        json.iter()
            .find(|&release| &release.name == version)
            .ok_or_else(|| eyre!("The specified version was not found"))?
            .clone()
    } else {
        json.iter()
            .find(|&release| !release.prerelease)
            .ok_or_else(|| eyre!("No available stable version found"))?
            .clone()
    };

    // Check if installed already
    if cache.find_version(&release.name).is_some() {
        return Err(eyre!("The specified version is already installed!"));
    }

    println!("Downloading Haxe {}", style(&release.name).yellow());

    let file_name = common::get_haxe_archive(&release.name)
        .expect("Unable to infer the file name of the tar file");

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
        version: release.name,
        archive_name: file_name.clone(),
        directory: cache.get_haxe_dir_name(file_name.as_str())?,
    })
}
