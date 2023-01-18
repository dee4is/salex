#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Configuration {}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Organization {
    pub _id: String,
    pub fullname: String,
    pub plan: Plan,
    pub director: String,
    pub integrations: Vec<String>,
    pub config: Option<Configuration>,
    pub created_at: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub enum Plan {
    Free = 0,
    Pro = 1,
}
