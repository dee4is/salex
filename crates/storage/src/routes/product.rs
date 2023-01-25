use std::collections::HashMap;

use axum::extract::{Query, State};
use futures::TryStreamExt;
use mongodb::bson::{self, doc, oid::ObjectId, Bson};
use proto::product::{FindProductsQuery, Product};

use salex_core::extractors::{self, auth::AuthData, bincode::Bincode, Result};

use super::AppState;

pub async fn get_products(
    State(state): State<AppState>,
    auth: AuthData,
    query: Query<FindProductsQuery>,
    Bincode(products): Bincode<Vec<String>>,
) -> Result<Bincode<Vec<Product>>> {
    let col = state
        .mongo
        .database(&auth.organization)
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
                    "storage.warehouse": warehouse,
                }
        })
    }
    let mut cursor = col.aggregate(pipeline, None).await?;
    let mut docs: Vec<Product> = vec![];
    while let Some(doc) = cursor.try_next().await? {
        docs.push(bson::from_document(doc)?);
    }

    Ok(extractors::bincode::Bincode(docs))
}

pub async fn insert_products(
    auth: AuthData,
    State(state): State<AppState>,
    Bincode(mut products): Bincode<Vec<Product>>,
) -> Result<Bincode<HashMap<usize, Bson>>> {
    let col = state
        .mongo
        .database(&auth.organization)
        .collection::<Product>("products");

    products
        .iter_mut()
        .for_each(|p| p._id = ObjectId::default().to_hex());

    let res = col.insert_many(&products, None).await?;

    tokio::spawn(async move {
        state
            .config
            .meili
            .insert_documents(format!("{}-products", auth.organization), &products)
            .await
            .unwrap();
    });

    Ok(extractors::bincode::Bincode(res.inserted_ids))
}
