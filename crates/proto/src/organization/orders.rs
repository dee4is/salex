#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct Config {
    pub strategy: Strategy,
}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub enum Strategy {
    RoundRobin,
    #[default]
    None, // Anyone can take order
}
