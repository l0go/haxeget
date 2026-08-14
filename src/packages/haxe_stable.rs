use super::common;
use crate::cache_directory::Cache;
use crate::github_schema;
use color_eyre::eyre::{eyre, Context, Result};
use console::style;

/*
 * Gets the Haxe archive from github
 */
pub fn download(cache: &Cache, version: &String) -> Result<String> {
    let json: github_schema::Root =
        ureq::get("https://api.github.com/repos/HaxeFoundation/haxe/releases")
            .header("User-Agent", "haxeget (https://github.com/l0go/haxeget)")
            .call()
            .wrap_err("Was unable to connect to Github API")?
            .into_body()
            .read_json()
            .wrap_err("Was unable to parse release JSON")?;

    let release = json
        .iter()
        .find(|&release| &release.name == version)
        .ok_or_else(|| eyre!("The specified version was not found"))?;

    println!("Downloading Haxe {}", style(&version).yellow());

    let file_name =
        common::get_haxe_archive(version).expect("Unable to infer the file name of the tar file");

    // Now we can find the url that matches that file name
    let binary_url = &release
        .assets
        .iter()
        .find(|&asset| asset.name == file_name)
        .expect("There was not a valid asset for that version and target...")
        .browser_download_url;

    let path = format!("{}/bin/{file_name}", cache.location);
    common::download_file(binary_url, &path).unwrap();

    Ok(file_name)
}
