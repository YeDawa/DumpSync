extern crate colored;

use colored::*;

use crate::utils::date::Date;

pub struct ErrorsAlerts;

impl ErrorsAlerts {

    pub fn env(e: &str) {
        let current_datetime = Date::date_time();

        eprintln!(
            "{} Failed to download the file: {}", 
            current_datetime.red().bold(), 
            e.red()
        );
    }

    pub fn dump(e: &str) {
        let current_datetime = Date::date_time();

        println!(
            "{} Failed to dump the database: {}", 
            current_datetime.red().bold(), 
            e.red()
        );
    }

    pub fn import(database: &str, command: &str, error: &str) {
        let current_datetime = Date::date_time();

        println!(
            "{} Failed to execute the command to '{}': '{}'. Error: '{}'", 
            current_datetime.red().bold(), 
            database.cyan(),
            command.yellow(),
            error.red()
        );
    }

    pub fn attempt(error: &str) {
        let current_datetime = Date::date_time();

        println!(
            "{} Error during backup execution: '{}'", 
            current_datetime.red().bold(), 
            error.red()
        );
    }

    pub fn max_attempts() {
        let current_datetime = Date::date_time();

        println!(
            "{} Maximum number of reconnection attempts reached. Shutting down.", 
            current_datetime.red().bold(),
        );
    }

    pub fn checksum(error: &str) {
        let current_datetime = Date::date_time();

        println!(
            "{} Error generating checksum': {}", 
            current_datetime.red().bold(),
            error.red()
        );
    }

    pub fn push() {
        let current_datetime = Date::date_time();

        println!(
            "{} Failed to upload dump to the cloud.", 
            current_datetime.red().bold(),
        );
    }

    pub fn pull(message: &str) {
        let current_datetime = Date::date_time();

        println!(
            "{} Failed to download dump from the cloud: {}", 
            current_datetime.red().bold(),
            message.red()
        );
    }

    pub fn open_link() {
        let current_datetime = Date::date_time();

        println!(
            "{} Failed to open the link in the browser.", 
            current_datetime.red().bold(),
        );
    }

    pub fn api_key() {
        let current_datetime = Date::date_time();

        println!(
            "{} Failed to set the API key in the environment variable.", 
            current_datetime.red().bold(),
        );
    }

}
