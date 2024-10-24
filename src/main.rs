pub mod engine;

use crate::engine::{
    env::Env,
    dump::Dump,
};

fn main() {
    Env::new();
    
    Dump::new(
        &Env::get_var("DB_USER"), 
        &Env::get_var("DB_PASSWORD"), 
        &Env::get_var("DB_NAME"), 
        &Env::get_var("BACKUP_PATH"), 
        Env::get_var_u64("DUMP_INTERVAL")
    ).make_dump();
}
