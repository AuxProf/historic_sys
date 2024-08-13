use super::model::{CreateUser, User};
use crate::AppState;
use actix_web::web;
use uuid::Uuid;

pub async fn create(
    app_state: web::Data<AppState>,
    user: web::Json<CreateUser>,
) -> Result<User, sqlx::Error> {
    let id = Uuid::new_v4();
    sqlx::query("INSERT INTO users (id, email) VALUES ($1, $2) RETURNING *")
        .bind(id)
        .bind(&user.email)
        .fetch_one(&app_state.postgress_cli)
        .await.map(|_| User {
            id,
            email: user.email.clone()
        })
}
