use clap::{App, Arg, ArgMatches};
use rand::Rng;
use std::{process, str};

fn main() {
    let mut pswd: String;
    let parsed_option = arg_parse()
        .value_of("PSWD_LENGTH")
        .unwrap()
        .parse::<u8>()
        .unwrap_or_else(|e| {
            eprintln!("error: {}", e);
            process::exit(1);
        });

    // To satisfy all validation cariterias a password must be at least 4
    // characters length, therefore the program will panic if provided
    // `PSWD_LENGTH` value will be less then 4
    if parsed_option < 4 {
        eprintln!(
            "error: to satisfy all security criterias password must be at least \
            4 characters length"
        );
        process::exit(1);
    }

    if arg_parse().occurrences_of("PSWD_LENGTH").eq(&0) {
        eprintln!("warning: password length was not provided. using default value.")
    };

    loop {
        pswd = (0..parsed_option)
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

fn arg_parse() -> ArgMatches<'static> {
    App::new(clap::crate_name!())
        .author(clap::crate_authors!("\n"))
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .arg(
            Arg::with_name("PSWD_LENGTH")
                //.index(1)
                // NOTE: If no Arg::short, or Arg::long have been defined,
                // you can optionally leave off the index method, and the index
                // will be assigned in order of evaluation. Utilizing the index
                // method allows for setting indexes out of order
                // https://docs.rs/clap/2.32.0/clap/struct.Arg.html#method.index
                .default_value("12")
                .help("Password length"),
        )
        .after_help(
            "Super trivial program that generates random password.\n\
        If `PSWD_LENGTH` option is not provided the programs uses default \
        length value, which is 12.",
        )
        .get_matches()
}
