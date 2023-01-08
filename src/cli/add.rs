use crate::cli;
use clap::{Args};
use crate::file;
use chrono::{DateTime, Days, Months, Utc};
use crate::error::ErrorKind;

#[derive(Args)]
pub struct AddArgs {
    pub path: std::path::PathBuf,

    #[arg(short, long)]
    pub year: Option<u64>,
    #[arg(short, long)]
    pub day: Option<u64>,
    #[arg(short, long)]
    pub month: Option<u64>
}

impl AddArgs {
    fn is_exp_time_set(&self) -> bool{
        self.day.is_some() || self.month.is_some() || self.year.is_some()
    }

    pub fn get_exp_date(&self) -> Result<DateTime<Utc>, ErrorKind> {
        let month = (self.year.unwrap_or(0) * 12) + self.month.unwrap_or(0);

        let mut date = Utc::now().checked_add_months(Months::new(month as u32)).ok_or(ErrorKind::InvalidTime);
        date = date?.checked_add_days(Days::new(self.day.unwrap_or(0))).ok_or( ErrorKind::InvalidTime);

        Ok(date?)
    }
}

impl cli::Execute for AddArgs {

    fn execute(&self) -> Result<(), ErrorKind> {
        let mut wrt = file::writer::Writer::new();

        if !self.is_exp_time_set() {
            return Err(ErrorKind::NoTimeSpecified);
        }

        wrt.add_entry(&self)
    }
}