use reqwest;

use std::{
    env,
    error::Error, 
};

use tokio::{
    fs::File,
    io::AsyncWriteExt,
};

use crate::{
    ui::success_alerts::SuccessAlerts,
    helpers::configs_files::DownloadConfigsFiles,

    constants::{
        urls::*,
        global::Global,
    },
};

pub struct DumpSyncInit;

impl DumpSyncInit {

    pub async fn initialize(&self) -> Result<(), Box<dyn Error>> {
        let response = reqwest::get(Urls::as_str(UrlsNames::AppConfigs)).await?;
        let content = response.bytes().await?;
        
        let mut file = File::create(Global::app_config()).await?;
        file.write_all(&content).await?;
        
        SuccessAlerts::settings();
        DownloadConfigsFiles.env_file(false, false).await?;
        Ok(())
    }

    pub fn load_db_config(&self) -> (String, String, String, String, u16) {
        let dbname = env::var("DB_NAME").or_else(|_| env::var("DS_DB_NAME")).unwrap_or_default();
        let host = env::var("DB_HOST").or_else(|_| env::var("DS_DB_HOST")).unwrap_or_default();
        let user = env::var("DB_USER").or_else(|_| env::var("DS_DB_USER")).unwrap_or_default();
        let password = env::var("DB_PASSWORD").or_else(|_| env::var("DS_DB_PASSWORD")).unwrap_or_default();
        
        let port = env::var("DB_PORT")
            .or_else(|_| env::var("DS_DB_PORT"))
            .unwrap_or_default()
            .parse::<u16>()
            .expect("Invalid port");
    
        (dbname, host, user, password, port)
    }

}
