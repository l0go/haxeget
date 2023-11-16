use super::cache_directory::Cache;

use console::style;
use std::{env, fs};

pub fn run_use(version: String) {
    let cache = Cache::new().expect("Cache was unable to be read");

    // Check if installed already
    let tar_version = cache.find_version(&version);
    if tar_version.is_none() {
        println!(
            "This version is not installed. Try running {}",
            style(format!("haxeget install {}", version)).yellow()
        );
        return;
    }
    let tar_version = tar_version.unwrap();

    link_binary(&cache, &tar_version, "haxe");
    link_binary(&cache, &tar_version, "haxelib");

    cache.set_current_version(&version, &tar_version);

    println!("🎉 You are now on Haxe {}", style(&version).yellow());
}

fn link_binary(cache: &Cache, version: &str, name: &str) {
    let _ = fs::remove_file(format!("{}/{name}", cache.location));
    let link = std::os::unix::fs::symlink(
        format!("{}/bin/{version}/{name}", cache.location),
        format!("{}/{name}", cache.location),
    );
    if let Err(error) = link {
        panic!(
            "Uh oh! I was unable to create a symlink: {}.\nPlease create an issue at: {}/issues",
            error,
            env!("CARGO_PKG_REPOSITORY")
        )
    }
}
