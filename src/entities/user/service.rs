use super::model::{CreateUser, User};
use crate::AppState;
use actix_web::web;
use uuid::Uuid;
use sqlx::Row;

pub async fn create(
    app_state: web::Data<AppState>,
    user: web::Json<CreateUser>,
) -> Result<User, sqlx::Error> {
    let id = Uuid::new_v4();
    sqlx::query("INSERT INTO users (id, email) VALUES ($1, $2) RETURNING *")
        .bind(id)
        .bind(&user.email)
        .fetch_one(&app_state.postgress_cli)
        .await
        .map(|_| User {
            id,
            email: user.email.clone(),
        })
}

pub async fn get_user(
    app_state: web::Data<AppState>,
    email: web::Path<String>,
) -> Option<User> {
    let eml = email.into_inner();

    // Query SQL corrigida com o placeholder correto
    let result = sqlx::query("SELECT id FROM users WHERE email = $1")
        .bind(&eml)
        .fetch_one(&app_state.postgress_cli)
        .await;

    match result {
        Ok(row) => {
            // Converte a linha para a struct User
            let user = User {
                id: row.get("id"),   // Acessa o valor da coluna "id"
                email: eml,          // Usa o email passado no path
            };
            Some(user)
        }
        Err(_) => None,
    }
}
