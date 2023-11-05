use super::cache_directory::Cache;

/*
 * Lists installed Haxe versions
 */
pub fn installed() {
    let cache = Cache::new();

    for version in cache.all_versions().unwrap().flatten() {
        let version = version.split_whitespace().next().unwrap();
        println!("{version}");
    }
}

/*
 * Prints out the current version
 */
pub fn current() {
    let cache = Cache::new();
    let current_version = cache.current_version();
    let version = current_version 
        .split_whitespace()
        .next()
        .unwrap();
    println!("Haxe {}", version);
}
