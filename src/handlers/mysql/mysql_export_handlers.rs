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
    prelude::*
};

use flate2::{
    Compression, 
    write::GzEncoder
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
            let encoder = GzEncoder::new(
                self.file.try_clone()?, Compression::default()
            );

            Ok(
                Writer::Compressed(
                    BufWriter::new(encoder)
                )
            )
        } else {
            Ok(
                Writer::Uncompressed(
                    BufWriter::new(self.file.try_clone()?)
                )
            )
        }
    }

    pub fn write_create_new_database(&self, writer: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        if self.database_if_not_exists {
            let queries = MySqlQueriesBuilders.create_database(&self.dbname)?;

            write!(writer, "{}", queries.0)?;
            writeln!(writer, "{}", queries.1)?;
            writeln!(writer, "{} {}\n", MySQLKeywords::Comments.as_str(), MySQLKeywords::FinalComments.as_str())?;
        }

        Ok(())
    }

    pub fn write_inserts_for_table(
        &self,
        table: &str,
        conn: &mut PooledConn,
        writer: &mut dyn Write,
    ) -> Result<(), Box<dyn Error>> {
        if self.dump_data {
            let columns: Vec<String> = conn.query_map(MySqlQueriesBuilders.show_columns(table), |row: Row| {
                let field: String = row.get("Field").unwrap();
                format!("`{}`", field)
            })?;

            const PAGE_SIZE: usize = 10_000;
            let mut offset = 0;
            let mut batch_count = 0;

            loop {
                let query = MySqlQueriesBuilders.select(table, Some(PAGE_SIZE), Some(offset));
                let rows: Vec<Row> = conn.query(query)?;

                if rows.is_empty() {
                    if offset == 0 {
                        writeln!(writer, "-- Table `{}` contains no data.", table)?;
                    }
                    break;
                }

                let rows_len = rows.len();
                for row in rows {
                    let values: Vec<String> = row
                        .unwrap()
                        .into_iter()
                        .map(|value| match value {
                            Value::NULL => "NULL".to_string(),
                            Value::Bytes(bytes) => {
                                let raw = String::from_utf8_lossy(&bytes);
                                format!("'{}'", HTMLHandlers.escape_for_sql(&raw))
                            }
                            Value::Int(i) => i.to_string(),
                            Value::UInt(u) => u.to_string(),
                            Value::Float(f) => f.to_string(),
                            _ => "NULL".to_string(),
                        })
                        .collect();

                    let insert_prefix = MySqlQueriesBuilders.insert_into_start(table, &columns, self.insert_ignore_into);
                    writeln!(writer, "{}({});", insert_prefix, values.join(", "))?;
                }

                batch_count += rows_len;
                offset += PAGE_SIZE;
            }

            writeln!(writer, "-- Total rows exported: {}", batch_count)?;

            if self.lock_tables {
                writeln!(writer, "{}", MySqlQueriesBuilders.unlock_tables(table))?;
            }
        }

        Ok(())
    }    

    pub fn write_structure_for_table(&self, table: &str, conn: &mut PooledConn, writer: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        writeln!(writer, "-- Exporting the table: `{}`", table)?;

        if self.lock_tables {
            writeln!(writer, "{}", MySqlQueriesBuilders.lock_tables(table))?;
        }
    
        if self.drop_table_if_exists {
            writeln!(writer, "{}", MySqlQueriesBuilders.drop_table(table))?;
        }
    
        let row: Row = conn.query_first(MySqlQueriesBuilders.show_create_table(table))?.unwrap();
        let create_table: String = row.get(1).expect("Error retrieving CREATE TABLE");
        writeln!(writer, "{};\n", create_table)?;
    
        let fk_query = MySqlQueriesBuilders.get_alter_table(table);
        let foreign_keys: Vec<Row> = conn.query(fk_query)?;

        for fk in foreign_keys {
            let constraint_name: String = fk.get("CONSTRAINT_NAME").unwrap();
            let column_name: String = fk.get("COLUMN_NAME").unwrap();
            let ref_table: String = fk.get("REFERENCED_TABLE_NAME").unwrap();
            let ref_column: String = fk.get("REFERENCED_COLUMN_NAME").unwrap();
            
            writeln!(
                writer, "{}", MySqlQueriesBuilders.get_foreign_keys(
                    table, &constraint_name, &column_name, &ref_table, &ref_column
                )
            )?;
        }
    
        let unique_query = MySqlQueriesBuilders.get_alter_table(table);
        let unique_keys: Vec<Row> = conn.query(unique_query)?;

        for uk in unique_keys {
            let constraint_name: String = uk.get("CONSTRAINT_NAME").unwrap();
            let column_name: String = uk.get("COLUMN_NAME").unwrap();
            
            writeln!(
                writer, "{}", MySqlQueriesBuilders.get_unique_keys(table, &constraint_name, &column_name)
            )?;
        }
    
        writeln!(writer, "{} {}\n", MySQLKeywords::Comments.as_str(), MySQLKeywords::FinalComments.as_str())?;
        Ok(())
    }    

}