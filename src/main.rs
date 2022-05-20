use clap::Parser;
use rand::Rng;

mod args;
use args::{Cli, Subcommands};

fn main() {
    let cli = Cli::parse();
    let mut pswd: String;

    let pswd_length = match cli.subcommands {
        Subcommands::Gen(opts) => opts.length,
    };

    // To satisfy all validation cariterias a password must be at least 4
    // characters length, therefore the program will panic if provided
    // `PSWD_LENGTH` value will be less then 4
    if pswd_length < 4 {
        eprintln!(
            "error: to satisfy all security criterias password must be at least \
            4 characters length"
        );
        std::process::exit(1);
    }

    loop {
        pswd = (0..pswd_length)
            .map(|_| rand::thread_rng().gen_range('!'..='~'))
            .collect();

        // We have to ensure that `pswd` String contains at least one number,
        // one sign, one lowercase and one uppercase character
        if pswd_validation(&pswd) {
            break;
        }
    }

    println!("{}", pswd);
}

fn pswd_validation(char_seq: &String) -> bool {
    if char_seq.contains(rand::thread_rng().gen_range('0'..='9'))
        && char_seq.contains(rand::thread_rng().gen_range('a'..='z'))
        && char_seq.contains(rand::thread_rng().gen_range('A'..='Z'))
        && (char_seq.contains(rand::thread_rng().gen_range('!'..='/'))
            || char_seq.contains(rand::thread_rng().gen_range(':'..='@'))
            || char_seq.contains(rand::thread_rng().gen_range('['..='`'))
            || char_seq.contains(rand::thread_rng().gen_range('{'..='~')))
    {
        return true;
    }

    false
}
