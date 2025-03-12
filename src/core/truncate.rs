use std::error::Error;

use mysql::{
    *, 
    prelude::*
};

use crate::{
    ui::success_alerts::SuccessAlerts, 
    
    core::{
        export::Export,
        connection::Connection,
    },

    handlers::{
        dump_handlers::DumpHandlers,
        mysql_queries_builders::MySqlQueriesBuilders,
    },
};

pub struct Truncate {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub dbname: String,
    pub backup_path: String,
    pub table: String,
    pub encrypt: Option<bool>,
}

impl Truncate {

    pub fn new(
        host: &str, 
        port: u16, 
        user: &str, 
        password: &str, 
        dbname: &str, 
        backup_path: &str,
        table: &str,
        encrypt: Option<bool>,
    ) -> Self {
        Self {
            host: host.to_string(),
            port,
            user: user.to_string(),
            password: password.to_string(),
            dbname: dbname.to_string(),
            backup_path: backup_path.to_string(),
            table: table.to_string(),
            encrypt,
        }
    }

    pub fn dump_table(&self) -> Result<(), &'static str> {
        let dump_file_path = DumpHandlers.generate_dump_file_truncate_path(&self.dbname, &self.table, &self.backup_path);
        let password = if self.password.is_empty() { "" } else { &self.password };

        Export::new(
            &self.host,
            self.port as u16,
            &self.user,
            password,
            &self.dbname,
            &dump_file_path,
            Some(self.encrypt.unwrap_or(false)),
            Some(self.table.clone())
        ).dump_table().map_err(|_| "Failed to generate dump file")?;

        Ok(())
    }

    pub fn table(&self) -> Result<(), Box<dyn Error>> {
        let pool = Connection {
            host: self.host.clone(),
            port: self.port,
            user: self.user.clone(),
            password: self.password.clone(),
            dbname: Some(self.dbname.clone()),
        }.create_pool()?;

        let _ = &self.dump_table()?;

        let mut conn = pool.get_conn()?;
        let query = MySqlQueriesBuilders.truncate_table(&self.table);
        conn.query_drop(query)?;

        SuccessAlerts::truncate(&self.table);
        Ok(())
    }

}
