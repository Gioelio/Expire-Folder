#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {
    CantOpenFile,
    NoTimeSpecified ,
    InvalidTime
}

impl ErrorKind {
    pub fn as_str(&self) -> &str {
        match self {
            ErrorKind::NoTimeSpecified => "No expiration time specified",
            ErrorKind::CantOpenFile => "Cannot open or write expire list file",
            ErrorKind::InvalidTime => "The time entered is invalid"
        }
    }
}