use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub mongo: String,
}

impl Default for Config {
    fn default() -> Self {
        config::Config::builder()
            .add_source(config::File::with_name("config.toml"))
            .build()
            .unwrap()
            .try_deserialize()
            .unwrap()
    }
}
