use speedy::{Readable, Writable};

use crate::warehouse::Warehouse;

#[derive(
    serde::Serialize, serde::Deserialize, Readable, Writable, Default, Clone, PartialEq, Eq,
)]
pub struct Storageable {
    pub _id: String,
    pub product_id: String,
    pub cell: Cell,
}

#[derive(
    serde::Serialize, serde::Deserialize, Readable, Writable, Default, Clone, PartialEq, Eq,
)]
pub struct Cell {
    pub _id: String,
    pub name: String,
    pub warehouse: Warehouse,
}
