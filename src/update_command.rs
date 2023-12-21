use super::cache_directory::Cache;
use crate::packages;
use color_eyre::eyre::Result;

use futures::executor;

/*
 * Installs a specific version of haxe
 */
pub async fn run_update() -> Result<()> {
    let cache = Cache::new().expect("Cache was unable to be read");

    let file_name = executor::block_on(packages::haxeget::download(&cache))?;
    cache.extract_archive(file_name.as_str(), "").unwrap();

    // Tada!
    println!("Update Complete!");

    Ok(())
}
