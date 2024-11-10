use chrono::Local;

use std::{
    fs,
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
        },
    },
    
};

use crate::{
    utils::generate::Generate,

    ui::{
        errors_alerts::ErrorsAlerts, 
        normal_alerts::NormalAlerts, 
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

static DUMP_COUNT: AtomicUsize = AtomicUsize::new(0);

impl Dump {

    pub fn new(
        host: &str,
        port: u64,
        user: &str,
        password: &str,
        dbname: &str,
        backup_path: &str,
        interval: Option<u64>,
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
        let dump_file_path = self.generate_dump_file_path();
        let password = if self.password.is_empty() { "" } else { &self.password };

        Export::new(
            &self.host,
            self.port as u16,
            &self.user,
            password,
            &self.dbname,
            &dump_file_path,
        )
        .dump()
        .map_err(|_| "Failed to generate dump file")?;

        DUMP_COUNT.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    fn generate_dump_file_path(&self) -> String {
        Path::new(&self.dump_file_path)
            .join(format!(
                "{}_{}_{}.sql",
                self.dbname.replace(|c: char| !c.is_alphanumeric(), "_"),
                Local::now().format("%Y_%m_%d_%H%M%S"),
                Generate.random_string(6)
            ))
            .to_str()
            .expect("Failed to convert PathBuf to str")
            .to_string()
    }

    fn get_most_recent_sql_file(&self) -> Option<String> {
        fs::read_dir(&self.dump_file_path)
            .ok()?
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().extension().map(|ext| ext == "sql").unwrap_or(false))
            .max_by_key(|entry| entry.metadata().ok().and_then(|meta| meta.modified().ok()))
            .map(|entry| entry.path().display().to_string())
    }

    pub fn export(&self) {
        let running = Arc::new(AtomicBool::new(true));
        self.setup_ctrlc_handler(running.clone());

        let (mut attempt, max_retries, retry_interval) = self.setup_retry_config();

        while running.load(Ordering::SeqCst) {
            if let Err(e) = self.exec() {
                self.handle_retry(&mut attempt, e, max_retries, retry_interval);
            } else {
                attempt = 0;
                thread::sleep(Duration::from_secs(self.interval));
            }
        }
    }

    fn setup_ctrlc_handler(&self, running: Arc<AtomicBool>) {
        let dump_file_path = self.dump_file_path.clone();
        let host = self.host.clone();
        let user = self.user.clone();
        let password = self.password.clone();
        let dbname = self.dbname.clone();
        let interval = self.interval;

        ctrlc::set_handler(move || {
            running.store(false, Ordering::SeqCst);
            
            let dump = Dump {
                host: host.clone(),
                port: 0,
                user: user.clone(),
                password: password.clone(),
                dbname: dbname.clone(),
                interval,
                dump_file_path: dump_file_path.clone(),
            };

            let dump_count = DUMP_COUNT.load(Ordering::SeqCst);

            if let Some(last_dump) = dump.get_most_recent_sql_file() {
                NormalAlerts::report(&dump_file_path, dump_count, &last_dump);
            }

            SuccessAlerts::terminate();
            process::exit(0);

        }).expect("Error setting Ctrl-C handler");
    }

    fn setup_retry_config(&self) -> (usize, u64, u64) {
        let max_retries = Configs.generic("connection", "max_retries").as_u64().unwrap_or(3);
        let retry_interval = Configs.generic("connection", "retry_connection_interval")
            .as_u64()
            .unwrap_or(60);

        (0, max_retries, retry_interval)
    }

    fn handle_retry(&self, attempt: &mut usize, error: &'static str, max_retries: u64, retry_interval: u64) {
        ErrorsAlerts::attempt(error);

        *attempt += 1;

        if *attempt >= max_retries as usize {
            ErrorsAlerts::max_attempts();
        } else {
            NormalAlerts::reconnect(*attempt as u64, max_retries);
            thread::sleep(Duration::from_secs(retry_interval));
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
