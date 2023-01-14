use speedy::{Readable, Writable};

#[derive(serde::Serialize, serde::Deserialize, Readable, Writable, Clone, PartialEq, Eq)]
pub struct Status {}
#[derive(serde::Serialize, serde::Deserialize, Readable, Writable, Clone, PartialEq, Eq)]
pub struct Order {
    pub _id: String,
    pub customer: super::customer::Customer,
    pub products: Vec<super::product::Product>,
    pub manager: super::manager::Manager,
    pub status: Status,
    pub created_at: i64,
}
