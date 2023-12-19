use super::common;
use crate::cache_directory::Cache;
use crate::github_schema;
use color_eyre::eyre::{eyre, Result};
use console::style;

/*
 * Gets the Haxe tarball from github
 */
pub async fn download(cache: &Cache, version: &String) -> Result<String> {
    let client = reqwest::Client::new();
    let json: github_schema::Root = client
        .get("https://api.github.com/repos/HaxeFoundation/haxe/releases")
        .header("User-Agent", "haxeget (https://github.com/l0go/haxeget)")
        .send()
        .await
        .expect("Was unable to connect to Github API")
        .json()
        .await
        .expect("Was unable to parse release JSON");

    let release = json
        .iter()
        .find(|&release| &release.name == version)
        .ok_or_else(|| eyre!("The specified version was not found"))?;

    println!("Downloading Haxe {}", style(&version).yellow());

    let file_name =
        common::get_archive_name(version).expect("Unable to infer the file name of the tar file");

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
