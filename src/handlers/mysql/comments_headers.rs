use std::{
    io::Write,
    error::Error,
};

use crate::utils::date::Date;

pub struct CommentsHeaders;

impl CommentsHeaders {

    pub fn core(&self, dbname: &str, writer: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        writeln!(writer, "-- Exporting using {} v.{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))?;
        writeln!(writer, "-- Database backup: {}", dbname)?;
        writeln!(writer, "-- Export date and time: {}", Date::timestamp())?;
        writeln!(writer, "-- ---------------------------------------------------\n")?;

        Ok(())
    }

    pub fn truncate(&self, dbname: &str, table: &str, writer: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        writeln!(writer, "-- Exporting using {} v.{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))?;
        writeln!(writer, "-- Database: {}", dbname)?;
        writeln!(writer, "-- Truncate table: {}", table)?;
        writeln!(writer, "-- Export date and time: {}", Date::timestamp())?;
        writeln!(writer, "-- ---------------------------------------------------\n")?;

        Ok(())
    }    

}