use std::sync::Arc;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use tokio::sync::RwLock;

pub struct DbProvider(Arc<RwLock<Pool<Sqlite>>>);
impl DbProvider {
    async fn get_db_pool() -> Pool<Sqlite> {
        let db_name = std::env::var("DB_NAME").expect("DB_NAME must be set");
        SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&db_name)
            .await
            .expect("Failed to initialize SQLite")
    }

    pub async fn new() -> Self {
        Self(Arc::new(RwLock::new(Self::get_db_pool().await)))
    }

    pub async fn get_db(&self) -> Pool<Sqlite> {
        self.0.read().await.clone()
    }

    pub async fn reload(&self) {
        let mut db_pool = self.0.write().await;
        *db_pool = Self::get_db_pool().await;
    }
}

impl Clone for DbProvider {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}