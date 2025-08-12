use std::cmp;

use crate::{
    ui::ui_base::UI,

    helpers::{
        env::Env,
        history::History,
    },
};

pub struct HistoryLogs;

impl HistoryLogs {

    pub fn new() -> Self {
        HistoryLogs
    }

    pub fn backups(&self, filter: Option<String>) {
        Env::new();
        UI::header();

        UI::section_header("Backup History", "info");
        let items = History::new().list_backups_with_filters(
            Some(filter.as_deref().unwrap_or(""))
        );
        
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
                            id, slug, db, filename, host, date, size, encrypt, compress,
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
