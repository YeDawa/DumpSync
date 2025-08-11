use std::env;
use std::cmp;

use crate::{
    args_cli::*,
    init::DumpSyncInit,

    core::{
        dump::Dump,
        truncate::Truncate,
    },

    helpers::{
        env::Env,
        history::History,
    },

    ui::ui_base::UI,
};

pub struct DumpSyncDumper;

impl DumpSyncDumper {

    pub fn import(&self, options: ImportOptions) {
        Env::new();
        UI::header();

        let backup_path = options.file.unwrap_or_else(|| Env::get_var("DS_DUMP_PATH"));
        let (dbname, host, user, password, port) = DumpSyncInit.load_db_config();

        UI::section_header("Importing dump to server", "info");
        Dump::new(
            &host, port, &user, &password, &dbname, &backup_path, None, &backup_path, None, None, None, None,
        ).import();
    }

    pub fn export(&self, options: ExportOptions) {
        Env::new();
        UI::header();

        let interval = options.interval.unwrap_or_else(|| {
            Env::get_var_u64("DS_DUMP_INTERVAL")
        });

        let pdf = options.pdf;
        let once = options.once;
        let encrypt = options.encrypt;
        let retain = options.retain;
        let backup_path = options.folder.unwrap_or_else(|| Env::get_var("DS_DUMP_PATH"));
        let (dbname, host, user, password, port) = DumpSyncInit.load_db_config();

        UI::label("Press CTRL+C to exit the tool", "normal");
        UI::section_header("Dumping the database", "info");

        Dump::new(
            &host, port, &user, &password, &dbname, &backup_path, Some(interval), &backup_path, Some(encrypt), Some(once), retain, Some(pdf),
        ).export();
    }

    pub fn truncate(&self, options: TruncateOptions) {
        Env::new();
        UI::header();

        let table = options.table;
        let encrypt = options.encrypt;
        let backup_path = options.folder.unwrap_or_else(|| Env::get_var("DS_DUMP_PATH"));
        let (dbname, host, user, password, port) = DumpSyncInit.load_db_config();

        UI::label("Press CTRL+C to exit the tool", "normal");
        UI::section_header("Truncate table", "info");

        let _ = Truncate::new(
            &host, port, &user, &password, &dbname, &backup_path, &table, Some(encrypt),
        ).table();
    }

    pub fn transfer(&self, options: TransferOptions) {
        Env::new();
        UI::header();

        let backup_path = options.file.unwrap();
        let (_, host, user, password, port) = DumpSyncInit.load_db_config();
        let dbname = env::var("DS_TRANSFER_DB_NAME").or_else(|_| env::var("DS_TRANSFER_DB_NAME")).unwrap_or_default();

        UI::section_header("Importing dump to server", "info");

        Dump::new(
            &host, port, &user, &password, &dbname, &backup_path, None, &backup_path, None, None, None, None,
        ).transfer();
    }

    pub fn history(&self) {
        Env::new();
        UI::header();

        UI::section_header("Backup History", "info");
        let items = History::new().list_backups_with_filters(None);
        
        match items {
            Ok(backups) => {
                if backups.is_empty() {
                    UI::label("No backups found", "warning");
                } else {
                    let mut max_id = 2;
                    let mut max_slug = 4;
                    let mut max_db = 8;
                    let mut max_filename = 8;
                    let mut max_host = 4;
                    let mut max_date = 10;
                    let mut max_size = 4;
                    let mut max_encrypt = 7;
                    let mut max_compress = 8;

                    for (id, slug, db, filename, host, created_at, size, encrypt, compress) in &backups {
                        max_id = cmp::max(max_id, id.to_string().len());
                        max_slug = cmp::max(max_slug, slug.len());
                        max_db = cmp::max(max_db, db.len());
                        max_filename = cmp::max(max_filename, filename.len());
                        max_host = cmp::max(max_host, host.len());

                        let replaced = created_at.replace("T", " ");
                        let date = replaced.split('.').next().unwrap_or(&created_at);

                        max_date = cmp::max(max_date, date.len());
                        max_size = cmp::max(max_size, size.to_string().len());
                        max_encrypt = cmp::max(max_encrypt, encrypt.to_string().len());
                        max_compress = cmp::max(max_compress, compress.to_string().len());
                    }

                    let print_row = |id: &str, slug: &str, db: &str, filename: &str, host: &str, date: &str, size: &str, encrypt: &str, compress: &str| {
                        println!(
                            "| {:<idw$} | {:<slugw$} | {:<dbw$} | {:<fnw$} | {:<hostw$} | {:<datew$} | {:>sizew$} | {:<encw$} | {:<compw$} |",
                            id,
                            slug,
                            db,
                            filename,
                            host,
                            date,
                            size,
                            encrypt,
                            compress,
                            idw = max_id,
                            slugw = max_slug,
                            dbw = max_db,
                            fnw = max_filename,
                            hostw = max_host,
                            datew = max_date,
                            sizew = max_size,
                            encw = max_encrypt,
                            compw = max_compress
                        );
                    };

                    let sep = format!(
                        "+-{:-<idw$}-+-{:-<slugw$}-+-{:-<dbw$}-+-{:-<fnw$}-+-{:-<hostw$}-+-{:-<datew$}-+-{:-<sizew$}-+-{:-<encw$}-+-{:-<compw$}-+",
                        "", "", "", "", "", "", "", "", "",
                        idw = max_id,
                        slugw = max_slug,
                        dbw = max_db,
                        fnw = max_filename,
                        hostw = max_host,
                        datew = max_date,
                        sizew = max_size,
                        encw = max_encrypt,
                        compw = max_compress
                    );

                    println!("{}", sep);
                    print_row(
                        "ID", "Slug", "Database", "Filename", "Host", "Created At", "Size", "Encrypt", "Compress"
                    );

                    println!("{}", sep);
                    for (id, slug, db, filename, host, created_at, size, encrypt, compress) in backups {
                        let replaced = created_at.replace("T", " ");
                        let date = replaced.split('.').next().unwrap_or(&created_at);
                        print_row(
                            &id.to_string(),
                            &slug,
                            &db,
                            &filename,
                            &host,
                            date,
                            &size.to_string(),
                            if encrypt { "true" } else { "false" },
                            if compress { "true" } else { "false" },
                        );
                    }
                    println!("{}", sep);
                }
            },
            Err(e) => UI::label(&format!("Error fetching history: {}", e), "error"),
        }
    }

}
