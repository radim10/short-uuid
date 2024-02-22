use std::fmt;
use uuid;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Error(pub ErrorKind);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ErrorKind {
    /// Uuid input format error
    UuidError(uuid::Error),

    /// Custom alphabet error
    CustomAlphabet(CustomAlphabetError),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum CustomAlphabetError {
    Length,
    EmptyAlphabet,
    DuplicateAlphabetCharacter,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::UuidError(e) => e.fmt(f),
            ErrorKind::CustomAlphabet(e) => e.fmt(f),
        }
    }
}

impl fmt::Display for CustomAlphabetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomAlphabetError::EmptyAlphabet => write!(f, "Alphabet cannot be empty"),
            CustomAlphabetError::DuplicateAlphabetCharacter => {
                write!(f, "Alphabet contains duplicate characters")
            }
            CustomAlphabetError::Length => {
                write!(f, "Alphabet must contain at least 2 characters")
            }
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct InvalidShortUuid;

impl fmt::Display for InvalidShortUuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid short uuid")
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ParseStrCustomError(pub ParseStrCustomErrorKind);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ParseStrCustomErrorKind {
    /// Custom alphabet error
    CustomAlphabet(CustomAlphabetError),

    InvalidShortUuid,
}
