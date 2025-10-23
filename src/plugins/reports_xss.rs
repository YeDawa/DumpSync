use csv::Writer;
use serde::Serialize;
use serde_json::to_writer_pretty;

use std::{
    fs::File, 
    io::Write,
    error::Error,
};

use crate::{
    utils::file::FileUtils,
    handlers::html_handlers::HTMLHandlers,
    ui::report_xss_alerts::ReportXSSAlerts,

    constants::{
        urls::*,
        global::*,
    }
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

    pub fn xml(&self, detections: Vec<(String, usize, String, String)>, output_path: &str) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(output_path)?;

        file.write_all(b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n")?;
        file.write_all(b"<XSSDetectionReport>\n")?;
    
        for (table, row_index, column, value) in detections {
            file.write_all(b"  <Detection>\n")?;
    
            file.write_all(format!("    <Table>{}</Table>\n", &table).as_bytes())?;
            file.write_all(format!("    <RowIndex>{}</RowIndex>\n", row_index).as_bytes())?;
            file.write_all(format!("    <Column>{}</Column>\n", &column).as_bytes())?;
            file.write_all(format!("    <Value>{}</Value>\n", &value).as_bytes())?;
    
            file.write_all(b"  </Detection>\n")?;
        }
    
        file.write_all(b"</XSSDetectionReport>\n")?;
    
        ReportXSSAlerts::generated(output_path);
        Ok(())
    }

    pub fn txt(&self, detections: Vec<(String, usize, String, String)>, output_path: &str) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(output_path)?;

        writeln!(file, "XSS Detection Report")?;
        writeln!(file, "====================")?;

        for (table, row_index, column, value) in detections {
            writeln!(file, "Table   : {}", table)?;
            writeln!(file, "Row     : {}", row_index)?;
            writeln!(file, "Column  : {}", column)?;
            writeln!(file, "Value   : {}", value)?;
            writeln!(file, "---------------------")?;
        }

        ReportXSSAlerts::generated(output_path);
        Ok(())
    }

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
        
        ReportXSSAlerts::generated(output_path);
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
        
        ReportXSSAlerts::generated(output_path);
        Ok(())
    }

    pub fn html(&self, detections: Vec<(String, usize, String, String)>, output_path: &str) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(output_path)?;
        file.write_all(format!("<html><head><title>{}: XSS Reports</title>", Global::app(GlobalNames::AppName)).as_bytes())?;
        file.write_all(format!("<link href='{}' rel='stylesheet'></head><body>", Urls::as_str(UrlsNames::CdnBootstrap)).as_bytes())?;

        file.write_all(format!(
            "<nav class='navbar navbar-dark navbar-expand-lg bg-dark'><div class='container-fluid'><img src='{}' height='36'/><a class='navbar-brand'>XSS Scan Results</a></div></nav>",
            Global::app(GlobalNames::AppIcon),
        ).as_bytes())?;

        file.write_all(b"<div class='container-fluid pt-3'><table class='table table-striped table-bordered table-hover table-dark'>")?;
        file.write_all(b"<tr><th>Table</th><th>Row Index</th><th>Column</th><th>Value</th></tr>")?;

        for (table, row_index, column, value) in detections {
            let encoded_table = HTMLHandlers.html_escape(&table);
            let encoded_column = HTMLHandlers.html_escape(&column);
            let encoded_value = HTMLHandlers.html_escape(&value);

            file.write_all(format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                encoded_table, row_index, encoded_column, encoded_value
            ).as_bytes())?;
        }

        file.write_all(b"</table></div></body></html>")?;
        
        ReportXSSAlerts::generated(output_path);
        Ok(())
    }
    
    pub fn autodetect(&self, detections: Vec<(String, usize, String, String)>, file_path: Option<&str>) -> Result<(), Box<dyn Error>> {
        if let Some(file_path) = file_path {
            let extension = FileUtils::extension(file_path);

            let result = match extension.as_str() {
                "txt" => self.txt(detections, file_path),
                "csv" => self.csv(detections, file_path),
                "xml" => self.xml(detections, file_path),
                "json" => self.json(detections, file_path),
                "html" => self.html(detections, file_path),
                _ => Ok(ReportXSSAlerts::invalid_format()),
            };

            result?;
        }

        Ok(())
    }

}
