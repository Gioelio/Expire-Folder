use clap::{Subcommand, Parser, Args};

#[derive(Parser)]
#[command(version, propagate_version = true, long_about = None)]
pub struct Cli {
   #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add(PathArg)
}

#[derive(Args)]
struct PathArg {
    #[arg(short, long)]
    path: Option<std::path::PathBuf>,
}