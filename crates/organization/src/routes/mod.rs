use axum::{
    routing::{delete, get, post, put},
    Router,
};
use mongodb::{options::ClientOptions, Client};
use tower_http::trace::TraceLayer;

use salex_core::config::Config;
use salex_core::extractors::Result;

mod organization;

#[derive(Clone)]
pub struct AppState {
    mongo: Client,
    config: Config,
}

pub async fn router() -> Result<Router> {
    let config = Config::default();
    let options = ClientOptions::parse(&config.mongo).await?;
    let mongo = Client::with_options(options)?;
    let state = AppState { mongo, config };

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "storage=debug,tower_http=debug")
    }
    tracing_subscriber::fmt::init();

    Ok(Router::new()
        .layer(TraceLayer::new_for_http())
        .with_state(state))
}
