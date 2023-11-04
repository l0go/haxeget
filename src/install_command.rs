use super::download;
use super::filesystem;

use futures::executor;
use console::style;

pub async fn run_install(version: String) {
    match filesystem::get_installed(&version) {
        Some(_) => { 
            println!("{}", style("This version is already installed!").yellow());
            return;
        },
        None => {},
    };

    println!("Downloading Haxe {}", style(&version).yellow());
    let download = executor::block_on(download::from_github(&version));

    let _ = match download {
        Ok(dld) => {
            let location = download::get_binary_directory(&dld.directory, &dld.file_name).unwrap();
            download::extract_tarball(dld.directory, dld.file_name).unwrap();
            filesystem::add_version_to_installed(&version, location);
        },
        Err(error) => panic!("Uh oh! Download failed: {}.\nPlease create an issue at: {}/issues", error, env!("CARGO_PKG_REPOSITORY"))
    };

    println!("Installation Complete!")
}
