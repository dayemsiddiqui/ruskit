use crate::framework::database::DatabaseError;
use sqlx::{Pool, Sqlite};

pub struct SchemaBuilder {
    statements: Vec<String>,
}

impl SchemaBuilder {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }

    pub fn create_table<F>(&mut self, table_name: &str, callback: F) -> &mut Self 
    where
        F: FnOnce(&mut Table)
    {
        let mut table = Table::new(table_name);
        callback(&mut table);
        let sql = table.to_sql();
        println!("Generated table SQL: {}", sql);
        self.statements.push(sql);
        self
    }

    pub fn drop_table(&mut self, table_name: &str) -> &mut Self {
        let sql = format!("DROP TABLE IF EXISTS {}", table_name);
        println!("Generated drop SQL: {}", sql);
        self.statements.push(sql);
        self
    }

    pub fn to_sql(&self) -> String {
        let sql = self.statements.join(";\n");
        println!("Final schema SQL: {}", sql);
        sql
    }
}

pub struct Table {
    name: String,
    columns: Vec<Column>,
    foreign_keys: Vec<ForeignKey>,
}

impl Table {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            columns: Vec::new(),
            foreign_keys: Vec::new(),
        }
    }

    pub fn id(&mut self) -> &mut Self {
        self.columns.push(Column {
            name: "id".to_string(),
            sql_type: "INTEGER".to_string(),
            constraints: vec!["PRIMARY KEY AUTOINCREMENT".to_string()],
        });
        self
    }

    pub fn integer(&mut self, name: &str) -> &mut Column {
        let column = Column {
            name: name.to_string(),
            sql_type: "INTEGER".to_string(),
            constraints: Vec::new(),
        };
        self.columns.push(column);
        self.columns.last_mut().unwrap()
    }

    pub fn text(&mut self, name: &str) -> &mut Column {
        let column = Column {
            name: name.to_string(),
            sql_type: "TEXT".to_string(),
            constraints: Vec::new(),
        };
        self.columns.push(column);
        self.columns.last_mut().unwrap()
    }

    pub fn real(&mut self, name: &str) -> &mut Column {
        let column = Column {
            name: name.to_string(),
            sql_type: "REAL".to_string(),
            constraints: Vec::new(),
        };
        self.columns.push(column);
        self.columns.last_mut().unwrap()
    }

    pub fn boolean(&mut self, name: &str) -> &mut Column {
        let column = Column {
            name: name.to_string(),
            sql_type: "BOOLEAN".to_string(),
            constraints: Vec::new(),
        };
        self.columns.push(column);
        self.columns.last_mut().unwrap()
    }

    pub fn timestamps(&mut self) -> &mut Self {
        self.integer("created_at").not_null().default("CURRENT_TIMESTAMP");
        self.integer("updated_at").not_null().default("CURRENT_TIMESTAMP");
        self
    }

    pub fn timestamp_iso_strings(&mut self) -> &mut Self {
        let created_at = Column {
            name: "created_at".to_string(),
            sql_type: "TEXT".to_string(),
            constraints: vec![
                "NOT NULL".to_string(),
                "DEFAULT (datetime('now'))".to_string()
            ],
        };
        println!("Created timestamp column: {:?}", created_at);
        self.columns.push(created_at);

        let updated_at = Column {
            name: "updated_at".to_string(),
            sql_type: "TEXT".to_string(),
            constraints: vec![
                "NOT NULL".to_string(),
                "DEFAULT (datetime('now'))".to_string()
            ],
        };
        println!("Created timestamp column: {:?}", updated_at);
        self.columns.push(updated_at);
        self
    }

    pub fn foreign(&mut self, column: &str) -> ForeignKeyBuilder {
        ForeignKeyBuilder::new(self, column)
    }

    pub fn to_sql(&self) -> String {
        let mut sql = format!("CREATE TABLE {} (\n", self.name);
        println!("Creating table: {}", self.name);
        
        // Add columns
        let mut parts = Vec::new();
        for column in &self.columns {
            let column_sql = format!("    {}", column.to_sql());
            println!("Generated SQL for column: {}", column_sql);
            parts.push(column_sql);
        }
        
        // Add foreign keys
        for fk in &self.foreign_keys {
            let fk_sql = format!("    {}", fk.to_sql());
            println!("Generated SQL for foreign key: {}", fk_sql);
            parts.push(fk_sql);
        }
        
        sql.push_str(&parts.join(",\n"));
        sql.push_str("\n)");
        println!("Final table SQL: {}", sql);
        sql
    }
}

#[derive(Debug)]
pub struct Column {
    name: String,
    sql_type: String,
    constraints: Vec<String>,
}

impl Column {
    pub fn not_null(&mut self) -> &mut Self {
        self.constraints.push("NOT NULL".to_string());
        self
    }

    pub fn nullable(&mut self) -> &mut Self {
        // Remove NOT NULL if it exists
        if let Some(pos) = self.constraints.iter().position(|x| x == "NOT NULL") {
            self.constraints.remove(pos);
        }
        self
    }

    pub fn unique(&mut self) -> &mut Self {
        self.constraints.push("UNIQUE".to_string());
        self
    }

    pub fn default(&mut self, value: &str) -> &mut Self {
        self.constraints.push(format!("DEFAULT {}", value));
        self
    }

    pub fn to_sql(&self) -> String {
        if self.constraints.is_empty() {
            format!("{} {}", self.name, self.sql_type)
        } else {
            format!("{} {} {}", self.name, self.sql_type, self.constraints.join(" "))
        }
    }
}

pub struct ForeignKey {
    column: String,
    references_table: String,
    references_column: String,
    on_delete: Option<String>,
    on_update: Option<String>,
}

impl ForeignKey {
    pub fn to_sql(&self) -> String {
        let mut sql = format!(
            "FOREIGN KEY ({}) REFERENCES {}({})",
            self.column, self.references_table, self.references_column
        );
        
        if let Some(on_delete) = &self.on_delete {
            sql.push_str(&format!(" ON DELETE {}", on_delete));
        }
        
        if let Some(on_update) = &self.on_update {
            sql.push_str(&format!(" ON UPDATE {}", on_update));
        }
        
        sql
    }
}

pub struct ForeignKeyBuilder<'a> {
    table: &'a mut Table,
    foreign_key: ForeignKey,
}

impl<'a> ForeignKeyBuilder<'a> {
    fn new(table: &'a mut Table, column: &str) -> Self {
        Self {
            table,
            foreign_key: ForeignKey {
                column: column.to_string(),
                references_table: String::new(),
                references_column: String::new(),
                on_delete: None,
                on_update: None,
            },
        }
    }

    pub fn references(mut self, table: &str, column: &str) -> Self {
        self.foreign_key.references_table = table.to_string();
        self.foreign_key.references_column = column.to_string();
        self
    }

    pub fn on_delete(mut self, action: &str) -> Self {
        self.foreign_key.on_delete = Some(action.to_string());
        self
    }

    pub fn on_update(mut self, action: &str) -> Self {
        self.foreign_key.on_update = Some(action.to_string());
        self
    }

    pub fn build(self) -> &'a mut Table {
        self.table.foreign_keys.push(self.foreign_key);
        self.table
    }
} 