use axum::extract::State;
use salex_core::extractors::{self, auth::AuthData, bincode::Bincode, Result};

use mongodb::bson::oid::ObjectId;
use proto::{manager::Manager, organization::Organization};

use super::AppState;

pub async fn insert_organization(
    State(state): State<AppState>,
    Bincode(mut org): Bincode<Organization>,
) -> Result<Bincode<Manager>> {
    let col = state
        .mongo
        .database("main")
        .collection::<Organization>("organizations");

    org._id = ObjectId::default().to_hex();

    let mut director = Manager::default();

    director._id = ObjectId::default().to_hex();

    director.organization = org._id.clone();

    col.insert_one(&org, None).await?;

    let col = state
        .mongo
        .database(&director.organization)
        .collection::<Manager>("managers");

    col.insert_one(&director, None).await?;

    Ok(extractors::bincode::Bincode(director))
}

pub async fn insert_manager_to_organization(
    State(state): State<AppState>,
    auth: AuthData,
    Bincode(mut mng): Bincode<Manager>,
) -> Result<Bincode<String>> {
    let col = state
        .mongo
        .database(&auth.organization)
        .collection::<Manager>("managers");

    mng._id = ObjectId::default().to_hex();
    mng.organization = auth.organization;

    let res = col.insert_one(&mng, None).await?;

    Ok(extractors::bincode::Bincode(
        res.inserted_id.as_str().unwrap().to_string(),
    ))
}
