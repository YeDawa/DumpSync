use std:: {
    error::Error,
    fs::read_to_string,
};

use serde_json::{
    Value,
    to_string_pretty,
};

use serde_yaml::from_str;

pub struct Converters {
    path: String,
}

impl Converters {

    pub fn new(path: String) -> Self {
        Self {
            path
        }
    }

    pub fn yaml_to_json(&self) -> Result<String, Box<dyn Error>> {
        let yaml_content = read_to_string(&self.path)?;
        let data: Value = from_str(&yaml_content)?;
        Ok(to_string_pretty(&data)?)
    }

}
