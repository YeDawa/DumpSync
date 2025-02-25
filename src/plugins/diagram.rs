use regex::Regex;
use std::error::Error;

use mysql::{
    *,
    prelude::*, 
};

use crate::{
    core::connection::Connection,
    handlers::queries_builders::MySqlQueriesBuilders,
};

#[derive(Debug)]
pub struct ColumnDefinition {
    pub name: String,
    pub col_type: String,
    pub key: Option<String>,
}

#[derive(Debug)]
pub struct TableDefinition {
    pub name: String,
    pub columns: Vec<ColumnDefinition>,
}

pub struct Diagram {
    host: String,
    port: u16,
    user: String,
    password: String,
    dbname: String,
    table: String,
}

impl Diagram {

    pub fn new(
        host: &str,
        port: u16,
        user: &str,
        password: &str,
        dbname: &str,
        table: &str,
    ) -> Self {
        Self {
            host: host.to_string(),
            port,
            user: user.to_string(),
            password: password.to_string(),
            dbname: dbname.to_string(),
            table: table.to_string(),
        }
    }

    fn parse_show_create_table(&self, sql: &str) -> Result<TableDefinition, Box<dyn Error>> {
        let table_name_re = Regex::new(r"(?i)CREATE\s+TABLE\s+`?(\w+)`?\s*\(")?;
        let table_name_caps = table_name_re
            .captures(sql)
            .ok_or("Table name not found")?;

        let table_name = table_name_caps.get(1).unwrap().as_str().to_string();
    
        let start = sql.find('(').ok_or("Opening parenthesis not found")?;
        let end = sql.rfind(')').ok_or("Closing parenthesis not found")?;
        let columns_str = &sql[start + 1..end];

        let column_lines: Vec<&str> = columns_str
            .lines()
            .map(|s| s.trim().trim_end_matches(','))
            .filter(|s| !s.is_empty())
            .collect();
    
        let mut columns = Vec::new();
        let mut constraints = Vec::new();
        let column_re = Regex::new(r"(?i)^`?(\w+)`?\s+([^\s]+)(.*)$")?;
        
        for line in &column_lines {
            let line_upper = line.to_uppercase();

            if line_upper.starts_with("PRIMARY KEY") || line_upper.starts_with("FOREIGN KEY") || line_upper.starts_with("KEY") || line_upper.starts_with("CONSTRAINT") {
                constraints.push(*line);
                continue;
            }
    
            if let Some(caps) = column_re.captures(line) {
                let col_name = caps.get(1).unwrap().as_str().to_string();
                let col_type = caps.get(2).unwrap().as_str().to_string();
                
                columns.push(ColumnDefinition {
                    name: col_name,
                    col_type,
                    key: None,
                });
            }
        }
    
        let cols_in_constraint_re = Regex::new(r"\(([^)]+)\)")?;
        for cons_line in constraints {
            let cons_line_upper = cons_line.to_uppercase();

            if let Some(caps) = cols_in_constraint_re.captures(cons_line) {
                let cols_str = caps.get(1).unwrap().as_str();

                let col_names: Vec<&str> = cols_str
                    .split(',')
                    .map(|s| s.trim().trim_matches('`'))
                    .collect();

                for col in col_names {
                    for column in columns.iter_mut() {
                        if column.name == col {
                            if cons_line_upper.starts_with("PRIMARY KEY") {
                                column.key = Some("PK".to_string());
                            } else if cons_line_upper.contains("FOREIGN KEY") || cons_line_upper.contains("REFERENCES") {
                                column.key = Some("FK".to_string());
                            } else if cons_line_upper.starts_with("KEY") && column.key.is_none() {
                                column.key = Some("KEY".to_string());
                            }
                        }
                    }
                }
            }
        }
    
        Ok(TableDefinition {
            name: table_name,
            columns,
        })
    }    

    fn generate_ascii_diagram_with_key(&self, table: &TableDefinition) -> String {
        let header_col = "Column";
        let header_type = "Type";
        let header_key = "Key";
    
        let col1_width = table.columns
            .iter()
            .map(|col| col.name.len())
            .max()
            .unwrap_or(0)
            .max(header_col.len());

        let col2_width = table.columns
            .iter()
            .map(|col| col.col_type.len())
            .max()
            .unwrap_or(0)
            .max(header_type.len());

        let col3_width = table.columns
            .iter()
            .map(|col| col.key.as_ref().map(|s| s.len()).unwrap_or(0))
            .max()
            .unwrap_or(0)
            .max(header_key.len());
    
        let border_line = format!(
            "+-{:-<width1$}-+-{:-<width2$}-+-{:-<width3$}-+",
            "",
            "",
            "",
            width1 = col1_width,
            width2 = col2_width,
            width3 = col3_width
        );
    
        let header_row = format![
            "| {:<width1$} | {:<width2$} | {:<width3$} |",
            header_col,
            header_type,
            header_key,
            width1 = col1_width,
            width2 = col2_width,
            width3 = col3_width
        ];
    
        let mut diagram = String::new();
        diagram.push_str(&format!("TABLE: {}\n", table.name));
        diagram.push_str(&border_line);
        diagram.push('\n');
        diagram.push_str(&header_row);
        diagram.push('\n');
        diagram.push_str(&border_line);
        diagram.push('\n');
    
        for col in &table.columns {
            let key_text = col.key.clone().unwrap_or_default();
            let row = format!(
                "| {:<width1$} | {:<width2$} | {:<width3$} |",
                col.name,
                col.col_type,
                key_text,
                width1 = col1_width,
                width2 = col2_width,
                width3 = col3_width
            );
            diagram.push_str(&row);
            diagram.push('\n');
        }
        
        diagram.push_str(&border_line);
        diagram
    }    

    pub async fn diagram(&self) -> Result<(), Box<dyn Error>> {
        let pool = Connection {
            host: self.host.clone(),
            port: self.port,
            user: self.user.clone(),
            password: self.password.clone(),
            dbname: Some(self.dbname.clone()),
        }
        .create_pool()?;
    
        let mut conn = pool.get_conn()?;
        let sql = MySqlQueriesBuilders.show_create_table(&self.table);
        let row: Option<(String, String)> = conn.query_first(sql)?;

        let table_sql: String = if let Some((_, create_table)) = row {
            create_table
        } else {
            return Err("No result found for the given table".into());
        };
    
        let table = self.parse_show_create_table(&table_sql)?;
        let diagram = self.generate_ascii_diagram_with_key(&table);
        println!("{}", diagram);
    
        Ok(())
    }    

}
