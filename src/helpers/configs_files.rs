use reqwest;
use tokio::fs as TkFs;

use std::{
    fs::File,
    io::Write,
    error::Error,

    path::{
        Path,
        PathBuf,
    },
};

use crate::{
    constants::{
        urls::Urls,
        folders::Folders,
    },

    ui::{
        errors_alerts::ErrorsAlerts,
        success_alerts::SuccessAlerts,
    },
};

pub struct DownloadConfigsFiles;

impl DownloadConfigsFiles {

    async fn force_mode(&self, file_path: PathBuf, force_mode: bool, response: reqwest::Response) -> Result<(), Box<dyn Error>> {
        if !force_mode {
            if !Path::new(&file_path).is_file() {
                let mut file = File::create(&file_path)?;
                let content = response.bytes().await?;
    
                file.write_all(&content)?;
            }
        } else {
            let mut file = File::create(&file_path)?;
            let content = response.bytes().await?;
    
            file.write_all(&content)?;
        }

        Ok(())
    }

    pub async fn env_file(&self, print: bool, force_mode: bool) -> Result<(), Box<dyn Error>> {
        let output_directory = &*Folders::APP_FOLDER;
        let uri = format!("{}{}", Urls::DOWNLOAD_FILES_URI, ".env.app");
    
        TkFs::create_dir_all(
            output_directory.clone()
        ).await?;
    
        let response = reqwest::get(uri).await?;
        if response.status().is_success() {
            let file_path = output_directory.join(".env");
            self.force_mode(file_path, force_mode, response).await?;
    
            if print == true {
                SuccessAlerts::env();
            }
        } else {
            let status_code = response.status().to_string();
            ErrorsAlerts::env(&status_code);
        }
    
        Ok(())
    }

}
