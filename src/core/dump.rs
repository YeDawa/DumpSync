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
            AtomicUsize, 
        }
    }, 
};

use crate::{
    ui::{
        errors_alerts::ErrorsAlerts,
        success_alerts::SuccessAlerts,
        reconnect_alerts::ReconnectAlerts,
    },

    core::{
        export::Export,
        import::Import,
        transfer::Transfer,
    },

    handlers::{
        dump_handlers::DumpHandlers,
        reports_handlers::ReportsHandlers,
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

    once: Option<bool>,
    max: Option<u64>,
    pdf: Option<bool>
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
        encrypt: Option<bool>,

        once: Option<bool>,
        max: Option<u64>,
        pdf: Option<bool>
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
            encrypt,

            once,
            max,
            pdf,
        }
    }

    fn get_final_path(&self) -> String {
        let path_join = Path::new(&self.path).join(&self.dbname);
        path_join.display().to_string()
    }

    fn exec(&self) -> Result<String, &'static str> {
        let dump_file_path = DumpHandlers.generate_dump_file_path(&self.dbname, &self.dump_file_path);
        let password = if self.password.is_empty() { "" } else { &self.password };

        Export::new(
            &self.host,
            self.port as u16,
            &self.user,
            password,
            &self.dbname,
            &dump_file_path,
            self.encrypt,
            None
        ).dump().map_err(|_| "Failed to generate dump file")?;

        DUMP_COUNT.fetch_add(1, Ordering::SeqCst);
        Ok(self.get_final_path())
    }

    fn setup_ctrlc_handler(&self, running: Arc<AtomicBool>) {
        let final_path = self.get_final_path();
        let interval = self.interval;
        let pdf = self.pdf.clone();
    
        ctrlc::set_handler(move || {
            running.store(false, Ordering::SeqCst);
            ReportsHandlers::new(
                &final_path, 
                &interval, 
                DUMP_COUNT.load(Ordering::SeqCst),
                pdf,
            ).report();
    
            SuccessAlerts::terminate();
            process::exit(0);
    
        }).expect("Error setting Ctrl-C handler");
    }

    fn retain(&self, attempt: &mut usize, max_retries: u64, retry_interval: u64) {
        if let Some(max) = self.max {
            let mut num_dump = 0;

            loop {
                let path = self.exec().unwrap_or_default();

                match self.exec() {
                    Ok(_) => {
                        num_dump += 1;
                    }

                    Err(e) => {
                        DumpHandlers.handle_retry(attempt, e, max_retries, retry_interval);
                    }
                }
        
                if num_dump >= max {
                    let dump_count = DUMP_COUNT.load(Ordering::SeqCst);

                    ReportsHandlers::new(
                        &path,
                        &self.interval, 
                        dump_count,
                        self.pdf,
                    ).report();

                    process::exit(0);
                }

                thread::sleep(Duration::from_secs(self.interval));
            }
        }
    }

    fn once(&self, attempt: &mut usize, max_retries: u64, retry_interval: u64) {
        if self.once.unwrap_or(false) {
            let mut success = false;
            let mut path = String::new();
    
            for _ in 0..max_retries {
                path = self.exec().unwrap_or_default();

                if !path.is_empty() {
                    success = true;
                    break;
                } else {
                    *attempt += 1;
                    ReconnectAlerts::reconnect(*attempt as u64, max_retries);
                    thread::sleep(Duration::from_secs(retry_interval));
                }
            }
    
            if success == false {
                ErrorsAlerts::max_attempts();
                process::exit(1);
            }

            ReportsHandlers::new(
                &path, 
                &self.interval, 
                DUMP_COUNT.load(Ordering::SeqCst),
                self.pdf
            ).report();
    
            process::exit(0);
        }
    }
    
    pub fn export(&self) {
        let running = Arc::new(AtomicBool::new(true));
        
        self.setup_ctrlc_handler(running.clone());
        let (mut attempt, max_retries, retry_interval) = DumpHandlers.setup_retry_config();

        self.once(&mut attempt, max_retries, retry_interval);
        self.retain(&mut attempt, max_retries, retry_interval);
        
        while running.load(Ordering::SeqCst) {
            if let Err(e) = self.exec() {
                DumpHandlers.handle_retry(&mut attempt, e, max_retries, retry_interval);
            } else {
                attempt = 0;
                thread::sleep(Duration::from_secs(self.interval));
            }
        }
    }

    pub fn import(&self) {
        Import::new(
            &self.host,
            self.port,
            &self.user,
            &self.password,
            &self.dbname,
            Some(self.dump_file_path.as_str()),
            Some(self.path.as_str()),
            None,
        ).dump().expect("Failed to import dump");
    }

    pub fn transfer(&self) {
        Transfer::new(
            &self.host, self.port, &self.user, &self.password, &self.dbname, &self.dump_file_path, &self.path,
        ).dump().expect("Failed to transfer dump");
    }

}
