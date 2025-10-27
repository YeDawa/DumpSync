extern crate open;
use rpassword::prompt_password;

use std::{
    path::PathBuf,

    fs::{
        File, 
        write, 
        read_to_string
    },

    io::{
        self, 
        Write, 
        Error as IoError
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

    pub fn add(&mut self, key: Option<String>, val: Option<String>) {
        let key = key.unwrap_or_else(|| {
            print!("Enter the variable name: ");
            io::stdout().flush().expect("Failed to flush buffer");

            let mut key = String::new();
            io::stdin().read_line(&mut key).expect("Failed to read variable name");
            key.trim().to_string().to_uppercase()
        });

        let value = val.unwrap_or_else(|| {
            prompt_password("Enter the variable value: ").unwrap()
        });

        self.entries.push((key, value));
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
