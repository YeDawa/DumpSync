use flate2::read::GzDecoder;

use mysql::{
    *,
    prelude::*
};

use std::{
    fs::File, 
    error::Error,

    io::{
        Read,
        BufReader, 
    }, 

    path::{
        Path, 
        PathBuf
    },
};

use crate::{
    ui::{
        errors_alerts::ErrorsAlerts,
        success_alerts::SuccessAlerts,
    },

    core::{
        encrypt::Encrypt,
        entropy::Entropy,
        connection::Connection,
    },

    handlers::{
        import_handlers::ImportHandlers,
        mysql::mysql_keywords::MySQLKeywords,
    },
};

pub struct Import {
    host: String,
    port: u16,
    user: String,
    password: String,
    dbname: String,
    path: Option<String>,
    ignore_drop_table: Option<bool>,
    dump_file_path: Option<String>,
    sql_content: Option<String>
}

impl Import {

    pub fn new(
        host: &str, 
        port: u16, 
        user: &str, 
        password: &str, 
        dbname: &str, 
        ignore_drop_table: Option<bool>,
        dump_file_path: Option<&str>, 
        path: Option<&str>,
        sql_content: Option<&str>,
    ) -> Self {
        Self {
            host: host.to_string(),
            port,
            ignore_drop_table,
            user: user.to_string(),
            password: password.to_string(),
            dbname: dbname.to_string(),
            path: path.map(|s| s.to_string()),
            dump_file_path: dump_file_path.map(|s| s.to_string()),
            sql_content: sql_content.map(|s| s.to_string()),
        }
    }

    fn complete_path(&self) -> Result<PathBuf, Box<dyn Error>> {
        let dump_file_path = self.dump_file_path.as_ref().ok_or("dump_file_path is None")?;
        let path = Path::new(dump_file_path);

        if path.is_absolute() {
            Ok(path.to_path_buf())
        } else {
            let base_path = self.path.as_ref().ok_or("path is None")?;
            Ok(Path::new(base_path).join(dump_file_path))
        }
    }

    fn process_statements(&self, conn: &mut PooledConn, dump_content: &str, dbname: &str) {
        let mut buffer = String::new();

        for line in dump_content.lines() {
            let trimmed_line = line.trim();

            if trimmed_line.is_empty() || trimmed_line.starts_with("--") {
                continue;
            }

            if self.ignore_drop_table.unwrap_or(false) {
                if trimmed_line.starts_with(MySQLKeywords::DropTable.as_str()) {
                    continue;
                }

                if trimmed_line.starts_with(MySQLKeywords::CreateTable.as_str()) {
                    let create_table_line = trimmed_line.replace(
                        MySQLKeywords::CreateTable.as_str(),
                        &format!(
                            "{} {}",
                            MySQLKeywords::CreateTable.as_str(),
                            MySQLKeywords::IfNotExists.as_str()
                        ),
                    );

                    buffer.push_str(&create_table_line);
                    continue;
                }
            }

            buffer.push_str(trimmed_line);
            buffer.push(' ');

            if trimmed_line.ends_with(");") || trimmed_line.ends_with(";") {
                let sql = buffer.trim();

                if !sql.is_empty() {
                    match conn.query_drop(sql) {
                        Ok(_) => {
                            if sql.to_uppercase().contains(MySQLKeywords::CreateTable.as_str()) {
                                let actual_table_name = if let Some(table_start) = sql.to_uppercase().find(MySQLKeywords::CreateTable.as_str()) {
                                    let after_create_table = &sql[table_start + 12..];
                                    let trimmed = after_create_table.trim();
                                    
                                    let table_part = if trimmed.to_uppercase().starts_with(MySQLKeywords::IfNotExists.as_str()) {
                                        trimmed[13..].trim()
                                    } else {
                                        trimmed
                                    };

                                    if let Some(backtick_start) = table_part.find('`') {
                                        if let Some(backtick_end) = table_part[backtick_start + 1..].find('`') {
                                            &table_part[backtick_start + 1..backtick_start + 1 + backtick_end]
                                        } else {
                                            table_part.split_whitespace().next().unwrap_or("unknown")
                                        }
                                    } else {
                                        table_part.split_whitespace().next().unwrap_or("unknown")
                                    }
                                } else {
                                    "unknown"
                                };

                                SuccessAlerts::table(actual_table_name);
                            }
                        }
                        Err(e) => ErrorsAlerts::import(dbname, sql, &e.to_string()),
                    }
                }

                buffer.clear();
            }
        }
    }

    pub fn dump_encrypted(&self) -> Result<(), Box<dyn Error>> {
        let pool = Connection {
            host: self.host.clone(),
            port: self.port,
            user: self.user.clone(),
            password: self.password.clone(),
            dbname: Some(self.dbname.clone()),
        }.create_mysql_pool()?;

        let mut conn = pool.get_conn()?;

        let dump_file_path = self.dump_file_path.as_ref().ok_or("dump_file_path is None")?;
        let decrypt = Encrypt::new(dump_file_path);
        let dump_content = String::from_utf8(decrypt.decrypt_and_read()?)?;

        let dump_content = ImportHandlers::new(&self.dbname, &dump_content).check_db_name();
        let _ = &self.process_statements(&mut conn, &dump_content, &self.dbname);

        SuccessAlerts::import(&self.dbname);
        Ok(())
    }

    pub fn dump_plain(&self) -> Result<(), Box<dyn Error>> {
        let pool = Connection {
            host: self.host.clone(),
            port: self.port,
            user: self.user.clone(),
            password: self.password.clone(),
            dbname: Some(self.dbname.clone()),
        }.create_mysql_pool()?;

        let mut conn = pool.get_conn()?;
        let is_compressed = self.dump_file_path.as_ref().map_or(false, |s| s.ends_with(".sql.gz"));

        let file = self.complete_path()?;

        let dump_content = if is_compressed {
            let file = File::open(file)?;
            let mut decoder = GzDecoder::new(BufReader::new(file));
            let mut content = String::new();
            decoder.read_to_string(&mut content)?;
            content
        } else {
            let dump_file_path = self.dump_file_path.as_ref().ok_or("dump_file_path is None")?;
            let mut file = File::open(dump_file_path)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            content
        };

        let dump_content = ImportHandlers::new(&self.dbname, &dump_content).check_db_name();
        let _ = &self.process_statements(&mut conn, &dump_content, &self.dbname);

        SuccessAlerts::import(&self.dbname);
        Ok(())
    }

    pub async fn dump_directly(&self) -> Result<(), Box<dyn Error>> {
        let pool = Connection {
            host: self.host.clone(),
            port: self.port,
            user: self.user.clone(),
            password: self.password.clone(),
            dbname: Some(self.dbname.clone()),
        }.create_mysql_pool()?;

        let mut conn = pool.get_conn()?;
        let sql_content = self.sql_content.as_deref().ok_or("sql_content is None")?;
        let dump_content = ImportHandlers::new(&self.dbname, sql_content).check_db_name();
        let _ = &self.process_statements(&mut conn, &dump_content, &self.dbname);

        Ok(())
    }

    pub fn dump(&self) -> Result<(), Box<dyn Error>> {
        let dump_file_path = self.dump_file_path.as_ref().ok_or("dump_file_path is None")?;

        if Entropy::new(dump_file_path.as_str()).calculate()? > 7.5 {
            let _ = self.dump_encrypted();
        } else {
            let _ =  self.dump_plain();
        }

        Ok(())
    }

}
