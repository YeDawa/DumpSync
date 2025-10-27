use serde::Deserialize;

use std::{
    io::Read,
    fs::File,
    error::Error,
};

use reqwest::{
    Body,
    Client,
    header::AUTHORIZATION,

    multipart::{
        Form,
        Part,
    },
};

use crate::{
    helpers::env::Env,
    utils::converter::Converter,

    constants::{
        urls::*,
        api_init::*,
        global::Global,
    },
};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Response {
    pub url: String,
    pub encrypted: bool,
    pub private: bool,
    pub size: u64,
    pub db_name: String,
    pub created_at: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ResponseUpload {
    pub success: bool,
    pub db_name: String,
    pub unique_id: String,
    pub message: String,
}

pub struct API {
    path: Option<String>,
    encrypted: Option<bool>,
    backup: Option<String>,
    dbname: Option<String>,
    interval: Option<u64>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct APIUpload {
    url: String,
    message: String,
}

impl API {

    pub fn new(
        path: Option<&str>,
        backup: Option<&str>,
        dbname: Option<&str>,
        encrypted: Option<bool>,
        interval: Option<u64>,
    ) -> Self {
        Self {
            path: path.map(|s| s.to_string()),
            dbname: dbname.map(|s| s.to_string()),
            backup: backup.map(|s| s.to_string()),

            interval,
            encrypted,
        }
    }

    fn token_value(&self) -> String {
        let api_token = Env.system(APIInit::as_str(ApiNames::Env));
        format!("Bearer {}", api_token)
    }

    pub async fn get(&self) -> Result<String, Box<dyn Error>> {
        let mut api_url = String::from(Urls::as_str(UrlsNames::DumpsyncApi));
        api_url.push_str("backups/");
        api_url.push_str(self.backup.as_deref().unwrap_or(""));
        api_url.push_str("/raw");

        let client = reqwest::Client::new();
        let request = client
            .get(api_url)
            .header(AUTHORIZATION, self.token_value());

        let response = request
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        Ok(response)
    }

    pub async fn upload(&self) -> Result<ResponseUpload, Box<dyn Error>> {
        let mut api_url = String::from(Urls::as_str(UrlsNames::DumpsyncApi));
        api_url.push_str("backups/create");

        let path = self.path.as_ref().ok_or("No path provided")?;
        let db_name = self.dbname.clone().unwrap_or_default();

        let client = Client::new();
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let settings_json = Converter::new(
            Global::app_config()
        ).yaml_to_json()?;

        let file_name = std::path::Path::new(path)
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or("Invalid file name")?;

        let file_part = Part::stream(Body::from(buffer))
            .file_name(file_name.to_string());

        let form = Form::new()
            .text("db_name", db_name)
            .text("settings", settings_json)
            .text("interval", self.interval.map_or("0".to_string(), |v| v.to_string()))
            .text("encrypted", self.encrypted.map_or("false".to_string(), |v| v.to_string()))
            .part("file", file_part);

        let response = client
            .post(api_url)
            .header(AUTHORIZATION, self.token_value())
            .multipart(form)
            .send()
            .await?
            .error_for_status()?;

        let response_raw = response.text().await?;
        let parsed: ResponseUpload = serde_json::from_str(&response_raw)?;
        Ok(parsed)
    }

}
