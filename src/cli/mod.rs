use clap::{Subcommand, Parser};
pub mod add;

#[derive(Parser)]
#[command(version, propagate_version = true, long_about = None)]
pub struct Cli {
   #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add(add::AddArgs)
}

pub trait Execute {
    fn execute(&self);
}