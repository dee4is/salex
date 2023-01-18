use std::collections::HashMap;


use crate::manager::Contact;

#[derive(
    serde::Serialize, serde::Deserialize, Default, Clone
)]
pub struct Warehouse {
    pub _id: String,
    pub address: String,
    pub contacts: HashMap<String, Vec<Contact>>, // Role -> Vec<Contact>
    pub about: String,
    pub schedule: String,
}
