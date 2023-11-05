use console::style;
use flate2::read::GzDecoder;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::{env, fs};
use tar::Archive;

pub struct Cache {
    pub location: String,
}

impl Cache {
    pub fn new() -> Cache {
        let path = Self::get_path().unwrap();

        // Create root
        if let Err(error) = fs::create_dir_all(&path) {
            println!(
                "{}: {}",
                style("Was unable to create cache directory").yellow(),
                error
            );
        }

        // Create internal directories
        Self::create_dir(path.clone(), "_current");
        Self::create_dir(path.clone(), "bin");

        // Create current files
        Self::create_file(path.clone(), "haxe_version");
        Self::create_file(path.clone(), "installed");

        Self { location: path }
    }

    /*
     * Gets the name of directory that the binaries are stored in based on the tar file
     */
    pub fn get_haxe_directory_from_tar(&self, file_name: &str) -> Result<String, Box<dyn Error>> {
        let tarball = fs::File::open(format!("{}/bin/{file_name}", self.location))?;
        let tar = GzDecoder::new(tarball);
        let mut archive = Archive::new(tar);

        // Get the name of the directory extracted
        let mut name = String::new();
        if let Some(file) = archive.entries().unwrap().next() {
            let file = file.unwrap();
            name.push_str(
                file.header()
                    .path()
                    .unwrap()
                    .as_ref()
                    .to_str()
                    .expect("Unable to get extracted directory name"),
            );
            name.truncate(name.len() - 1);
        }

        Ok(name)
    }

    /*
     * Returns the name of the directory that the version is located in
     */
    pub fn find_version(&self, version: &String) -> Option<String> {
        if let Ok(lines) = Self::read_lines(self.location.clone() + "/_current/installed") {
            for line in lines.flatten() {
                let mut cached_version = line.split_whitespace();
                let ver = cached_version.next().unwrap();
                let directory = cached_version.next().unwrap();

                if ver == version {
                    return Some(directory.to_owned());
                }
            }
        }

        None
    }

    /*
     * Adds a version to the installed cache
     * This is just a list of all of the versions that are currently installed
     */
    pub fn add_version(&self, version: &String, binary_directory: String) {
        let mut installed = OpenOptions::new()
            .append(true)
            .create(true)
            .open(self.location.clone() + "/_current/installed")
            .expect("Cannot open installed cache");

        installed
            .write_fmt(format_args!("{} {}\n", version, binary_directory))
            .expect("Cannot write to installed cache");
    }

    /*
     * Removes the version from the installed cache
     * Does the opposite of the previous function
     */
    pub fn remove_version(&self, version: &String) {
        let file = self.location.clone() + "/_current/installed";

        let mut buffer = String::new();
        if let Ok(lines) = Self::read_lines(&file) {
            for line in lines.flatten() {
                if !line.contains(version) {
                    buffer.push_str(&format!("{}\n", &line));
                }
            }
        }

        let _ = fs::remove_file(&file);

        let mut installed = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&file)
            .expect("Cannot open installed cache");

        if let Err(error) = installed.write_fmt(format_args!("{}", buffer)) {
            println!(
                "{}: {}",
                style("Was unable to remove the version from the installed cache").yellow(),
                error
            );
        }
    }

    /*
     * Gets the current version
     */
    pub fn current_version(&self) -> String {
        fs::read_to_string(self.location.clone() + "/_current/haxe_version").unwrap()
    }

    pub fn set_current_version(&self, version: &String, tar_version: &String) {
        let mut current_version = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(self.location.clone() + "/_current/haxe_version")
            .unwrap();

        current_version
            .write_fmt(format_args!("{} {}", version, tar_version))
            .expect("Cannot write to current version cache");
    }

    /*
     * Returns all installed versions
     */
    pub fn all_versions(
        &self,
    ) -> Result<std::io::Lines<std::io::BufReader<std::fs::File>>, std::io::Error> {
        Self::read_lines(self.location.clone() + "/_current/installed")
    }

    /*
     * Utility that extracts the haxe tarball
     */
    pub fn extract_tarball(&self, file_name: String) -> Result<(), Box<dyn Error>> {
        let tarball = fs::File::open(format!("{}/bin/{file_name}", self.location))?;
        let tar = GzDecoder::new(tarball);
        let mut archive = Archive::new(tar);

        archive.unpack(format!("{}/bin/", self.location))?;
        fs::remove_file(format!("{}/bin/{file_name}", self.location))?;

        Ok(())
    }

    /*
     * Utility for spitting out all of the lines in a file
     */
    fn read_lines<P>(file_name: P) -> io::Result<io::Lines<io::BufReader<std::fs::File>>>
    where
        P: AsRef<Path>,
    {
        let file = std::fs::File::open(file_name)?;
        Ok(io::BufReader::new(file).lines())
    }

    /*
     * Create a directory in the cache folder
     */
    fn create_dir(path: String, name: &str) {
        if let Err(error) = fs::create_dir_all(path + "/" + name) {
            println!(
                "{}: {}",
                style("Was unable to create cache directory").yellow(),
                error
            );
        }
    }

    /*
     * Create a file in the cache/_current directory
     */
    fn create_file(path: String, name: &str) {
        let _ = OpenOptions::new()
            .create(true)
            .open(path + "/_current/" + name);
    }

    /*
     * Gets the cache directory's path
     */
    fn get_path() -> Result<String, String> {
        let mut directory_path = String::new();
        let home_dir = env::var("HOME").unwrap();

        if cfg!(target_os = "linux") && cfg!(target_arch = "x86_64") {
            directory_path.push_str(
                (env::var("XDG_BIN_HOME").unwrap_or((home_dir + "/.local/bin").to_owned())
                    + "/haxeget")
                    .as_str(),
            );
        } else if cfg!(target_os = "macos") {
            directory_path.push_str((home_dir + "/.haxeget").as_str());
        } else {
            return Err("Your operating system and/or architecture is unsupported".to_owned());
        }

        Ok(directory_path)
    }
}
