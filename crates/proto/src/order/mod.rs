#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Status {
    pub _id: String,
    pub slug: String,
    pub triggers: Vec<Trigger>,
    pub theme: Option<String>,
    pub updated_at: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Trigger {}
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Order {
    pub _id: String,
    pub customer: String,      // id of customer
    pub products: Vec<String>, // ids of products
    pub manager: String,
    pub status: Status,
    pub activity: Vec<String>,
    pub notes: Vec<Note>,
    pub created_at: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Note {
    pub manager: String,
    pub body: String,
    pub done: Option<bool>,
    pub assign: Option<String>, // Manager id
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Activity {
    pub manager: String,
    pub action: String,
    pub created_at: i64,
}
