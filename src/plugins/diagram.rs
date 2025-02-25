use std::error::Error;

use mysql::{
    *,
    prelude::*, 
};

use crate::{
    core::connection::Connection,

    handlers::{
        diagram_handlers::DiagramHandlers,
        queries_builders::MySqlQueriesBuilders,
    },
};

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

    pub async fn diagram(&self) -> Result<(), Box<dyn Error>> {
        let pool = Connection {
            host: self.host.clone(),
            port: self.port,
            user: self.user.clone(),
            password: self.password.clone(),
            dbname: Some(self.dbname.clone()),
        }.create_pool()?;
    
        let mut conn = pool.get_conn()?;
        let sql = MySqlQueriesBuilders.show_create_table(&self.table);
        let row: Option<(String, String)> = conn.query_first(sql)?;

        let table_sql: String = if let Some((_, create_table)) = row {
            create_table
        } else {
            return Err("No result found for the given table".into());
        };
    
        let table = DiagramHandlers.parse_show_create_table(&table_sql)?;
        let diagram = DiagramHandlers.generate_ascii_diagram_with_key(&table);
        println!("{}", diagram);
    
        Ok(())
    }    

}
