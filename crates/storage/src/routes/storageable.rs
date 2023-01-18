use std::collections::HashMap;

use axum::extract::{Path, Query, State};
use mongodb::bson::{doc, oid::ObjectId};
use proto::storage::Storageable;
use serde::Deserialize;

use crate::extractors::{self, bincode::Bincode, Result};

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
    Path((cell, product)): Path<(String, String)>,
    State(state): State<AppState>,
    batch: Option<Query<Batch>>,
) -> Result<Bincode<HashMap<usize, String>>> {
    let count = batch.unwrap_or_default().count;
    let mut items = vec![];
    for _ in 0..count {
        items.push(Storageable {
            product: product.clone(),
            cell: cell.clone(),
            _id: ObjectId::default().to_hex(),
            ..Default::default()
        })
    }
    let col = state
        .mongo
        .database("storage")
        .collection::<proto::storage::Storageable>("storageable");
    let res = col.insert_many(items, None).await?;

    Ok(extractors::bincode::Bincode(
        res.inserted_ids
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    ObjectId::parse_str(v.as_str().unwrap()).unwrap().to_hex(),
                )
            })
            .collect(),
    ))
}

pub async fn consume_storageable(
    State(state): State<AppState>,
    Bincode(ids): Bincode<Vec<String>>,
) -> Result<Bincode<u64>> {
    let col = state
        .mongo
        .database("storage")
        .collection::<proto::storage::Storageable>("storageable");

    let res = col.delete_many(doc! {"_id": {"$in": ids}}, None).await?;

    Ok(extractors::bincode::Bincode(res.deleted_count))
}

pub async fn get_remainders(
    State(state): State<AppState>,
    Bincode(product_ids): Bincode<Vec<String>>,
) -> Result<Bincode<HashMap<String, u64>>> {
    let col = state
        .mongo
        .database("storage")
        .collection::<proto::storage::Storageable>("storageable");

    let mut remainders = HashMap::default();
    for product_id in product_ids {
        let count = col
            .count_documents(doc! {"product": &product_id}, None)
            .await?;
        remainders.insert(product_id, count);
    }

    Ok(extractors::bincode::Bincode(remainders))
}

#[cfg(test)]
mod tests {
    use axum::{body::Body, http::Request};
    use std::net::TcpListener;
    use std::{collections::HashMap, net::SocketAddr};

    #[tokio::test]
    async fn add_storageable() -> anyhow::Result<()> {
        let listener = TcpListener::bind("127.0.0.1:8000".parse::<SocketAddr>().unwrap()).unwrap();
        let addr = listener.local_addr().unwrap();

        let app = crate::routes::router().await.unwrap();

        tokio::spawn(async move {
            axum::Server::from_tcp(listener)
                .unwrap()
                .serve(app.into_make_service())
                .await
                .unwrap();
        });

        let client = hyper::Client::new();

        let payload = proto::storage::Cell {
            _id: "".into(),
            name: "A10".into(),
            warehouse: "".into(),
        };

        let response = client
            .request(
                Request::builder()
                    .method("PUT")
                    .uri(format!("http://{addr}/cell"))
                    .body(Body::from(bincode::serialize(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        dbg!(&body);
        let cell_id: String = bincode::deserialize(&body).unwrap();
        dbg!(&cell_id);
        let product_id = String::from("0");
        let response = client
            .request(
                Request::builder()
                    .method("PUT")
                    .uri(format!(
                        "http://{addr}/{cell_id}/product/{product_id}?count=60",
                    ))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        dbg!(&body);
        let inserted: HashMap<usize, String> = bincode::deserialize(&body).unwrap();
        dbg!(inserted);

        Ok(())
    }
}
