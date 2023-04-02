#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {
    NoTimeSpecified,
    InvalidTime,
    CantGetAbsPath,
    WrongFormat,
    CantWriteToFile,
    CantFindElement,
    FileNotFound
}

impl ErrorKind {
    pub fn as_str(&self) -> &str {
        match self {
            ErrorKind::NoTimeSpecified => "No expiration time specified",
            ErrorKind::InvalidTime => "The time entered is invalid",
            ErrorKind::CantGetAbsPath => "Cannot get absolute path from relative",
            ErrorKind::WrongFormat => "Wrong data format, cannot parse",
            ErrorKind::CantWriteToFile => "Cannot write to file",
            ErrorKind::CantFindElement => "Cannot find element in file",
            ErrorKind::FileNotFound => "File not found on filesystem"
        }
    }
}

