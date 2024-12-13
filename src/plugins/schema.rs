use serde::Serialize;
use serde_json::to_string_pretty;

use std::{
    fs,
    error::Error,
};

use mysql::{
    *,
    prelude::*, 
};

use crate::{
    core::connection::Connection,
    ui::schema_alerts::SchemaAlerts,
    helpers::queries_builders::QueriesBuilders,
};

#[derive(Serialize, Debug)]
struct Column {
    name: String,
    data_type: String,
    is_nullable: bool,
    is_primary_key: bool,
}

#[derive(Serialize, Debug)]
struct ForeignKey {
    column: String,
    referenced_table: String,
    referenced_column: String,
}

#[derive(Serialize, Debug)]
struct Table {
    name: String,
    columns: Vec<Column>,
    foreign_keys: Vec<ForeignKey>,
}

pub struct Schema {
    host: String,
    port: u16,
    user: String,
    password: String,
    dbname: String,

    file: String,
}

impl Schema {

    pub fn new(
        host: &str,
        port: u16,
        user: &str,
        password: &str,
        dbname: &str,

        file: &str,
    ) -> Self {
        Self {
            host: host.to_string(),
            port,
            user: user.to_string(),
            password: password.to_string(),
            dbname: dbname.to_string(),

            file: file.to_string(),
        }
    }

    pub fn create(&self) -> Result<(), Box<dyn Error>> {
        let pool = Connection {
            host: self.host.clone(),
            port: self.port,
            user: self.user.clone(),
            password: self.password.clone(),
            dbname: Some(self.dbname.clone()),
        }.create_pool()?;
    
        let mut conn = pool.get_conn()?;
        let tables: Vec<String> = conn.query(QueriesBuilders.get_table_names())?;

        let mut schema = Vec::new();

        for table in tables {
            let columns: Vec<(String, String, String, String)> = conn.query(QueriesBuilders.table_info(&table))?;

            let column_data: Vec<Column> = columns
                .iter()
                .map(|col| Column {
                    name: col.0.clone(),
                    data_type: col.1.clone(),
                    is_nullable: col.2 == "YES",
                    is_primary_key: col.3 == "PRI",
                })
                .collect();

            let foreign_keys: Vec<(String, String, String)> = conn.query(QueriesBuilders::foreign_key_info(&table))?;

            let foreign_key_data: Vec<ForeignKey> = foreign_keys
                .iter()
                .map(|fk| ForeignKey {
                    column: fk.0.clone(),
                    referenced_table: fk.1.clone(),
                    referenced_column: fk.2.clone(),
                })
                .collect();

            schema.push(Table {
                name: table,
                columns: column_data,
                foreign_keys: foreign_key_data,
            });
        }

        let json_schema = to_string_pretty(&schema)?;
        fs::write(&self.file, json_schema)?;

        SchemaAlerts::success(&self.file);
        Ok(())
    }

}