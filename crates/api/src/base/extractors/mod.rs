use axum::{http::StatusCode, response::IntoResponse};

pub mod auth;
pub mod bincode;

// Make our own error that wraps `anyhow::Error`.
#[derive(Debug)]
pub enum Error {
    Any(anyhow::Error),
    Bad((StatusCode, String)),
}

pub type Result<T> = core::result::Result<T, Error>;

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Error::Any(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong: {e}"),
            )
                .into_response(),
            Error::Bad(e) => (StatusCode::BAD_REQUEST, e).into_response(),
        }
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for Error
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self::Any(err.into())
    }
}
