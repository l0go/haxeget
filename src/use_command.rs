use super::cache_directory::Cache;

use color_eyre::eyre::Result;

pub fn run_use(version: String) -> Result<()> {
    let cache = Cache::new().expect("Cache was unable to be read");
    crate::packages::common::link_haxe(&cache, version)
}
