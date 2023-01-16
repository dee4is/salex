use speedy::{Readable, Writable};

use crate::storage::Storageable;

#[derive(
    serde::Serialize, serde::Deserialize, Default, Readable, Writable, Clone, PartialEq, Eq,
)]
pub struct Dimensions {
    pub width: i32,
    pub height: i32,
    pub length: i32,
    pub weight: i32,
}
#[derive(
    serde::Serialize, serde::Deserialize, Default, Readable, Writable, Clone, PartialEq, Eq,
)]
pub struct Product {
    pub _id: String,
    pub name: String,
    pub store_id: String,
    pub photos: Vec<String>,
    pub price: String,
    pub quantity: i32,
    pub dimensions: Dimensions,
    pub group: Option<Group>,
    pub controlled_by_group: bool,
    pub storage: Option<Vec<Storageable>>,
    pub created_at: i64,
}

#[derive(
    serde::Serialize, serde::Deserialize, Default, Readable, Writable, Clone, PartialEq, Eq,
)]
pub struct Group {
    pub _id: String,
    pub name: String,
    pub price: String,
    pub dimensions: Dimensions,
    pub created_at: i64,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct FindProductsQuery {
    pub storage: Option<u8>,
    pub warehouse: Option<String>,
}
