pub enum TablesNames {
    Backups,
}

impl TablesNames {

    pub fn as_str(&self) -> &'static str {
        match self {
            TablesNames::Backups => "backups",
        }
    }

}