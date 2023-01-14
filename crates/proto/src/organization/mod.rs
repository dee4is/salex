use speedy::{Readable, Writable};

#[derive(serde::Serialize, serde::Deserialize, Readable, Writable, Clone, PartialEq, Eq)]
pub struct Configuration {}

#[derive(serde::Serialize, serde::Deserialize, Readable, Writable, Clone, PartialEq, Eq)]
pub struct Organization {
    pub _id: String,
    pub fullname: String,
    pub plan: Plan,
    pub director: String,
    pub integrations: Vec<String>,
    pub config: Option<Configuration>,
    pub created_at: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Readable, Writable, Clone, PartialEq, Eq)]
pub enum Plan {
    Free = 0,
    Pro = 1,
}
