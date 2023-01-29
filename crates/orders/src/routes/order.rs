use axum::extract::State;
use salex_core::extractors::{self, auth::AuthData, bincode::Bincode, Result};

use mongodb::bson::{doc, oid::ObjectId};
use proto::order::Order;

use super::AppState;

pub async fn insert_order(
    State(state): State<AppState>,
    auth: AuthData,
    Bincode(orders): Bincode<Vec<Order>>,
) -> Result<Bincode<Vec<String>>> {
    let col = state
        .mongo
        .database(&auth.organization)
        .collection::<Order>("orders");

    let mut ids = vec![];

    let orders = orders
        .into_iter()
        .map(|mut o| {
            o._id = ObjectId::default().to_hex();
            ids.push(o._id.clone());
            o
        })
        .collect::<Vec<Order>>();

    col.insert_many(&orders, None).await?;

    tokio::spawn(async move {
        let index = state.meili.index("orders");
        index.add_documents(&orders, Some("_id")).await.unwrap();
    });

    Ok(extractors::bincode::Bincode(ids))
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
