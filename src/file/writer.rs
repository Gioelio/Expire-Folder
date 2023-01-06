use std::fs::File;
use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};
use crate::{error, file};
use serde::{Serialize, Deserialize};
use crate::error::ErrorKind;

pub struct Writer {
    path: PathBuf
}

#[derive(Serialize, Deserialize)]
struct Row {
    path: PathBuf
}

impl Row {
    pub fn from_file(line: &str) -> Option<Row> {
        serde_json::from_str(line).ok()?
    }
}

impl Writer {

    pub fn new() -> Writer{
        Writer::from(file::ROOT_FILE_PATH)
    }

    pub fn from(path: &str) -> Writer {
        let path = Path::new(path);

        if !path.exists() {
            let prefix = path.parent().unwrap();
            std::fs::create_dir_all(prefix).unwrap();
            File::create(path).unwrap();
        }

        return Writer{path: PathBuf::from(path)};
    }

    fn get_file(&self) -> File {
        std::fs::OpenOptions::new().write(true).append(true).read(true).open(&self.path).unwrap()
    }

    fn get_abs_path(path: &PathBuf) -> PathBuf {
        std::path::absolute(path).expect("Cannot get absolute path from relative. Check you have enter a correct path")
    }

    pub fn add_entry(&mut self, path: &PathBuf) -> Result<(), ErrorKind>{
        let mut file = self.get_file();
        let abs_path = Writer::get_abs_path(&path);
        let row = Row{path: abs_path};

        let res = writeln!(file, "{}", serde_json::to_string(&row).unwrap());
        if res.is_err() {
            return Err(ErrorKind::CantOpenFile);
        }

        Ok(())
    }

    pub fn check_entry(&mut self, path: &PathBuf) -> bool {
        let file = self.get_file();

        let reader = std::io::BufReader::new(file);
        let abs_path = Writer::get_abs_path(&path);

        for line in reader.lines() {
            let row = Row::from_file(line.unwrap().as_str()).unwrap_or_else(|| {
                return Row{path: PathBuf::from("")}
            });
            if row.path.eq(&abs_path) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
pub mod test {
    use std::path::PathBuf;
    use crate::file::writer::Writer;
    use serial_test::serial;

    const TEST_FILE_PATH: &str = "./test/.config/ExpireFolder/expirelist";

    #[test]
    #[serial]
    pub fn delete_folder(){
        let path = PathBuf::from("./test");
        if path.exists() {
            std::fs::remove_dir_all("./test").unwrap();
        }
        let path = PathBuf::from("./test");
        assert!(!path.exists())
    }

    #[test]
    #[serial]
    pub fn save_entry() {
        let mut wrt = Writer::from(TEST_FILE_PATH);
        wrt.add_entry(&PathBuf::from("./target"));
        assert!(wrt.check_entry(&PathBuf::from("./target")));
    }

}