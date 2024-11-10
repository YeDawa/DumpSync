use std::{
    fs::File,
    error::Error,

    io::{
        self,
        Write,
        BufWriter,
    },
};

use mysql::{
    *,
    Row,
    prelude::*
};

use crate::{
    ui::success_alerts::SuccessAlerts,

    utils::{
        date::Date,
        file::FileUtils,
    },

    engine::{
        configs::Configs,
        connection::Connection,
    },
};

pub struct Export {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub dbname: String,
    pub dump_file_path: String,
}

impl Export {
    
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

    fn comments_header(&self, writer: &mut BufWriter<File>) -> Result<(), Box<dyn Error>> {
        writeln!(writer, "-- Exporting using {} v.{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))?;
        writeln!(writer, "-- Database backup: {}", self.dbname)?;
        writeln!(writer, "-- Export date and time: {}", Date::timestamp())?;
        writeln!(writer, "-- ---------------------------------------------------\n")?;

        Ok(())
    }

    fn write_create_new_database(&self, writer: &mut BufWriter<File>) -> Result<(), Box<dyn Error>> {
        let database_if_not_exists = Configs.boolean("exports", "database_if_not_exists", true);

        if database_if_not_exists {
            writeln!(writer, "CREATE DATABASE IF NOT EXISTS `{}`;", self.dbname)?;
            writeln!(writer, "USE `{}`;", self.dbname)?;
            writeln!(writer, "-- ---------------------------------------------------\n")?;
        }

        Ok(())
    }

    fn write_inserts_for_table(&self, table: &str, conn: &mut PooledConn, writer: &mut BufWriter<File>) -> Result<(), Box<dyn Error>> {
        let dump_data = Configs.boolean("exports", "dump_data", true);
        let insert_ignore_into = Configs.boolean("exports", "insert_ignore_into", false);

        if dump_data {
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

                    let line = if insert_ignore_into {
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

    fn write_structure_for_table(&self, table: &str, conn: &mut PooledConn, writer: &mut BufWriter<File>) -> Result<(), Box<dyn Error>> {
        let drop_table_if_exists = Configs.boolean("exports", "drop_table_if_exists", false);

        writeln!(writer, "-- Exporting the table: `{}`", table)?;

        if drop_table_if_exists {
            writeln!(writer, "DROP TABLE IF EXISTS `{}`;", table)?;
        }

        let row: Row = conn.query_first(format!("SHOW CREATE TABLE `{}`", table))?.unwrap();
        let create_table: String = row.get(1).expect("Error retrieving CREATE TABLE");

        writeln!(writer, "{};\n", create_table)?;
        Ok(())
    }

    pub fn dump(&self) -> Result<(), Box<dyn Error>> {
        let pool = Connection {
            host: self.host.clone(),
            port: self.port,
            user: self.user.clone(),
            password: self.password.clone(),
            dbname: Some(self.dbname.clone()),
        }.create_pool()?;

        if FileUtils::check_path_exists(&self.dump_file_path) {
            FileUtils::create_path(&self.dump_file_path);
        }

        let mut conn = pool.get_conn()?;
        let file = File::create(&self.dump_file_path)?;
        let mut writer = BufWriter::new(file);

        let _ = &self.comments_header(&mut writer)?;
        let _ = &self.write_create_new_database(&mut writer)?;

        let tables: Vec<String> = conn.query("SHOW TABLES")?;
        for table in tables {
            self.write_structure_for_table(&table, &mut conn, &mut writer)?;
            self.write_inserts_for_table(&table, &mut conn, &mut writer)?;
            writeln!(writer, "-- End of table `{}`", table)?;
        }
    
        SuccessAlerts::dump(&self.dump_file_path);
        io::stdout().flush().unwrap();
        
        Ok(())
    }

}
