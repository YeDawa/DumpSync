pub struct MySqlQueriesBuilders;

impl MySqlQueriesBuilders {    

    pub fn show_tables(&self) -> String {
        "SHOW TABLES".to_string()
    }

    pub fn show_create_table(&self, table: &str) -> String {
        format!("SHOW CREATE TABLE `{}`;", table)
    }

    pub fn drop_table(&self, table: &str) -> String {
        format!("DROP TABLE IF EXISTS `{}`;", table)
    }

    pub fn create_database_not_exists(&self, dbname: &str) -> String {
        format!("CREATE DATABASE IF NOT EXISTS `{}`;", dbname)
    }

    pub fn create_database(&self, dbname: &str) -> Result<(String, String), String> {
        let create_db = format!("CREATE DATABASE IF NOT EXISTS `{}`;\n", dbname);
        let use_db = format!("USE `{}`;", dbname);

        Ok((create_db, use_db))
    }

    pub fn insert_into(&self, table: &str, values: Vec<String>, ignore: bool) -> String {
        let insert_ignore = if ignore { "INSERT IGNORE INTO" } else { "INSERT INTO" };

        format!(
            "{} `{}` VALUES\n{};",
            insert_ignore,
            table,
            values.join(",\n")
        )
    }

    pub fn select(&self, table: &str, offset: Option<usize>, limit: Option<usize>) -> String {
        let mut query = format!("SELECT * FROM `{}`", table);

        if let Some(l) = limit {
            query.push_str(&format!(" LIMIT {}", l));
        }

        if let Some(o) = offset {
            query.push_str(&format!(" OFFSET {}", o));
        }

        query
    }

    pub fn get_table_names(&self) -> String {
        "SELECT TABLE_NAME FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_SCHEMA = DATABASE();".to_string()
    }

    pub fn table_info(&self, table_name: &str) -> String {
        format!(
            r#"
            SELECT COLUMN_NAME, DATA_TYPE, IS_NULLABLE, COLUMN_KEY
            FROM INFORMATION_SCHEMA.COLUMNS
            WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = '{}';
            "#,
            table_name
        )
    }

    pub fn foreign_key_info(table_name: &str) -> String {
        format!(
            r#"
            SELECT COLUMN_NAME, REFERENCED_TABLE_NAME, REFERENCED_COLUMN_NAME
            FROM INFORMATION_SCHEMA.KEY_COLUMN_USAGE
            WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = '{}' AND REFERENCED_TABLE_NAME IS NOT NULL;
            "#,
            table_name
        )
    }
    
}
