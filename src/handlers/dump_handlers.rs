use chrono::Local;

use std::{
    fs, 
    thread, 
    process, 
    path::Path, 
    time::Duration,
};

use crate::{
    helpers::configs::Configs,
    utils::generate::Generate,

    ui::{
        errors_alerts::ErrorsAlerts, 
        reconnect_alerts::ReconnectAlerts,
    },
};

pub struct DumpHandlers;

impl DumpHandlers {

    pub fn generate_dump_file_path(&self, dbname: &str, dump_file_path: &str) -> String {
        Path::new(&dump_file_path)
            .join(format!(
                "{}_{}_{}.sql",
                dbname.replace(|c: char| !c.is_alphanumeric(), "_"),
                Local::now().format("%Y_%m_%d_%H%M%S"),
                Generate.random_string(6)
            ))
            .to_str()
            .expect("Failed to convert PathBuf to str")
            .to_string()
    }

    pub fn get_most_recent_sql_file(&self, dump_file_path: &str) -> Option<String> {
        fs::read_dir(&dump_file_path)
            .ok()?
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().extension().map(|ext| ext == "sql").unwrap_or(false))
            .max_by_key(|entry| entry.metadata().ok().and_then(|meta| meta.modified().ok()))
            .map(|entry| entry.path().display().to_string())
    }

    pub fn setup_retry_config(&self) -> (usize, u64, u64) {
        let max_retries = Configs.generic("connection", "max_retries").as_u64().unwrap_or(3);
        let retry_interval = Configs.generic("connection", "retry_connection_interval")
            .as_u64()
            .unwrap_or(60);

        (0, max_retries, retry_interval)
    }

    pub fn handle_retry(&self, attempt: &mut usize, error: &'static str, max_retries: u64, retry_interval: u64) {
        ErrorsAlerts::attempt(error);

        *attempt += 1;
        if *attempt >= max_retries as usize {
            ErrorsAlerts::max_attempts();
            process::exit(1);
        } else {
            ReconnectAlerts::reconnect(*attempt as u64, max_retries);
            thread::sleep(Duration::from_secs(retry_interval));
        }
    }

}
