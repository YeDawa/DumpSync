extern crate open;

use std::{
    path::PathBuf,
    io::Error as IoError,

    fs::{
        File, 
        write, 
        read_to_string
    },
};

use crate::constants::folders::Folders;

pub struct WriteEnv {
    entries: Vec<(String, String)>,
}

impl WriteEnv {
    
    pub fn new() -> Self {
        Self {
            entries: Vec::new()
        }
    }

    pub fn add(&mut self, key: String, val: String) {
        self.entries.push((key, val));
    }

    pub fn save(&self) -> Result<(), IoError> {
        let app_folder = &*Folders::APP_FOLDER;
        let env_path: PathBuf = app_folder.join(".env");

        if !env_path.exists() {
            File::create(&env_path)?;
        }

        let mut contents = read_to_string(&env_path).unwrap_or_default();
        let mut lines: Vec<String> = contents
            .lines()
            .map(|line| line.to_string())
            .collect();

        for (key, value) in &self.entries {
            let mut found = false;

            for line in &mut lines {
                if line.starts_with(&format!("{}=", key)) {
                    *line = format!("{}=\"{}\"", key, value);
                    found = true;
                    break;
                }
            }

            if !found {
                lines.push(format!("{}=\"{}\"", key, value));
            }
        }

        contents = lines.join("\n") + "\n";
        write(&env_path, contents)?;
        Ok(())
    }

}
