use regex::Regex;

use mysql::{
    *,
    prelude::*
};

use std::{
    fs::File, 
    io::Read, 
    error::Error,
};

use crate::{
    engine::connection::Connection,

    ui::{
        errors_alerts::ErrorsAlerts,
        success_alerts::SuccessAlerts,
    },
};

pub struct Import {
    host: String,
    port: u16,
    user: String,
    password: String,
    dbname: String,
    dump_file_path: String,
}

impl Import {

    pub fn new(host: &str, port: u16, user: &str, password: &str, dbname: &str, dump_file_path: &str) -> Self {
        Self {
            host: host.to_string(),
            port,
            user: user.to_string(),
            password: password.to_string(),
            dbname: dbname.to_string(),
            dump_file_path: dump_file_path.to_string(),
        }
    }

    pub fn dump(&self) -> Result<(), Box<dyn Error>> {
        let pool = Connection {
            host: self.host.clone(),
            port: self.port,
            user: self.user.clone(),
            password: self.password.clone(),
            dbname: Some(self.dbname.clone()),
        }.create_pool()?;

        let mut conn = pool.get_conn()?;

        let mut dump_file = File::open(&self.dump_file_path)?;
        let mut dump_content = String::new();
        dump_file.read_to_string(&mut dump_content)?;

        let create_table_regex = Regex::new(r"(?i)CREATE TABLE\s+`?(\w+)`?").unwrap();

        for statement in dump_content.split(';') {
            let trimmed = statement.trim();

            if !trimmed.is_empty() {
                match conn.query_drop(trimmed) {
                    Ok(_) => {
                        if let Some(captures) = create_table_regex.captures(trimmed) {
                            if let Some(table_name) = captures.get(1) {
                                SuccessAlerts::table(table_name.as_str());
                            }
                        }
                    },

                    Err(e) => ErrorsAlerts::import(&self.dbname, trimmed, &e.to_string()),
                }
            }
        }

        SuccessAlerts::import(&self.dbname);
        Ok(())
    }
}
