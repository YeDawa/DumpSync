use std::{
    thread, 
    process, 
    time::Duration,
    
    sync::{
        Arc,

        atomic::{
            AtomicBool, 
            AtomicUsize, 
            Ordering 
        }
    }, 
};

use crate::{
    handlers::dump_handlers::DumpHandlers,
    
    ui::{
        report_alerts::ReportAlerts, 
        success_alerts::SuccessAlerts
    },

    core::{
        export::Export,
        import::Import,
        transfer::Transfer,
    },
};

pub struct Dump {
    port: u16,
    path: String,
    host: String,
    user: String,
    interval: u64,
    dbname: String,
    password: String,
    dump_file_path: String,
    encrypt: Option<bool>,
}

static DUMP_COUNT: AtomicUsize = AtomicUsize::new(0);

impl Dump {

    pub fn new(
        host: &str,
        port: u16,
        user: &str,
        password: &str,
        dbname: &str,
        backup_path: &str,
        interval: Option<u64>,
        path: &str,
        encrypt: Option<bool>
    ) -> Self {
        Self {
            port: port,
            host: host.to_string(),
            user: user.to_string(),
            dbname: dbname.to_string(),
            password: password.to_string(),
            dump_file_path: backup_path.to_string(),
            interval: interval.unwrap_or(3600),
            path: path.to_string(),
            encrypt
        }
    }

    fn exec(&self) -> Result<(), &'static str> {
        let dump_file_path = DumpHandlers.generate_dump_file_path(&self.dbname, &self.dump_file_path);
        let password = if self.password.is_empty() { "" } else { &self.password };

        Export::new(
            &self.host,
            self.port as u16,
            &self.user,
            password,
            &self.dbname,
            &dump_file_path,
            self.encrypt
        ).dump().map_err(|_| "Failed to generate dump file")?;

        DUMP_COUNT.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    pub fn export(&self) {
        let running = Arc::new(AtomicBool::new(true));
        
        self.setup_ctrlc_handler(running.clone());
        let (mut attempt, max_retries, retry_interval) = DumpHandlers.setup_retry_config();

        while running.load(Ordering::SeqCst) {
            if let Err(e) = self.exec() {
                DumpHandlers.handle_retry(&mut attempt, e, max_retries, retry_interval);
            } else {
                attempt = 0;
                thread::sleep(Duration::from_secs(self.interval));
            }
        }
    }

    fn setup_ctrlc_handler(&self, running: Arc<AtomicBool>) {
        let dump_file_path_clone = self.dump_file_path.clone();
        let host_clone = self.host.clone();
        let user_clone = self.user.clone();
        let port_clone = self.port.clone();
        let password_clone = self.password.clone();
        let dbname_clone = self.dbname.clone();
        let interval_clone = self.interval;
        let path_clone = self.path.clone();
        let encrypt_clone = self.encrypt.clone();

        ctrlc::set_handler(move || {
            running.store(false, Ordering::SeqCst);
            
            let _dump = Dump {
                host: host_clone.clone(),
                port: port_clone.clone(),
                user: user_clone.clone(),
                password: password_clone.clone(),
                dbname: dbname_clone.clone(),
                interval: interval_clone,
                dump_file_path: dump_file_path_clone.clone(),
                path: path_clone.clone(),
                encrypt: encrypt_clone
            };

            let dump_count = DUMP_COUNT.load(Ordering::SeqCst);

            if let Some(last_dump) = DumpHandlers.get_most_recent_sql_file(&dump_file_path_clone) {
                ReportAlerts::report(&dump_file_path_clone, dump_count, &last_dump);
            }

            SuccessAlerts::terminate();
            process::exit(0);

        }).expect("Error setting Ctrl-C handler");
    }

    pub fn import(&self) {
        Import::new(
            &self.host,
            self.port,
            &self.user,
            &self.password,
            &self.dbname,
            &self.dump_file_path,
            &self.path,
        ).dump().expect("Failed to import dump");
    }

    pub fn transfer(&self) {
        Transfer::new(
            &self.host,
            self.port,
            &self.user,
            &self.password,
            &self.dbname,
            &self.dump_file_path,
            &self.path,
        ).dump().expect("Failed to transfer dump");
    }

}
