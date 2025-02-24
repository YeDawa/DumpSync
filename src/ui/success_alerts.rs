extern crate colored;

use colored::*;

use crate::utils::date::Date;

pub struct SuccessAlerts;

impl SuccessAlerts {

    pub fn dump(file: &str) {
        let current_datetime = Date::date_time();
    
        println!(
            "\r{} Dump successfully completed and saved at {}", 
            current_datetime.green().bold(), 
            file.blue()
        );
    }

    pub fn table(table: &str) {
        let current_datetime = Date::date_time();
    
        println!(
            "{} Table '{}' successfully imported.", 
            current_datetime.green().bold(), 
            table.blue()
        );
    }

    pub fn truncate(table: &str) {
        let current_datetime = Date::date_time();
    
        println!(
            "{} Table '{}' successfully truncated.", 
            current_datetime.green().bold(), 
            table.blue()
        );
    }

    pub fn import(database: &str) {
        let current_datetime = Date::date_time();
    
        println!("{}", "-".repeat(16));
        println!(
            "{} Dump successfully imported into the database `{}`", 
            current_datetime.green().bold(), 
            database.blue()
        );
    }

    pub fn terminate() {
        let current_datetime = Date::date_time();

        println!(
            "\n{} {}",
            current_datetime.green().bold(),
            "Process terminated by user. Exiting gracefully...".red().bold(),
        );
    }

    pub fn settings() {
        let current_datetime = Date::date_time();
    
        println!(
            "\r{} The settings file was successfully created", 
            current_datetime.green().bold(),
        );
    }

}
