extern crate colored;

use colored::*;

pub struct ReportAlerts;

impl ReportAlerts {

    pub fn generated(output_path: &str) {
        println!("{}", "-".repeat(50));
        println!("Report generated and salved in: {}", output_path.green());
    }

    pub fn invalid_format() {
        let message = "Invalid file format, only TXT, CSV, HTM/HTML and JSON are supported.";

        println!("{}", "-".repeat(50));
        println!(
            "{}", message.red().bold(), 
        );
    }

}
