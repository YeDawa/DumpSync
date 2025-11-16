use std::{
    fs::File,
    sync::Arc,
    error::Error,
    io::BufReader,
};

use chrono::{
    DateTime, 
    NaiveDateTime
};

use mysql::{
    *, 
    prelude::*,
};

use rayon::prelude::*;
use serde_json::Value;

use crate::{
    cmd::connection::Connection,
    ui::success_alerts::SuccessAlerts,

    handlers::mysql::{
        mysql_keywords::MySQLKeywords,
        mysql_queries_builders::MySqlQueriesBuilders,
    },
};

pub struct ImportDumpData {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub dbname: String,
    pub json_path: String,
    pub chunk_size: usize,
}

impl ImportDumpData {

    pub fn new(host: &str, port: u16, user: &str, password: &str, dbname: &str, json_path: &str) -> Self {
        Self {
            host: host.to_string(),
            port,
            user: user.to_string(),
            password: password.to_string(),
            dbname: dbname.to_string(),
            json_path: json_path.to_string(),

            chunk_size: 800,
        }
    }

    fn extract_tables(&self, array: &[Value]) -> Vec<String> {
        use std::collections::HashSet;
        let mut set = HashSet::new();

        for entry in array {
            if let Some(model) = entry.get("model").and_then(|v| v.as_str()) {
                if let Some(table) = model.split('.').nth(1) {
                    set.insert(table.to_string());
                }
            }
        }

        set.into_iter().collect()
    }

    fn truncate_all_tables(&self, tables: &[String], pool: &Arc<Pool>) -> Result<(), Box<dyn Error>> {
        let mut conn = pool.get_conn()?;
        conn.query_drop(MySqlQueriesBuilders.use_db(&self.dbname))?;

        for table in tables {
            let sql = MySqlQueriesBuilders.truncate_table(table);
            conn.query_drop(sql)?;
        }

        Ok(())
    }

    fn fix_datetime_format(&self, s: &str) -> String {
        if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
            return dt.naive_utc().format("%Y-%m-%d %H:%M:%S").to_string();
        }

        if let Ok(dt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
            return dt.format("%Y-%m-%d %H:%M:%S").to_string();
        }

        s.to_string()
    }

    fn json_to_mysql(&self, v: &Value) -> String {
        match v {
            Value::Null => MySQLKeywords::Null.as_str().to_string(),
            Value::String(s) => {
                let fixed = self.fix_datetime_format(s);
                let escaped = fixed.replace('\\', "\\\\")
                    .replace('\'', "\\'")
                    .replace('"', "\\\"")
                    .replace('\n', "\\n")
                    .replace('\r', "\\r")
                    .replace('\0', "\\0");

                format!("'{}'", escaped)
            },
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => (*b as i32).to_string(),
            _ => MySQLKeywords::Null.as_str().to_string(),
        }
    }

    fn process_chunk(&self, index: usize, chunk: &[Value], pool: &Arc<Pool>) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut conn = pool.get_conn()?;
        conn.query_drop(MySqlQueriesBuilders.use_db(&self.dbname))?;
        let mut tx = conn.start_transaction(TxOpts::default())?;

        for entry in chunk {
            if let Err(err) = self.import_entry(entry, &mut tx) {
                eprintln!("Chunk {} entry error: {}", index, err);
            }
        }

        tx.commit()?;
        Ok(())
    }

    fn import_entry(&self, entry: &Value, conn: &mut Transaction) -> Result<(), Box<dyn Error + Send + Sync>> {
        let model = entry
            .get("model")
            .and_then(|v| v.as_str())
            .ok_or("Missing model")?;

        let table = model
            .split('.')
            .nth(1)
            .ok_or("Invalid model format")?;

        let pk = entry.get("pk").ok_or("Missing PK")?;
        let fields = entry
            .get("fields")
            .and_then(|v| v.as_object())
            .ok_or("Missing fields")?;

        let mut columns = vec![];
        let mut values = vec![];

        for (field, value) in fields {
            columns.push(format!("`{}`", field));
            values.push(self.json_to_mysql(value));
        }

        let mut columns = vec!["id".to_string()];
        let mut values = vec![self.json_to_mysql(pk)];

        for (field, value) in fields {
            if field == "id" {
                continue;
            }

            columns.push(format!("`{}`", field));
            values.push(self.json_to_mysql(value));
        }

        let sql = MySqlQueriesBuilders.insert_into_start_json(table, &columns, &values, false);
        conn.query_drop(sql)?;
        Ok(())
    }

    pub fn import_parallel(&self) -> Result<(), Box<dyn Error>> {
        let file = File::open(&self.json_path)?;
        let reader = BufReader::new(file);
        let json: Value = serde_json::from_reader(reader)?;
        let array = json.as_array().ok_or("JSON root must be an array")?;

        let pool = Arc::new(
            Connection {
                host: self.host.clone(),
                port: self.port,
                user: self.user.clone(),
                password: self.password.clone(),
                dbname: Some(self.dbname.clone()),
            }
            .create_mysql_pool()?,
        );

        let tables = self.extract_tables(array);
        self.truncate_all_tables(&tables, &pool)?;

        array
            .par_chunks(self.chunk_size)
            .enumerate()
            .for_each(|(chunk_index, chunk)| {
                if let Err(err) = self.process_chunk(chunk_index, chunk, &pool) {
                    eprintln!("Chunk {} failed: {}", chunk_index, err);
                }
            });

        SuccessAlerts::import(&self.dbname);
        Ok(())
    }

}
