use std::{
    fs,
    time::SystemTime,
};

use crate::{
    utils::file::FileUtils,
    ui::report_alerts::ReportAlerts,
};

pub struct ReportsHandlers;

impl ReportsHandlers {

    fn get_most_recent_sql_file(&self, dump_file_path: &str) -> Option<(String, String)> {
        fs::read_dir(&dump_file_path)
            .ok()?
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().extension().map(|ext| ext == "sql").unwrap_or(false))
            .max_by_key(|entry| entry.metadata().ok().and_then(|meta| meta.modified().ok()).unwrap_or(SystemTime::UNIX_EPOCH))
            .and_then(|entry| {
                let path = entry.path();
                let file_size = entry.metadata().ok()?.len();
                Some((path.display().to_string(), FileUtils::size(file_size)))
            })
    }

    pub fn report(&self, path: &str, interval: usize, counter: usize) {
        if let Some((last_dump, size)) = self.get_most_recent_sql_file(&path) {
            ReportAlerts::report(&path, counter, &last_dump, &size, interval as usize);
        }
    }

}
