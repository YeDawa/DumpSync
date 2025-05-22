extern crate chrono;

use std::path::PathBuf;
use once_cell::sync::Lazy;
use dirs_next::config_dir;

use crate::constants::global::Global;

pub struct Folders;

impl Folders {

    pub const APP_FOLDER: Lazy<PathBuf> = Lazy::new(|| {
        let mut path = config_dir().expect("No config directory");
        path.push(Global::APP_NAME);
        path
    });
    
}
