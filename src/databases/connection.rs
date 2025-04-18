use sea_orm::{Database, DatabaseConnection};
use std::sync::Arc;
use anyhow::Result;

#[derive(Clone)]
pub struct DatabaseManager {
    conn: Arc<DatabaseConnection>,
}

impl DatabaseManager {
    pub async fn new(database_url: &str) -> Result<Self> {
        let conn = Database::connect(database_url).await?;
        Ok(Self {
            conn: Arc::new(conn),
        })
    }

    pub fn get_connection(&self) -> Arc<DatabaseConnection> {
        self.conn.clone()
    }
}

// 实现默认的数据库URL
impl Default for DatabaseManager {
    fn default() -> Self {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/crypto_db".to_string());
        
        // 注意：在实际使用时，应该使用异步运行时来初始化
        // 这里使用 Default trait 只是为了方便测试
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async {
                Self::new(&database_url)
                    .await
                    .expect("Failed to connect to database")
            })
    }
}
