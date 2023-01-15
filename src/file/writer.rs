use std::fs::File;
use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use crate::cli::add::AddArgs;
use crate::error::ErrorKind;
use crate::file::{HOME_FILE_CONFIG, util};
use crate::file;

pub struct Writer {
    path: PathBuf
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Row {
    pub path: PathBuf,
    #[serde(deserialize_with = "deserialize_datetime", serialize_with = "serialize_datetime")]
    pub expiration: DateTime<Utc>
}

fn serialize_datetime<T>(datetime: &DateTime<Utc>, serializer: T) -> Result<T::Ok, T::Error> where T: Serializer {
    let s = datetime.to_rfc3339();
    serializer.serialize_str(&s)
}

fn deserialize_datetime<'de, T>(deserializer: T) -> Result<DateTime<Utc>, T::Error>
    where T: Deserializer<'de> {

    let s = String::deserialize(deserializer)?;

    Ok(DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc))

}

impl Row {
    pub fn from_file(line: &str) -> Option<Row> {
        serde_json::from_str(line).ok()?
    }

    pub fn get_days_expired(&self) -> i64 {
        let now = Utc::now();
        let diff = now.signed_duration_since(self.expiration);
        diff.num_days()
    }

    pub fn get_days_left(&self) -> i64 {
        self.get_days_expired() * -1
    }

    pub fn is_expired(&self) -> bool {
        self.expiration < Utc::now()
    }
}

impl Writer {

    pub fn new() -> Writer{
        Writer::from(HOME_FILE_CONFIG)
    }

    pub fn from(path: &str) -> Writer {
        let path = util::apply_home_dir(path);
        let path = Path::new(path.as_str());

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
        let str = util::apply_home_dir(path.as_os_str().to_str().unwrap());
        let path = PathBuf::from(str);
        std::path::absolute(path).expect(ErrorKind::CantGetAbsPath.as_str())
    }

    pub fn add_entry(&mut self, add_args: &AddArgs) -> Result<(), ErrorKind>{
        let abs_path = Writer::get_abs_path(&add_args.path);

        let row = Row{path: abs_path, expiration: add_args.get_exp_date()?};
        self.store_entry(&row)
    }

    fn store_entry(&self, row: &Row) -> Result<(),ErrorKind> {
        let mut file = self.get_file();
        let ser =  serde_json::to_string(&row).expect(ErrorKind::WrongFormat.as_str());
        let res = writeln!(file, "{}", ser);
        if res.is_err() {
            return Err(ErrorKind::CantWriteToFile);
        }

        Ok(())
    }

    pub fn remove_entry(&self, path: &PathBuf) -> Result<(), ErrorKind> {
        util::delete_element_from_fs(path);
        let abs_path = Writer::get_abs_path(path);
        let mut file = self.get_file();
        let mut lines = Vec::new();
        for line in std::io::BufReader::new(&file).lines() {
            let line = line.unwrap();
            let row = Row::from_file(&line).unwrap();
            if row.path != abs_path {
                lines.push(line);
            }
        }

        file.set_len(0).unwrap();
        for line in lines {
            writeln!(file, "{}", line).unwrap();
        }

        Ok(())
    }

    pub fn get_all(&self) -> Vec<Row> {
        let mut rows = Vec::new();
        let file = self.get_file();
        for line in std::io::BufReader::new(&file).lines() {
            let line = line.unwrap();
            match Row::from_file(&line) {
                Some(row) => rows.push(row),
                None => {}
            }
        }

        rows.sort_by(|a, b| a.get_days_expired().cmp(&b.get_days_expired()));

        rows
    }

    pub fn get_expired(&self) -> Vec<Row> {
        self.get_all().iter().filter(|row| row.is_expired()).cloned().collect()
    }

    pub fn check_entry(&mut self, path: &PathBuf) -> bool {
        let path = Writer::get_abs_path(path);
        self.find(|row| -> bool {
            row.path.eq(&path)
        }).is_some()
    }

