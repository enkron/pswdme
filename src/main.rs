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

    loop {
        pswd = (0..parsed_option)
            .map(|_| rand::thread_rng().gen_range('!'..='~'))
            .collect();

        // We have to ensure that `pswd` String contains at least one number,
        // one sign, one lowercase and one uppercase character
        if pswd.contains(rand::thread_rng().gen_range('0'..='9'))
            && pswd.contains(rand::thread_rng().gen_range('a'..='z'))
            && pswd.contains(rand::thread_rng().gen_range('A'..='Z'))
            && (pswd.contains(rand::thread_rng().gen_range('!'..='/'))
                || pswd.contains(rand::thread_rng().gen_range(':'..='@'))
                || pswd.contains(rand::thread_rng().gen_range('['..='`'))
                || pswd.contains(rand::thread_rng().gen_range('{'..='~')))
        {
            break;
        }
    }

    if parsed_option.eq(&0) {
        eprintln!("warning: password length was not provided. using default value.")
    };

    println!("{}", pswd);
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
        lenght value, which is 12.",
        )
        .get_matches()
}
