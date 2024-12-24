extern crate colored;

use colored::*;

use crate::utils::date::Date;

pub struct ChecksumAlerts;

impl ChecksumAlerts {

    pub fn file(file: &str) {
        println!("File: {}", file.blue());
    }

    pub fn checksum(file: &str) {
        let current_datetime = Date::date_time();
    
        println!(
            "\r{} The checksum was successfully generated and saved in: {}", 
            current_datetime.green().bold(),
            file.blue()
        );
    }

    pub fn printable(algo: &str, hash: &str) {
        println!(
            "{}: {}",
            algo.cyan(), hash.yellow()
        );
    }

}
