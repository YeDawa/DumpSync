use open;
use rpassword::prompt_password;

use std::process::Command;

use crate::{
    constants::urls::Urls,

    ui::{
        errors_alerts::ErrorsAlerts,
        success_alerts::SuccessAlerts,
    },
};

pub struct Login;

impl Login {

    pub fn new() -> Self {
        Self
    }

    pub fn print(&self) {
        let url = Urls::DUMPSYNC_API_KEY;
        println!("Open URL {} for get the API Key", url);

        if open::that(url).is_err() {
            ErrorsAlerts::open_link();
        }
    }

    pub fn save_var(&self) {
        let api_key = prompt_password("Enter the api key [input is hiding]: ")
            .expect("Error reading the password");

        println!("{}", api_key);

        let status = if cfg!(target_os = "windows") {
            let cmd = format!("$env:DS_API_KEY = '{}';", api_key);
            Command::new("powershell")
                .args(["-Command", &cmd])
                .status()
        } else {
            let cmd = format!("export DS_API_KEY='{}';", api_key);
            Command::new("sh")
                .arg("-c")
                .arg(&cmd)
                .status()
        };

        match status {
            Ok(e) => println!("API Key saved: {}", e),
            Err(_) => ErrorsAlerts::api_key(),
        }
    }

}