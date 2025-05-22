extern crate reqwest;

use std::error::Error;
use crate::cloud::api::API;

pub struct Push {
    path: String,
    dbname: String,
    encrypted: bool,
}

impl Push {

    pub fn new(
        path: &str,
        dbname: &str,
        encrypted: bool,
    ) -> Self {
        Self {
            path: path.to_string(),
            dbname: dbname.to_string(),
            encrypted,
        }
    }

    pub async fn push(&self) -> Result<(), Box<dyn Error>> {
        match API::new(
            Some(&self.path),
            None,
            Some(&self.dbname),
            Some(self.encrypted),
        ).upload().await {
            Ok(data) => {
                println!("{:?}", data);
            }

            Err(e) => {
                eprintln!("{}", e);
            }
        }

        Ok(())
    }

}
