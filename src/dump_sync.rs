use reqwest;
use clap::Parser;

use std::{
    env,
    error::Error, 
};

use tokio::{
    fs::File,
    io::AsyncWriteExt,
};

use crate::{
    args_cli::*,
    addons::DumpSyncAddons,
    dumper::DumpSyncDumper,
    ui::success_alerts::SuccessAlerts,

    constants::{
        urls::Urls,
        global::Global,
    },
};

pub struct DumpSync;

impl DumpSync {

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

    async fn initialize(&self) -> Result<(), Box<dyn Error>> {
        let response = reqwest::get(Urls::APP_CONFIGS).await?;
        let content = response.bytes().await?;
        
        let mut file = File::create(Global::app_config()).await?;
        file.write_all(&content).await?;
        
        SuccessAlerts::settings();
        Ok(())
    }

    pub async fn init(&self) -> Result<(), Box<dyn Error>> {
        match Cli::parse().command {
            Commands::Init => self.initialize().await?,
            Commands::Export(options) => DumpSyncDumper.export(options),
            Commands::Import(options) => DumpSyncDumper.import(options),
            Commands::Schema(options) => DumpSyncAddons.schema(options)?,
            Commands::Visual(options) => DumpSyncAddons.visual(options).await,
            Commands::Share(options) => DumpSyncAddons.share(options).await?,
            Commands::Scan(options) => DumpSyncAddons.scan_xss(options).await?,
            Commands::Transfer(options) => DumpSyncDumper.transfer(options),
            Commands::Checksum(options) => DumpSyncAddons.checksum(options),
            Commands::Truncate(options) => DumpSyncDumper.truncate(options),
            Commands::Pull(_options) => todo!(),
        }

        Ok(())
    }

}
