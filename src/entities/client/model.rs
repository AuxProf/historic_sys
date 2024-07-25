use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Client {
    pub id: Uuid,
    pub login: String,
    pub password: String
}

#[derive(Serialize, Deserialize)]
pub struct BaseClient {
    pub login: String,
    pub password: String
}


#[derive(Debug, Serialize, Deserialize)]
pub struct JWTClaims {
    pub exp: usize,
    pub login: String
}
