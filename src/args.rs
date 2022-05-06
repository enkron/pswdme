use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(version, author, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommands: Subcommands,
}

#[derive(Subcommand, Debug)]
pub enum Subcommands {
    /// Generate family options
    Gen(GenOpts),
}

#[derive(Args, Debug)]
pub struct GenOpts {
    /// Password length
    #[clap(default_value_t = 12)]
    pub length: u8,
}
