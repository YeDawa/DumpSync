extern crate colored;

use colored::*;

use crate::utils::date::Date;

pub struct ErrorsAlerts;

impl ErrorsAlerts {

    pub fn dump(e: &str) {
        let current_datetime = Date::date_time();

        print!(
            "{} Failed to dump the database: {}", 
            current_datetime.red().bold(), 
            e.red()
        );
    }

    pub fn import(database: &str, command: &str, error: &str) {
        let current_datetime = Date::date_time();

        print!(
            "{} Failed to execute the command to '{}': '{}'. Error: '{}'", 
            current_datetime.red().bold(), 
            database.cyan(),
            command.yellow(),
            error.red()
        );
    }

}
