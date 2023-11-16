use super::cache_directory::Cache;
use console::style;

/*
 * Uninstalls the specified version
 */
pub fn run_uninstall(version: String) {
    let cache = Cache::new().expect("Cache was unable to be read");

    // Check if already installed
    if cache.find_version(&version).is_none() {
        println!("{}", style("This version was not found").yellow());
        return;
    }

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
    if let Err(error) = std::fs::remove_dir_all(haxe_directory) {
        println!(
            "{}: {}",
            style("Was unable to remove directory").yellow(),
            error
        );
    }

    cache.remove_version(&version);
}

fn delete_symlink(directory: &str, name: &str) {
    let _ = std::fs::remove_file(format!("{}/{}", directory, name));
}
