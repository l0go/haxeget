use super::filesystem;
use super::github_schema;

use console::style;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use std::cmp::min;
use std::fs;
use std::io::Write;

pub struct Download {
    pub file_name: String,
    pub directory: String,
}

/*
 * Gets the executable from github and installs
 */
pub async fn from_github(version: &String) -> Result<Download, ()> {
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
        .find(|&release| &release.name == version);

    let release = match release {
        Some(_) => release.unwrap(),
        None => {
            println!("That version was not found");
            std::process::exit(0);
        },
    };

    println!("Downloading Haxe {}", style(&version).yellow());

    // Figure out the file name based on the target
    // Currently only supports linux and macOS
    let mut file_name = String::from("haxe-");
    file_name.push_str(version);

    let directory = filesystem::get_directory_name();
    let directory = match directory {
        Ok(_) => directory.unwrap(),
        Err(error) => panic!(
            "Uh oh! I was unable to find the directory: {}.\nPlease create an issue at: {}/issues",
            error,
            env!("CARGO_PKG_REPOSITORY")
        ),
    };

    let file_name = match filesystem::get_file_name(version) {
        Ok(_) => file_name,
        Err(error) => panic!(
            "Uh oh! I was unable to infer the file name of the tar file: {}.\nPlease create an issue at: {}/issues",
            error,
            env!("CARGO_PKG_REPOSITORY")
        ),
    };

    // Create the working directory if it doesn't exist 
    if let Err(error) = fs::create_dir_all(directory.to_owned() + "/bin") {
        panic!(
            "Uh oh! I was unable to create the working directory: {:?}.\nPlease create an issue at: {}/issues",
            error,
            env!("CARGO_PKG_REPOSITORY")
        );
    }


    // Now we can find the url that matches that file name
    let binary_url = &release
        .assets
        .iter()
        .find(|&asset| asset.name == file_name)
        .expect("There was not a valid asset for that version and target...")
        .browser_download_url;

    let path = format!("{directory}/bin/{file_name}");
    download_file(&client, binary_url, &path).await.unwrap();

    Ok(Download {
        file_name,
        directory,
    })
}

// "Borrowed" from https://gist.github.com/giuliano-oliveira/4d11d6b3bb003dba3a1b53f43d81b30d
async fn download_file(client: &reqwest::Client, url: &str, path: &str) -> Result<(), String> {
    let res = client
        .get(url)
        .send()
        .await
        .or(Err(format!("Failed to GET from '{}'", &url)))?;
    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", &url))?;

    // Indicatif setup
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::with_template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.yellow/red}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
                 .unwrap());

    // download chunks
    let mut file = fs::File::create(path).or(Err(format!("Failed to create file '{}'", path)))?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err("Error while downloading file".to_string()))?;
        file.write_all(&chunk)
            .or(Err("Error while writing to file".to_string()))?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message("🎉 Done Downloading!".to_string());
    Ok(())
}
