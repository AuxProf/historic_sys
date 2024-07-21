use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String
}

#[derive(Serialize, Deserialize)]
pub struct CreateUser {
    pub email: String
}

