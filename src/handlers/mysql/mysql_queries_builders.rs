use crate::handlers::mysql::mysql_keywords::MySQLKeywords;

pub struct MySqlQueriesBuilders;

impl MySqlQueriesBuilders {    

    pub fn use_db(&self, dbname: &str) -> String {
        format!("{} `{}`;", MySQLKeywords::Use.as_str(), dbname)
    }

    pub fn show_tables(&self) -> String {
        MySQLKeywords::ShowTables.as_str().to_string()
    }

    pub fn lock_tables(&self, table: &str) -> String {
        format!("{} `{}`;", MySQLKeywords::LockTables.as_str(), table)
    }

    pub fn unlock_tables(&self, table: &str) -> String {
        format!("{} `{}`;", MySQLKeywords::UnlockTables.as_str(), table)
    }

    pub fn truncate_table(&self, table: &str) -> String {
        format!("{} `{}`;", MySQLKeywords::TruncateTable.as_str(), table)
    }

    pub fn show_create_table(&self, table: &str) -> String {
        format!("{} `{}`;", MySQLKeywords::ShowCreateTable.as_str(), table)
    }

    pub fn drop_table(&self, table: &str) -> String {
        format!("{} {} `{}`;", MySQLKeywords::DropTable.as_str(), MySQLKeywords::IfExists.as_str(), table)
    }

    pub fn create_database_not_exists(&self, dbname: &str) -> String {
        format!("{} {} `{}`;", MySQLKeywords::CreateDatabase.as_str(), MySQLKeywords::IfNotExists.as_str(), dbname)
    }

    pub fn get_table_names(&self) -> String {
        format!("{};", MySQLKeywords::GetTableNames.as_str())
    }

    pub fn show_columns(&self, table: &str) -> String {
        format!("{} `{}`;", MySQLKeywords::ShowColumns.as_str(), table)
    }

    pub fn table_info(&self, table: &str) -> String {
        format!("{} = '{}'", MySQLKeywords::TableInfo.as_str(), table)
    }

    pub fn foreign_key_info(&self, table: &str) -> String {
        format!("{} = '{}' {};", MySQLKeywords::ForeignKeyInfo.as_str(), table, MySQLKeywords::AndReferencedIsNotNull.as_str())
    }

    pub fn get_alter_table(&self, table: &str) -> String {
        format!("{} = '{}' {}", MySQLKeywords::GetAlterTable.as_str(), table, MySQLKeywords::AndReferencedIsNotNull.as_str())
    }

    pub fn create_database(&self, dbname: &str) -> Result<(String, String), String> {
        let create_db = format!(
            "{} {} `{}`;\n", MySQLKeywords::CreateDatabase.as_str(), MySQLKeywords::IfNotExists.as_str(), dbname
        );

        let use_db = format!("{} `{}`;", MySQLKeywords::Use.as_str(), dbname);
        Ok((create_db, use_db))
    }

    pub fn select(&self, table: &str, offset: Option<usize>, limit: Option<usize>) -> String {
        let mut query = format!("{} `{}`", MySQLKeywords::SelectFrom.as_str(), table);

        if let Some(l) = limit {
            query.push_str(&format!(" {} {}", MySQLKeywords::Limit.as_str(), l));
        }

        if let Some(o) = offset {
            query.push_str(&format!(" {} {}", MySQLKeywords::Offset.as_str(), o));
        }

        query
    }

    pub fn get_foreign_keys(&self, table: &str, constraint_name: &str, column_name: &str, ref_table: &str, ref_column: &str) -> String {
        format!(
            "{} `{}` {} `{}` {} (`{}`) {} `{}` (`{}`);",
            MySQLKeywords::AlterTable.as_str(),
            table, 
            MySQLKeywords::AddConstraint.as_str(),
            constraint_name, 
            MySQLKeywords::ForeignKey.as_str(),
            column_name, 
            MySQLKeywords::References.as_str(),
            ref_table, 
            ref_column
        )
    }
    
    pub fn get_unique_keys(&self, table: &str, constraint_name: &str, column_name: &str) -> String {
        format!(
            "{} `{}` {} `{}` {} (`{}`);",
            MySQLKeywords::AlterTable.as_str(), 
            table, 
            MySQLKeywords::AddConstraint.as_str(), 
            constraint_name, 
            MySQLKeywords::Unique.as_str(), 
            column_name
        )
    }
    
    pub fn get_primary_key(&self, table: &str) -> String {
        format!("{} `{}` {}", MySQLKeywords::ShowKeysFrom.as_str(), table, MySQLKeywords::WherePrimaryKey.as_str())
    }

    pub fn insert_into_start(&self, table: &str, columns: &[String], values: &[String], insert_ignore: bool) -> String {
        let prefix = if insert_ignore { 
            MySQLKeywords::InsertIgnore.as_str() 
        } else { 
            MySQLKeywords::InsertInto.as_str() 
        };

        format!(
            "{} `{}` ({}) {} ({}) {} id=id;",
            prefix,
            table,
            columns.join(", "),
            MySQLKeywords::Values.as_str(),
            values.join(", "),
            MySQLKeywords::OnDuplicateKeyUpdate.as_str()
        )
    }

}