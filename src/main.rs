#![feature(absolute_path)]

mod cli;
mod file;
mod error;

use clap::Parser;
use crate::cli::Execute;
use crate::error::ErrorKind;

fn main() {

    let cli = cli::Cli::parse();

    match &cli.command {
        cli::Commands::Add(add_args) => {
            let err = add_args.execute().err();
            if err.is_some() {
                panic!("{}", err.unwrap().as_str());
            }
        }
    }

}
