use clap::{App, Arg, ArgMatches};
use rand::Rng;
use std::{process, str};

fn main() {
    let pswd: String = (0..arg_parse()
        .value_of("PSWD_LENGTH")
        .unwrap()
        .parse::<u8>()
        .unwrap_or_else(|e| {
            eprintln!("{}", e);
            process::exit(1);
        }))
        .map(|_| rand::thread_rng().gen_range('!'..'~'))
        .collect();

    if arg_parse().occurrences_of("PSWD_LENGTH").eq(&0) {
        eprintln!("warning: password length was not provided. using default value.")
    }

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
            "Super trivial program that generates random passwrod.\n\
        If `PSWD_LENGTH` option is not provided the programs uses default \
        lenght value, which is 12.",
        )
        .get_matches()
}
