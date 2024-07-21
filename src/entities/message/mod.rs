use chrono::TimeDelta;
use uuid::Uuid;
use crate::traits::crud::Crud;

pub struct Message {
    id: Uuid,
    chat_id: Uuid,
    role: String,
    content: String,
    created_at: TimeDelta
}


impl Crud for Message {
}