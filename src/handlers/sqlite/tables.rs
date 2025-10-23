use crate::handlers::sqlite::tables_names::TablesNames;

pub enum Table {
    FileName,
}

pub struct Tables;

impl Tables {

    pub fn as_str(&self, table: Table) -> &'static str {
        match table {
            Table::FileName => "history.db",
        }
    }

    pub fn history(&self) -> String {
        format!("CREATE TABLE IF NOT EXISTS {} (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            slug TEXT NOT NULL,
            db TEXT NOT NULL,
            host TEXT NOT NULL,
            filename TEXT NOT NULL,
            compress BOOLEAN NOT NULL,
            encrypt BOOLEAN NOT NULL,
            size INTEGER NOT NULL,
            created_at TEXT NOT NULL
        )", TablesNames::Backups.as_str())
    }

}