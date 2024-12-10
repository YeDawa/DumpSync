use csv::Writer;
use std::error::Error;

use crate::ui::scan_alerts::ScanAlerts;

pub struct Reports;

impl Reports {

    pub fn xss(&self, detections: Vec<(String, usize, String, String)>, output_path: &str) -> Result<(), Box<dyn Error>> {
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
    
}
