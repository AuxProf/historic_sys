use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Chat {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub assistent_id: String,
    pub thread_id: String,
    pub created_at: DateTime<Local>
}

#[derive(Serialize, Deserialize)]
pub struct TitleChat {
    pub id: Uuid,
    pub title: String
}

#[derive(Serialize, Deserialize)]
pub struct CreateChat {
    pub title: String,
    pub assistent_id: String,
    pub thread_id: String
}

#[derive(Serialize, Deserialize)]
pub struct GPTInfo {
    pub assistent_id: String,
    pub thread_id: String
}
