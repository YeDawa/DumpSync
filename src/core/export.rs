use std::{
    fs::File,
    error::Error,
};

use chrono::Utc;

use mysql::{
    *, 
    prelude::*
};

use crate::{
    ui::success_alerts::SuccessAlerts, 
    
    cmd::{
        encrypt::Encrypt,
        connection::Connection,
    },

    utils::{
        file::FileUtils,
        generate::Generate,
    },

    helpers::{
        configs::Configs,
        history::History,
    },

    handlers::{
        comments_headers::CommentsHeaders,

        mysql::{
            mysql_export_handlers::ExportHandlers,
            mysql_queries_builders::MySqlQueriesBuilders,
        },
    },
};

pub struct Export {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub dbname: String,
    pub dump_file_path: String,
    pub encrypt: Option<bool>,
    pub table: Option<String>,
}

impl Export {

    pub fn new(
        host: &str, 
        port: u16, 
        user: &str, 
        password: &str, 
        dbname: &str, 
        dump_file_path: &str,
        encrypt: Option<bool>,
        table: Option<String>
    ) -> Self {
        Self {
            host: host.to_string(),
            port,
            user: user.to_string(),
            password: password.to_string(),
            dbname: dbname.to_string(),
            dump_file_path: dump_file_path.to_string(),
            encrypt,
            table,
        }
    }

    pub fn dump(&self) -> Result<(), Box<dyn Error>> {
        let compress_data = Configs.boolean("exports", "compress_data", false);

        let dump_file_path = if compress_data {
            format!("{}.gz", self.dump_file_path)
        } else {
            self.dump_file_path.clone()
        };

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

        CommentsHeaders.core(&self.dbname, writer.as_write())?;
        export_handlers.write_create_new_database(writer.as_write())?;

        let tables: Vec<String> = conn.query(MySqlQueriesBuilders.show_tables())?;
        let ignore_tables = Configs.list("exports", "ignore_tables").unwrap_or_default();

        for table in tables {
            if ignore_tables.contains(&serde_yaml::Value::String(table.clone())) {
                writeln!(writer.as_write(), "-- Table `{}` is ignored.", table)?;
                continue;
            }

            export_handlers.write_structure_for_table(&table, &mut conn, writer.as_write())?;
            export_handlers.write_inserts_for_table(&table, &mut conn, writer.as_write())?;
            writeln!(writer.as_write(), "-- End of table `{}`\n", table)?;
        }

        if self.encrypt.unwrap_or(false) {
            let _ = Encrypt::new(&dump_file_path).encrypt();
        } else {
            SuccessAlerts::dump(&dump_file_path);
        }

        let file_size = FileUtils::file_size(&dump_file_path)? as i64;
        History::new().insert_backup(
            &Generate.random_string(16),
            &dump_file_path,
            &self.dbname,
            &self.host,
            &Utc::now().to_rfc3339(),
            file_size,
            self.encrypt.unwrap_or(false),
            compress_data,
        )?;

        Ok(())
    }

    pub fn dump_table(&self) -> Result<(), Box<dyn Error>> {
        let compress_data = Configs.boolean("exports", "compress_data", false);

        let dump_file_path = if compress_data {
            format!("{}.gz", self.dump_file_path)
        } else {
            self.dump_file_path.clone()
        };

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

        CommentsHeaders.truncate(&self.dbname, &table, writer.as_write())?;
        export_handlers.write_inserts_for_table(&table, &mut conn, writer.as_write())?;
        writeln!(writer.as_write(), "-- End of table `{}`", table)?;

        if self.encrypt.unwrap_or(false) {
            let _ = Encrypt::new(&dump_file_path).encrypt();
        } else {
            SuccessAlerts::dump(&dump_file_path);
        }

        Ok(())
    }

}