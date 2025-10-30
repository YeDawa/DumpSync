use serde::Deserialize;
use serde_json::from_str;

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
    helpers::converter::Converter,

    constants::{
        global::Global,

        api::{
            api_token::APIToken,
            api_endpoints::APIEndpoints,
        },
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
#[derive(Debug, Deserialize, Default)]
pub struct ResponseUpload {
    pub success: bool,
    pub message: String,
    pub url: String,
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

    pub async fn get(&self) -> Result<String, Box<dyn Error>> {
        let endpoint = format!("{}/raw", self.backup.as_deref().unwrap_or(""));
        let api_url = APIEndpoints.backups(&endpoint);

        let client = reqwest::Client::new();
        let request = client
            .get(api_url)
            .header(AUTHORIZATION, APIToken.value());

        let response = request
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        Ok(response)
    }

    pub async fn post(&self) -> Result<ResponseUpload, Box<dyn Error>> {
        let api_url = APIEndpoints.backups("create");
        let db_name = self.dbname.clone().unwrap_or_default();
        let path = self.path.as_ref().ok_or("No path provided")?;

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

        let interval_str = self.interval.map_or("0".to_string(), |v| v.to_string());
        let encrypted_str = self.encrypted.map_or("false".to_string(), |v| v.to_string());

        let form = Form::new()
            .text("db_name", db_name)
            .text("path", path.clone())
            .text("settings", settings_json)
            .text("interval", interval_str)
            .text("encrypted", encrypted_str)
            .text("privacy", "private".to_string())
            .part("file", file_part);

        let response = client
            .post(api_url)
            .header(AUTHORIZATION, APIToken.value())
            .multipart(form)
            .send()
            .await?;

        let response_raw = response.text().await?;

        let parsed = match from_str::<ResponseUpload>(&response_raw) {
            Ok(json) => json,
            Err(_) => {
                ResponseUpload {
                    message: response_raw.clone(),
                    ..Default::default()
                }
            }
        };

        Ok(parsed)
    }

}
