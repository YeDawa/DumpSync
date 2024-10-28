use chrono::Local;

use std::{
    thread,
    fs::File,
    path::Path,
    time::Duration,
    process::Command,

    sync::{
        Arc,

        atomic::{
            Ordering,
            AtomicBool, 
        },
    },

    io::{
        self, 
        Write
    },
};

use crate::{
    utils::generate::Generate,

    ui::{
        errors_alerts::ErrorsAlerts,
        success_alerts::SuccessAlerts,
    },
};

pub struct Dump {
    user: String,
    interval: u64,
    dbname: String,
    password: String,
    dump_file_path: String,
}

impl Dump {

    pub fn new(user: &str, password: &str, dbname: &str, backup_path: &str, interval: u64) -> Self {
        Self {
            user: user.to_string(),
            password: password.to_string(),
            dbname: dbname.to_string(),
            interval,
            dump_file_path: backup_path.to_string(),
        }
    }

    fn exec(&self) {
        let unique_id = Generate.random_string(6);
        let dump_file_path = format!(
            "{}backup_{}_{}_{}.sql",
            self.dump_file_path,
            self.dbname.replace(|c: char| !c.is_alphanumeric(), "_"),
            Local::now().format("%Y_%m_%d_%H%M%S"),
            unique_id
        );

        let output = if self.password.is_empty() {
            Command::new("mysqldump")
                .arg("-u")
                .arg(&self.user)
                .arg(&self.dbname)
                .output()
                .expect("Failed to execute mysqldump")
        } else {
            Command::new("mysqldump")
                .arg("-u")
                .arg(&self.user)
                .arg("-p")
                .arg(&self.password)
                .arg(&self.dbname)
                .output()
                .expect("Failed to execute mysqldump")
        };

        if output.status.success() {
            let mut file = File::create(Path::new(&dump_file_path)).expect("Could not create the dump file.");
            file.write_all(&output.stdout).expect("Failed to write to the file.");

            SuccessAlerts::dump(&dump_file_path);
            io::stdout().flush().unwrap();
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            ErrorsAlerts::dump(&stderr);
        }
    }

    pub fn make_dump(&self) {
        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();

        ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
            SuccessAlerts::terminate();
        }).expect("Error setting Ctrl-C handler");

        while running.load(Ordering::SeqCst) {
            self.exec();
            thread::sleep(Duration::from_secs(self.interval));
        }
    }
    
}
