use chrono::TimeDelta;
use uuid::Uuid;

pub struct Message {
    id: Uuid,
    chat_id: Uuid,
    role: String,
    content: String,
    created_at: TimeDelta
}