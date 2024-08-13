use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local};
use crate::entities::file::model::ListFile;
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
    pub thread_id: String,
    pub title: String
}

#[derive(Serialize, Deserialize)]
pub struct FilesChat {
    pub id: Uuid,
    pub title: String,
    pub files: Vec<ListFile>
}

#[derive(Serialize, Deserialize)]
pub struct GPTInfo {
    pub assistent_id: String,
    pub thread_id: String
}

#[derive(Serialize, Deserialize)]
pub struct CreateChat {
    pub title: String,
    pub user_id: Uuid,
}