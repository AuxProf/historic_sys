use chrono::TimeDelta;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct ContentMessage {
    content: String
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    id: Uuid,
    chat_id: Uuid,
    role: String,
    content: String,
    created_at: TimeDelta
}

#[derive(Serialize, Deserialize)]
pub struct CreateMessage {
    role: String,
    content: String,
}
