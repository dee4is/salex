use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

pub struct AuthData {
    pub organization: String,
    pub manager: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthData
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let organization = parts
            .headers
            .get(proto::organization::header::ORGANIZATION)
            .ok_or((
                StatusCode::BAD_REQUEST,
                format!(
                    "{} header is missing",
                    proto::organization::header::ORGANIZATION
                ),
            ))
            .unwrap();
        let manager = parts
            .headers
            .get(proto::manager::header::MANAGER)
            .ok_or((
                StatusCode::BAD_REQUEST,
                format!(
                    "{} header is missing",
                    proto::organization::header::ORGANIZATION
                ),
            ))
            .unwrap();
        Ok(AuthData {
            organization: organization.clone().to_str().unwrap().to_string(),
            manager: manager.clone().to_str().unwrap().to_string(),
        })
    }
}
