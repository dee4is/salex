pub mod database;
// mod entities;
// Stops the client from outputing a huge number of warnings during compilation.
#[allow(warnings, unused)]
pub mod prisma;

// pub trait ToSql {
//     fn to_sql(&self) -> String;
// }

// #[async_trait::async_trait]
// pub trait Entitie<T> {
//     const TABLE_NAME: &'static str;
//     type Filter: ToSql;

//     async fn find(
//         pool: &sqlx::MySqlPool,
//         filter: Self::Filter,
//     ) -> BoxStream<Result<T, sqlx::Error>>;
//     async fn find_by_id(pool: &sqlx::MySqlPool, id: u64) -> anyhow::Result<T>;
//     async fn insert(
//         pool: &sqlx::MySqlPool,
//         meili: &meilisearch_sdk::Client,
//         item: T,
//     ) -> anyhow::Result<u64>;
//     async fn update(pool: &sqlx::MySqlPool, item: T) -> anyhow::Result<()>;
//     async fn delete(pool: &sqlx::MySqlPool, id: u64) -> anyhow::Result<()>;

//     async fn index(meili: &meilisearch_sdk::Client, item: &T) -> anyhow::Result<()>;
// }
