extern crate reqwest;

use std::{
    error::Error,

    io::{
        ErrorKind, 
        Error as ErrorIo, 
    },
};

use crate::{
    cloud::api::API,
    core::import::Import,
    constants::protocols::Protocols,
    ui::errors_alerts::ErrorsAlerts,
};

pub struct Pull {
    host: String,
    port: u16,
    user: String,
    password: String,
    dbname: String,
    backup: String,
}

impl Pull {

    pub fn new(
        host: &str,
        port: u16,
        user: &str,
        password: &str,
        dbname: &str,
        backup: &str,
    ) -> Self {
        Self {
            host: host.to_string(),
            port,
            user: user.to_string(),
            password: password.to_string(),
            dbname: dbname.to_string(),
            backup: backup.to_string(),
        }
    }

    async fn pull_url(&self, url: &str) -> Result<String, Box<dyn Error>> {
        let response = reqwest::get(url).await?;
        
        if !response.status().is_success() {
            let status_code = response.status();
            let error_message = format!("Failed to download SQL data: HTTP {}", status_code);
            ErrorsAlerts::dump(&error_message);
            
            return Err(Box::new(ErrorIo::new(
                ErrorKind::Other,
                error_message,
            )));
        }
        
        let sql_content = response.text().await?;

        Import::new(
            &self.host,
            self.port,
            &self.user,
            &self.password,
            &self.dbname,
            None,
            None,
            None,
            Some(&sql_content),
        ).dump_directly().await?;

        Ok(sql_content)
    }

    async fn pull_dumpsync(&self, backup: &str) -> Result<(), Box<dyn Error>> {
        match API::new(
            None,
            Some(backup),
            None,
            None,
            None,
            None,
        ).get().await {
            Ok(sql_content) => {
                Import::new(
                    &self.host,
                    self.port,
                    &self.user,
                    &self.password,
                    &self.dbname,
                    None,
                    None,
                    None,
                    Some(&sql_content),
                ).dump_directly().await?;
            }

            Err(_) => {
                ErrorsAlerts::dump("Failed to pull dump from Cloud.");
            }   
        }

        Ok(())
    }

    pub async fn pull(&self) -> Result<(), Box<dyn Error>> {
        match self.backup.as_str() {
            s if s.starts_with(Protocols::Http.as_str()) || s.starts_with(Protocols::Https.as_str()) => {
                self.pull_url(s).await?;
            }
            _ => {
                self.pull_dumpsync(&self.backup).await?;
            }
        };

        Ok(())
    }

}
