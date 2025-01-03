extern crate colored;

use colored::*;

use crate::utils::date::Date;

pub struct NormalAlerts;

impl NormalAlerts {

    pub fn reconnect(attempt: u64, max_retries: u64) {
        let current_datetime = Date::date_time();
    
        println!(
            "\r{} Reconnection attempt in 5 seconds... (Attempt {}/{})", 
            current_datetime.green().bold(), 
            attempt.to_string().blue(),
            max_retries.to_string().yellow()
        );
    }

    pub fn report(dump_file_path: &str, dump_count: usize, last_dump: &str) {
        println!("\nFinal Report:\n");

        println!("Directory: {}", dump_file_path.bold().blue());
        println!("Total number of dumps: {}", dump_count.to_string().bold().blue());
        println!("Last dump: {}", last_dump.bold().cyan());
    }

}
