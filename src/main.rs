pub mod cli;

use clap::Parser;
use crate::cli::Execute;

fn main() {

    let cli = cli::Cli::parse();

    match &cli.command {
        cli::Commands::Add(add_args) => {
            add_args.execute();
        }
    }

}
