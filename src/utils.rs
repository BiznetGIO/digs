use std::fs;
use std::path::{Path, PathBuf};

pub fn current_dir() -> PathBuf {
    std::env::current_dir().unwrap()
}

pub fn is_domain(val: &str) -> Result<(), String> {
    if val.contains(".") {
        Ok(())
    } else {
        Err(String::from("Invalid domain name."))
    }
}

pub fn is_exist(val: &str) -> Result<(), String> {
    let path = Path::new(val);

    if path.exists() {
        Ok(())
    } else {
        Err(String::from("No such file."))
    }
}

pub fn read_file(path: &Path) -> String {
    let path = Path::new(path);

    fs::read_to_string(path).expect("Can't read file")
}
