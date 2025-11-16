use std::env;

use crate::{
    args_cli::*,
    ui::ui_base::UI,
    helpers::env::Env,
    init::DumpSyncInit,

    core::{
        dump::Dump,
        truncate::Truncate,
        dump_data::DumpData,
    },
};

pub struct DumpSyncDumper;

impl DumpSyncDumper {

    pub fn import(&self, options: ImportOptions) {
        Env::new();
        UI::header();

        let ignore_drop_table = options.ignore_drop_table;
        let backup_path = options.file.unwrap_or_else(|| Env::get_var("DS_DUMP_PATH"));
        let (dbname, host, user, password, port) = DumpSyncInit.load_db_config();

        UI::section_header("Importing dump to server", "info");
        Dump::new(
            &host, port, &user, &password, &dbname, &backup_path, None, &backup_path, None, Some(ignore_drop_table), None, None, None,
        ).import();
    }

    pub fn import_json(&self, options: ImportDataOptions) {
        Env::new();
        UI::header();

        let backup_path = options.file;
        let (dbname, host, user, password, port) = DumpSyncInit.load_db_config();

        UI::section_header("Importing JSON dump to server", "info");
        Dump::new(
            &host, port, &user, &password, &dbname, &backup_path, None, &backup_path, None, None, None, None, None,
        ).import_json();
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
            &host, port, &user, &password, &dbname, &backup_path, Some(interval), &backup_path, Some(encrypt), None, Some(once), retain, Some(pdf),
        ).export();
    }
    
    pub fn export_dumpdata(&self, options: DumpDataOptions) {
        Env::new();
        UI::header();

        let table = options.table;
        let dump_file_path = options.folder.unwrap_or_else(|| Env::get_var("DS_DUMP_PATH"));
        let (dbname, host, user, password, port) = DumpSyncInit.load_db_config();
        UI::section_header("Dumping the database", "info");

        let _ = DumpData::new(
            &host, port, &user, &password, &dbname, &dump_file_path, table,
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
            &host, port, &user, &password, &dbname, &backup_path, None, &backup_path, None, None, None, None, None,
        ).transfer();
    }

}
