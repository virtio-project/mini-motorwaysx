use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub steam_id: u64,
    pub name: String,
    pub score: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Statics {
    pub steam_name: String,
    pub steam_id: u64,
    pub build_id: u64,
    pub game_language: String,
    pub app_owner: u64,
}
