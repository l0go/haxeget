use super::cache_directory::Cache;
use color_eyre::eyre::Result;

pub fn run_use(version_name: String) -> Result<()> {
    let cache = Cache::new().expect("Cache was unable to be read");
    match version_name.as_str() {
        //"ceramic" => crate::packages::ceramic::link_ceramic(&cache),
        //"neko" => crate::packages::neko::link_neko(&cache),
        _ => crate::packages::common::link_haxe(
            &cache,
            cache
                .find_version(&version_name)
                .to_owned()
                .expect("Version must exist"),
        ),
    }
}
