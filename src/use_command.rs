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

    link_binary(&cache, &tar_version, "haxe")?;
    link_binary(&cache, &tar_version, "haxelib")?;

    cache.set_current_version(&version, &tar_version);

    println!("🎉 You are now on Haxe {}", style(&version).yellow());
    Ok(())
}

fn link_binary(cache: &Cache, version: &str, name: &str) -> Result<()> {
    let _ = fs::remove_file(format!("{}/{name}", cache.location));
    std::os::unix::fs::symlink(
        format!("{}/bin/{version}/{name}", cache.location),
        format!("{}/{name}", cache.location),
    ).wrap_err("I was unable to create a symlink from {cache.version}/bin/{version}/{name} to {cache.version}/{name}")?;

    Ok(())
}
