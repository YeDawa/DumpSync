extern crate colored;
use std::collections::HashSet;

use colored::*;

pub struct ReportAlerts;

impl ReportAlerts {

    pub fn report(dump_file_path: &str, dump_count: usize, last_dump: &str, size: &str, interval: usize) {
        println!("\nFinal Report:\n");

        println!("Directory: {}", dump_file_path.bold().blue());
        println!("Interval: {} seconds", interval.to_string().bold().blue());
        println!("Total number of dumps: {}", dump_count.to_string().bold().blue());
        println!("Last dump: {} ({})", last_dump.bold().cyan(), size.bold().yellow());
    }

    pub fn tables(tables: &HashSet<String>) {
        println!("\nTables dumped:");

        for table in tables {
            println!(" - {}", table.bold().blue());
        }
    }

    pub fn no_tables() {
        let message = "No tables found in the dump.";
        println!("{}", message.bold().red());
    }

    pub fn success_pdf(file: &str) {
        let message = "Report generated successfully. Saved in: ".to_string();
        println!("\n{}{}", message.bold().green(), file.bold().cyan());
    }

}
