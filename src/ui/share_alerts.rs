extern crate colored;

use colored::*;

use crate::constants::global::Global;

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

    pub fn api_key_missing() {
        let api_link = Global::PASTEBIN_API_URI;
        let message = "Please provide a valid API key. Click this link to get one";

        Self::error("API key is missing or empty");
        
        println!(
            "{}: {}",
            message.yellow(),
            api_link.blue()
        );
    }

}
