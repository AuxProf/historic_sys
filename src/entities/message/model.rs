use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct ContentMessage {
    pub content: String
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub chat_id: Uuid,
    pub role: String,
    pub content: String,
    pub created_at: DateTime<Local>
}

#[derive(Serialize, Deserialize)]
pub struct CreateMessage {
    pub role: String,
    pub content: String,
}
