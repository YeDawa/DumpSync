use std::{
    env,
    error::Error, 
};

use crate::{
    args_cli::*,
    dump_sync::DumpSync,

    ui::ui_base::UI,
    helpers::env::Env,

    plugins::{
        schema::Schema,
        diagram::Diagram,
        scan_xss::ScanXSS,
        pastebin::Pastebin,
        checksum::Checksum,
    },
};

pub struct DumpSyncAddons;

impl DumpSyncAddons {

    pub async fn scan_xss(&self, options: ScanOptions) -> Result<(), Box<dyn Error>> {
        Env::new();
        UI::header();

        let table = options.table;
        let payload = options.payload;

        let file = options.file;
        let offset = options.offset.unwrap_or(0);
        let limit = options.limit.unwrap_or(99999999999);
        let (dbname, host, user, password, port) = DumpSync.load_db_config();

        let header = format!("Scanning table(s): '{}'", table);
        UI::section_header(&header, "info");

        ScanXSS::new(
            &host, port, &user, &password, &dbname, &table, payload.as_deref(), Some(offset), Some(limit), file.as_deref(),
        ).scan().await.expect("Failed to scan tables for XSS");

        Ok(())
    }

    pub fn schema(&self, options: SchemaOptions) -> Result<(), Box<dyn Error>> {
        Env::new();
        UI::header();

        let file = options.file;
        let (dbname, host, user, password, port) = DumpSync.load_db_config();

        let header = "Generating schema file".to_string();
        UI::section_header(&header, "info");

        Schema::new(
            &host, port, &user, &password, &dbname, &file,
        ).create()?;

        Ok(())
    }

    pub async fn share(&self, options: ShareOptions) -> Result<(), Box<dyn Error>> {
        Env::new();
        UI::header();

        let file = options.file;
        let privacy = options.privacy.unwrap_or("unlisted".to_string());
        let api_key = env::var("PASTEBIN_API_KEY").unwrap_or_default();

        let header = format!("Sharing file: '{}'", file);
        UI::section_header(&header, "info");

        Pastebin::new(&file, &api_key, &privacy).share().await?;
        Ok(())
    }

    pub fn checksum(&self, options: ChecksumOptions) {
        Env::new();
        UI::header();

        let file = options.file;
        let output = options.output;

        UI::section_header("Generating checksum", "info");

        if let Err(e) = Checksum::new(
            &file,
            output.as_deref(),
        ).generated() {
            eprintln!("Error generating checksum: {}", e);
        }
    }

    pub async fn visual(&self, options: VisualOptions) {
        Env::new();
        UI::header();

        let table = options.table;
        let (dbname, host, user, password, port) = DumpSync.load_db_config();

        let header = format!("Generating ER diagram for table: '{}'", table);
        UI::section_header(&header, "info");

        let _ = Diagram::new(
            &host, port, &user, &password, &dbname, &table,
        ).diagram().await;
    }

}
