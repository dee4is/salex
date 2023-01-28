use axum::routing::patch;
use axum::{routing::put, Router};
use tower_http::trace::TraceLayer;

use salex_core::extractors::Result;
use salex_core::{config::Config, AppState};

mod organization;

pub async fn router() -> Result<Router> {
    let config = Config::default();
    let state = AppState::new(config).await?;

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "storage=debug,tower_http=debug")
    }
    tracing_subscriber::fmt::init();

    Ok(Router::new()
        .layer(TraceLayer::new_for_http())
        .route("/", put(organization::insert_organization))
        .route(
            "/manager",
            put(organization::insert_manager_to_organization),
        )
        .route("/config", patch(organization::update_organization_config))
        .with_state(state))
}
