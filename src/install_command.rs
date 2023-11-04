use super::download;
use super::filesystem;

use console::style;
use futures::executor;

pub async fn run_install(version: String) {
    // Check if installed already
    if filesystem::get_installed(&version).is_some() {
        println!("{}", style("This version is already installed!").yellow());
        return;
    }

    // Downloads the haxe .tar.gz file
    println!("Downloading Haxe {}", style(&version).yellow());
    let download = executor::block_on(download::from_github(&version));

    // If download was successful, we will extract the tarball and store the version
    match download {
        Ok(dld) => {
            let location =
                filesystem::get_binary_directory(&dld.directory, &dld.file_name).unwrap();
            filesystem::extract_tarball(dld.directory, dld.file_name).unwrap();
            filesystem::add_version_to_installed(&version, location);
        }
        Err(error) => panic!(
            "Uh oh! Download failed: {}.\nPlease create an issue at: {}/issues",
            error,
            env!("CARGO_PKG_REPOSITORY")
        ),
    };

    // Tada
    println!("Installation Complete!")
}
