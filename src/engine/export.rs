use mysql::{
    *,
    prelude::*
};

use std::{
    io,
    fs::File,
    error::Error,

    io::{
        Write,
        BufWriter,
    },
};

use chrono::Utc;

use crate::ui::{
    errors_alerts::ErrorsAlerts,
    success_alerts::SuccessAlerts,
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

    pub fn dump(&self) -> Result<(), Box<dyn Error>> {
        let mut opts_builder = OptsBuilder::new()
            .ip_or_hostname(Some(&self.host))
            .tcp_port(self.port)
            .user(Some(&self.user))
            .db_name(Some(&self.dbname));

        if !self.password.is_empty() {
            opts_builder = opts_builder.pass(Some(&self.password));
        }

        let pool = match Pool::new(
            Opts::from(opts_builder)
        ) {
            Ok(pool) => pool,
            Err(e) => {
                ErrorsAlerts::dump(&e.to_string());
                return Err(Box::new(e));
            }
        };

        let mut conn = pool.get_conn()?;
        let file = File::create(&self.dump_file_path)?;
        let mut writer = BufWriter::new(file);
    
        let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        writeln!(writer, "-- Database backup: {}", self.dbname)?;
        writeln!(writer, "-- Export date and time: {}", timestamp)?;
        writeln!(writer, "-- ---------------------------------------------------\n")?;
    
        let tables: Vec<String> = conn.query("SHOW TABLES")?;
        for table in tables {
            writeln!(writer, "-- Exporting the table: `{}`", table)?;

            let row: Row = conn.query_first(format!("SHOW CREATE TABLE `{}`", table))?.unwrap();
            let create_table: String = row.get(1).expect("Error retrieving CREATE TABLE");
    
            writeln!(writer, "{};\n", create_table)?;
    
            let rows: Vec<Row> = conn.query(format!("SELECT * FROM `{}`", table))?;
            if rows.is_empty() {
                writeln!(writer, "-- Table `{}` contains no data.", table)?;
            } else {
                for row in rows {
                    let values: Vec<String> = row.unwrap().into_iter().map(|value| match value {
                        Value::NULL => "NULL".to_string(),
                        Value::Bytes(bytes) => format!("'{}'", String::from_utf8_lossy(&bytes)),
                        Value::Int(int) => int.to_string(),
                        Value::UInt(uint) => uint.to_string(),
                        Value::Float(float) => float.to_string(),
                        _ => "NULL".to_string(),
                    }).collect();
    
                    writeln!(writer, "INSERT INTO `{}` VALUES ({});", table, values.join(", "))?;
                }
            }
    
            writeln!(writer, "-- End of table `{}`\n", table)?;
        }
    
        SuccessAlerts::dump(&self.dump_file_path);
        io::stdout().flush().unwrap();
        
        Ok(())
    }

}