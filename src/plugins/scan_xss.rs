use std::error::Error;

use mysql::{
    *,
    prelude::*, 
};

use crate::{
    core::connection::Connection,
    plugins::reports_xss::ReportsXSS,
    
    ui::{
        ui_base::UI,
        scan_alerts::ScanAlerts,
    },

    handlers::mysql::{
        scan_handlers::ScanHandlers,
        mysql_queries_builders::MySqlQueriesBuilders,
    }
};

pub struct ScanXSS {
    host: String,
    port: u16,
    user: String,
    password: String,
    dbname: String,

    table: String,
    payload: Option<String>,
    offset: Option<u64>,
    limit: Option<u64>,
    file: Option<String>,
}

impl ScanXSS {

    pub fn new(
        host: &str,
        port: u16,
        user: &str,
        password: &str,
        dbname: &str,

        table: &str,
        payload: Option<&str>,
        offset: Option<u64>,
        limit: Option<u64>,
        file: Option<&str>,
    ) -> Self {
        Self {
            host: host.to_string(),
            port,
            user: user.to_string(),
            password: password.to_string(),
            dbname: dbname.to_string(),

            table: table.to_string(),
            payload: payload.map(|s| s.to_string()),
            offset,
            limit,
            file: file.map(|s| s.to_string()),
        }
    }

    pub async fn scan(&self) -> Result<(), Box<dyn Error>> {
        let pool = Connection {
            host: self.host.clone(),
            port: self.port,
            user: self.user.clone(),
            password: self.password.clone(),
            dbname: Some(self.dbname.clone()),
        }.create_mysql_pool()?;
    
        let mut conn = pool.get_conn()?;
        let patterns = ScanHandlers.read_patterns(self.payload.clone()).await?;
        let mut detections = Vec::new();
    
        let tables: Vec<&str> = self.table.split(',')
            .map(|t| t.trim())
            .filter(|t| !t.is_empty())
            .collect();
    
        for table in &tables {
            let mut xss_count = 0;

            if tables.len() > 1 {
                let text = format!("Table: '{}'", table);
                UI::label(&text, "info");
            }

            let query = MySqlQueriesBuilders.select(table, self.offset.map(|o| o as usize), self.limit.map(|l| l as usize));
            let rows: Vec<Row> = conn.query(query)?;
    
            for (row_index, row) in rows.iter().enumerate() {
                for (col_index, column) in row.columns_ref().iter().enumerate() {
                    let value: Option<String> = row.get(col_index);
    
                    if let Some(value_str) = value.as_ref() {
                        if ScanHandlers.is_potential_xss(value_str, &patterns) {
                            let row_index = row_index + 1;
                            let column = column.name_str();
                            ScanAlerts::detected(table, row_index, &column, &value_str);
    
                            detections.push((
                                table.to_string(),
                                row_index,
                                column.to_string(),
                                value_str.to_string(),
                            ));

                            xss_count += 1;
                        }
                    }
                }
            }

            if xss_count == 0 {
                ScanAlerts::not_detected(table);
            }

            if tables.len() > 1 {
                print!("\n");
            }
        }
    
        let file_path = self.file.as_deref();
        ReportsXSS.autodetect(detections, file_path)?;
        Ok(())
    }    
    
}
