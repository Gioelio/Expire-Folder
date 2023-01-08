use std::path::PathBuf;

pub fn apply_home_dir(str: &str) -> String {
    let home: PathBuf = dirs::home_dir().unwrap();
    str.replace("~", home.to_str().unwrap())
}
