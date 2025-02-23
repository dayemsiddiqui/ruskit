use sea_orm::{DatabaseConnection, Statement, ConnectionTrait, TransactionTrait, DbBackend, ExecResult};
use serde_json::Value;
use std::future::Future;
use crate::framework::testing::{DatabaseAssertions, TransactionTest, TestCase};

impl DatabaseAssertions for TestCase {
    fn assert_database_has(&self, table: &str, data: Value) -> &Self {
        let exists = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async {
                let mut sql = format!("SELECT EXISTS(SELECT 1 FROM {} WHERE 1=1", table);
                let mut values = Vec::new();
                
                // Add where clauses for each key-value pair
                if let Some(obj) = data.as_object() {
                    for (key, value) in obj {
                        sql.push_str(&format!(" AND {} = ?", key));
                        values.push(value.to_string());
                    }
                }
                
                sql.push_str(") AS exists");
                
                let stmt = Statement::from_sql_and_values(
                    DbBackend::Sqlite,
                    &sql,
                    values.into_iter().map(|v| v.into()).collect::<Vec<_>>()
                );
                
                let result: ExecResult = self.db.execute(stmt).await.unwrap();
                result.rows_affected() > 0
            });
            
        assert!(
            exists,
            "Unable to find row in database table '{}' matching the attributes: {:?}",
            table,
            data
        );
        
        self
    }

    fn assert_database_missing(&self, table: &str, data: Value) -> &Self {
        let exists = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async {
                let mut sql = format!("SELECT EXISTS(SELECT 1 FROM {} WHERE 1=1", table);
                let mut values = Vec::new();
                
                // Add where clauses for each key-value pair
                if let Some(obj) = data.as_object() {
                    for (key, value) in obj {
                        sql.push_str(&format!(" AND {} = ?", key));
                        values.push(value.to_string());
                    }
                }
                
                sql.push_str(") AS exists");
                
                let stmt = Statement::from_sql_and_values(
                    DbBackend::Sqlite,
                    &sql,
                    values.into_iter().map(|v| v.into()).collect::<Vec<_>>()
                );
                
                let result: ExecResult = self.db.execute(stmt).await.unwrap();
                result.rows_affected() > 0
            });
            
        assert!(
            !exists,
            "Found unexpected row in database table '{}' matching the attributes: {:?}",
            table,
            data
        );
        
        self
    }

    fn assert_database_count(&self, table: &str, count: i64) -> &Self {
        let actual_count = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async {
                let stmt = Statement::from_sql_and_values(
                    DbBackend::Sqlite,
                    &format!("SELECT COUNT(*) FROM {}", table),
                    vec![]
                );
                
                let result: ExecResult = self.db.execute(stmt).await.unwrap();
                result.rows_affected() as i64
            });
            
        assert_eq!(
            actual_count,
            count,
            "Expected {} records in '{}' but found {}",
            count,
            table,
            actual_count
        );
        
        self
    }
}

impl TransactionTest for DatabaseConnection {
    fn run_in_transaction<F, Fut>(&self, test: F) -> Fut
    where
        F: FnOnce() -> Fut + Clone,
        Fut: Future<Output = ()>
    {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async {
                let tx = self.begin().await.unwrap();
                
                // Run the test
                test.clone()().await;
                
                // Always rollback after test
                tx.rollback().await.unwrap();
            });
            
        test()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[tokio::test]
    async fn test_database_assertions() {
        // This is just a basic example, in a real app you'd use your actual models
        let test_case = TestCase::new(axum::Router::new());
        
        // Insert test data
        QueryBuilder::table("users")
            .insert(json!({
                "name": "Test User",
                "email": "test@example.com"
            }))
            .await
            .unwrap();
            
        // Test assertions
        test_case
            .assert_database_has("users", json!({
                "email": "test@example.com"
            }))
            .assert_database_count("users", 1)
            .assert_database_missing("users", json!({
                "email": "nonexistent@example.com"
            }));
    }
} 