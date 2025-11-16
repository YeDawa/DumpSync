use std::{
    error::Error,
    fs::File,
    io::{Write, BufWriter},
};

use mysql::*;
use mysql::prelude::*;
use serde_json::{json, Value};

use crate::{
    utils::file::FileUtils,
    helpers::configs::Configs,
    cmd::connection::Connection,
    ui::success_alerts::SuccessAlerts,

    handlers::{
        dump_handlers::DumpHandlers,
        mysql::mysql_queries_builders::MySqlQueriesBuilders,
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

    pub fn export(&self) -> Result<(), Box<dyn Error>> {
        let dump_file_path = DumpHandlers.generate_dump_json_file_path(
            &self.dbname,
            &self.dump_file_path
        );

        FileUtils::create_path(&dump_file_path);
        let file = File::create(&dump_file_path)?;
        let mut writer = BufWriter::new(file);

        let pool = Connection {
            host: self.host.clone(),
            port: self.port,
            user: self.user.clone(),
            password: self.password.clone(),
            dbname: Some(self.dbname.clone()),
        }.create_mysql_pool()?;

        let mut conn = pool.get_conn()?;

        writer.write_all(b"[\n")?;

        let mut is_first = true;
        match &self.table {
            Some(t) => self.dump_one_table(&mut conn, &mut writer, t, &mut is_first)?,
            None     => self.dump_all_tables(&mut conn, &mut writer, &mut is_first)?,
        }

        writer.write_all(b"\n]")?;
        writer.flush()?;

        SuccessAlerts::dump(&dump_file_path);
        Ok(())
    }

    fn dump_all_tables(&self, conn: &mut PooledConn, writer: &mut BufWriter<File>, is_first: &mut bool) -> Result<(), Box<dyn Error>> {
        let rows: Vec<Row> = conn.query(MySqlQueriesBuilders.show_tables())?;
        let ignore_tables = Configs.list("exports", "ignore_tables").unwrap_or_default();

        for row in rows {
            let table: String = row.get(0).unwrap();

            if ignore_tables.contains(&serde_yaml::Value::String(table.clone())) {
                continue;
            }

            self.dump_rows(conn, writer, &table, is_first)?;
        }

        Ok(())
    }

    fn dump_one_table(&self, conn: &mut PooledConn, writer: &mut BufWriter<File>, table: &str, is_first: &mut bool) -> Result<(), Box<dyn Error>> {
        self.dump_rows(conn, writer, table, is_first)
    }

    fn dump_rows(&self, conn: &mut PooledConn, writer: &mut BufWriter<File>, table: &str, is_first: &mut bool) -> Result<(), Box<dyn Error>> {
        let pk = self.get_primary_key(conn, table)?;
        let rows: Vec<Row> = conn.exec(format!("SELECT * FROM `{table}`"), ())?;

        for row in rows {
            let obj = self.row_to_django_obj("app", table, &pk, &row)?;
            let js = serde_json::to_string(&obj)?;

            if !*is_first {
                writer.write_all(b",\n")?;
            }
            *is_first = false;

            writer.write_all(js.as_bytes())?;
        }

        Ok(())
    }

    fn get_primary_key(&self, conn: &mut PooledConn, table: &str) -> Result<String, Box<dyn Error>> {
        let sql = format!("SHOW KEYS FROM `{}` WHERE Key_name='PRIMARY'", table);

        let rows: Vec<Row> = conn.query(sql)?;

        let col = rows.first()
            .and_then(|r| r.get::<String, _>("Column_name"))
            .ok_or("Primary key not found")?;

        Ok(col)
    }

    fn row_to_django_obj(&self, app: &str, table: &str, pk_column: &str, row: &Row) -> Result<Value, Box<dyn Error>> {
        let mut fields = serde_json::Map::new();
        let mut pk_value = Value::Null;

        let columns = row.columns_ref();

        for (i, col) in columns.iter().enumerate() {
            let name = col.name_str().to_string();
            let raw: Option<mysql::Value> = row.get(i);

            let val = match raw.unwrap_or(mysql::Value::NULL) {
                mysql::Value::NULL => Value::Null,
                mysql::Value::Bytes(b) => Value::String(String::from_utf8_lossy(&b).to_string()),
                mysql::Value::Int(x) => json!(x),
                mysql::Value::UInt(x) => json!(x),
                mysql::Value::Float(x) => json!(x),
                mysql::Value::Double(x) => json!(x),
                mysql::Value::Date(y, m, d, hh, mm, ss, _) =>
                    json!(format!("{y:04}-{m:02}-{d:02}T{hh:02}:{mm:02}:{ss:02}Z")),
                other => json!(format!("{:?}", other)),
            };

            if name == pk_column {
                pk_value = val;
            } else {
                fields.insert(name, val);
            }
        }

        Ok(json!({
            "model": format!("{app}.{table}"),
            "pk": pk_value,
            "fields": fields
        }))
    }

}
