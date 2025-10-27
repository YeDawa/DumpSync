extern crate reqwest;

use std::error::Error;

use crate::{
    cloud::api::API,

    ui::{
        errors_alerts::ErrorsAlerts,
        success_alerts::SuccessAlerts,
    }
};

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
            None,
            None,
        ).upload().await {
            Ok(data) => {
                SuccessAlerts::push(&data.message);
            }

            Err(_) => {
                ErrorsAlerts::push();
            }
        }

        Ok(())
    }

}
