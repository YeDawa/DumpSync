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
            AtomicBool, 
        },
    },
};

use crate::{
    engine::export::Export,
    utils::generate::Generate,
    ui::success_alerts::SuccessAlerts,
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
        interval: u64
    ) -> Self {
        Self {
            host: host.to_string(),
            port,
            user: user.to_string(),
            password: password.to_string(),
            dbname: dbname.to_string(),
            interval,
            dump_file_path: backup_path.to_string(),
        }
    }

    fn exec(&self) {
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
        ).dump().expect("Failed to execute mysqldump");
    }

    pub fn make_dump(&self) {
        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();

        ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
            SuccessAlerts::terminate();
            process::exit(0);
        }).expect("Error setting Ctrl-C handler");

        while running.load(Ordering::SeqCst) {
            self.exec();
            thread::sleep(Duration::from_secs(self.interval));
        }
    }
    
}
