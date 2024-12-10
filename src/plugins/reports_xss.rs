use csv::Writer;
use std::fs::File;
use serde::Serialize;
use std::error::Error;
use serde_json::to_writer_pretty;

use crate::ui::scan_alerts::ScanAlerts;

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
    
}
