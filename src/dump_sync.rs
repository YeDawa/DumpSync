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

    ui::ui_base::UI,
    core::dump::Dump,
    helpers::env::Env,
    constants::global::Global,
    ui::success_alerts::SuccessAlerts,

    plugins::{
        scan_xss::ScanXSS,
        pastebin::Pastebin,
    },
};

pub struct DumpSync;

impl DumpSync {

    async fn initialize(&self) -> Result<(), Box<dyn Error>> {
        let response = reqwest::get(Global::APP_CONFIGS).await?;
        let content = response.bytes().await?;
        
        let mut file = File::create(Global::app_config()).await?;
        file.write_all(&content).await?;
        
        SuccessAlerts::settings();
        Ok(())
    }

    fn import(&self, options: ImportOptions) {
        Env::new();
        UI::header();

        let dbname = options.database.unwrap_or_else(|| {
            env::var("DB_NAME").or_else(|_| env::var("DS_DB_NAME")).unwrap_or_default()
        });

        let backup_path = options.file.unwrap_or_else(|| Env::get_var("DS_DUMP_PATH"));

        let host = env::var("DB_HOST").or_else(|_| env::var("DS_DB_HOST")).unwrap_or_default();
        let user = env::var("DB_USER").or_else(|_| env::var("DS_DB_USER")).unwrap_or_default();
        let password = env::var("DB_PASSWORD").or_else(|_| env::var("DS_DB_PASSWORD")).unwrap_or_default();

        let port = env::var("DB_PORT")
            .or_else(|_| env::var("DS_DB_PORT"))
            .unwrap_or_default()
            .parse::<u64>()
            .expect("Invalid port");

        UI::section_header("Importing dump to server", "info");

        Dump::new(
            &host, 
            port, 
            &user, 
            &password, 
            &dbname, 
            &backup_path, 
            None, 
            &backup_path,
        ).import();
    }

    fn export(&self, options: ExportOptions) {
        Env::new();
        UI::header();

        let dbname = options.database.unwrap_or_else(|| {
            env::var("DB_NAME").or_else(|_| env::var("DS_DB_NAME")).unwrap_or_default()
        });

        let interval = options.interval.unwrap_or_else(|| {
            Env::get_var_u64("DS_DUMP_INTERVAL")
        });

        let backup_path = options.folder.unwrap_or_else(|| Env::get_var("DS_DUMP_PATH"));

        let host = env::var("DB_HOST").or_else(|_| env::var("DS_DB_HOST")).unwrap_or_default();
        let user = env::var("DB_USER").or_else(|_| env::var("DS_DB_USER")).unwrap_or_default();
        let password = env::var("DB_PASSWORD").or_else(|_| env::var("DS_DB_PASSWORD")).unwrap_or_default();

        let port = env::var("DB_PORT")
            .or_else(|_| env::var("DS_DB_PORT"))
            .unwrap_or_default()
            .parse::<u64>()
            .expect("Invalid port");

        UI::label("Press CTRL+C to exit the tool", "normal");
        UI::section_header("Dumping the database", "info");

        Dump::new(
            &host, 
            port, 
            &user, 
            &password, 
            &dbname, 
            &backup_path, 
            Some(interval), 
            &backup_path,
        ).export();
    }

    async fn scan_xss(&self, options: ScanOptions) -> Result<(), Box<dyn Error>> {
        Env::new();
        UI::header();

        let table = options.table;
        let payload = options.payload;

        let offset = options.offset.unwrap_or(0);
        let limit = options.limit.unwrap_or(99999999999);
        let file = options.file;

        let dbname = options.database.unwrap_or_else(|| {
            env::var("DB_NAME").or_else(|_| env::var("DS_DB_NAME")).unwrap_or_default()
        });

        let host = env::var("DB_HOST").or_else(|_| env::var("DS_DB_HOST")).unwrap_or_default();
        let user = env::var("DB_USER").or_else(|_| env::var("DS_DB_USER")).unwrap_or_default();
        let password = env::var("DB_PASSWORD").or_else(|_| env::var("DS_DB_PASSWORD")).unwrap_or_default();

        let port = env::var("DB_PORT")
            .or_else(|_| env::var("DS_DB_PORT"))
            .unwrap_or_default()
            .parse::<u64>()
            .expect("Invalid port");

        let header = format!("Scaning table: '{}'", table);
        UI::section_header(&header, "info");

        ScanXSS::new(
            &host,
            port as u16,
            &user,
            &password,
            &dbname,
            &table,
            payload.as_deref(),
            Some(offset),
            Some(limit),
            file.as_deref(),
        ).scan().await.expect("Failed to scan tables for XSS");

        Ok(())
    }

    fn transfer(&self, options: TransferOptions) {
        Env::new();
        UI::header();

        let backup_path = options.file.unwrap();
        let dbname = env::var("DS_TRANSFER_DB_NAME").or_else(|_| env::var("DS_TRANSFER_DB_NAME")).unwrap_or_default();

        let host = env::var("DS_TRANSFER_HOST").or_else(|_| env::var("DS_TRANSFER_HOST")).unwrap_or_default();
        let user = env::var("DS_TRANSFER_USER").or_else(|_| env::var("DS_TRANSFER_USER")).unwrap_or_default();
        let password = env::var("DS_TRANSFER_PASSWORD").or_else(|_| env::var("DS_TRANSFER_PASSWORD")).unwrap_or_default();

        let port = env::var("DS_TRANSFER_PORT")
            .or_else(|_| env::var("DS_TRANSFER_DB_PORT"))
            .unwrap_or_default()
            .parse::<u64>()
            .expect("Invalid port");

        UI::section_header("Importing dump to server", "info");

        Dump::new(
            &host, 
            port, 
            &user, 
            &password, 
            &dbname, 
            &backup_path, 
            None, 
            &backup_path, 
        ).transfer();
    }

    async fn share(&self, options: ShareOptions) -> Result<(), Box<dyn Error>> {
        Env::new();
        UI::header();

        let file = options.file.unwrap();
        let privacy = options.privacy.unwrap_or("unlisted".to_string());
        let api_key = env::var("PASTEBIN_API_KEY").unwrap_or_default();

        let header = format!("Sharing file: '{}'", file);
        UI::section_header(&header, "info");

        Pastebin::new(&file, &api_key, &privacy).share().await?;
        Ok(())
    }

    pub async fn init(&self) -> Result<(), Box<dyn Error>> {
        let cli = Cli::parse();

        match cli.command {
            Commands::Init => {
                self.initialize().await?;
            },

            Commands::Export(options) => {
                self.export(options);
            },

            Commands::Import(options) => {
                self.import(options);
            },

            Commands::Transfer(options) => {
                self.transfer(options);
            },

            Commands::Scan(options) => {
                self.scan_xss(options).await?;            
            },

            Commands::Share(options) => {
                self.share(options).await?;
            },
        }

        Ok(())
    }

}
