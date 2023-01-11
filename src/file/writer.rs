use std::fs::File;
use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use crate::cli::add::AddArgs;
use crate::error::ErrorKind;
use crate::file::{HOME_FILE_CONFIG, util};

pub struct Writer {
    path: PathBuf
}

#[derive(Debug, Deserialize, Serialize)]
struct Row {
    path: PathBuf,
    #[serde(deserialize_with = "deserialize_datetime", serialize_with = "serialize_datetime")]
    expiration: DateTime<Utc>
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
        let mut file = self.get_file();
        let abs_path = Writer::get_abs_path(&add_args.path);

        let row = Row{path: abs_path, expiration: add_args.get_exp_date()?};
        let ser =  serde_json::to_string(&row).expect(ErrorKind::WrongFormat.as_str());
        let res = writeln!(file, "{}", ser);
        if res.is_err() {
            return Err(ErrorKind::CantWriteToFile);
        }

        Ok(())
    }

    pub fn check_entry(&mut self, path: &PathBuf) -> bool {
        let file = self.get_file();

        let reader = std::io::BufReader::new(file);
        let abs_path = Writer::get_abs_path(&path);

        for line in reader.lines() {
            let row = Row::from_file(line.unwrap().as_str()).unwrap();
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
    use crate::cli::add::AddArgs;
    use crate::error::ErrorKind;

    const TEST_FILE_PATH: &str = "./test/.config/ExpireFolder/expirelist.conf";

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
        delete_folder();
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

}