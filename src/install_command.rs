use super::download;
use super::filesystem;

use console::style;
use futures::executor;

pub async fn run_install(version: String) {
    // Check if installed already
    if filesystem::find_installed(&version).is_some() {
        println!("{}", style("This version is already installed!").yellow());
        return;
    }

    // Downloads the haxe .tar.gz file
    let download = executor::block_on(download::from_github(&version));

    // If download was successful, we will extract the tarball and store the version
    if let Ok(dld) = download {
        let location =
            filesystem::get_binary_directory(&dld.directory, &dld.file_name).unwrap();
        filesystem::extract_tarball(dld.directory, dld.file_name).unwrap();
        filesystem::add_version_to_installed(&version, location);
    }

    // Tada
    println!("Installation Complete!")
}
