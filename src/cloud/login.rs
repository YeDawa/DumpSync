use open;
use rpassword::prompt_password;

use crate::{
    helpers::write_env::WriteEnv,

    ui::{
        errors_alerts::ErrorsAlerts,
        success_alerts::SuccessAlerts,
    },

    constants::{
        urls::*,
        global::Global,
    },
};

pub struct Login;

impl Login {

    pub fn new() -> Self {
        Self
    }

    pub fn print(&self) {
        let url = Urls::as_str(UrlsNames::DumpsyncApiKey);
        println!("Open URL {} for get the API Key", url);

        if open::that(url).is_err() {
            ErrorsAlerts::open_link();
        }
    }

    pub fn save_var(&self) {
        let api_key = prompt_password("Enter the api key [input is hiding]: ")
            .expect("Error reading the password");
        
        WriteEnv::new(
            Some(Global::DS_API_ENV.to_string()),
            Some(api_key)
        ).edit_env_var()
            .expect("Error writing the env file");

        SuccessAlerts::api_key();
    }

}