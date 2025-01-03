extern crate colored;

use colored::*;

use crate::utils::date::Date;

pub struct ReconnectAlerts;

impl ReconnectAlerts {

    pub fn reconnect(attempt: u64, max_retries: u64) {
        let current_datetime = Date::date_time();
    
        println!(
            "\r{} Reconnection attempt in 5 seconds... (Attempt {}/{})", 
            current_datetime.green().bold(), 
            attempt.to_string().blue(),
            max_retries.to_string().yellow()
        );
    }

}
