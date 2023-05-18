use clap::{Parser, Subcommand};
use rand::{thread_rng, Rng};

mod validation;
use crate::validation::validate;
use crate::validation::ValidationError;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    quiet: bool,
    #[command(subcommand)]
    commands: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(aliases = ["g", "gen"], about = "Password generator")]
    Generate {
        #[arg(index = 1, help = "Password length")]
        length: u8,
    },
}

fn main() {
    let args = Args::parse();

    match args.commands {
        Some(Commands::Generate { length }) => loop {
            let pswd = (0..length)
                .map(|_| thread_rng().gen_range('!'..='~'))
                .collect();

            match validate(&pswd) {
                Ok(v) => {
                    println!("{v}");
                    break;
                }
                Err(e) => match e {
                    ValidationError::WeakPassword => continue,
                    ValidationError::TooShortPassword(s) => panic!("{s}"),
                },
            }
        },
        None => {
            println!("TODO: turn on interactive mode if no options provided");
        }
    }
}
