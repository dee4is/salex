use crate::manager::Contact;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Customer {
    pub created_at: i64,
    pub id: String,
    pub name: String,
    pub surname: String,
    pub contacts: Vec<Contact>,
}
