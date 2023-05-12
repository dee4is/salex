pub mod database;
mod entities;

pub use entities::*;

#[async_trait::async_trait]
pub trait Entitie<T> {
    const TABLE_NAME: &'static str;

    async fn find(pool: &sqlx::MySqlPool) -> anyhow::Result<Vec<T>>;
    async fn find_by_id(pool: &sqlx::MySqlPool, id: i32) -> anyhow::Result<T>;
    async fn insert(pool: &sqlx::MySqlPool, item: T) -> anyhow::Result<()>;
    async fn update(pool: &sqlx::MySqlPool, item: T) -> anyhow::Result<()>;
    async fn delete(pool: &sqlx::MySqlPool, id: i32) -> anyhow::Result<()>;
    async fn count(pool: &sqlx::MySqlPool) -> anyhow::Result<i64>;
    async fn exists(pool: &sqlx::MySqlPool, id: i32) -> anyhow::Result<bool>;
}
