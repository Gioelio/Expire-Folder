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
    pub all: bool
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

        if !self.all {
            println!("Expired files:");
            if !exp_list.is_empty() {
                for entry in exp_list {
                    List::print_row(&entry);

                    if self.remove {
                        wrt.remove_entry(&entry.path).unwrap_or_else(|err| {
                            println!("\t\t{} {}", "Error:".red(), err.as_str().red());
                        });
                    }
                }
                println!("\n({} For removing all elements in the list, add the {} flag)", "Hint:".yellow(), "--remove".italic());
            } else {
                println!("   No expired files (to see unexpired files, add the {} flag)", "--no-exp".italic());
            }
        }

        if self.all {
            let all_rows: Vec<Row> = wrt.get_all_ordered().to_vec();
            if !all_rows.is_empty() {

                println!("Yet Expired:");
                let expired_rows: Vec<&Row> = all_rows.iter().filter(|row| row.is_expired()).collect();
                if expired_rows.is_empty() {
                    println!("   No expired files");
                }
                expired_rows.iter().for_each(|row| List::print_row(row));

                println!("Not expired yet:");
                let not_expired_rows: Vec<&Row> = all_rows.iter().filter(|row| !row.is_expired()).collect();
                if not_expired_rows.is_empty() {
                    println!("   No files not expired yet");
                }
                not_expired_rows.iter().for_each(|row| List::print_row(row));
            }
        }

        Ok(())
    }

}

