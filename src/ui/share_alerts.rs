extern crate colored;

use colored::*;

pub struct ShareAlerts;

impl ShareAlerts {

    pub fn success(link: &str) {
        println!(
            "Success! Link: {}",
            link.blue()
        );
    }

    pub fn error(message: &str) {
        println!(
            "An error occurred: {}", message.red().bold(),
        );
    }

}
