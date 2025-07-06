use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub app: AppSettings,
}

#[derive(Debug, Deserialize)]
pub struct AppSettings {
    pub url: String,
    pub port: u16,
}


#[derive(Debug, Deserialize)]
pub struct FieldRequest {
    pub field: String,
}

#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    pub(crate) query: String,
}