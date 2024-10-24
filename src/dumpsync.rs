use clap::Parser;
extern crate colored;

use crate::args_cli::Flags;

use crate::engine::{
    env::Env,
    dump::Dump,
};

use colored::*;
use figlet_rs::FIGfont;

pub struct DumpSync;

impl DumpSync {

    pub fn init() {
        let name = env!("CARGO_PKG_NAME");
        let standard_font = FIGfont::standard().unwrap();
        
        if let Some(title) = standard_font.convert(&name) {
            println!("{}", &title.to_string().bold().cyan());
        }

        Env::new();

        let dbname = if Flags::parse().database != None {
            Flags::parse().database.unwrap()
        } else {
            Env::get_var("DB_NAME")
        };

        let interval = if Flags::parse().interval != None {
            Flags::parse().interval.unwrap()
        } else {
            Env::get_var_u64("DUMP_INTERVAL")
        };

        let backup_path = if Flags::parse().folder != None {
            Flags::parse().folder.unwrap()
        } else {
            Env::get_var("BACKUP_PATH")
        };
        
        Dump::new(
            &Env::get_var("DB_USER"), 
            &Env::get_var("DB_PASSWORD"), 
            &dbname, 
            &backup_path, 
            interval
        ).make_dump();
    }

}
