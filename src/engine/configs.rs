use serde_yaml::Value;

use std::{
    fs::File, 
    io::BufReader
};

use crate::consts::global::Global;

pub struct Configs;

impl Configs {
    
    fn default_config() -> Value {
        serde_yaml::from_str::<Value>(
            r#"
            exports:
                dump_data: true
                drop_table_if_exists: true
                database_if_not_exists: true
            "#
        ).unwrap()
    }

    pub fn load() -> Value {
        let file = File::open(Global::app_config());

        match file {
            Ok(f) => {
                let reader = BufReader::new(f);
                serde_yaml::from_reader(reader).unwrap_or_else(|_| {
                    Self::default_config()
                })
            }
            Err(_) => {
                Self::default_config()
            }
        }
    }
    
}
