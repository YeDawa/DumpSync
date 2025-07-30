pub struct MySqlQueriesBuilders;

impl MySqlQueriesBuilders {    

    pub fn show_tables(&self) -> String {
        "SHOW TABLES".to_string()
    }

    pub fn truncate_table(&self, table: &str) -> String {
        format!("TRUNCATE TABLE `{}`;", table)
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

    pub fn table_info(&self, table: &str) -> String {
        format!(
            r#"
            SELECT COLUMN_NAME, DATA_TYPE, IS_NULLABLE, COLUMN_KEY
            FROM INFORMATION_SCHEMA.COLUMNS
            WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = '{}';
            "#,
            table
        )
    }

    pub fn foreign_key_info(&self, table: &str) -> String {
        format!(
            r#"
            SELECT COLUMN_NAME, REFERENCED_TABLE_NAME, REFERENCED_COLUMN_NAME
            FROM INFORMATION_SCHEMA.KEY_COLUMN_USAGE
            WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = '{}' AND REFERENCED_TABLE_NAME IS NOT NULL;
            "#,
            table
        )
    }

    pub fn get_alter_table(&self, table: &str) -> String {
        format!(
            r#"
            SELECT CONSTRAINT_NAME, COLUMN_NAME, REFERENCED_TABLE_NAME, REFERENCED_COLUMN_NAME 
            FROM information_schema.KEY_COLUMN_USAGE 
            WHERE TABLE_NAME = DATABASE() AND TABLE_SCHEMA = '{}'
            AND REFERENCED_TABLE_NAME IS NOT NULL
            "#,
            table
        )
    }

    pub fn get_foreign_keys(&self, table: &str, constraint_name: &str, column_name: &str, ref_table: &str, ref_column: &str) -> String {
        format!(
            "ALTER TABLE `{}` ADD CONSTRAINT `{}` FOREIGN KEY (`{}`) REFERENCES `{}` (`{}`);",
            table, constraint_name, column_name, ref_table, ref_column
        )
    }
    
    pub fn get_unique_keys(&self, table: &str, constraint_name: &str, column_name: &str) -> String {
        format!(
            "ALTER TABLE `{}` ADD CONSTRAINT `{}` UNIQUE (`{}`);",
            table, constraint_name, column_name
        )
    }

    pub fn show_columns(&self, table: &str) -> String {
        format!("SHOW COLUMNS FROM `{}`", table)
    }

    pub fn insert_into_start(&self, table: &str, columns: &[String], insert_ignore: bool) -> String {
        let cmd = if insert_ignore { "INSERT IGNORE INTO" } else { "INSERT INTO" };
        format!("{} `{}` ({}) VALUES ", cmd, table, columns.join(", "))
    }
    
}
