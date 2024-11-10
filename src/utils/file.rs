use std::{
    fs,
    path::Path
};

pub struct FileUtils;

impl FileUtils {

    pub fn create_path(path: &str) {
        if let Some(parent_dir) = Path::new(path).parent() {
            let _ = fs::create_dir_all(parent_dir);
        }
    }

}
