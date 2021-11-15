use rand::Rng;

fn main() {
    const PSWD_LENGTH: u8 = 12;

    let char_set: &[u8] = b"\
        abcdefghijklmnopqrstuvwxyz\
        ABCDEFGHIJKLMNOPQRSTUVWXYZ\
        0123456789\
        ()[]{}~`'!@#$%^&*\"";

    let mut rng = rand::thread_rng();

    let pswd: String = (0..PSWD_LENGTH)
        .map(|_| {
            let idx = rng.gen_range(0..char_set.len());
            char_set[idx] as char
        })
        .collect();

    println!("{}", pswd);
}
