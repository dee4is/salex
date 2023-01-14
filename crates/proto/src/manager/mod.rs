use speedy::{Readable, Writable};

use crate::warehouse::Warehouse;

#[derive(serde::Serialize, serde::Deserialize, Readable, Writable, Clone, PartialEq, Eq)]
pub struct Manager {
    pub _id: String,
    pub fullname: String,
    pub login: String,
    pub password: String,
    pub warehouse: Warehouse,
    pub contacts: Vec<Contact>,
    pub created_at: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Readable, Writable, Clone, PartialEq, Eq)]
pub enum Contact {
    Phone(String),
}
