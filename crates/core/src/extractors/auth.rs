use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

pub struct AuthData {
    pub organization: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthData
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        if let Some(organization) = parts.headers.get(proto::organization::header::ORGANIZATION) {
            Ok(AuthData {
                organization: organization.clone().to_str().unwrap().to_string(),
            })
        } else {
            Err((
                StatusCode::BAD_REQUEST,
                format!(
                    "{} header is missing",
                    proto::organization::header::ORGANIZATION
                ),
            ))
        }
    }
}
