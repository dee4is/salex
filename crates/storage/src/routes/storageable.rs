use std::collections::HashMap;

use axum::extract::{Path, Query, State};
use futures::TryStreamExt;
use mongodb::bson::{self, doc};
use proto::{product::Product, storage::Storageable};
use serde::Deserialize;

use crate::extractors::{self, speedy::Speedy, Result};

use super::AppState;

#[derive(Deserialize)]
pub struct Batch {
    count: usize,
}

impl Default for Batch {
    fn default() -> Self {
        Self { count: 1 }
    }
}

pub async fn add_storageable(
    Path((cell_id, product_id)): Path<(String, String)>,
    State(state): State<AppState>,
    batch: Option<Query<Batch>>,
) -> Result<Speedy<HashMap<usize, String>>> {
    let count = batch.unwrap_or_default().count;
    let item = Storageable {
        product_id,
        cell_id,
        ..Default::default()
    };
    let mut items = vec![];
    for _ in 1..count {
        items.push(&item)
    }
    let col = state
        .mongo
        .database("storage")
        .collection::<proto::storage::Storageable>("storageable");
    let res = col.insert_many(items, None).await?;

    Ok(extractors::speedy::Speedy(
        res.inserted_ids
            .into_iter()
            .map(|(k, v)| (k, v.to_string()))
            .collect(),
    ))
}

pub async fn consume_storageable(
    State(state): State<AppState>,
    Speedy(ids): Speedy<Vec<String>>,
) -> Result<Speedy<u64>> {
    let col = state
        .mongo
        .database("storage")
        .collection::<proto::storage::Storageable>("storageable");

    let res = col.delete_many(doc! {"_id": {"$in": ids}}, None).await?;

    Ok(extractors::speedy::Speedy(res.deleted_count))
}

pub async fn get_products_by_storageables(
    Speedy(storageable_ids): Speedy<Vec<String>>,
    State(state): State<AppState>,
) -> Result<Speedy<Vec<Product>>> {
    let col = state
        .mongo
        .database("storage")
        .collection::<proto::storage::Storageable>("storageable");

    let mut cursor = col
        .aggregate(
            vec![
                doc! {
                    "$match": {
                        "_id": {"$in": storageable_ids},
                    }
                },
                doc! {
                    "$lookup": {
                        "from": "storageable",
                        "localField": "product_id",
                        "foreignField": "_id",
                        "as": "products"
                    }
                },
            ],
            None,
        )
        .await?;
    let mut docs: Vec<Product> = vec![];
    while let Some(doc) = cursor.try_next().await? {
        docs.push(bson::from_document(doc.get_document("products")?.clone())?);
    }
    todo!("Need research how exacly aggregations work");

    Ok(extractors::speedy::Speedy(docs))
}

pub async fn get_remainders(
    State(state): State<AppState>,
    Speedy(product_ids): Speedy<Vec<String>>,
) -> Result<Speedy<HashMap<String, u64>>> {
    let col = state
        .mongo
        .database("storage")
        .collection::<proto::storage::Storageable>("storageable");

    let mut remainders = HashMap::default();
    for product_id in product_ids {
        let count = col
            .count_documents(doc! {"product_id": &product_id}, None)
            .await?;
        remainders.insert(product_id, count);
    }

    Ok(extractors::speedy::Speedy(remainders))
}
