use crate::warehouse::Warehouse;

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct Manager {
    pub _id: String,
    pub fullname: String,
    pub login: String,
    pub password: String,
    pub organization: String,
    pub warehouse: Warehouse,
    pub contacts: Vec<Contact>,
    pub created_at: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub enum Contact {
    Phone(String),
    #[default]
    None,
}
