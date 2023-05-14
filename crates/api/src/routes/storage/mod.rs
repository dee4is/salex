use axum::{
    routing::{delete, get, post, put},
    Router,
};
use tower_http::trace::TraceLayer;

use crate::base::AppState;
use crate::base::{config::Config, extractors::Result};

mod product;
mod storageable;
mod warehouse;

pub async fn router(config: &Config) -> Result<Router> {
    let state = AppState::new(config.clone()).await?;

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "storage=debug,tower_http=debug")
    }
    tracing_subscriber::fmt::init();

    Ok(Router::new()
        .layer(TraceLayer::new_for_http())
        // .route("/consume", delete(storageable::consume_storageable))
        // .route("/remainders", post(storageable::get_remainders))
        .route("/products", post(product::find_products))
        .route("/products", put(product::insert_products))
        .route("/cells/:warehouse_id/:cell_id", put(warehouse::insert_cell))
        // .route(
        //     "/storageables/:warehouse_id/:cell_id",
        //     get(storageable::get_storageables),
        // )
        // .route(
        //     "/scan/:cell_id/:storageable_id",
        //     post(storageable::scan_storageable),
        // )
        // .route(
        //     "/storageables/insert/:cell_id/:product_id",
        //     put(storageable::insert_storageable),
        // )
        .with_state(state))
}
