use axum::{
    async_trait,
    body::{self, Body, Bytes},
    extract::FromRequest,
    http::Request,
    response::{IntoResponse, Response},
};
use speedy::{LittleEndian, Readable, Writable};

pub struct Speedy<T>(pub T);

#[async_trait]
impl<S, B, T: for<'a> Readable<'a, LittleEndian>> FromRequest<S, B> for Speedy<T>
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
        let proto = T::read_from_buffer(&body)
            .map_err(|e| super::Error(anyhow::anyhow!(e)).into_response())?;

        Ok(Self(proto))
    }
}

impl<T: Writable<LittleEndian>> IntoResponse for Speedy<T> {
    fn into_response(self) -> Response {
        let res = self.0.write_to_vec().unwrap();

        // its often easiest to implement `IntoResponse` by calling other implementations
        Response::builder()
            .header("Content-Type", "application/speedy")
            .body(body::boxed(Body::from(res)))
            .unwrap()
    }
}
