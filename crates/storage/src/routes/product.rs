use std::collections::HashMap;

use axum::extract::{Path, Query, State};
use futures::TryStreamExt;
use mongodb::bson::{self, doc, Document};
use proto::{
    product::{FindProductsQuery, Product},
    storage::Storageable,
    Value,
};
use serde::Deserialize;

use crate::extractors::{self, speedy::Speedy, Result};

use super::AppState;

pub async fn get_products(
    State(state): State<AppState>,
    query: Query<FindProductsQuery>,
    Speedy(products): Speedy<Vec<String>>,
) -> Result<Speedy<Vec<Product>>> {
    let col = state
        .mongo
        .database("storage")
        .collection::<Product>("products");
    let mut pipeline = vec![];
    if products.len() > 0 {
        pipeline.push(doc! {
            "$match": {
                "_id": {"$in": products},
            }
        })
    }
    if let Some(_) = query.storage {
        pipeline.push(doc! {
            "$lookup": {
                "from": "storageable",
                "localField": "_id",
                "foreignField": "product",
                "as": "storage"
            }
        })
    }
    if let Some(warehouse) = &query.warehouse {
        pipeline.push(doc! {
                "$match": {
                    "warehouse": warehouse,
                }
        })
    }
    let mut cursor = col.aggregate(pipeline, None).await?;
    let mut docs: Vec<Product> = vec![];
    while let Some(doc) = cursor.try_next().await? {
        docs.push(bson::from_document(doc)?);
    }

    Ok(extractors::speedy::Speedy(docs))
}
