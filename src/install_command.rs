use super::cache_directory::Cache;
use super::download;
use super::use_command;
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

    // Downloads the haxe .tar.gz file
    let download: std::prelude::v1::Result<String, color_eyre::eyre::Error> = if version.eq("nightly") {
        executor::block_on(download::download_nightly(&cache))
    }  else {
        executor::block_on(download::from_github(&cache, &version))
    };

    // If download was successful, we will extract the tarball and store the version
    if let Ok(file_name) = download {
        let location = cache.get_haxe_directory_from_tar(&file_name).unwrap();
        cache.extract_tarball(file_name).unwrap();
        cache.add_version(&version, location);
    }

    use_command::run_use(version)?;

    // Tada!
    println!("Installation Complete!");

    Ok(())
}
