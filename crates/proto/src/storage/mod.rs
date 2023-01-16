use speedy::{Readable, Writable};

use crate::{product::Product, warehouse::Warehouse, Value};

#[derive(
    serde::Serialize, serde::Deserialize, Readable, Writable, Default, Clone, PartialEq, Eq,
)]
pub struct Storageable {
    pub _id: String,
    pub product: Value<Product>,
    pub cell: Value<Cell>,
    pub warehouse: Value<Warehouse>,
}

#[derive(
    serde::Serialize, serde::Deserialize, Readable, Writable, Default, Clone, PartialEq, Eq,
)]
pub struct Cell {
    pub _id: String,
    pub name: String,
    pub warehouse: Value<Warehouse>,
}
