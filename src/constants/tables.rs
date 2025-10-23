pub struct Tables;

impl Tables {

    pub fn history(&self) -> &'static str {
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
        )"
    }

}