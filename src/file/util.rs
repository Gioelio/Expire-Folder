use std::path::PathBuf;
use crate::error::ErrorKind;

pub fn apply_home_dir(str: &str) -> String {
    let home: PathBuf = dirs::home_dir().unwrap();
    str.replace('~', home.to_str().unwrap())
}

pub fn delete_element_from_fs(path: &PathBuf) -> Result<(), ErrorKind> {
    if !path.exists() {
        return Err(ErrorKind::FileNotFound);
    }
    if path.is_dir() {
        std::fs::remove_dir_all(path).unwrap();
    } else if path.is_file() {
        std::fs::remove_file(path).unwrap();
    }
    return Ok(());
}
