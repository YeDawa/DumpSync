use chrono::Local;

use rand::{
    Rng,
    distributions::Alphanumeric,
};

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
            dump_file_path: backup_path.to_string()
        }
    }

    fn random_string(&self, size: usize) -> String {
        let mut rng = rand::thread_rng();
        
        (0..size)
            .map(|_| rng.sample(Alphanumeric) as char)
            .collect()
    }

    fn exec(&self) {
        let unique_id = &self.random_string(6);

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
            println!("Dump successfully completed and saved at {}", dump_file_path);
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
