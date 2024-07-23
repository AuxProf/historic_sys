use chrono::TimeDelta;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct File {
    id: Uuid,
    user_id: Uuid,
    name: String,
    file_id: String,
    file_path: String,
    file_content: String,
    created_at: TimeDelta
}

#[derive(Serialize, Deserialize)]
pub struct CreateFile {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
pub struct ListFile {
    id: Uuid,
    name: String
}

