pub mod header {
    pub const MANAGER: &str = "X-Manager-Id";
}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct Manager {
    pub _id: String,
    pub fullname: String,
    pub login: String,
    pub password: String,
    pub organization: String,
    pub warehouse: String, // Warehouse
    pub contacts: Vec<Contact>,
    pub acl: ACL,
    pub created_at: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub enum Contact {
    Phone(String),
    #[default]
    None,
}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct ACL {
    pub orders: bool,
    pub statistic: bool,
    pub storage: bool,
    pub rights: bool, // Can edit ACL for others
    pub calls: bool,
    pub organization: bool, // Edit any info in organization
}
