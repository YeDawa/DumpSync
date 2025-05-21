use dotenvy::dotenv;

use std::env;

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

    pub fn get_var_u64(var: &str) -> u64 {
        env::var(var).expect(
            &format!("{} is not defined in the .env", var)
        ).parse().expect(
            &format!("{} is not a valid number", var)
        )
    }

}