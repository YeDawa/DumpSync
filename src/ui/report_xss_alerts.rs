extern crate colored;

use colored::*;

use crate::constants::global::Global;

pub struct ReportXSSAlerts;

impl ReportXSSAlerts {

    pub fn generated(output_path: &str) {
        println!("Report generated and salved in: {}", output_path.green());
    }

    pub fn invalid_format() {
        let formats = Global::formats_supported().join(", ");
        let message = format!(
            "Invalid file format, only {} are supported.",
            formats.to_uppercase()
        );

        println!(
            "{}", message.red().bold(), 
        );
    }

}
