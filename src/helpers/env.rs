use dotenvy::dotenv;

use std::{
    env,
    process::Command,
};

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

    pub fn get_system_var(var: &str) -> String {
        let output = if cfg!(target_os = "windows") {
            let cmd = format!("echo $env:{}", var);
            Command::new("powershell")
                .args(["-Command", &cmd])
                .output()
        } else {
            let cmd = format!("echo ${}", var);
            Command::new("sh")
                .arg("-c")
                .arg(&cmd)
                .output()
        };

        match output {
            Ok(output) => String::from_utf8_lossy(&output.stdout).trim().to_string(),
            Err(e) => panic!("Failed to get environment variable: {}", e),
        }
    }

    pub fn get_var_u64(var: &str) -> u64 {
        env::var(var).expect(
            &format!("{} is not defined in the .env", var)
        ).parse().expect(
            &format!("{} is not a valid number", var)
        )
    }

}