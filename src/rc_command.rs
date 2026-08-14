use super::cache_directory::Cache;
use super::use_command;
use crate::packages;
use color_eyre::eyre::{Result, eyre};
use serde_json::Value;

/*
 * Installs a specific version of haxe
 */
pub fn run_rc() -> Result<()> {
    let cache = Cache::new().expect("Cache was unable to be read");

    let contents = match std::fs::read_to_string("./.haxerc") {
        Ok(body) => body,
        Err(_) => return Err(eyre!("Unable to read .haxerc file, does it exist?")),
    };

    let json: Value = serde_json::from_str(&contents)?;
    let version = json["version"]
        .as_str()
        .expect("Version is not the valid type")
        .to_string();

    // Check if installed already
    if cache.find_version(&version).is_some() {
        use_command::run_use(version.clone())?;
        return Ok(());
    }

    // Downloads the haxe archive file
    let download = packages::haxe_stable::download(&cache, &version);

    if let Ok(ver) = download {
        cache
            .extract_archive(ver.directory.as_str(), "bin")
            .unwrap();
        cache.get_haxe_dir_name(ver.directory.as_str()).unwrap();

        cache.add_version(ver.clone());
        use_command::run_use(ver.version)?;
    };

    // Tada!
    println!("Installation Complete!");

    Ok(())
}
