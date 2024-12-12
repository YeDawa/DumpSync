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

    pub fn extension(file_path: &str) -> String {
        let extension = Path::new(file_path)
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();

        extension.to_lowercase()
    }

    pub fn content(file_path: &str) -> String {
        fs::read_to_string(file_path)
            .unwrap_or_default()
    }

    pub fn exists(file_path: &str) -> bool {
        fs::metadata(file_path).is_ok()
    }

}
