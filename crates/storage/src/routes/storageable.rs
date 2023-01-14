use proto::storage::Storageable;

use crate::extractors::{speedy::Speedy, Result};

pub async fn get(Speedy(body): Speedy<Storageable>) -> Result<Speedy<Storageable>> {
    Ok(Speedy(body))
}

pub async fn test() -> Result<Speedy<Storageable>> {
    Ok(Speedy(Storageable {
        _id: "test".into(),
        ..Default::default()
    }))
}
