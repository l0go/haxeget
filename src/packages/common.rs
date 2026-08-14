// This module contains functions that show up in more than one package
use crate::cache_directory::Cache;
use color_eyre::eyre::{eyre, Result, WrapErr};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::{fs, io};

/*
 * Downloads a file and renders a pretty progress bar
 * Originally based on https://gist.github.com/giuliano-oliveira/4d11d6b3bb003dba3a1b53f43d81b30d
 * ureq port based on https://gist.github.com/Roshan-R/8bd44d93e47f409614a5d1574cd16cb8
 */
pub fn download_file(url: &str, path: &str) -> Result<()> {
    let res = ureq::get(url)
        .call()
        .or(Err(eyre!("Failed to GET from '{}'", &url)))?;

    let total_size: u64 = res
        .headers()
        .get("Content-Length")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse().ok())
        .ok_or_else(|| eyre!("Failed to get content length from '{}'", &url))?;

    // Indicatif setup
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::with_template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.yellow/red}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
                 .unwrap());

    // download chunks
    let mut file = fs::File::create(path).wrap_err("Failed to create file '{path}'")?;

    io::copy(&mut pb.wrap_read(res.into_body().into_reader()), &mut file).unwrap();
    pb.finish_with_message("🎉 Done Downloading!".to_string());
    Ok(())
}

/*
 * Infers the name of the haxe archive based on the version name
 */
pub fn get_haxe_archive(version: &str) -> Result<String> {
    let mut file_name = String::new();

    if version == "nightly" {
        file_name.push_str("haxe_latest");
        if (cfg!(target_os = "linux") && cfg!(target_arch = "x86_64")) || cfg!(target_os = "macos")
        {
            file_name.push_str(".tar.gz");
        } else if cfg!(target_os = "windows") {
            file_name.push_str(".zip");
        } else {
            return Err(eyre!(
                "Your operating system and/or architecture is unsupported".to_owned()
            ));
        }
    } else {
        file_name.push_str("haxe-");
        file_name.push_str(version);

        if cfg!(target_os = "linux") && cfg!(target_arch = "x86_64") {
            file_name.push_str("-linux64.tar.gz");
        } else if cfg!(target_os = "macos") {
            file_name.push_str("-osx.tar.gz");
        } else if cfg!(target_os = "windows") {
            if cfg!(target_arch = "x86_64") {
                file_name.push_str("-win64.zip");
            } else {
                file_name.push_str("-win.zip");
            }
        } else {
            return Err(eyre!(
                "Your operating system and/or architecture is unsupported".to_owned()
            ));
        }
    }

    Ok(file_name)
}

pub fn link(cache: &Cache, version: &str, from: &str, to: &str) -> Result<()> {
    #[cfg(any(windows, doc))]
    return link_windows(cache, version, from, to); //https://github.com/l0go/haxeget/issues/12

    let _ = fs::remove_file(format!("{}/{from}", cache.location));

    // unix
    #[cfg(all(not(target_os = "hermit"), any(unix, doc)))]
    std::os::unix::fs::symlink(
        format!("{}/bin/{version}/{from}", cache.location),
        format!("{}/{to}", cache.location),
    )
    .wrap_err(format!(
        "I was unable to create a symlink from {}/bin/{version}/{from} to {}/{to}",
        cache.location, cache.location
    ))?;

    Ok(())
}

#[cfg(any(windows, doc))]
fn link_windows(cache: &Cache, version: &str, from: &str, to: &str) -> Result<()> {
    let mut ver: String = String::from(version);
    let _ = fs::remove_dir(format!("{}\\{from}", cache.location));
    if version.ends_with(".zip") {
        //https://github.com/l0go/haxeget/issues/12
        ver = cache.check_if_folder_exists_or_extract(version).unwrap();
    }

    // windows
    if from == "std" {
        std::os::windows::fs::symlink_dir(
            format!("{}\\bin\\{ver}\\{from}", cache.location),
            format!("{}\\{to}", cache.location),
        )
        .wrap_err(format!(
            "I was unable to create a symlink from {0}\\bin\\{ver} to {0}\\{from}",
            cache.current_version()
        ))?;
    } else {
        std::os::windows::fs::symlink_dir(
            format!("{}\\bin\\{ver}", cache.location),
            format!("{}\\{to}", cache.location),
        )
        .wrap_err(format!(
            "I was unable to create a symlink from {0}\\bin\\{ver} to {0}\\{from}",
            cache.current_version()
        ))?;
    }

    Ok(())
}

pub fn link_haxe(cache: &Cache, version: String) -> Result<()> {
    // Check if not installed already
    let tar_version = cache.find_version(&version).ok_or_else(|| {
        eyre!("This version is not installed. Try running `haxeget install {version}`")
    })?;

    link(cache, &tar_version, "haxe", "haxe")?;
    link(cache, &tar_version, "haxelib", "haxelib")?;
    link(cache, &tar_version, "std", "std")?;

    cache.set_current_version(&version, &tar_version);

    println!("🎉 You are now on Haxe {}", style(&version).yellow());
    if cfg!(target_os = "windows") {
        // Check if HAXEPATH is set
        if std::env::var("HAXEPATH").is_err() {
            println!("Note: You will need to run `setx /M HAXEPATH {}` and add `%HAXEPATH%` to your PATH vars to use this version of Haxe!", Cache::get_path().unwrap() + "\\haxe");
        }

        // Check if HAXEPATH is in PATH
        let path = std::env::var("PATH").unwrap_or_default();
        let haxepath = format!("{}\\haxe", Cache::get_path().unwrap());

        if !path.contains(&haxepath) {
            println!("Warning: HAXEPATH is not in your PATH. Add `%HAXEPATH%` to your PATH vars to use this version of Haxe!");
        }
    } else if std::env::var("HAXE_STD_PATH").is_err() {
        // Handle the case for non-windows OS here
        println!("Note: You will need to add `export HAXE_STD_PATH={}/std/` to your shell config (i.e ~/.bashrc or ~/.zshrc)", Cache::get_path().unwrap());
    }

    Ok(())
}
