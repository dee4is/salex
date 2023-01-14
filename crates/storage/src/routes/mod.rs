use axum::{
    routing::{get, post},
    Router,
};

mod storageable;

pub fn router() -> Router {
    Router::new()
        .route("/", post(storageable::get))
        .route("/", get(storageable::test))
}
