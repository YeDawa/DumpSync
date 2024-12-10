use regex::Regex;

use crate::consts::regexp::RegExp;

pub struct ImportHandlers {
    dbname: String,
    dump_content: String,
}

impl ImportHandlers {

    pub fn new(dbname: &str, dump_content: &str) -> Self {
        Self {
            dbname: dbname.to_string(),
            dump_content: dump_content.to_string(),
        }
    }

    pub fn check_db_name(&self) -> String {
        let use_db_regex = Regex::new(RegExp::USE_CASE).unwrap();
        let db_regex = Regex::new(RegExp::CREATE_DATABASE_CASES).unwrap();
        
        let content = db_regex.replace_all(&self.dump_content, |caps: &regex::Captures| {
            let db_name = if &caps[2] != self.dbname {
                self.dbname.clone()
            } else {
                caps[2].to_string()
            };
    
            format!("CREATE DATABASE IF NOT EXISTS `{}`;", db_name)
        });

        let dump_content = use_db_regex.replace_all(&content, |caps: &regex::Captures| {
            if &caps[2] != &self.dbname {
                format!("{}{}{};", &caps[1], &self.dbname, &caps[3])
            } else {
                caps[0].to_string()
            }
        });
    
        dump_content.to_string()
    }
    
}