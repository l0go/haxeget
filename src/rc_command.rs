use super::cache_directory::Cache;
use super::use_command;
use crate::packages;
use serde_json::Value;
use color_eyre::eyre::{eyre, Result};

use futures::executor;

/*
 * Installs a specific version of haxe
 */
pub async fn run_rc() -> Result<()> {
    let cache = Cache::new().expect("Cache was unable to be read");

    let contents = match std::fs::read_to_string("./.haxerc") {
        Ok(body) => body,
        Err(_) => return Err(eyre!("Unable to read .haxerc file, does it exist?")),
    };

    let json: Value = serde_json::from_str(&contents)?;
    let version = json["version"].as_str().expect("Version is not the valid type").to_string();

    // Check if installed already
    if cache.find_version(&version).is_some() {
        use_command::run_use(version.clone())?;
        return Ok(());
    }

    // Downloads the haxe archive file
    let download = executor::block_on(packages::haxe_stable::download(&cache, &version));

    if let Ok(file_name) = download {
        let location = {
            cache.extract_archive(file_name.as_str(), "bin").unwrap();
            cache.get_haxe_dir_name(file_name.as_str()).unwrap()
        };

        cache.add_version(&version, location);
    };

    use_command::run_use(version)?;

    // Tada!
    println!("Installation Complete!");

    Ok(())
}
