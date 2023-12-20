use super::cache_directory::Cache;
use super::use_command;
use crate::packages;
use color_eyre::eyre::{eyre, Result};

use futures::executor;

/*
 * Installs a specific version of haxe
 */
pub async fn run_install(version: String) -> Result<()> {
    let cache = Cache::new().expect("Cache was unable to be read");

    // Check if installed already
    if cache.find_version(&version).is_some() {
        return Err(eyre!("The specified version is already installed!"));
    }

    // Downloads the haxe archive file
    match version.as_str() {
        "haxeget" => {
            let file_name = executor::block_on(packages::haxeget::download(&cache))?;
            cache.extract_tarball(file_name, "").unwrap();
        }
        _ => {
            let download = if version.as_str().eq("nightly") {
                executor::block_on(packages::haxe_nightly::download(&cache))
            } else {
                executor::block_on(packages::haxe_stable::download(&cache, &version))
            };

            if let Ok(file_name) = download {
                let location = cache.get_haxe_dir_name(&file_name).unwrap();
                cache.extract_tarball(file_name, "bin").unwrap();
                cache.add_version(&version, location);
            };
        }
    };

    use_command::run_use(version)?;

    // Tada!
    println!("Installation Complete!");

    Ok(())
}
