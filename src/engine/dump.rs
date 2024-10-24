use chrono::Local;

use std::{
    thread,
    fs::File,
    io::Write,
    path::Path,
    time::Duration,
    process::Command,
};

pub struct Dump {
    user: String,
    password: String,
    dbname: String,
    interval: u64,
    dump_file_path: String,
}

impl Dump {

    pub fn new(user: &str, password: &str, dbname: &str, backup_path: &str, interval: u64) -> Self {
        Self {
            user: user.to_string(),
            password: password.to_string(),
            dbname: dbname.to_string(),
            interval,
            
            dump_file_path: format!(
                "{}backup_{}_{}.sql",
                backup_path.to_string(),
                dbname.replace(|c: char| !c.is_alphanumeric(), "_"),
                Local::now().format("%Y_%m_%d_%H%M%S")
            )
        }
    }

    fn exec(&self) {
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
            let mut file = File::create(Path::new(&self.dump_file_path)).expect("Could not create the dump file.");
            file.write_all(&output.stdout).expect("Failed to write to the file.");
            println!("Dump successfully completed and saved at {}", self.dump_file_path);
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("Error: {}.", stderr);
        }
    }

    pub fn make_dump(&self) {
        loop {
            self.exec();
            thread::sleep(Duration::from_secs(self.interval));
        }
    }

}