use std::collections::HashMap;

use super::model::{CreateChat, TitleChat};
use crate::AppState;
use actix_web::web;
use sqlx::{postgres::PgRow, Row};
use uuid::Uuid;

pub async fn get_chat_list(
    app_state: web::Data<AppState>,
    user_id: Uuid,
) -> Vec<TitleChat> {
    let chats = sqlx::query(
        "SELECT id, thread_id, title FROM chats WHERE user_id = $1 ORDER BY created_at ASC",
    )
    .bind(&user_id)
    .fetch_all(&app_state.postgress_cli)
    .await
    .unwrap_or_else(|_| vec![]);

    // Extrair os IDs dos chats como Uuid
    let chat_ids: Vec<Uuid> = chats
        .iter()
        .map(|chat| chat.get::<Uuid, _>("id"))
        .collect();

    let files = sqlx::query(
        "SELECT cf.chat_id, cf.file_id, f.name FROM chat_file cf LEFT JOIN files f ON f.file_id = cf.file_id WHERE cf.chat_id = ANY($1)",
    )
    .bind(&chat_ids)
    .fetch_all(&app_state.postgress_cli)
    .await
    .unwrap_or_else(|_| vec![]);

    let mut files_map: HashMap<Uuid, Vec<String>> = HashMap::new();

    for file in files {
        let chat_id: Uuid = file.get("chat_id");
        let entry = files_map.entry(chat_id).or_insert_with(Vec::new);
        entry.push(file.get("file_id"));
    }

    let chat_list = chats
        .into_iter()
        .map(|chat| {
            let id: Uuid = chat.get("id");
            TitleChat {
                thread_id: chat.get("thread_id"),
                title: chat.get("title"),
                files: files_map.remove(&id).unwrap_or_else(Vec::new),
            }
        })
        .collect::<Vec<TitleChat>>();

    chat_list
}

pub async fn create(
    app_state: web::Data<AppState>,
    chat: web::Json<CreateChat>,
    thread_id: String,
) -> Result<TitleChat, sqlx::Error> {
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
    .map(|_| TitleChat {
        thread_id: thread_id,
        title: chat.title.clone(),
        files: Vec::new()
    })
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
