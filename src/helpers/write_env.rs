extern crate open;
use rpassword::prompt_password;

use std::{
    path::PathBuf,

    fs::{
        write,
        read_to_string,
    },

    io::{
        self,
        Write,
        Error as IoError,
    },
};

use crate::{
    constants::folders::Folders,
    ui::success_alerts::SuccessAlerts,
};

pub struct WriteEnv {
    key: String,
    value: String,
}

impl WriteEnv {

    pub fn new(key: Option<String>, val: Option<String>) -> Self {
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

        Self { key, value }
    }

    pub fn edit_env_var(&self) -> Result<(), IoError> {
        let app_folder = &*Folders::APP_FOLDER;
        let env_path: PathBuf = app_folder.join(".env");

        let mut contents = read_to_string(&env_path)?;
        let mut lines: Vec<String> = contents.lines().map(|line| line.to_string()).collect();

        for line in &mut lines {
            if line.starts_with(&self.key) {
                *line = format!("{}=\"{}\"", self.key, self.value);
                break;
            }
        }

        contents = lines.join("\n");
        write(env_path, contents)?;

        SuccessAlerts::edit_env(&self.key);
        Ok(())
    }
  
}
