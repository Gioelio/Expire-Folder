use std::path::PathBuf;

pub fn apply_home_dir(str: &str) -> String {
    let home: PathBuf = dirs::home_dir().unwrap();
    str.replace("~", home.to_str().unwrap())
}

pub fn delete_element_from_fs(path: &PathBuf) {
    if path.is_dir() {
        std::fs::remove_dir_all(path).unwrap();
    } else if path.is_file() {
        std::fs::remove_file(path).unwrap();
    }
}
