pub mod header {
    pub const ORGANIZATION: &str = "X-Organization-Id";
}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct Configuration {}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct Organization {
    pub _id: String,
    pub fullname: String,
    pub plan: Plan,
    pub integrations: Vec<String>,
    pub config: Option<Configuration>,
    pub created_at: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub enum Plan {
    #[default]
    Free = 0,
    Pro = 1,
}
