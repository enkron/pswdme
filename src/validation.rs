use rand::{thread_rng, Rng};

pub fn validate(seq: &String) -> bool {
    // Ensure the `seq` contains at least one number, sign, lowercase and upper case character
    if seq.contains(thread_rng().gen_range('0'..='9'))
        && seq.contains(thread_rng().gen_range('a'..='z'))
        && seq.contains(thread_rng().gen_range('A'..='Z'))
        && (seq.contains(thread_rng().gen_range('!'..='/'))
            || seq.contains(thread_rng().gen_range(':'..='@'))
            || seq.contains(thread_rng().gen_range('['..='`'))
            || seq.contains(thread_rng().gen_range('{'..='~')))
    {
        return true;
    }
    false
}
