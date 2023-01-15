use clap::{Subcommand, Parser};
pub mod add;
pub mod list;

use crate::error;

#[derive(Parser)]
#[command(version, propagate_version = true, long_about = None)]
pub struct Cli {
   #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add(add::AddArgs),
    List(list::List)
}

pub trait Execute {
    fn execute(&self) -> Result<(), error::ErrorKind>;
}