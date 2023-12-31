use super::cache_directory::Cache;
use console::style;

/*
 * Lists installed Haxe versions
 */
pub fn installed() {
    let cache = Cache::new().expect("Cache was unable to be read");

    for version in cache.all_versions().unwrap().flatten() {
        let version = version.split_whitespace().next().unwrap();
        println!("{version}");
    }
}

/*
 * Prints out the current version
 */
pub fn current() {
    let cache = Cache::new().expect("Cache was unable to be read");
    let current_version = cache.current_version();

    if cache.current_version().is_empty() {
        println!("{}", style("You are currently not on any version").yellow());
        return;
    }

    let version = current_version.split_whitespace().next().unwrap();
    println!("Haxe {}", version);
}
