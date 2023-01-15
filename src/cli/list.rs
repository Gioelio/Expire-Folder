use crate::cli;
use clap::{Args};
use crate::error;
use crate::file::writer::{Row, Writer};
use colored::Colorize;

#[derive(Args)]
pub struct List {
    #[arg(short, long)]
    pub remove: bool,

    #[arg(short, long, default_value_t = false)]
    pub exp: bool
}

impl List {
    fn print_row(entry: &Row){
        if entry.is_expired() {
            println!("   - {} ({} days ago)", entry.path.as_os_str().to_str().unwrap(), entry.get_days_expired().to_string().as_str().red());
        } else {
            println!("   - {} ({} days remaining)", entry.path.as_os_str().to_str().unwrap(), entry.get_days_left().to_string().as_str().green());
        }
    }
}

impl cli::Execute for List {

    fn execute(&self) -> Result<(), error::ErrorKind> {
        let wrt = Writer::new();
        let exp_list = wrt.get_expired();

        println!("Expired files:");
        if exp_list.len() != 0 {
            for entry in exp_list {
                List::print_row(&entry);

                if self.remove {
                    wrt.remove_entry(&entry.path)?;
                }

            }
            println!("\n(suggest: For removing all elements in the list, add the --remove flag)");
        } else {
            println!("   No expired files");
        }

        if !self.exp {
            let no_exp_row: Vec<Row> = wrt.get_all().iter().filter(|row| !row.is_expired()).cloned().collect();
            if no_exp_row.len() != 0 {
                println!("\nNot expired yet:");
                let _ = no_exp_row.iter().for_each(|row| List::print_row(row));
            }
        }

        Ok(())
    }

}

