extern crate colored;
extern crate figlet_rs;

use colored::*;
use figlet_rs::FIGfont;

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
        let name = env!("CARGO_PKG_NAME");
        let standard_font = FIGfont::standard().unwrap();
        
        if let Some(title) = standard_font.convert(&name) {
            println!("{}", &title.to_string().bold().cyan());

            println!(
                "Version: {} | Author: {} | License: {} | Home: {}",

                env!("CARGO_PKG_VERSION").bold().green(),
                env!("CARGO_PKG_AUTHORS").bold().cyan(),
                env!("CARGO_PKG_LICENSE").bold().blue(),
                env!("CARGO_PKG_HOMEPAGE").bold().yellow()
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
