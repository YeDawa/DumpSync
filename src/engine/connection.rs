use mysql::{
    Pool, 
    Opts, 
    OptsBuilder
};

use std::error::Error;
use crate::ui::errors_alerts::ErrorsAlerts;

pub struct Connection {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub dbname: String,
}

impl Connection {

    pub fn create_pool(&self) -> Result<Pool, Box<dyn Error>> {
        let mut opts_builder = OptsBuilder::new()
            .ip_or_hostname(Some(&self.host))
            .tcp_port(self.port)
            .user(Some(&self.user))
            .db_name(Some(&self.dbname));

        if !self.password.is_empty() {
            opts_builder = opts_builder.pass(Some(&self.password));
        }

        Pool::new(Opts::from(opts_builder)).map_err(|e| {
            ErrorsAlerts::dump(&e.to_string());
            Box::new(e) as Box<dyn Error>
        })
    }

}
