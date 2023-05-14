use crate::prisma::PrismaClient;

pub async fn load(url: &str) -> anyhow::Result<PrismaClient> {
    Ok(PrismaClient::_builder().build().await?)
}

// pub async fn migrate(pool: &MySqlPool) -> anyhow::Result<()> {
//     sqlx::migrate!().run(pool).await?;
//     Ok(())
// }
