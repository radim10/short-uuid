use core::fmt;
use std::ops::Deref;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Error(pub ErrorKind);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ErrorKind {
    UuidError(uuid::Error),

    /// Empty alphabet
    EmptyAlphabet,

    /// Invalid alphabet
    InvalidAlphabet,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::EmptyAlphabet => write!(f, "Alphabet is empty"),
            ErrorKind::InvalidAlphabet => write!(f, "Alphabet is invalid"),
            ErrorKind::UuidError(e) => e.fmt(f),
        }
    }
}

// impl Deref for Error {
//     type Target = ErrorKind;
//
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
