use axum::extract::State;
use salex_core::extractors::{self, auth::AuthData, bincode::Bincode, Result};

use mongodb::bson::{doc, oid::ObjectId};
use proto::order::Order;

use super::AppState;

pub async fn insert_order(
    State(state): State<AppState>,
    auth: AuthData,
    Bincode(mut order): Bincode<Order>,
) -> Result<Bincode<String>> {
    let col = state
        .mongo
        .database(&auth.organization)
        .collection::<Order>("orders");

    order._id = ObjectId::default().to_hex();

    col.insert_one(&order, None).await?;

    let id = order._id.clone();

    tokio::spawn(async move {
        let index = state.meili.index("orders");
        index.add_documents(&[order], Some("_id")).await.unwrap();
    });

    Ok(extractors::bincode::Bincode(id))
}

pub async fn update_order(
    State(state): State<AppState>,
    auth: AuthData,
    Bincode(order): Bincode<Order>,
) -> Result<Bincode<u64>> {
    let col = state
        .mongo
        .database(&auth.organization)
        .collection::<Order>("orders");

    let res = col
        .replace_one(doc! {"_id": &order._id}, &order, None)
        .await?;

    tokio::spawn(async move {
        let index = state.meili.index("orders");
        index.add_or_update(&[order], Some("_id")).await.unwrap();
    });

    Ok(extractors::bincode::Bincode(res.modified_count))
}
