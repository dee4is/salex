use axum::{
    routing::{delete, post, put},
    Router,
};
use mongodb::{options::ClientOptions, Client};

use crate::{config::Config, extractors::Result};

mod cell;
mod product;
mod storageable;

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
    Ok(Router::new()
        .route("/consume", delete(storageable::consume_storageable))
        .route("/cell", put(cell::new_cell))
        .route("/remainders", post(storageable::get_remainders))
        .route("/products", post(product::get_products))
        .route("/:cell_id/:storageable_id", post(cell::scan_storageable))
        .route("/:cell_id/:product_id", put(storageable::add_storageable))
        .with_state(state))
}
