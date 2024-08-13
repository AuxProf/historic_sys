use super::model::{CreateChat, TitleChat};
use crate::AppState;
use actix_web::web;
use sqlx::{postgres::PgRow, Row};
use uuid::Uuid;

pub async fn get_chat_list(
    app_state: web::Data<AppState>,
    user_id: web::Path<Uuid>,
) -> Vec<TitleChat>{
    let result =
        sqlx::query("SELECT thread_id, title FROM chats WHERE user_id = $1 ORDER BY created_at ASC")
            .bind(&user_id.into_inner())
            .fetch_all(&app_state.postgress_cli)
            .await;

    match result {
        Ok(chats) => 
            chats
                .iter()
                .map(|chat| TitleChat {
                    thread_id: chat.get("thread_id"),
                    title: chat.get("title"),
                })
                .collect::<Vec<TitleChat>>(),
       
        Err(_) => Vec::new(),
    }
}

pub async fn create(
    app_state: web::Data<AppState>,
    chat: web::Json<CreateChat>,
    thread_id: String,
) -> Result<PgRow, sqlx::Error> {
    let now = chrono::offset::Utc::now();
    sqlx::query(
        "INSERT INTO chats (id, user_id, title, thread_id, created_at) VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(&chat.user_id)
    .bind(&chat.title)
    .bind(&thread_id)
    .bind(&now)
    .fetch_one(&app_state.postgress_cli)
    .await
}

pub async fn delete(
    app_state: web::Data<AppState>,
    thread_id: web::Path<Uuid>,
) -> Result<PgRow, sqlx::Error> {
    //TODO: Chamada do GPT para remover thread
    sqlx::query("DELETE FROM chats WHERE thread_id = $1 RETURNING *")
        .bind(&thread_id.into_inner())
        .fetch_one(&app_state.postgress_cli)
        .await
}

pub async fn update_thread_id(
    app_state: web::Data<AppState>,
    thread_id: String,
    new_thread_id: String,
) -> Result<PgRow, sqlx::Error> {
    sqlx::query("UPDATE chats SET thread_id = $2 WHERE thread_id = $1 RETURNING *")
        .bind(&thread_id)
        .bind(&new_thread_id)
        .fetch_one(&app_state.postgress_cli)
        .await
}
