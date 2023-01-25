use axum::extract::State;
use futures::future::join_all;
use salex_core::extractors::{self, auth::AuthData, bincode::Bincode, Result};

use mongodb::bson::oid::ObjectId;
use proto::{manager::Manager, organization::Organization, warehouse::Warehouse};

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

    let org_res = col.insert_one(&org, None);

    let col = state
        .mongo
        .database(&org._id)
        .collection::<Warehouse>("warehouses");

    let mut warehouse = Warehouse::default();

    warehouse._id = ObjectId::default().to_hex();

    let war_res = col.insert_one(&warehouse, None);

    let mut director = Manager::default();

    director._id = ObjectId::default().to_hex();

    director.organization = org._id.clone();

    director.warehouse = warehouse._id.clone();

    let col = state
        .mongo
        .database(&director.organization)
        .collection::<Manager>("managers");

    let mn_res = col.insert_one(&director, None);

    futures::try_join!(org_res, war_res, mn_res)?;

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
