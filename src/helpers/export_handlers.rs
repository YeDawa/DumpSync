use std::{
    fs::File,
    error::Error,

    io::{
        Write, 
        BufWriter
    },
};

use flate2::{
    Compression, 
    write::GzEncoder
};

use mysql::{
    *, 
    Row, 
    prelude::*
};

use crate::{
    utils::date::Date,
    helpers::configs::Configs,
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
            compress_data: Configs.boolean("exports", "compress_data", false),
            insert_ignore_into: Configs.boolean("exports", "insert_ignore_into", false),
            drop_table_if_exists: Configs.boolean("exports", "drop_table_if_exists", false),
            database_if_not_exists: Configs.boolean("exports", "database_if_not_exists", true),
        }
    }

    pub fn create_writer(&self) -> Result<Writer, std::io::Error> {
        if self.compress_data {
            let encoder = GzEncoder::new(self.file.try_clone()?, Compression::default());
            Ok(Writer::Compressed(BufWriter::new(encoder)))
        } else {
            Ok(Writer::Uncompressed(BufWriter::new(self.file.try_clone()?)))
        }
    }

    pub fn comments_header(&self, writer: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        writeln!(writer, "-- Exporting using {} v.{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))?;
        writeln!(writer, "-- Database backup: {}", self.dbname)?;
        writeln!(writer, "-- Export date and time: {}", Date::timestamp())?;
        writeln!(writer, "-- ---------------------------------------------------\n")?;

        Ok(())
    }

    pub fn write_create_new_database(&self, writer: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        if self.database_if_not_exists {
            writeln!(writer, "CREATE DATABASE IF NOT EXISTS `{}`;", self.dbname)?;
            writeln!(writer, "USE `{}`;", self.dbname)?;
            writeln!(writer, "-- ---------------------------------------------------\n")?;
        }

        Ok(())
    }

    pub fn write_inserts_for_table(&self, table: &str, conn: &mut PooledConn, writer: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        if self.dump_data {
            let rows: Vec<Row> = conn.query(format!("SELECT * FROM `{}`", table))?;

            if rows.is_empty() {
                writeln!(writer, "-- Table `{}` contains no data.", table)?;
            } else {
                for row in rows {
                    let values: Vec<String> = row.clone().unwrap().into_iter().map(|value| match value {
                        Value::NULL => "NULL".to_string(),
                        Value::Bytes(bytes) => format!("'{}'", String::from_utf8_lossy(&bytes)),
                        Value::Int(int) => int.to_string(),
                        Value::UInt(uint) => uint.to_string(),
                        Value::Float(float) => float.to_string(),
                        _ => "NULL".to_string(),
                    }).collect();

                    let line = if self.insert_ignore_into {
                        format!("INSERT IGNORE INTO `{}` VALUES ({});", table, values.join(", "))
                    } else {
                        format!("INSERT INTO `{}` VALUES ({});", table, values.join(", "))
                    };

                    writeln!(writer, "{}", line)?;
                }
            }
        }

        Ok(())
    }

    pub fn write_structure_for_table(&self, table: &str, conn: &mut PooledConn, writer: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        writeln!(writer, "-- Exporting the table: `{}`", table)?;

        if self.drop_table_if_exists {
            writeln!(writer, "DROP TABLE IF EXISTS `{}`;", table)?;
        }

        let row: Row = conn.query_first(format!("SHOW CREATE TABLE `{}`", table))?.unwrap();
        let create_table: String = row.get(1).expect("Error retrieving CREATE TABLE");
        writeln!(writer, "{};\n", create_table)?;

        Ok(())
    }

}