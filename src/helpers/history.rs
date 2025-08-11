use rusqlite::{
    params, 
    Result,
    Connection, 
};

use crate::constants::{
    global::Global,
    folders::Folders, 
};

pub struct History {
    db_path: String,
}

impl History {

    pub fn new() -> Self {
        History {
            db_path: Folders::APP_FOLDER.join(Global::DB_HISTORY_FILE).to_string_lossy().to_string(),
        }
    }

    pub fn init_db(&self) -> Result<()> {
        let conn = Connection::open(&self.db_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS backups (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                slug TEXT NOT NULL,
                db TEXT NOT NULL,
                host TEXT NOT NULL,
                filename TEXT NOT NULL,
                compress BOOLEAN NOT NULL,
                encrypt BOOLEAN NOT NULL,
                size INTEGER NOT NULL,
                created_at TEXT NOT NULL
            )",
            [],
        )?;
        
        Ok(())
    }

    pub fn insert_backup(&self, slug: &str, filename: &str, db: &str, host: &str, created_at: &str, size: i64, encrypt: bool, compress: bool) -> Result<()> {
        let conn = Connection::open(&self.db_path)?;
        conn.execute(
            "INSERT INTO backups (slug, filename, db, host, created_at, size, encrypt, compress) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![slug, filename, db, host, created_at, size, encrypt, compress],
        )?;
    
        Ok(())
    }

}