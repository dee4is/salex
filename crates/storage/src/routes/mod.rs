use axum::{
    routing::{delete, post, put},
    Router,
};
use mongodb::{options::ClientOptions, Client};

use crate::extractors::Result;

mod cell;
mod storageable;

#[derive(Clone)]
pub struct AppState {
    mongo: Client,
}

pub async fn router() -> Result<Router> {
    let options = ClientOptions::parse(std::env::var("MONGO_URI")?).await?;
    let mongo = Client::with_options(options)?;
    let state = AppState { mongo };
    Ok(Router::new()
        .route("/consume", delete(storageable::consume_storageable))
        .route("/cell", put(cell::new_cell))
        .route("/remainders", post(storageable::get_remainders))
        .route("/:cell_id/:storageable_id", post(cell::scan_storageable))
        .route("/:cell_id/:product_id", put(storageable::add_storageable))
        .with_state(state))
}
