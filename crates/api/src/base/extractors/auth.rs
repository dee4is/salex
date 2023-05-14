use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

pub struct AuthData {
    pub organization: i32,
    pub manager: i32,
}

const MANAGER_HEADER: &str = "X-Manager";
const ORGANIZATION_HEADER: &str = "X-Organization";

#[async_trait]
impl<S> FromRequestParts<S> for AuthData
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let organization = parts
            .headers
            .get(ORGANIZATION_HEADER)
            .ok_or((
                StatusCode::BAD_REQUEST,
                format!("{} header is missing", ORGANIZATION_HEADER),
            ))
            .unwrap();
        let manager = parts
            .headers
            .get(MANAGER_HEADER)
            .ok_or((
                StatusCode::BAD_REQUEST,
                format!("{} header is missing", MANAGER_HEADER),
            ))
            .unwrap();
        Ok(AuthData {
            organization: organization.clone().to_str().unwrap().parse().unwrap(),
            manager: manager.clone().to_str().unwrap().parse().unwrap(),
        })
    }
}
