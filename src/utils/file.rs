use std::{
    fs,
    path::Path
};

pub struct FileUtils;

impl FileUtils {

    pub fn check_path_exists(path: &str) -> bool {
        Path::new(&path).exists()
    }

    pub fn create_path(path: &str) {
        if !Self::check_path_exists(path) {
            fs::create_dir_all(path).expect(
                &"Error creating directory".to_string()
            );
        }
    }

}
