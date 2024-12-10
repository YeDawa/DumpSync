use clap::Parser;
use std::error::Error;

use reqwest;

use tokio::{
    fs::File,
    io::AsyncWriteExt,
};

use crate::{
    args_cli::*,

    ui::ui_base::UI,
    helpers::env::Env,
    engine::dump::Dump,
    consts::global::Global,
    ui::success_alerts::SuccessAlerts,
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
            std::env::var("DB_NAME").or_else(|_| std::env::var("DS_DB_NAME")).unwrap_or_default()
        });

        let backup_path = options.file.unwrap_or_else(|| Env::get_var("DS_DUMP_PATH"));

        let host = std::env::var("DB_HOST").or_else(|_| std::env::var("DS_DB_HOST")).unwrap_or_default();
        let user = std::env::var("DB_USER").or_else(|_| std::env::var("DS_DB_USER")).unwrap_or_default();
        let password = std::env::var("DB_PASSWORD").or_else(|_| std::env::var("DS_DB_PASSWORD")).unwrap_or_default();

        let port = std::env::var("DB_PORT")
            .or_else(|_| std::env::var("DS_DB_PORT"))
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
            "", 
            None,
            None,
            None,
        ).import();
    }

    fn export(&self, options: ExportOptions) {
        Env::new();
        UI::header();

        let dbname = options.database.unwrap_or_else(|| {
            std::env::var("DB_NAME").or_else(|_| std::env::var("DS_DB_NAME")).unwrap_or_default()
        });

        let interval = options.interval.unwrap_or_else(|| {
            Env::get_var_u64("DS_DUMP_INTERVAL")
        });

        let backup_path = options.folder.unwrap_or_else(|| Env::get_var("DS_DUMP_PATH"));

        let host = std::env::var("DB_HOST").or_else(|_| std::env::var("DS_DB_HOST")).unwrap_or_default();
        let user = std::env::var("DB_USER").or_else(|_| std::env::var("DS_DB_USER")).unwrap_or_default();
        let password = std::env::var("DB_PASSWORD").or_else(|_| std::env::var("DS_DB_PASSWORD")).unwrap_or_default();

        let port = std::env::var("DB_PORT")
            .or_else(|_| std::env::var("DS_DB_PORT"))
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
            "", 
            None,
            None,
            None,
        ).export();
    }

    async fn scan_xss(&self, options: ScanOptions) -> Result<(), Box<dyn Error>> {
        Env::new();
        UI::header();

        let table = options.table;
        let payload = options.payload.unwrap_or_else(|| {
            Global::XSS_DETECT_REGEX.to_string()
        });

        let offset = options.offset.unwrap_or(0);
        let limit = options.limit.unwrap_or(99999999999);

        let dbname = options.database.unwrap_or_else(|| {
            std::env::var("DB_NAME").or_else(|_| std::env::var("DS_DB_NAME")).unwrap_or_default()
        });

        let host = std::env::var("DB_HOST").or_else(|_| std::env::var("DS_DB_HOST")).unwrap_or_default();
        let user = std::env::var("DB_USER").or_else(|_| std::env::var("DS_DB_USER")).unwrap_or_default();
        let password = std::env::var("DB_PASSWORD").or_else(|_| std::env::var("DS_DB_PASSWORD")).unwrap_or_default();

        let port = std::env::var("DB_PORT")
            .or_else(|_| std::env::var("DS_DB_PORT"))
            .unwrap_or_default()
            .parse::<u64>()
            .expect("Invalid port");

        let header = format!("Scaning table: '{}'", table);
        UI::section_header(&header, "info");

        Dump::new(
            &host, 
            port, 
            &user, 
            &password, 
            &dbname, 
            "", 
            None, 
            "", 
            &table, 
            Some(&payload),
            Some(offset),
            Some(limit),
        ).scan_xss().await;
        Ok(())
    }

    fn transfer(&self, options: TransferOptions) {
        Env::new();
        UI::header();

        let backup_path = options.file.unwrap();
        let dbname = std::env::var("DS_TRANSFER_DB_NAME").or_else(|_| std::env::var("DS_TRANSFER_DB_NAME")).unwrap_or_default();

        let host = std::env::var("DS_TRANSFER_HOST").or_else(|_| std::env::var("DS_TRANSFER_HOST")).unwrap_or_default();
        let user = std::env::var("DS_TRANSFER_USER").or_else(|_| std::env::var("DS_TRANSFER_USER")).unwrap_or_default();
        let password = std::env::var("DS_TRANSFER_PASSWORD").or_else(|_| std::env::var("DS_TRANSFER_PASSWORD")).unwrap_or_default();

        let port = std::env::var("DS_TRANSFER_PORT")
            .or_else(|_| std::env::var("DS_TRANSFER_DB_PORT"))
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
            "", 
            None,
            None,
            None,
        ).transfer();
    }

    pub async fn init(&self) -> Result<(), Box<dyn Error>> {
        let cli = Cli::parse();

        match cli.command {
            Commands::Export(options) => {
                self.export(options);
            },

            Commands::Import(options) => {
                self.import(options);
            },

            Commands::Init => {
                self.initialize().await?;
            },

            Commands::Transfer(options) => {
                self.transfer(options);
            },

            Commands::Scan(options) => {
                self.scan_xss(options).await?;            
            },
        }

        Ok(())
    }

}
