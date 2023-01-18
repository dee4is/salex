use axum::{http::StatusCode, response::IntoResponse};

pub mod bincode;

// Make our own error that wraps `anyhow::Error`.
#[derive(Debug)]
pub struct Error(anyhow::Error);

pub type Result<T> = core::result::Result<T, Error>;

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for Error
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
