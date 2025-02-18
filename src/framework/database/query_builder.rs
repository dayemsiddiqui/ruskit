use sqlx::{FromRow, Row, sqlite::SqliteRow};
use crate::framework::database::{get_pool, DatabaseError};

pub struct QueryBuilder {
    table: String,
    conditions: Vec<String>,
    bindings: Vec<String>,
    select: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
    order_by: Option<(String, String)>,
}

impl QueryBuilder {
    pub fn table(table: &str) -> Self {
        Self {
            table: table.to_string(),
            conditions: Vec::new(),
            bindings: Vec::new(),
            select: None,
            limit: None,
            offset: None,
            order_by: None,
        }
    }

    pub fn select(mut self, columns: &str) -> Self {
        self.select = Some(columns.to_string());
        self
    }

    pub fn where_clause(mut self, column: &str, operator: &str, value: impl ToString) -> Self {
        self.conditions.push(format!("{} {} ?", column, operator));
        self.bindings.push(value.to_string());
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn order_by(mut self, column: &str, direction: &str) -> Self {
        self.order_by = Some((column.to_string(), direction.to_string()));
        self
    }

    pub async fn get<T>(self) -> Result<Vec<T>, DatabaseError> 
    where
        T: for<'r> FromRow<'r, SqliteRow> + Send + Unpin,
    {
        let pool = get_pool()?;
        let mut query = format!(
            "SELECT {} FROM {}",
            self.select.unwrap_or_else(|| "*".to_string()),
            self.table
        );

        if !self.conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&self.conditions.join(" AND "));
        }

        if let Some((column, direction)) = self.order_by {
            query.push_str(&format!(" ORDER BY {} {}", column, direction));
        }

        if let Some(limit) = self.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }

        if let Some(offset) = self.offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }

        let mut query_builder = sqlx::query_as(&query);
        for binding in self.bindings {
            query_builder = query_builder.bind(binding);
        }

        let results = query_builder.fetch_all(pool.as_ref()).await?;
        Ok(results)
    }

    pub async fn first<T>(self) -> Result<Option<T>, DatabaseError>
    where
        T: for<'r> FromRow<'r, SqliteRow> + Send + Unpin,
    {
        let results = self.limit(1).get::<T>().await?;
        Ok(results.into_iter().next())
    }

    pub async fn count(self) -> Result<i64, DatabaseError> {
        let pool = get_pool()?;
        let mut query = format!("SELECT COUNT(*) as count FROM {}", self.table);

        if !self.conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&self.conditions.join(" AND "));
        }

        let mut query_builder = sqlx::query(&query);
        for binding in self.bindings {
            query_builder = query_builder.bind(binding);
        }

        let result = query_builder
            .fetch_one(pool.as_ref())
            .await?
            .try_get::<i64, _>(0)?;

        Ok(result)
    }
} 