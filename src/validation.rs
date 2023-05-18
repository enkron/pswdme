use rand::{thread_rng, Rng};
use std::{error::Error, fmt};

pub fn validate(seq: &String) -> Result<&str, ValidationError> {
    if seq.len() < 4 {
        return Err(ValidationError::TooShortPassword(
            "password less then 4 symbols",
        ));
    }
    // Ensure the `seq` contains at least one number, sign, lowercase and upper case character
    if !(seq.contains(thread_rng().gen_range('0'..='9'))
        && seq.contains(thread_rng().gen_range('a'..='z'))
        && seq.contains(thread_rng().gen_range('A'..='Z'))
        && (seq.contains(thread_rng().gen_range('!'..='/'))
            || seq.contains(thread_rng().gen_range(':'..='@'))
            || seq.contains(thread_rng().gen_range('['..='`'))
            || seq.contains(thread_rng().gen_range('{'..='~'))))
    {
        return Err(ValidationError::WeakPassword);
    }
    Ok(seq)
}

#[derive(Debug)]
pub enum ValidationError<'a> {
    WeakPassword,
    TooShortPassword(&'a str),
}

impl<'a> Error for ValidationError<'a> {}

impl<'a> fmt::Display for ValidationError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ValidationError::WeakPassword => write!(f, "{:?}", self),
            ValidationError::TooShortPassword(s) => {
                writeln!(f, "{s}")
            }
        }
    }
}
