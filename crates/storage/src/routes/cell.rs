use crate::extractors::{self, bincode::Bincode, Result};
use axum::extract::{Path, State};
use futures::stream::TryStreamExt;

use mongodb::bson::{doc, oid::ObjectId};
use proto::storage::{Cell, Storageable};

use super::AppState;

pub async fn new_cell(
    State(state): State<AppState>,
    Bincode(mut cell): Bincode<Cell>,
) -> Result<Bincode<String>> {
    let col = state
        .mongo
        .database("storage")
        .collection::<proto::storage::Cell>("cells");

    cell._id = ObjectId::default().to_string();

    let res = col.insert_one(&cell, None).await?;

    Ok(extractors::bincode::Bincode(
        ObjectId::parse_str(res.inserted_id.as_str().unwrap())
            .unwrap()
            .to_hex(),
    ))
}

pub async fn scan_storageable(
    Path((cell_id, storageable_id)): Path<(String, String)>,
    State(state): State<AppState>,
) -> Result<Bincode<u64>> {
    let col = state
        .mongo
        .database("storage")
        .collection::<proto::storage::Cell>("storageable");

    let res = col
        .update_one(
            doc! {"_id": storageable_id},
            doc! { "$set": {
                "cell_id": cell_id
            } },
            None,
        )
        .await?;

    Ok(extractors::bincode::Bincode(res.modified_count))
}

pub async fn get_storageables(
    Path(cell_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Bincode<Vec<Storageable>>> {
    let col = state
        .mongo
        .database("storage")
        .collection::<Storageable>("storageable");
    let mut docs = vec![];
    let mut cursor = col.find(doc! {"cell": cell_id}, None).await?;
    while let Some(doc) = cursor.try_next().await? {
        docs.push(doc);
    }

    Ok(extractors::bincode::Bincode(docs))
}
