extern crate reqwest;

use std::error::Error;

use crate::{
    cloud::api::API,
    cmd::entropy::Entropy,

    ui::{
        errors_alerts::ErrorsAlerts,
        success_alerts::SuccessAlerts,
    }
};

pub struct Push {
    path: String,
    dbname: String,
    interval: u64,
}

impl Push {

    pub fn new(
        path: &str,
        dbname: &str,
        interval: u64,
    ) -> Self {
        Self {
            path: path.to_string(),
            dbname: dbname.to_string(),
            interval,
        }
    }

    pub async fn push(&self) -> Result<(), Box<dyn Error>> {
        let encrypted = if Entropy::new(&self.path).calculate()? > 7.5 {
            true
        } else {
            false
        };

        match API::new(
            Some(&self.path),
            None,
            Some(&self.dbname),
            Some(encrypted),
            Some(self.interval),
        ).post().await {
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
