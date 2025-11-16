use std::{
    fs::File,
    error::Error,

    io::{
        Write, 
        BufWriter, 
        Error as IoError
    },
};

use mysql::{
    *, 
    Row, 
    prelude::*,
};

use flate2::{
    Compression,
    write::GzEncoder,
};

use serde_json::{
    json, 
    to_writer_pretty, 
    Value as JsonValue
};

use crate::{
    helpers::configs::Configs,

    handlers::{
        html_handlers::HTMLHandlers,

        mysql::{
            mysql_keywords::MySQLKeywords,
            mysql_queries_builders::MySqlQueriesBuilders,
        },
    },
};

pub enum Writer {
    Uncompressed(BufWriter<File>),
    Compressed(BufWriter<GzEncoder<File>>),
}

impl Writer {

    pub fn as_write(&mut self) -> &mut dyn Write {
        match self {
            Writer::Uncompressed(w) => w,
            Writer::Compressed(w) => w,
        }
    }

}

pub struct ExportHandlers {
    file: File,
    dbname: String,

    dump_data: bool,
    lock_tables: bool,
    compress_data: bool,
    insert_ignore_into: bool,
    drop_table_if_exists: bool,
    database_if_not_exists: bool,
}

impl ExportHandlers {
    pub fn new(file: File, dbname: &str) -> Self {
        Self {
            file,
            dbname: dbname.to_string(),

            dump_data: Configs.boolean("exports", "dump_data", true),
            lock_tables: Configs.boolean("exports", "lock_tables", false),
            compress_data: Configs.boolean("exports", "compress_data", false),
            insert_ignore_into: Configs.boolean("exports", "insert_ignore_into", false),
            drop_table_if_exists: Configs.boolean("exports", "drop_table_if_exists", false),
            database_if_not_exists: Configs.boolean("exports", "database_if_not_exists", true),
        }
    }

    pub fn create_writer(&self) -> Result<Writer, IoError> {
        if self.compress_data {
            let encoder = GzEncoder::new(self.file.try_clone()?, Compression::default());
            Ok(Writer::Compressed(BufWriter::new(encoder)))
        } else {
            Ok(Writer::Uncompressed(BufWriter::new(
                self.file.try_clone()?
            )))
        }
    }

    pub fn write_create_new_database(&self, writer: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        if self.database_if_not_exists {
            let (create_db, use_db) = MySqlQueriesBuilders.create_database(&self.dbname)?;

            writeln!(writer, "{}\n{}", create_db, use_db)?;

            writeln!(writer, "{} {}\n", 
                MySQLKeywords::Comments.as_str(),
                MySQLKeywords::FinalComments.as_str()
            )?;
        }

        Ok(())
    }

    fn mysql_to_json(value: &Value) -> JsonValue {
        match value {
            Value::NULL => JsonValue::Null,
            Value::Bytes(bytes) => {
                JsonValue::String(String::from_utf8_lossy(bytes).to_string())
            }
            Value::Int(i) => json!(i),
            Value::UInt(u) => json!(u),
            Value::Float(f) => json!(f),
            _ => JsonValue::Null,
        }
    }

    pub fn write_inserts_for_table(&self,  table: &str, conn: &mut PooledConn, writer: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        if !self.dump_data {
            return Ok(());
        }

        let rows: Vec<Row> = conn.query(MySqlQueriesBuilders.select(table, None, None))?;

        let columns: Vec<String> = conn.query_map(
            MySqlQueriesBuilders.show_columns(table),
            |row: Row| format!("`{}`", row.get::<String,_>("Field").unwrap())
        )?;

        if rows.is_empty() {
            writeln!(writer, "-- Table `{}` contains no data.", table)?;
        } else {
            let mut values_batch = Vec::new();

            for row in rows {
                let values: Vec<String> = row
                    .clone()
                    .unwrap()
                    .into_iter()
                    .map(|value| match value {
                        Value::NULL => MySQLKeywords::Null.as_str().to_string(),
                        Value::Bytes(bytes) => {
                            let escaped = HTMLHandlers.escape_for_sql(
                                &String::from_utf8_lossy(&bytes)
                            );
                            format!("'{}'", escaped)
                        }
                        Value::Int(i) => i.to_string(),
                        Value::UInt(u) => u.to_string(),
                        Value::Float(f) => f.to_string(),
                        _ => MySQLKeywords::Null.as_str().to_string(),
                    })
                    .collect();

                values_batch.push(format!("({})", values.join(", ")));
            }

            let insert_cmd = MySqlQueriesBuilders.insert_into_start(
                table,
                &columns,
                &values_batch,
                self.insert_ignore_into,
            );

            writeln!(writer, "{}", insert_cmd)?;
        }

        if self.lock_tables {
            writeln!(writer, "{}", MySqlQueriesBuilders.unlock_tables(table))?;
        }

        Ok(())
    }

