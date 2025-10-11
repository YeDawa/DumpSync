pub enum MySQLKeywords {
    DropTable,
}

impl MySQLKeywords {

    pub fn as_str(&self) -> &'static str {
        match self {
            MySQLKeywords::DropTable => "DROP TABLE",
        }
    }

}