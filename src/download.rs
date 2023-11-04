use super::filesystem;
use super::github_schema;

use std::fs;
use std::error::Error;
use std::cmp::min;
use std::io::Write;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use flate2::read::GzDecoder;
use tar::Archive;

pub struct Download {
    pub file_name: String,
    pub directory: String,
}

/*
 * Gets the executable from github and installs 
 */
pub async fn from_github(version: &String) -> Result<Download, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let json: github_schema::Root = client.get("https://api.github.com/repos/HaxeFoundation/haxe/releases")
        .header("User-Agent", "haxeget (https://github.com/logo4poop/haxeget)")
        .send()
        .await
        .expect("Was unable to connect to Github API")
        .json()
        .await
        .expect("Was unable to parse release JSON");
    

    let release = json.iter()
        .find(|&release| &release.name == version)
        .expect("The provided version was not found");
  
    // Figure out the file name based on the target
    // Currently only supports linux and macOS
    let mut file_name = String::from("haxe-");
    file_name.push_str(&version);
    
    let directory = filesystem::get_directory_name()?;
    let file_name = filesystem::get_file_name(version)?;

    // Create a directory...
    fs::create_dir_all(directory.to_owned() + "/bin")?;

    // Now we can find the url that matches that file name
    let binary_url = &release.assets.iter()
        .find(|&asset| asset.name == file_name)
        .expect("There was not a valid asset for that version and target...").browser_download_url;

    let path = format!("{directory}/bin/{file_name}");
    download_file(&client, &binary_url, &path).await?;

    Ok(Download {file_name, directory})
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
        let chunk = item.or(Err(format!("Error while downloading file")))?;
        file.write_all(&chunk)
            .or(Err(format!("Error while writing to file")))?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message(format!("🎉 Done Downloading!"));
    return Ok(());
}

// Extract tarball
pub fn extract_tarball(directory: String, file_name: String) -> Result<(), Box<dyn Error>> {
    let tarball = fs::File::open(format!("{directory}/bin/{file_name}"))?;
    let tar = GzDecoder::new(tarball);
    let mut archive = Archive::new(tar);
    
    archive.unpack(format!("{directory}/bin/"))?;
    fs::remove_file(format!("{directory}/bin/{file_name}"))?;
  
    Ok(()) 
}

pub fn get_binary_directory(directory: &str, file_name: &str) -> Result<String, Box<dyn Error>> {
    let tarball = fs::File::open(format!("{directory}/bin/{file_name}"))?;
    let tar = GzDecoder::new(tarball);
    let mut archive = Archive::new(tar);
   
    // Get the name of the directory extracted
    let mut name = String::new();
    for file in archive.entries().unwrap() {
        let file = file.unwrap();
        name.push_str(file.header()
                      .path().unwrap()
                      .as_ref().to_str()
                      .expect("Unable to get extracted directory name"));
        break;
    }
    name.truncate(name.len() - 1);

    Ok(name) 
}
