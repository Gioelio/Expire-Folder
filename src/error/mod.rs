#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {
    CantOpenFile,
    NoTimeSpecified,
    InvalidTime,
    CantGetAbsPath,
    WrongFormat,
    CantWriteToFile,
}

impl ErrorKind {
    pub fn as_str(&self) -> &str {
        match self {
            ErrorKind::NoTimeSpecified => "No expiration time specified",
            ErrorKind::CantOpenFile => "Cannot open or write expire list file",
            ErrorKind::InvalidTime => "The time entered is invalid",
            ErrorKind::CantGetAbsPath => "Cannot get absolute path from relative",
            ErrorKind::WrongFormat => "Wrong data format, cannot parse",
            ErrorKind::CantWriteToFile => "Cannot write to file",
        }
    }
}

