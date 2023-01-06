use crate::cli;
use clap::{Args};
use crate::file;
use crate::error;

#[derive(Args)]
pub struct AddArgs {
    path: std::path::PathBuf,

    #[arg(short, long)]
    year: Option<u64>,
    #[arg(short, long)]
    day: Option<u64>,
    #[arg(short, long)]
    month: Option<u64>
}

impl AddArgs {
    fn is_exp_time_set(&self) -> bool{
        self.day.is_some() || self.month.is_some() || self.year.is_some()
    }
}

impl cli::Execute for AddArgs {

    fn execute(&self) -> Result<(), error::ErrorKind> {

        let mut wrt = file::writer::Writer::new();
        if !self.is_exp_time_set() {
            return Err(error::ErrorKind::NoTimeSpecified);
        }

        wrt.add_entry(&self.path)
    }
}