pub mod config;
pub mod extractors;

#[derive(Clone)]
pub struct AppState {
    pub mongo: mongodb::Client,
    pub meili: meilisearch_sdk::Client,
    pub config: config::Config,
}

impl AppState {
    pub async fn new(config: config::Config) -> anyhow::Result<Self> {
        let options = mongodb::options::ClientOptions::parse(&config.mongo).await?;
        let mongo = mongodb::Client::with_options(options)?;
        let meili = meilisearch_sdk::Client::new(&config.meili.uri, &config.meili.key);
        Ok(Self {
            mongo,
            config,
            meili,
        })
    }
}
