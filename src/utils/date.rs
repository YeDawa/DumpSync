extern crate chrono;

use chrono::Local;

pub struct Date;

impl Date {

    pub fn date_time() -> String {
        let local_time = Local::now();
    
        let date_formated = local_time.format("%Y-%m-%d").to_string();
        let hour_formated = local_time.format("%H:%M:%S").to_string();
    
        format!("{} {}", date_formated, hour_formated)
    }

}
