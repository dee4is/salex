// use std::collections::HashMap;

// use axum::extract::{Path, Query, State};
// use futures::TryStreamExt;
// use proto::prisma::*;
// use serde::Deserialize;

// use crate::base::extractors::{self, auth::AuthData, bincode::Bincode, Result};

// use super::AppState;

// #[derive(Deserialize)]
// pub struct Batch {
//     count: usize,
// }

// impl Default for Batch {
//     fn default() -> Self {
//         Self { count: 1 }
//     }
// }

// pub async fn insert_storageable(
//     Path((cell, product)): Path<(String, String)>,
//     auth: AuthData,
//     State(state): State<AppState>,
//     batch: Option<Query<Batch>>,
// ) -> Result<Bincode<HashMap<usize, String>>> {
//     let count = batch.unwrap_or_default().count;
//     let mut items = vec![];
//     for _ in 0..count {
//         items.push(Storageable {
//             product: product.clone(),
//             cell: cell.clone(),
//             _id: ObjectId::default().to_hex(),
//             ..Default::default()
//         })
//     }
//     let col = state
//         .mongo
//         .database(&auth.organization)
//         .collection::<proto::storage::Storageable>("storageable");
//     let res = col.insert_many(items, None).await?;

//     Ok(extractors::bincode::Bincode(
//         res.inserted_ids
//             .into_iter()
//             .map(|(k, v)| {
//                 (
//                     k,
//                     ObjectId::parse_str(v.as_str().unwrap()).unwrap().to_hex(),
//                 )
//             })
//             .collect(),
//     ))
// }

// pub async fn consume_storageable(
//     State(state): State<AppState>,
//     auth: AuthData,
//     Bincode(ids): Bincode<Vec<String>>,
// ) -> Result<Bincode<u64>> {
//     let col = state
//         .mongo
//         .database(&auth.organization)
//         .collection::<proto::storage::Storageable>("storageable");

//     let res = col.delete_many(doc! {"_id": {"$in": ids}}, None).await?;

//     Ok(extractors::bincode::Bincode(res.deleted_count))
// }

// pub async fn get_remainders(
//     State(state): State<AppState>,
//     auth: AuthData,
//     Bincode(product_ids): Bincode<Vec<String>>,
// ) -> Result<Bincode<HashMap<String, u64>>> {
//     let col = state
//         .mongo
//         .database(&auth.organization)
//         .collection::<proto::storage::Storageable>("storageable");

//     let mut remainders = HashMap::default();
//     for product_id in product_ids {
//         let count = col
//             .count_documents(doc! {"product": &product_id}, None)
//             .await?;
//         remainders.insert(product_id, count);
//     }

//     Ok(extractors::bincode::Bincode(remainders))
// }

// pub async fn get_storageables(
//     Path((warehouse_id, cell_id)): Path<(String, String)>,
//     auth: AuthData,
//     State(state): State<AppState>,
// ) -> Result<Bincode<Vec<Storageable>>> {
//     let col = state
//         .mongo
//         .database(&auth.organization)
//         .collection::<Storageable>("storageable");
//     let mut docs = vec![];
//     let mut cursor = col
//         .find(doc! {"cell._id": cell_id, "warehouse": warehouse_id}, None)
//         .await?;
//     while let Some(doc) = cursor.try_next().await? {
//         docs.push(doc);
//     }

//     Ok(extractors::bincode::Bincode(docs))
// }

// pub async fn scan_storageable(
//     Path((cell_id, storageable_id)): Path<(String, String)>,
//     auth: AuthData,
//     State(state): State<AppState>,
// ) -> Result<Bincode<u64>> {
//     let col = state
//         .mongo
//         .database(&auth.organization)
//         .collection::<proto::storage::Cell>("storageable");

//     let res = col
//         .update_one(
//             doc! {"_id": storageable_id},
//             doc! { "$set": {
//                 "cell": cell_id
//             } },
//             None,
//         )
//         .await?;

//     Ok(extractors::bincode::Bincode(res.modified_count))
// }

// #[cfg(test)]
// mod tests {
//     use axum::{body::Body, http::Request};
//     use std::net::TcpListener;
//     use std::{collections::HashMap, net::SocketAddr};

//     #[tokio::test]
//     async fn add_storageable() -> anyhow::Result<()> {
//         let listener = TcpListener::bind("127.0.0.1:8000".parse::<SocketAddr>().unwrap()).unwrap();
//         let addr = listener.local_addr().unwrap();

//         let app = crate::routes::router().await.unwrap();

//         tokio::spawn(async move {
//             axum::Server::from_tcp(listener)
//                 .unwrap()
//                 .serve(app.into_make_service())
//                 .await
//                 .unwrap();
//         });

//         let client = hyper::Client::new();

//         let warehouse_id = "empty";

//         let response = client
//             .request(
//                 Request::builder()
//                     .method("PUT")
//                     .uri(format!("http://{addr}/{warehouse_id}/cells/A10"))
//                     .body(Body::empty())
//                     .unwrap(),
//             )
//             .await
//             .unwrap();

//         let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
//         dbg!(&body);
//         let cell_id: String = bincode::deserialize(&body).unwrap();
//         dbg!(&cell_id);
//         let product_id = String::from("0");
//         let response = client
//             .request(
//                 Request::builder()
//                     .method("PUT")
//                     .uri(format!(
//                         "http://{addr}/{cell_id}/product/{product_id}?count=60",
//                     ))
//                     .body(Body::empty())
//                     .unwrap(),
//             )
//             .await
//             .unwrap();
//         let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
//         dbg!(&body);
//         let inserted: HashMap<usize, String> = bincode::deserialize(&body).unwrap();
//         dbg!(inserted);

//         Ok(())
//     }
// }
