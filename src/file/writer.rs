use std::fs::{File, canonicalize};
use std::io::{BufRead, BufReader, Lines, Write};
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use crate::cli::add::AddArgs;
use crate::error::ErrorKind;
use crate::file::{HOME_FILE_CONFIG, util};

pub struct Writer {
    path: PathBuf,
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
        let str = path.as_os_str().to_str().unwrap();
        let exp_path = shellexpand::full(str).expect(ErrorKind::CantGetAbsPath.as_str());
        let can_path = canonicalize(exp_path.as_ref()).expect(ErrorKind::CantGetAbsPath.as_str());
        can_path
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

        self.get_file().set_len(0).unwrap();
        self.iter().for_each(|row| {
            if row.path.ne(path) {
                self.store_entry(&row).unwrap();
            }
        });

        util::delete_element_from_fs(path);

        Ok(())
    }

    pub fn get_all(&self) -> Vec<Row> {
        let mut vec: Vec<Row> = self.iter().collect();
        vec.sort_by(|a, b| b.get_days_expired().cmp(&a.get_days_expired()));
        vec
    }

    pub fn get_expired(&self) -> Vec<Row> {
        self.get_all().iter().filter(|row| row.is_expired()).cloned().collect()
    }

    pub fn check_entry(&mut self, path: &PathBuf) -> bool {
        let path = Writer::get_abs_path(path);
        self.iter().find(|row| -> bool {
            row.path.eq(&path)
        }).is_some()
    }

    pub fn iter(&self) -> WriterIterator {
        WriterIterator{_iter: BufReader::new(self.get_file()).lines()}
    }

}

pub struct WriterIterator {
    _iter: Lines<BufReader<File>>
}

impl Iterator for WriterIterator {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {

        let mut row = None;

        while row.is_none() {
            let next_value = self._iter.next();
            if next_value.is_none() {
                break;
            }

            let line = next_value.unwrap().unwrap();
            row = Row::from_file(line.as_str());
        }

        if row.is_some() {
            return Some(row.unwrap());
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
    #[should_panic]
    pub fn inexistent_file() {
        delete_config_folder();
        let path = PathBuf::from("./test/test.txt");
        let mut wrt = Writer::from(TEST_FILE_PATH);

        assert_eq!(wrt.get_expired().len(), 0);
        assert!(wrt.check_entry(&path));
    }

    #[test]
    #[serial]
    pub fn not_found_entry() {
        delete_config_folder();
        let path = PathBuf::from("./test");
        let mut wrt = Writer::from(TEST_FILE_PATH);

        assert_eq!(wrt.check_entry(&path), false);
        assert_eq!(wrt.get_expired().len(), 0);
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