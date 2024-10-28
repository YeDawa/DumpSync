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

        let dbname = if Flags::parse().database != None {
            Flags::parse().database.unwrap()
        } else {
            Env::get_var("DB_NAME")
        };

        let interval = if Flags::parse().interval != None {
            Flags::parse().interval.unwrap()
        } else {
            Env::get_var_u64("DS_DUMP_INTERVAL")
        };

        let backup_path = if Flags::parse().folder != None {
            Flags::parse().folder.unwrap()
        } else {
            Env::get_var("DS_DUMP_PATH")
        };

        UI::label("Press CTRL+C to exit the tool", "normal");
        
        UI::section_header("Dumping the database", "info");

        Dump::new(
            &Env::get_var("DB_USER"), 
            &Env::get_var("DB_PASSWORD"), 
            &dbname, 
            &backup_path, 
            interval
        ).make_dump();
    }

}
