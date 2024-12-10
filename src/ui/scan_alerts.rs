extern crate colored;

use colored::*;

pub struct ScanAlerts;

impl ScanAlerts {

    pub fn detected(table: &str, row_index: usize, column: &str, value: &str) {
        println!(
            "Possible XSS detected in table '{}', row {}, column '{}': {}",
            table.blue(), row_index.to_string().green(), column.cyan(), value.yellow()
        );
    }

    pub fn reports_generated(output_path: &str) {
        println!("{}", "-".repeat(50));
        println!("Reports generated in: {}", output_path.green());
    }

}
