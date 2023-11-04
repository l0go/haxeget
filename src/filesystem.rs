use std::env;

pub fn get_directory_name() -> Result<String, String> {
    let mut directory_path = String::new();
    let home_dir = env::var("HOME").unwrap();

    if cfg!(target_os = "linux") && cfg!(target_arch = "x86_64") {
        directory_path.push_str((env::var("XDG_BIN_HOME").unwrap_or((home_dir + "/.local/bin").to_owned()) + "/haxeget").as_str());
    } else if cfg!(target_os = "macos") {
        directory_path.push_str((home_dir + "/home/logo/.haxeget").as_str());
    } else {
        return Err("Your operating system and/or architecture is unsupported".to_owned());
    }

    Ok(directory_path)
}

pub fn get_file_name(version: String) -> Result<String, String> {
    let mut file_name = String::from("haxe-") + version.as_str();

    if cfg!(target_os = "linux") && cfg!(target_arch = "x86_64") {
        file_name.push_str("-linux64.tar.gz");
    } else if cfg!(target_os = "macos") {
        file_name.push_str("-osx.tar.gz");
    } else {
        return Err("Your operating system and/or architecture is unsupported".to_owned());
    }

    Ok(file_name)
}