    pub fn write_json_for_table(&self, table: &str, conn: &mut PooledConn, writer: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        if !self.dump_data {
            return Ok(());
        }

        let rows: Vec<Row> = conn.query(MySqlQueriesBuilders.select(table, None, None))?;

        let columns: Vec<(String, bool)> = conn.query_map(
            MySqlQueriesBuilders.show_columns(table),
            |row: Row| {
                let field: String = row.get("Field").unwrap();
                let key_type = row.get_opt::<String,_>("Key").and_then(|r| r.ok());
                (field, key_type.as_deref() == Some("PRI"))
            }
        )?;

        let mut json_rows = Vec::new();

        for row in rows {
            let mut fields = serde_json::Map::new();
            let mut pk_value = JsonValue::Null;

            for (idx, (col, is_pk)) in columns.iter().enumerate() {
                let val = row.as_ref(idx).unwrap();
                let json_val = Self::mysql_to_json(val);

                if *is_pk {
                    pk_value = json_val.clone();
                }

                fields.insert(col.clone(), json_val);
            }

            json_rows.push(json!({
                "model": format!("{}.{}", self.dbname, table),
                "pk": pk_value,
                "fields": fields
            }));
        }

        to_writer_pretty(&mut *writer, &json_rows)?;
        writeln!(writer)?;

        Ok(())
    }

    pub fn write_structure_for_table(&self, table: &str, conn: &mut PooledConn, writer: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        writeln!(writer, "-- Exporting table: `{}`", table)?;

        if self.lock_tables {
            writeln!(writer, "{}", MySqlQueriesBuilders.lock_tables(table))?;
        }

        if self.drop_table_if_exists {
            writeln!(writer, "{}", MySqlQueriesBuilders.drop_table(table))?;
        }

        let row: Row =
            conn.query_first(MySqlQueriesBuilders.show_create_table(table))?
            .expect("CREATE TABLE missing");

        let create_stmt: String = row.get(1).unwrap();
        writeln!(writer, "{};\n", create_stmt)?;

        for fk in conn.query::<Row, _>(MySqlQueriesBuilders.get_alter_table(table))? {
            let cname: String = fk.get(MySQLKeywords::ConstraintName.as_str()).unwrap();
            let col: String = fk.get(MySQLKeywords::ColumnName.as_str()).unwrap();
            let rtable: String = fk.get(MySQLKeywords::ReferencedTableName.as_str()).unwrap();
            let rcol: String = fk.get(MySQLKeywords::ReferencedColumnName.as_str()).unwrap();

            writeln!(
                writer,
                "{}",
                MySqlQueriesBuilders.get_foreign_keys(table, &cname, &col, &rtable, &rcol)
            )?;
        }

        for uk in conn.query::<Row,_>(MySqlQueriesBuilders.get_alter_table(table))? {
            let cname: String = uk.get(MySQLKeywords::ConstraintName.as_str()).unwrap();
            let col: String = uk.get(MySQLKeywords::ColumnName.as_str()).unwrap();

            writeln!(
                writer,
                "{}",
                MySqlQueriesBuilders.get_unique_keys(table, &cname, &col)
            )?;
        }

        writeln!(
            writer,
            "{} {}\n",
            MySQLKeywords::Comments.as_str(),
            MySQLKeywords::FinalComments.as_str()
        )?;

        Ok(())
    }

}
