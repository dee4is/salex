use speedy::{Readable, Writable};

#[derive(serde::Serialize, serde::Deserialize, Readable, Writable, Clone, PartialEq, Eq)]
pub struct Dimensions {
    pub width: i32,
    pub height: i32,
    pub length: i32,
    pub weight: i32,
}
#[derive(serde::Serialize, serde::Deserialize, Readable, Writable, Clone, PartialEq, Eq)]
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
    pub created_at: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Readable, Writable, Clone, PartialEq, Eq)]
pub struct Group {
    pub _id: String,
    pub name: String,
    pub price: String,
    pub dimensions: Dimensions,
    pub created_at: i64,
}
