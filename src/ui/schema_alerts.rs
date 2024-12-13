extern crate colored;

use colored::*;

pub struct SchemaAlerts;

impl SchemaAlerts {

    pub fn success(file: &str) {
        println!(
            "Schema successfully saved at {}",
            file.blue()
        );
    }

}
