use std::collections::HashMap;

use speedy::{Readable, Writable};

use crate::manager::Contact;

#[derive(
    serde::Serialize, serde::Deserialize, Readable, Writable, Default, Clone, PartialEq, Eq,
)]
pub struct Warehouse {
    pub _id: String,
    pub address: String,
    pub contacts: HashMap<String, Vec<Contact>>, // Role -> Vec<Contact>
    pub about: String,
    pub schedule: String,
}