    pub fn find<F>(&mut self, comp: F) -> Option<Row> where F: Fn(&Row) -> bool {
        let file = self.get_file();
        let reader = std::io::BufReader::new(file);

        for line in reader.lines() {
            let row = Row::from_file(line.unwrap().as_str()).unwrap();
            if comp(&row) {
                return Some(row);
            }
        }
        None
    }
}

#[cfg(test)]
pub mod test {
    use std::fs;
    use std::fs::File;
    use std::path::PathBuf;
    use crate::file::writer::{Row, Writer};
    use serial_test::serial;
    use crate::cli::add::AddArgs;
    use crate::error::ErrorKind;
    use crate::file::util;

    const TEST_FILE_PATH: &str = "./test/.config/ExpireFolder/expirelist.conf";

    pub fn delete_config_folder(){
        let path = PathBuf::from("./test");
        util::delete_element_from_fs(&path);
        let path = PathBuf::from("./test");
        assert!(!path.exists())
    }

    #[test]
    #[serial]
    pub fn big_year() {
        let mut wrt = Writer::from(TEST_FILE_PATH);
        let add_args = AddArgs {
            path: PathBuf::from("./target"),
            year: Some(100000000000),
            day: Some(1),
            month: None,
        };

        assert_eq!(wrt.add_entry(&add_args).err().unwrap(), ErrorKind::InvalidTime);
    }

    #[test]
    #[serial]
    pub fn save_entry() {
        let mut wrt = Writer::from(TEST_FILE_PATH);
        let add_args = AddArgs {
            path: PathBuf::from("./target"),
            year: Some(0),
            day: Some(1),
            month: None,
        };
        wrt.add_entry(&add_args).unwrap();
        assert!(wrt.check_entry(&PathBuf::from("./target")));
    }

    pub fn create_file_expired(path: &PathBuf) {
        let wrt = Writer::from(TEST_FILE_PATH);
        File::create(path).unwrap();
        let path = Writer::get_abs_path(&PathBuf::from(path));
        let row = Row{path, expiration: "2020-01-01T00:00:00Z".parse().unwrap()};
        wrt.store_entry(&row).unwrap();
    }

    pub fn create_folder_expired(path: &PathBuf) {
        let wrt = Writer::from(TEST_FILE_PATH);
        fs::create_dir(path).unwrap();
        let path = Writer::get_abs_path(&PathBuf::from(path));
        let row = Row{path, expiration: "2020-01-01T00:00:00Z".parse().unwrap()};
        wrt.store_entry(&row).unwrap();
    }

    #[test]
    #[serial]
    pub fn check_expire_list() {
        delete_config_folder();
        save_entry();   //to add an entry not expired
        let path = PathBuf::from("./test/test.txt");
        let mut wrt = Writer::from(TEST_FILE_PATH);
        create_file_expired(&path);

        assert!(wrt.check_entry(&path));
        assert_eq!(wrt.get_expired().len(), 1);
        assert_eq!(wrt.get_expired()[0].path, Writer::get_abs_path(&path));
    }


    #[test]
    #[serial]
    pub fn file_deletion() {
        delete_config_folder();
        let mut wrt = Writer::from(TEST_FILE_PATH);
        let path = PathBuf::from("./test/test.txt");

        create_file_expired(&path);
        assert!(wrt.check_entry(&path));

        wrt.remove_entry(&path).expect("Error while removing entry");
        assert!(wrt.get_expired().is_empty());
        assert_eq!(path.exists(), false);
    }

    #[test]
    #[serial]
    pub fn folder_deletion(){
        delete_config_folder();
        let mut wrt = Writer::from(TEST_FILE_PATH);
        let path = PathBuf::from("./test/test");

        create_folder_expired(&path);
        assert!(wrt.check_entry(&path));

        wrt.remove_entry(&path).expect("Error while removing entry");
        assert!(wrt.get_expired().is_empty());
        assert_eq!(path.exists(), false);
    }

}