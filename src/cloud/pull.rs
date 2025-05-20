extern crate reqwest;

use std::{
    error::Error,

    io::{
        ErrorKind, 
        Error as ErrorIo, 
    },
};

use crate::{
    core::import::Import,
    ui::errors_alerts::ErrorsAlerts,
};

pub struct Pull {
    host: String,
    port: u16,
    user: String,
    password: String,
    dbname: String,

    path: String, 
    backup: String,
}

impl Pull {

    pub fn new(
        host: &str,
        port: u16,
        user: &str,
        password: &str,
        dbname: &str,
        
        path: &str,
        backup: &str,
    ) -> Self {
        Self {
            host: host.to_string(),
            port,
            user: user.to_string(),
            password: password.to_string(),
            dbname: dbname.to_string(),

            path: path.to_string(),
            backup: backup.to_string(),
        }
    }

    pub async fn import_sql_from_url(&self) -> Result<String, Box<dyn Error>> {
        let file_name = &self.backup.split('/').last().unwrap_or("dump.sql");
        let response = reqwest::get(&self.backup).await?;
        
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
            &self.path,
            &file_name,
        ).dump_directly(&sql_content.to_string()).await?;
        Ok(sql_content)
    }

}
