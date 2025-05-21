use regex::Regex;
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
    constants::regexp::RegExp,
    handlers::import_handlers::ImportHandlers,

    ui::{
        errors_alerts::ErrorsAlerts,
        success_alerts::SuccessAlerts,
    },

    core::{
        encrypt::Encrypt,
        connection::Connection,
    },
};

pub struct Import {
    host: String,
    port: u16,
    user: String,
    password: String,
    dbname: String,
    path: Option<String>,
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
        dump_file_path: Option<&str>, 
        path: Option<&str>,
        sql_content: Option<&str>,
    ) -> Self {
        Self {
            host: host.to_string(),
            port,
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
        let create_table_regex = Regex::new(RegExp::CREATE_TABLE).unwrap();
        let mut buffer = String::new();

        for line in dump_content.lines() {
            let trimmed_line = line.trim();

            if trimmed_line.is_empty() || trimmed_line.starts_with("--") {
                continue;
            }

            buffer.push_str(trimmed_line);
            buffer.push(' ');

            if trimmed_line.ends_with(");") || trimmed_line.ends_with(";") {
                let sql = buffer.trim();

                if !sql.is_empty() {
                    match conn.query_drop(sql) {
                        Ok(_) => {
                            if let Some(captures) = create_table_regex.captures(sql) {
                                if let Some(table_name) = captures.get(1) {
                                    SuccessAlerts::table(table_name.as_str());
                                }
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
        }.create_pool()?;

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
        }.create_pool()?;

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
        }.create_pool()?;

        let mut conn = pool.get_conn()?;
        let sql_content = self.sql_content.as_deref().ok_or("sql_content is None")?;
        let dump_content = ImportHandlers::new(&self.dbname, sql_content).check_db_name();
        let _ = &self.process_statements(&mut conn, &dump_content, &self.dbname);

        Ok(())
    }

    pub fn dump(&self) -> Result<(), Box<dyn Error>> {
        let dump_file_path = self.dump_file_path.as_ref().ok_or("dump_file_path is None")?;
        if Encrypt::new(dump_file_path.as_str()).calculate_entropy()? > 7.5 {
            let _ = self.dump_encrypted();
        } else {
            let _ =  self.dump_plain();
        }

        Ok(())
    }

}
