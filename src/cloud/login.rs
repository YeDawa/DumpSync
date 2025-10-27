use open;
use rpassword::prompt_password;

use crate::{
    helpers::write_env::WriteEnv,
    
    ui::{
        ui_base::UI,
        errors_alerts::ErrorsAlerts,
        success_alerts::SuccessAlerts,
    },

    constants::api::{
        api_names::ApiNames,
        api_endpoints::APIEndpoints,
    },
};

pub struct Login;

impl Login {

    pub fn new() -> Self {
        Self
    }

    pub fn print(&self) {
        let url = APIEndpoints.login();
        let message = format!("Opening URL {} to retrieve the API Key", url);
        UI::label(&message, "normal");

        if open::that(url).is_err() {
            ErrorsAlerts::open_link();
        }
    }

    pub fn save_var(&self) {
        let api_key = prompt_password("Enter the api key [input is hiding]: ")
            .expect("Error reading the password");
        
        WriteEnv::new(
            Some(ApiNames::Env.as_str().to_owned()),
            Some(api_key)
        ).edit_env_var().expect("Error writing the env file");

        SuccessAlerts::api_key();
    }

}