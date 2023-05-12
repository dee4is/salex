pub mod extractors;

pub mod config;
use sqlx::MySqlPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: MySqlPool,
    pub meili: meilisearch_sdk::Client,
    pub config: config::Config,
}

impl AppState {
    pub async fn new(config: config::Config) -> anyhow::Result<Self> {
        let meili = meilisearch_sdk::Client::new(&config.meili.uri, &config.meili.key);
        let pool = proto::database::load(&config.mysql).await?;
        Self::migrate(&pool).await?;
        Ok(Self {
            pool,
            config,
            meili,
        })
    }

    async fn migrate(pool: &MySqlPool) -> anyhow::Result<()> {
        proto::database::migrate(pool).await?;
        Ok(())
    }
}
