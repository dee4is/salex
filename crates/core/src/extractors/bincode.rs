use axum::{
    async_trait,
    body::{self, Body, Bytes},
    extract::FromRequest,
    http::Request,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

pub struct Bincode<T>(pub T);

#[async_trait]
impl<S, B, T: for<'a> Deserialize<'a>> FromRequest<S, B> for Bincode<T>
where
    Bytes: FromRequest<S, B>,
    B: Send + 'static,
    S: Send + Sync,
{
    type Rejection = axum::response::Response;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let body = Bytes::from_request(req, state)
            .await
            .map_err(IntoResponse::into_response)?;
        let proto: T = bincode::deserialize(&body)
            .map_err(|e| super::Error::Any(anyhow::anyhow!(e)).into_response())?;

        Ok(Self(proto))
    }
}

impl<T: Serialize> IntoResponse for Bincode<T> {
    fn into_response(self) -> Response {
        let res = bincode::serialize(&self.0).unwrap();

        // its often easiest to implement `IntoResponse` by calling other implementations
        Response::builder()
            .header("Content-Type", "application/bincode")
            .body(body::boxed(Body::from(res)))
            .unwrap()
    }
}
