extern crate colored;

use colored::*;

pub struct ReportXSSAlerts;

impl ReportXSSAlerts {

    pub fn generated(output_path: &str) {
        println!("Report generated and salved in: {}", output_path.green());
    }

    pub fn invalid_format() {
        let message = "Invalid file format, only TXT, XML, CSV, HTML and JSON are supported.";

        println!(
            "{}", message.red().bold(), 
        );
    }

}
