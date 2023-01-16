#![allow(clippy::not_unsafe_ptr_arg_deref)]

// Speedy
use speedy::{Readable, Writable};

pub mod customer;
pub mod manager;
pub mod order;
pub mod organization;
pub mod product;
pub mod storage;
pub mod warehouse;

#[derive(
    serde::Serialize, serde::Deserialize, Default, Readable, Writable, Clone, PartialEq, Eq,
)]
#[serde(untagged)]
pub enum Value<T> {
    Data(T),
    Id(String),
    #[default]
    Default,
}
