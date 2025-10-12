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
    pub dbname: Option<String>,
}

impl Connection {

    pub fn create_mysql_pool(&self) -> Result<Pool, Box<dyn Error>> {
        let mut opts_builder = OptsBuilder::new()
            .ip_or_hostname(Some(&self.host))
            .tcp_port(self.port)
            .user(Some(&self.user));

        if !self.password.is_empty() {
            opts_builder = opts_builder.pass(
                Some(&self.password)
            );
        }

        if let Some(ref dbname) = self.dbname {
            if !dbname.is_empty() {
                opts_builder = opts_builder.db_name(Some(dbname));
            } else {
                opts_builder = opts_builder.db_name::<String>(None);
            }
        } else {
            opts_builder = opts_builder.db_name::<String>(None);
        }

        Pool::new(Opts::from(opts_builder)).map_err(|e| {
            ErrorsAlerts::dump(&e.to_string());
            Box::new(e) as Box<dyn Error>
        })
    }

}
