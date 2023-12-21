use super::cache_directory::Cache;
use color_eyre::eyre::Result;

pub fn run_use(version: String) -> Result<()> {
    let cache = Cache::new().expect("Cache was unable to be read");
    match version.as_str() {
        "ceramic" => crate::packages::ceramic::link_ceramic(&cache),
        _ => crate::packages::common::link_haxe(&cache, version),
    }
}
