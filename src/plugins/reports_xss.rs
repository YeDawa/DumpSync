use csv::Writer;
use serde::Serialize;
use serde_json::to_writer_pretty;

use std::{
    fs::File, 
    io::Write,
    error::Error,
};

use crate::{
    ui::scan_alerts::ScanAlerts,
    handlers::reports_handlers::ReportsHandlers,
};

#[derive(Serialize)]
struct Detection {
    table: String,
    row_index: usize,
    column: String,
    value: String,
}

pub struct ReportsXSS;

impl ReportsXSS {

    pub fn csv(&self, detections: Vec<(String, usize, String, String)>, output_path: &str) -> Result<(), Box<dyn Error>> {
        let mut writer = Writer::from_path(output_path)?;
        writer.write_record(&["Table", "Row Index", "Column", "Value"])?;

        for (table, row_index, column, value) in detections {
            writer.write_record(&[
                table,
                row_index.to_string(),
                column,
                value,
            ])?;
        }

        writer.flush()?;
        
        ScanAlerts::reports_generated(output_path);
        Ok(())
    }

    pub fn json(&self, detections: Vec<(String, usize, String, String)>, output_path: &str) -> Result<(), Box<dyn Error>> {
        let detections: Vec<Detection> = detections
            .into_iter()
            .map(|(table, row_index, column, value)| Detection {
                table,
                row_index,
                column,
                value,
            })
            .collect();

        let file = File::create(output_path)?;
        to_writer_pretty(file, &detections)?;
        
        ScanAlerts::reports_generated(output_path);
        Ok(())
    }

    pub fn html(&self, detections: Vec<(String, usize, String, String)>, output_path: &str) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(output_path)?;
        file.write_all(b"<html><head><title>XSS Reports</title><link href='https://cdn.jsdelivr.net/npm/bootstrap@3.3.7/dist/css/bootstrap.min.css' rel='stylesheet'></head>")?;
        file.write_all(b"<body><table class='table'>")?;
        file.write_all(b"<tr><th>Table</th><th stryle='padding: 5px !important;'>Row Index</th><th>Column</th><th>Value</th></tr>")?;

        for (table, row_index, column, value) in detections {
            let encoded_table = ReportsHandlers.html_escape(&table);
            let encoded_column = ReportsHandlers.html_escape(&column);
            let encoded_value = ReportsHandlers.html_escape(&value);

            file.write_all(format!(
                "<tr style=''><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                encoded_table, row_index, encoded_column, encoded_value
            ).as_bytes())?;
        }

        file.write_all(b"</table></body></html>")?;
        
        ScanAlerts::reports_generated(output_path);
        Ok(())
    }
    
}
