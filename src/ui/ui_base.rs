extern crate colored;
extern crate figlet_rs;

use colored::*;
use figlet_rs::FIGfont;

use crate::constants::global::*;

pub struct UI;

impl UI {

    fn colorize(text: &str, level: &str) -> String {
        let message = match level {
            "normal" => text.bold().to_string(),
            "info" => text.bold().blue().to_string(),
            "warning" => text.bold().yellow().to_string(),
            "error" => text.bold().red().to_string(),
            "success" => text.bold().green().to_string(),
            _ => text.bold().to_string(),
        };

        message
    }

    pub fn header() {
        let name = Global::app(GlobalNames::AppName);
        let standard_font = FIGfont::standard().unwrap();
        
        if let Some(title) = standard_font.convert(&name) {
            println!("{}", &title.to_string().bold().cyan());

            println!(
                "Version: {} | Author: {} | License: {} | Home: {}",

                Global::app(GlobalNames::AppVersion).bold().green(),
                Global::app(GlobalNames::AppAuthor).bold().cyan(),
                Global::app(GlobalNames::AppLicense).bold().blue(),
                Global::app(GlobalNames::AppHome).bold().yellow()
            );
        }
    }
    
    pub fn section_header(text: &str, level: &str) {
        let message = Self::colorize(&text.to_uppercase(), level);
        println!("\n{}\n", message);
    }
    
    pub fn label(text: &str, level: &str) {
        let message = Self::colorize(&text, level);
        println!("{}", message);
    }

}
