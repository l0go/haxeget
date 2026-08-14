use super::cache_directory::Cache;
use crate::packages;
use color_eyre::eyre::Result;

/*
 * Installs a specific version of haxe
 */
pub fn run_update() -> Result<()> {
    let cache = Cache::new().expect("Cache was unable to be read");

    let version = packages::haxeget::download(&cache)?;
    cache
        .extract_archive(version.directory.as_str(), "")
        .unwrap();

    // Tada!
    println!("Update Complete!");

    Ok(())
}
