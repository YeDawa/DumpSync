pub struct RegExp;

impl RegExp {
    
    pub const USE_CASE: &'static str = r"(?i)(USE\s+`?)(\w+)(`?)";
    pub const CREATE_TABLE: &'static str = r"(?i)CREATE TABLE\s+`?(\w+)`?";
    pub const CREATE_TABLE_INSERTS: &'static str = r"(?i)\b(?:CREATE\s+TABLE|INSERT\s+INTO)\s+`?(\w+)`?";
    pub const CREATE_DATABASE_CASES: &'static str = r"(?i)CREATE DATABASE\s+(`?)(\w+)(`?)\s*(IF NOT EXISTS)?;";
    
}
