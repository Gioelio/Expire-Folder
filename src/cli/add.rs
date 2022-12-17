use crate::cli;
use std::fs;
use clap::{Args};

#[derive(Args)]
pub struct AddArgs {
    path: std::path::PathBuf,
}

impl cli::Execute for AddArgs {

    fn execute(&self) {
        let abs_path = fs::canonicalize(&self.path).expect("Cannot get absolute path from relative. Check your path");

    }


}