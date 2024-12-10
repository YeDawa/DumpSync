use regex::Regex;
use flate2::read::GzDecoder;

use mysql::{
    *,
    prelude::*
};

use std::{
    fs::File, 
    error::Error,

    path::{
        Path, 
        PathBuf
    },

    io::{
        Read,
        BufReader, 
    }, 
};

use crate::{
    consts::regexp::RegExp,
    engine::connection::Connection,
    handlers::import_handlers::ImportHandlers,

    ui::{
        errors_alerts::ErrorsAlerts,
        success_alerts::SuccessAlerts,
    },
};

pub struct Import {
    host: String,
    port: u16,
    user: String,
    password: String,
    dbname: String,
    path: String,
    dump_file_path: String,
}

impl Import {

    pub fn new(host: &str, port: u16, user: &str, password: &str, dbname: &str, dump_file_path: &str, path: &str) -> Self {
        Self {
            host: host.to_string(),
            port,
            user: user.to_string(),
            password: password.to_string(),
            dbname: dbname.to_string(),
            path: path.to_string(),
            dump_file_path: dump_file_path.to_string(),
        }
    }

    fn complete_path(&self) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let path = Path::new(&self.dump_file_path);

        if path.is_absolute() {
            Ok(path.to_path_buf())
        } else {
            let dump_file_path = Path::new(&self.dump_file_path);
            Ok(dump_file_path.join(&self.path))
        }
    }

    pub fn dump(&self) -> Result<(), Box<dyn Error>> {
        let pool = Connection {
            host: self.host.clone(),
            port: self.port,
            user: self.user.clone(),
            password: self.password.clone(),
            dbname: Some(self.dbname.clone()),
        }.create_pool()?;

        let mut conn = pool.get_conn()?;
        let is_compressed = self.dump_file_path.ends_with(".sql.gz");

        let file = self.complete_path()?;

        let dump_content = if is_compressed {
            let file = File::open(file)?;

            let mut decoder = GzDecoder::new(BufReader::new(file));
            let mut content = String::new();

            decoder.read_to_string(&mut content)?;
            content
        } else {
            let mut file = File::open(&self.dump_file_path)?;
            let mut content = String::new();

            file.read_to_string(&mut content)?;
            content
        };

        let dump_content = ImportHandlers::new(&self.dbname, &dump_content).check_db_name();

        let create_table_regex = Regex::new(RegExp::CREATE_TABLE).unwrap();

        for statement in dump_content.split(';') {
            let trimmed = statement.trim();

            if !trimmed.is_empty() {
                match conn.query_drop(trimmed) {
                    Ok(_) => {
                        if let Some(captures) = create_table_regex.captures(trimmed) {
                            if let Some(table_name) = captures.get(1) {
                                SuccessAlerts::table(table_name.as_str());
                            }
                        }
                    }
                    Err(e) => ErrorsAlerts::import(&self.dbname, trimmed, &e.to_string()),
                }
            }
        }

        SuccessAlerts::import(&self.dbname);
        Ok(())
    }

}
