use reqwest::blocking;
use serde_yaml::Value;

use std::{
    error::Error,
    io::BufReader,

    fs::{
        File, 
        metadata
    },
};

use crate::consts::global::Global;

pub struct Configs;

impl Configs {
    
    fn default_config(&self) -> Result<Value, Box<dyn Error>> {
        let response = blocking::get(Global::APP_CONFIGS)?.text()?;
        let config: Value = serde_yaml::from_str(&response)?;
        Ok(config)
    }

    pub fn load(&self) -> Value {
        let file_path = Global::app_config();
        
        match metadata(&file_path) {
            Ok(_) => {
                let file = File::open(file_path).expect("Failed to open local config file");
                let reader = BufReader::new(file);

                serde_yaml::from_reader(reader).unwrap_or_else(|_| {
                    self.default_config().expect("Error loading default config")
                })
            }

            Err(_) => {
                self.default_config().expect("Error loading default config")
            }
        }
    }

    pub fn generic(&self, section: &str, option: &str) -> Value {
        let configs = self.load();

        configs
            .get(section)
            .and_then(|conn| conn.get(option))
            .cloned()
            .unwrap_or(serde_yaml::Value::Null)
    }

    pub fn boolean(&self, section: &str, option: &str, default: bool) -> bool {
        let configs = self.load();

        configs
            .get(section)
            .and_then(|exports| exports.get(option))
            .and_then(|val| val.as_bool())
            .unwrap_or(default)
    }

}
