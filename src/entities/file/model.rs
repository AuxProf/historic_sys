use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct File {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub file_id: String,
    pub created_at: DateTime<Local>
}

#[derive(Serialize, Deserialize)]
pub struct CreateFile {
    pub name: String,
    pub user_id: Uuid
}

#[derive(Serialize, Deserialize)]
pub struct FileList {
    pub file_id: String,
    pub name: String
}

#[derive(Serialize, Deserialize)]
pub struct FileChat {
    pub file_id: String,
    pub chat_id: Uuid,
}