use chrono::Local;

use std::{
    thread,
    process,
    path::Path,
    time::Duration,

    sync::{
        Arc,

        atomic::{
            Ordering, 
            AtomicBool
        },
    },
};

use crate::{
    utils::generate::Generate,

    ui::{
        normal_alerts::NormalAlerts,
        errors_alerts::ErrorsAlerts, 
        success_alerts::SuccessAlerts
    }, 

    engine::{
        export::Export,
        import::Import,
        configs::Configs,
    }, 
};

pub struct Dump {
    port: u64,
    host: String,
    user: String,
    interval: u64,
    dbname: String,
    password: String,
    dump_file_path: String,
}

impl Dump {

    pub fn new(
        host: &str, 
        port: u64, 
        user: &str, 
        password: &str, 
        dbname: &str, 
        backup_path: &str, 
        interval: Option<u64>
    ) -> Self {
        Self {
            host: host.to_string(),
            port,
            user: user.to_string(),
            password: password.to_string(),
            dbname: dbname.to_string(),
            interval: interval.unwrap_or(3600),
            dump_file_path: backup_path.to_string(),
        }
    }

    fn exec(&self) -> Result<(), &'static str> {
        let dump_file_path = Path::new(&self.dump_file_path)
            .join(format!(
                "backup_{}_{}_{}.sql",
                self.dbname.replace(|c: char| !c.is_alphanumeric(), "_"),
                Local::now().format("%Y_%m_%d_%H%M%S"),
                Generate.random_string(6)
            ));

        let password = if !self.password.is_empty() {
            self.password.as_str()
        } else {
            ""
        };

        Export::new(
            &self.host,
            self.port as u16,
            &self.user,
            password,
            &self.dbname,
            dump_file_path.to_str().expect("Failed to convert PathBuf to str"),
        ).dump().map_err(|_| "Failed to generate dump file")
    }

    pub fn export(&self) {
        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();
        
        ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
            SuccessAlerts::terminate();
            process::exit(0);
        }).expect("Error setting Ctrl-C handler");
        
        let mut attempt = 0;
        let max_retries = Configs.conn("max_retries").as_u64().unwrap();
        let retry_connection_interval = Configs.conn("retry_connection_interval").as_u64().unwrap();

        while running.load(Ordering::SeqCst) {
            match self.exec() {
                Ok(_) => {
                    attempt = 0;
                }

                Err(e) => {
                    ErrorsAlerts::attempt(e);
                    attempt += 1;

                    if attempt >= 3 {
                        ErrorsAlerts::max_attempts();
                        break;
                    } else {
                        NormalAlerts::reconnect(attempt, max_retries);
                        thread::sleep(Duration::from_secs(retry_connection_interval));
                        continue;
                    }
                }
            }

            thread::sleep(Duration::from_secs(self.interval));
        }
    }

    pub fn import(&self) {
        Import::new(
            &self.host,
            self.port as u16,
            &self.user,
            &self.password,
            &self.dbname,
            &self.dump_file_path,
        ).dump().expect("Failed to import dump");
    }

}
