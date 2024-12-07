use regex::Regex;

pub struct TransferHandlers {
    dbname: String,
    dump_content: String,
}

impl TransferHandlers {

    pub fn new(dbname: &str, dump_content: &str) -> Self {
        Self {
            dbname: dbname.to_string(),
            dump_content: dump_content.to_string(),
        }
    }

    pub fn check_db_name(&self) -> String {
        let create_db_regex = Regex::new(r"(?i)(CREATE DATABASE IF NOT EXISTS\s+`?)(\w+)(`?)").unwrap();
        let use_db_regex = Regex::new(r"(?i)(USE\s+`?)(\w+)(`?)").unwrap();
    
        let content = create_db_regex.replace_all(&self.dump_content, |caps: &regex::Captures| {
            if &caps[2] != &self.dbname {
                format!("{}{}{};", &caps[1], &self.dbname, &caps[3])
            } else {
                caps[0].to_string()
            }
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