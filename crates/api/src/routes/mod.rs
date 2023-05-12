use axum::{
    routing::{patch, put},
    Router,
};
use tower_http::trace::TraceLayer;

use crate::base::extractors::Result;
use crate::base::{config::Config, AppState};

mod order;
mod organization;
mod storage;

pub async fn router() -> Result<Router> {
    let config = Config::default();
    let state = AppState::new(config).await?;

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "orders=debug,tower_http=debug")
    }
    tracing_subscriber::fmt::init();

    Ok(Router::new()
        .layer(TraceLayer::new_for_http())
        .route("/", put(order::insert_order))
        .route("/", patch(order::update_order))
        .with_state(state))
}
