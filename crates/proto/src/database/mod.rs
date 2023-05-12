use sqlx::{mysql::MySqlPoolOptions, MySqlPool};

pub async fn load(url: &str) -> anyhow::Result<MySqlPool> {
    Ok(MySqlPoolOptions::new()
        .max_connections(5)
        .connect(url)
        .await?)
}

pub async fn migrate(pool: &MySqlPool) -> anyhow::Result<()> {
    sqlx::migrate!().run(pool).await?;
    Ok(())
}
