mod cli;
mod file;
mod error;

use clap::Parser;
use crate::cli::Execute;

fn main() {

    let cli = cli::Cli::parse();

    match &cli.command {
        cli::Commands::Add(add_args) => {
            let err = add_args.execute().err();
            if err.is_some() {
                eprint!("{}", err.unwrap().as_str());
            }
        }
        cli::Commands::List(list_args) => {
            let err = list_args.execute().err();
            if err.is_some() {
                eprint!("{}", err.unwrap().as_str());
            }
        }
    }

}
