use clap::Parser;

use crate::{
    args_cli::Flags,
    ui::ui_base::UI,

    engine::{
        env::Env,
        dump::Dump,
    },
};

pub struct DumpSync;

impl DumpSync {

    pub fn init() {
        Env::new();
        UI::header();

        let dbname = Flags::parse().database.unwrap_or_else(|| {
            std::env::var("DB_NAME").or_else(|_| std::env::var("DS_DB_NAME")).unwrap_or_default()
        });

        let interval = if Flags::parse().interval != None {
            Flags::parse().interval.unwrap()
        } else {
            Env::get_var_u64("DS_DUMP_INTERVAL")
        };

        let backup_path = Flags::parse().folder.unwrap_or_else(|| Env::get_var("DS_DUMP_PATH"));

        let host = std::env::var("DB_HOST").or_else(|_| std::env::var("DS_DB_HOST")).unwrap_or_default();
        let user = std::env::var("DB_USER").or_else(|_| std::env::var("DS_DB_USER")).unwrap_or_default();
        let password = std::env::var("DB_PASSWORD").or_else(|_| std::env::var("DS_DB_PASSWORD")).unwrap_or_default();
        let port = std::env::var("DB_PORT").or_else(|_| std::env::var("DS_DB_PORT")).unwrap_or_default().parse::<u64>().expect("");

        UI::label("Press CTRL+C to exit the tool", "normal");
        
        UI::section_header("Dumping the database", "info");

        Dump::new(&host, port, &user, &password, &dbname, &backup_path, interval).make_dump();
    }

}
