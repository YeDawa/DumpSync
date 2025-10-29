use rpassword::prompt_password;

use crate::{
    utils::open::Open,
    helpers::write_env::WriteEnv,

    ui::{
        ui_base::UI,
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
        Open::new(url).link();
    }

    pub fn save_var(&self) {
        let api_key = prompt_password("Enter the api key [input is hiding]: ")
            .expect("Error reading the password");
        
        let mut writer = WriteEnv::new();
        writer.add(ApiNames::Env.as_str().to_owned(), api_key);
        
        writer.save().expect("Error writing the env file");
        SuccessAlerts::api_key();
    }

}