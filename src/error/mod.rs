#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {
    NoTimeSpecified,
    CantOpenFile
}

impl ErrorKind {
    pub fn as_str(&self) -> &str {
        match *self {
            ErrorKind::NoTimeSpecified => "No expiration time specified",
            ErrorKind::CantOpenFile => "Cannot open or write expire list file"
        }
    }
}