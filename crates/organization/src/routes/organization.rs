use axum::extract::State;
use salex_core::extractors::{self, auth::AuthData, bincode::Bincode, Result};

use mongodb::bson::{doc, oid::ObjectId, to_bson};
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

    let warehouse = Warehouse {
        _id: ObjectId::default().to_hex(),
        ..Default::default()
    };

    let war_res = col.insert_one(&warehouse, None);

    let director = Manager {
        _id: ObjectId::default().to_hex(),
        organization: org._id.clone(),
        warehouse: warehouse._id.clone(),
        ..Default::default()
    };

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

pub async fn update_organization_config(
    State(state): State<AppState>,
    auth: AuthData,
    Bincode(conf): Bincode<proto::organization::Configuration>,
) -> Result<Bincode<String>> {
    let col = state
        .mongo
        .database(&auth.organization)
        .collection::<Manager>("managers");
    let manager = col
        .find_one(doc! {"_id": auth.manager}, None)
        .await?
        .unwrap();

    if manager.acl.organization {
        let col = state
            .mongo
            .database(&auth.organization)
            .collection::<Manager>("organizations");
        col.update_one(
            doc! {"_id": auth.organization},
            doc! {"$set": {"config": to_bson(&conf).unwrap()}},
            None,
        )
        .await?;
    } else {
        return Err(anyhow::anyhow!("Not have permissions").into());
    }

    Ok(extractors::bincode::Bincode("".into()))
}
