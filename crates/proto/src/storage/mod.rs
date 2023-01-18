use crate::{product::Product, warehouse::Warehouse};

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct Storageable {
    pub _id: String,
    pub product: String,
    pub cell: String,
    pub warehouse: String,
}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct Cell {
    pub _id: String,
    pub name: String,
    pub warehouse: String,
}
