use std::sync::Arc;

pub mod extractors;

pub mod config;

#[derive(Clone)]
pub struct AppState {
    pub prisma: Arc<proto::prisma::PrismaClient>,
    pub meili: meilisearch_sdk::Client,
    pub config: config::Config,
}

impl AppState {
    pub async fn new(config: config::Config) -> anyhow::Result<Self> {
        let meili = meilisearch_sdk::Client::new(&config.meili.uri, &config.meili.key);
        let prisma = Arc::new(proto::database::load(&config.mysql).await?);
        // Self::migrate(&pool).await?;
        Ok(Self {
            prisma,
            config,
            meili,
        })
    }

    // async fn migrate(pool: &MySqlPool) -> anyhow::Result<()> {
    //     proto::database::migrate(pool).await?;
    //     Ok(())
    // }
}
