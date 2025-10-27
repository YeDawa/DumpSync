use std:: {
    error::Error,
    fs::read_to_string,
};

use serde_json::{
    Value,
    to_string_pretty,
};

use serde_yaml::from_str;

pub struct Converters;

impl Converters {

    pub fn yaml_to_json(&self, path: &str) -> Result<String, Box<dyn Error>> {
        let yaml_content = read_to_string(path)?;
        let data: Value = from_str(&yaml_content)?;
        Ok(to_string_pretty(&data)?)
    }

}
