use super::cache_directory::Cache;
use super::github_schema;

use color_eyre::eyre::{eyre, Result, WrapErr};
use console::style;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use std::cmp::min;
use std::fs;
use std::io::Write;

/*
 * Gets the specified executable from github
 * I hate this function, but I only have limited time
 */
pub async fn from_github(cache: &Cache, version: &String) -> Result<String> {
    let client = reqwest::Client::new();
    let json: github_schema::Root = client
        .get("https://api.github.com/repos/HaxeFoundation/haxe/releases")
        .header(
            "User-Agent",
            "haxeget (https://github.com/logo4poop/haxeget)",
        )
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
        get_tarball_name(version).expect("Unable to infer the file name of the tar file");

    // Now we can find the url that matches that file name
    let binary_url = &release
        .assets
        .iter()
        .find(|&asset| asset.name == file_name)
        .expect("There was not a valid asset for that version and target...")
        .browser_download_url;

    let path = format!("{}/bin/{file_name}", cache.location);
    download_file(&client, binary_url, &path).await.unwrap();

    Ok(file_name)
}

/*
 * Downloads a file and renders a pretty progress bar
 * "Borrowed" from https://gist.github.com/giuliano-oliveira/4d11d6b3bb003dba3a1b53f43d81b30d
 */
async fn download_file(client: &reqwest::Client, url: &str, path: &str) -> Result<()> {
    let res = client
        .get(url)
        .send()
        .await
        .or(Err(eyre!("Failed to GET from '{}'", &url)))?;
    let total_size = res
        .content_length()
        .ok_or_else(|| eyre!("Failed to get content length from '{}'", &url))?;

    // Indicatif setup
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::with_template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.yellow/red}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
                 .unwrap());

    // download chunks
    let mut file = fs::File::create(path).wrap_err("Failed to create file '{path}'")?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.wrap_err("Error while downloading file")?;
        file.write_all(&chunk)
            .wrap_err("Error while writing file")?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message("🎉 Done Downloading!".to_string());
    Ok(())
}

/*
 * Infers the name of the tarball
 */
pub fn get_tarball_name(version: &str) -> Result<String> {
    let mut file_name = String::from("haxe-") + version;

    if cfg!(target_os = "linux") && cfg!(target_arch = "x86_64") {
        file_name.push_str("-linux64.tar.gz");
    } else if cfg!(target_os = "macos") {
        file_name.push_str("-osx.tar.gz");
    } else {
        return Err(eyre!(
            "Your operating system and/or architecture is unsupported".to_owned()
        ));
    }

    Ok(file_name)
}
