use clap::Parser;

mod args;
use args::{Cli, Subcommands};

fn main() {
    let cli = Cli::parse();
    //dbg!(&cli);
    match cli.subcommands {
        Subcommands::Gen(opts) => {
            println!("{}", opts.length);
        }
    }
}
