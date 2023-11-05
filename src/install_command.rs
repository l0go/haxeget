use super::download;
use super::cache_directory::Cache;

use console::style;
use futures::executor;

/*
 * Installs a specific version of haxe
 */
pub async fn run_install(version: String) {
    let cache = Cache::new();

    // Check if installed already
    if cache.find_version(&version).is_some() {
        println!("{}", style("This version is already installed!").yellow());
        return;
    }

    // Downloads the haxe .tar.gz file
    let download = executor::block_on(download::from_github(&cache, &version));

    // If download was successful, we will extract the tarball and store the version
    if let Ok(file_name) = download {
        let location = cache.get_haxe_directory_from_tar(&file_name).unwrap();
        cache.extract_tarball(file_name).unwrap();
        cache.add_version(&version, location);
    }

    // Tada!
    println!("Installation Complete!")
}
