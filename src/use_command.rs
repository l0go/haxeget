use super::cache_directory::Cache;

use color_eyre::eyre::{eyre, Result, WrapErr};
use console::style;
use std::fs;

pub fn run_use(version: String) -> Result<()> {
    let cache = Cache::new().expect("Cache was unable to be read");

    // Check if installed already
    let tar_version = cache.find_version(&version).ok_or_else(|| {
        eyre!("This version is not installed. Try running `haxeget install {version}`")
    })?;

    link(&cache, &tar_version, "haxe")?;
    link(&cache, &tar_version, "haxelib")?;
    link(&cache, &tar_version, "std")?;

    cache.set_current_version(&version, &tar_version);

    println!("🎉 You are now on Haxe {}", style(&version).yellow());
    if cfg!(target_os = "windows"){
        println!("Note: You will need to run `setx /M HAXEPATH {}` and add `%HAXEPATH%` to your PATH vars to use this version of Haxe!", Cache::get_path().unwrap() + "\\haxe");
    } else {
        if std::env::var("HAXE_STD_PATH").is_err() {
            println!("Note: You will need to add `export HAXE_STD_PATH={}/std/` to your shell config (i.e ~/.bashrc or ~/.zshrc)", Cache::get_path().unwrap());
        }
    }
    
    Ok(())
}

fn link(cache: &Cache, version: &str, name: &str) -> Result<()> {
    if cfg!(target_os = "windows"){
        let _ = fs::remove_dir(format!("{}\\{name}", cache.location));
    } else {
        let _ = fs::remove_file(format!("{}/{name}", cache.location));
    }

    // unix
    #[cfg(all(not(target_os = "hermit"), any(unix, doc)))]
    std::os::unix::fs::symlink(
        format!("{}/bin/{version}/{name}", cache.location),
        format!("{}/{name}", cache.location),
    ).wrap_err("I was unable to create a symlink from {cachever}/bin/{version}/{name} to {cachever}/{name}")?;

    // windows
    #[cfg(any(windows, doc))]
    if name == "std" {
        std::os::windows::fs::symlink_dir(
            format!("{}\\bin\\{version}\\{name}", cache.location),
            format!("{}\\{name}", cache.location),
        ).wrap_err(format!("I was unable to create a symlink from {0}\\bin\\{version} to {0}\\{name}", cache.current_version()))?;
    } else {
        std::os::windows::fs::symlink_dir(
            format!("{}\\bin\\{version}", cache.location),
            format!("{}\\{name}", cache.location),
        ).wrap_err(format!("I was unable to create a symlink from {0}\\bin\\{version} to {0}\\{name}", cache.current_version()))?;
    }

    Ok(())
}
