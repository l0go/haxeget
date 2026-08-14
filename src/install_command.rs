use super::cache_directory::Cache;
use super::use_command;
use crate::packages;
use color_eyre::eyre::Result;

/*
 * Installs a specific version of haxe
 */
pub fn run_install(version: String) -> Result<()> {
    let cache = Cache::new().expect("Cache was unable to be read");

    // Downloads the haxe archive file
    let ver = match version.as_str() {
        //"ceramic" => executor::block_on(packages::ceramic::download(&cache)),
        "nightly" => packages::haxe_nightly::download(&cache),
        "neko" => packages::neko::download(&cache),
        _ => packages::haxe_stable::download(&cache, &version),
    }?;

    if version.eq("neko") {
        let neko_dir = Cache::get_path().unwrap() + "/bin/neko";
        let _ = std::fs::remove_dir_all(&neko_dir);
        let _ = std::fs::create_dir(neko_dir);
        cache
            .extract_archive(ver.archive_name.as_str(), "bin/neko")
            .unwrap();
    } else {
        cache
            .extract_archive(ver.archive_name.as_str(), "bin")
            .unwrap();
    };

    println!("{} {} {}", ver.version, ver.archive_name, ver.directory);
    cache.add_version(ver.clone());
    use_command::run_use(ver.version)?;

    // Tada!
    println!("Installation Complete!");

    Ok(())
}
