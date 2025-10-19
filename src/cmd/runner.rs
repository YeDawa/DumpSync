use mysql::{
    *,
    prelude::*
};

use crate::{
    ui::{
        errors_alerts::ErrorsAlerts,
        success_alerts::SuccessAlerts,
    },

    handlers::{
        syntax_skip_handlers::SyntaxSkip,
        mysql::mysql_keywords::MySQLKeywords,
    },
};

pub struct Runner;

impl Runner {

    pub fn import(&self, conn: &mut PooledConn, dump_content: &str, dbname: &str, ignore_drop_table: Option<bool>) {
        let mut buffer = String::new();
        let mut tables_ignored = Vec::new();

        for line in dump_content.lines() {
            let trimmed_line = line.trim();

            if trimmed_line.contains(SyntaxSkip::SkipTables.as_str()) {
                let table_names = if let Some(start) = trimmed_line.find('"') {
                    if let Some(end) = trimmed_line[start + 1..].find('"') {
                        &trimmed_line[start + 1..start + 1 + end]
                    } else {
                        continue;
                    }
                } else {
                    continue;
                };

                if table_names == "unknown" || table_names.is_empty() {
                    continue;
                }

                for table_name in table_names.split(',') {
                    tables_ignored.push(table_name.trim().to_string());
                }
                
                continue;
            }

            if trimmed_line.is_empty() || trimmed_line.starts_with(MySQLKeywords::Comments.as_str()) {
                continue;
            }

            if trimmed_line.contains(SyntaxSkip::SkipLine.as_str()) {
                continue;
            }

            if ignore_drop_table.unwrap_or(false) {
                if trimmed_line.starts_with(MySQLKeywords::DropTable.as_str()) {
                    continue;
                }

                if trimmed_line.starts_with(MySQLKeywords::CreateTable.as_str()) {
                    let create_table_line = trimmed_line.replace(
                        MySQLKeywords::CreateTable.as_str(),
                        &format!(
                            "{} {}",
                            MySQLKeywords::CreateTable.as_str(),
                            MySQLKeywords::IfNotExists.as_str()
                        ),
                    );

                    buffer.push_str(&create_table_line);
                    continue;
                }
            }

            buffer.push_str(trimmed_line);
            buffer.push(' ');

            if trimmed_line.ends_with(");") || trimmed_line.ends_with(";") {
                let sql = buffer.trim();

                if !sql.is_empty() {
                    let (should_skip, table_name) = if sql.to_uppercase().contains(MySQLKeywords::Insert.as_str()) || sql.to_uppercase().contains(MySQLKeywords::CreateTable.as_str()) {
                        let found_table = tables_ignored.iter().find(|table| {
                            sql.contains(&format!("`{}`", table))
                        });
                        
                        if let Some(table) = found_table {
                            (true, table.clone())
                        } else {
                            (false, String::new())
                        }
                    } else {
                        (false, String::new())
                    };

                    if should_skip {
                        SuccessAlerts::table_ignored(&table_name);
                        buffer.clear();
                        continue;
                    }

                    match conn.query_drop(sql) {
                        Ok(_) => {
                            if sql.to_uppercase().contains(MySQLKeywords::CreateTable.as_str()) {
                                let actual_table_name = if let Some(table_start) = sql.to_uppercase().find(MySQLKeywords::CreateTable.as_str()) {
                                    let trimmed = &sql[table_start + 12..].trim();
                                    
                                    let table_part = if trimmed.to_uppercase().starts_with(MySQLKeywords::IfNotExists.as_str()) {
                                        trimmed[13..].trim()
                                    } else {
                                        trimmed
                                    };

                                    if let Some(backtick_start) = table_part.find('`') {
                                        if let Some(backtick_end) = table_part[backtick_start + 1..].find('`') {
                                            &table_part[backtick_start + 1..backtick_start + 1 + backtick_end]
                                        } else {
                                            table_part.split_whitespace().next().unwrap_or("unknown")
                                        }
                                    } else {
                                        table_part.split_whitespace().next().unwrap_or("unknown")
                                    }
                                } else {
                                    "unknown"
                                };

                                SuccessAlerts::table(actual_table_name);
                            }
                        }
                        Err(e) => ErrorsAlerts::import(dbname, sql, &e.to_string()),
                    }
                }

                buffer.clear();
            }
        }
    }

}
