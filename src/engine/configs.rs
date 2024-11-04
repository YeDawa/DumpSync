use reqwest;
use serde_yaml::Value;

use std::{
    fs::File, 
    error::Error,
    io::BufReader,
};

use crate::consts::global::Global;

pub struct Configs;

impl Configs {
    
    fn default_config(&self) -> Result<Value, Box<dyn Error>> {
        let response = reqwest::blocking::get(Global::APP_CONFIGS)?.text()?;
        let config: Value = serde_yaml::from_str(&response)?;
        Ok(config)
    }

    pub fn load(&self) -> Value {
        let file = File::open(Global::app_config());

        match file {
            Ok(f) => {
                let reader = BufReader::new(f);

                serde_yaml::from_reader(reader).unwrap_or_else(|_| {
                    self.default_config().expect("Error loading default config")
                })
            }

            Err(_) => {
                self.default_config().expect("Error loading default config")
            }
        }
    }

}
