use rand::{thread_rng, Rng};

use crate::error::ValidationError;

const MIN_PASSWORD_LENGHT: usize = 4;

pub fn validate(seq: &String) -> Result<&str, ValidationError> {
    let pswd_length = seq.len();
    if pswd_length < MIN_PASSWORD_LENGHT {
        return Err(ValidationError::TooShortPassword(format!(
            "password length is too short ({pswd_length} < {MIN_PASSWORD_LENGHT})",
        )));
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
