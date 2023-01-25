use hyper::{Body, Request};
use serde::Serialize;

use crate::config::Meili;
use crate::extractors::Result;

impl Meili {
    pub async fn insert_documents<T: Serialize>(&self, index: String, docs: &Vec<T>) -> Result<()> {
        let client = hyper::Client::new();
        let payload = serde_json::to_vec(docs)?;
        client
            .request(
                Request::builder()
                    .method("PUT")
                    .uri(&format!("{}/indexes/{index}/documents", self.uri))
                    .body(Body::from(payload))?,
            )
            .await?;
        Ok(())
    }
}
