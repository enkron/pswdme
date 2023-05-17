use clap::{Parser, Subcommand};
use rand::{thread_rng, Rng};

mod validation;
use crate::validation::validate;

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

            if let Ok(v) = validate(&pswd) {
                println!("{v}");
                break;
            }
        },
        None => {
            println!("TODO: turn on interactive mode if no options provided");
        }
    }
}
