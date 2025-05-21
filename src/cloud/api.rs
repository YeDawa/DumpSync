use serde::Deserialize;

use std::{
    env,
    error::Error,

    io::{
        ErrorKind,
        Error as ErrorIo, 
    },
};

use reqwest::header::{
    CONTENT_TYPE,
    AUTHORIZATION, 
};

use crate::{
    helpers::env::Env,
    constants::urls::Urls,
}; 

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Response {
    pub url: String,
    pub encrypted: bool,
    pub private: bool,
    pub size: u64,
    pub db_name: String,

    #[serde(rename = "createdAt")]
    pub created_at: String,
}

pub struct API {
    backup: String,
}

impl API {

    pub fn new(backup: &str) -> Self {
        Self {
            backup: backup.to_string(),
        }
    }

    pub async fn get(&self) -> Result<Response, Box<dyn Error>> {
        let mut api_url = String::from(Urls::DUMPSYNC_API);
        api_url.push_str("backups/get/");
        api_url.push_str(&self.backup);

        let api_token = env::var("DS_API_KEY").unwrap_or_else(|_| {
            Env::get_var("DS_API_KEY")
        });

        if api_token.is_empty() {
            return Err(Box::new(ErrorIo::new(
                ErrorKind::Other,
                "API token is not set",
            )));
        }

        let client = reqwest::Client::new();
        let response = client
            .get(api_url)
            .header(AUTHORIZATION, format!("Bearer {}", api_token))
            .header(CONTENT_TYPE, "application/json")
            .send()
            .await?
            .error_for_status()?
            .json::<Response>()
            .await?;

        Ok(response)
    }

}
