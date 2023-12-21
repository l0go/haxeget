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
    let download = match version.as_str() {
        "ceramic" => executor::block_on(packages::ceramic::download(&cache)),
        "nightly" => executor::block_on(packages::haxe_nightly::download(&cache)),
        _ => executor::block_on(packages::haxe_stable::download(&cache, &version)),
    };

    if let Ok(file_name) = download {
        let location = if version.eq("ceramic") {
            let ceramic_dir = Cache::get_path().unwrap() + "/bin/ceramic";
            let _ = std::fs::remove_dir_all(&ceramic_dir);
            let _ = std::fs::create_dir(ceramic_dir);
            cache.extract_zip(file_name.as_str(), "bin/ceramic").unwrap();
            "ceramic".to_string()
        } else {
            cache.extract_archive(file_name.as_str(), "bin").unwrap();
            cache.get_haxe_dir_name(&file_name.as_str()).unwrap()
        };

        cache.add_version(&version, location);
    };

    use_command::run_use(version)?;

    // Tada!
    println!("Installation Complete!");

    Ok(())
}
