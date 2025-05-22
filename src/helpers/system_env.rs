extern crate open;

use std::{
    env,
    sync::Once,
};

use crate::constants::folders::Folders;

pub struct Env;

impl Env {
    
    pub fn env_var(&self, key: &str) -> String {
        let load_env: Once = Once::new();

        load_env.call_once(|| {
            dotenvy::from_path(
                &Folders::APP_FOLDER.join(".env")
            ).ok();
        });
    
        env::var(key).expect(&format!("{} not set", key))
    }
  
}
