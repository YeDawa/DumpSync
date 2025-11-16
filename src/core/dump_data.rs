use std::{
    fs::File,
    error::Error,
};

use mysql::{
    *, 
    prelude::*
};

use crate::{
    utils::file::FileUtils,
    helpers::configs::Configs,
    cmd::connection::Connection,
    ui::success_alerts::SuccessAlerts, 

    handlers::{
        dump_handlers::DumpHandlers,

        mysql::{
            mysql_export_handlers::ExportHandlers,
            mysql_queries_builders::MySqlQueriesBuilders,
        },
    },
};

pub struct DumpData {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub dbname: String,
    pub dump_file_path: String,
    pub table: Option<String>,
}

impl DumpData {

    pub fn new(
        host: &str, 
        port: u16, 
        user: &str, 
        password: &str, 
        dbname: &str, 
        dump_file_path: &str,
        table: Option<String>,
    ) -> Self {
        Self {
            host: host.to_string(),
            port,
            user: user.to_string(),
            password: password.to_string(),
            dbname: dbname.to_string(),
            dump_file_path: dump_file_path.to_string(),
            table,
        }
    }

    pub fn export_full(&self) -> Result<(), Box<dyn Error>> {
        let dump_file_path = DumpHandlers.generate_dump_json_file_path(&self.dbname, &self.dump_file_path);
        let export_handlers = ExportHandlers::new(
            File::create(dump_file_path.clone())?, 
            &self.dbname
        );

        let pool = Connection {
            host: self.host.clone(),
            port: self.port,
            user: self.user.clone(),
            password: self.password.clone(),
            dbname: Some(self.dbname.clone()),
        }.create_mysql_pool()?;

        FileUtils::create_path(&dump_file_path.clone());

        let mut conn = pool.get_conn()?;
        let mut writer = export_handlers.create_writer()?;

        let tables: Vec<String> = conn.query(MySqlQueriesBuilders.show_tables())?;
        let ignore_tables = Configs.list("exports", "ignore_tables").unwrap_or_default();

        for table in tables {
            if ignore_tables.contains(&serde_yaml::Value::String(table.clone())) {
                continue;
            }

            export_handlers.write_json_for_table(&table, &mut conn, writer.as_write())?;
        }

        SuccessAlerts::dump(&dump_file_path);
        Ok(())
    }

    pub fn dump_data_table(&self) -> Result<(), Box<dyn Error>> {
        let dump_file_path = DumpHandlers.generate_dump_json_file_path(&self.dbname, &self.dump_file_path);
        let export_handlers = ExportHandlers::new(
            File::create(dump_file_path.clone())?, 
            &self.dbname
        );

        let pool = Connection {
            host: self.host.clone(),
            port: self.port,
            user: self.user.clone(),
            password: self.password.clone(),
            dbname: Some(self.dbname.clone()),
        }.create_mysql_pool()?;

        FileUtils::create_path(&dump_file_path.clone());

        let mut conn = pool.get_conn()?;
        let mut writer = export_handlers.create_writer()?;
        let table = self.table.as_deref().unwrap_or("");

        export_handlers.write_json_for_table(&table, &mut conn, writer.as_write())?;
        SuccessAlerts::dump(&dump_file_path);
        Ok(())
    }

    pub fn export(&self) -> Result<(), Box<dyn Error>> {
        if let Some(_) = &self.table {
            self.dump_data_table()?;
        } else {
            self.export_full()?;
        }

        Ok(())
    }

}