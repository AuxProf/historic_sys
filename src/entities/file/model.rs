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
pub struct ShowFile {
    pub id: Uuid,
    pub name: String,
    pub file_id: String
}

#[derive(Serialize, Deserialize)]
pub struct CreateFile {
    pub name: String,
    pub file_id: String
}

#[derive(Serialize, Deserialize)]
pub struct ListFile {
    pub id: Uuid,
    pub name: String
}

