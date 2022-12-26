use crate::cli;
use clap::{Args};
use crate::file;

#[derive(Args)]
pub struct AddArgs {
    path: std::path::PathBuf,
}

impl cli::Execute for AddArgs {

    fn execute(&self) {

        let mut wrt = file::writer::Writer::new();
        wrt.add_entry(&self.path);
    }
}