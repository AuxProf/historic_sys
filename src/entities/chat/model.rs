use chrono::TimeDelta;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Chat {
    id: Uuid,
    user_id: Uuid,
    title: String,
    created_at: TimeDelta
}

#[derive(Serialize, Deserialize)]
pub struct TitleChat {
    id: Uuid,
    title: String
}

#[derive(Serialize, Deserialize)]
pub struct CreateChat {
    title: String
}
