use std::{error::Error, fmt};

#[derive(Debug)]
pub enum ValidationError {
    WeakPassword,
    TooShortPassword(String),
}

impl Error for ValidationError {}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::WeakPassword => write!(f, "{:?}", self),
            ValidationError::TooShortPassword(s) => {
                writeln!(f, "{s}")
            }
        }
    }
}
