use dotenvy::dotenv;

use std::{
    env,
    sync::Once,
};

use crate::constants::folders::Folders;

pub struct Env;

impl Env {

    pub fn new() {
        dotenv().ok();
    }

    pub fn get_var(var: &str) -> String {
        env::var(var).expect(
            &format!("{} is not defined in the .env", var)
        )
    }

    pub fn system(&self, key: &str) -> String {
        let load_env: Once = Once::new();

        load_env.call_once(|| {
            dotenvy::from_path(
                &Folders::APP_FOLDER.join(".env")
            ).ok();
        });
    
        env::var(key).expect(&format!("{} not set", key))
    }

    pub fn get_var_u64(var: &str) -> u64 {
        env::var(var).expect(
            &format!("{} is not defined in the .env", var)
        ).parse().expect(
            &format!("{} is not a valid number", var)
        )
    }

}