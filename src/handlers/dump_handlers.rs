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
        let sanitized = dbname.replace(|c: char| !c.is_alphanumeric(), "_");
        let folder = Path::new(dump_file_path).join(&sanitized);
        fs::create_dir_all(&folder).expect("Failed to create dump folder");

        format!(
            "{}/{}_{}_{}.sql",
            folder.display(),
            sanitized,
            Local::now().format("%Y_%m_%d_%H%M%S"),
            Generate.random_string(6)
        )
    }

    pub fn generate_dump_file_truncate_path(&self, dbname: &str, table: &str, dump_file_path: &str) -> String {
        let sanitized = dbname.replace(|c: char| !c.is_alphanumeric(), "_");
        let folder = Path::new(dump_file_path).join(&sanitized);
        fs::create_dir_all(&folder).expect("Failed to create dump folder");

        format!(
            "{}/{}_{}_{}_{}.sql",
            folder.display(),
            sanitized,
            table.replace(|c: char| !c.is_alphanumeric(), "_"),
            Local::now().format("%Y_%m_%d_%H%M%S"),
            Generate.random_string(6)
        )
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
