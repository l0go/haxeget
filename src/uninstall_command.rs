use super::cache_directory::Cache;
use color_eyre::eyre::{eyre, Result, WrapErr};

/*
 * Uninstalls the specified version
 */
pub fn run_uninstall(version: String) -> Result<()> {
    let cache = Cache::new().expect("Cache was unable to be read");

    // Check if already installed
    cache
        .find_version(&version)
        .ok_or_else(|| eyre!("The specified version was not found"))?;

    // Check if it is the currently used version
    // If so, delete the symlinks
    let current_version = cache.current_version();
    if current_version.is_empty() || current_version.split_whitespace().next().unwrap() == version {
        delete_symlink(&cache.location, "haxe");
        delete_symlink(&cache.location, "haxelib");
    }

    let haxe_directory = format!(
        "{}/bin/{}",
        cache.location,
        cache.find_version(&version).unwrap_or("".to_owned())
    );
    std::fs::remove_dir_all(haxe_directory).wrap_err("Was unable to remove directory")?;

    cache.remove_version(&version);

    Ok(())
}

fn delete_symlink(directory: &str, name: &str) {
    let _ = std::fs::remove_file(format!("{}/{}", directory, name));
}
