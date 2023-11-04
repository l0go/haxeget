use super::filesystem;

pub fn installed() {
    for version in filesystem::get_installed().unwrap().flatten() {
        let version = version.split_whitespace().next().unwrap();
        println!("{version}");
    }
}
