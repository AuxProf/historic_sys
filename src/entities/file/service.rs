use super::model::{CreateFile, File};
use crate::AppState;
use actix_web::web;
use sqlx::{postgres::PgRow, Row};
use uuid::Uuid;

pub async fn create(
    app_state: web::Data<AppState>,
    file: web::Json<CreateFile>,
    file_id: String,
) -> Option<File> {
    let now = chrono::offset::Utc::now();
    let result: Result<sqlx::postgres::PgRow, sqlx::Error> =  sqlx::query(
        "INSERT INTO files (id, user_id, name, file_id, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(&file.user_id)
    .bind(&file.name)
    .bind(&file_id)
    .bind(&now)
    .fetch_one(&app_state.postgress_cli)
    .await;

    match result {
        Ok(f) => Some(File {
            id: f.get("id"),
            user_id: file.user_id,
            name: file.name.clone(),
            file_id: file_id,
            created_at: now.into(),
        }),
        Err(_) => None,
    }
}

pub async fn delete(app_state: web::Data<AppState>, file_id: String) -> Result<PgRow, sqlx::Error> {
    sqlx::query("DELETE FROM files WHERE file_id = $1 RETURNING *")
        .bind(&file_id)
        .fetch_one(&app_state.postgress_cli)
        .await
}

pub async fn atach_file(
    app_state: web::Data<AppState>,
    chat_id: Uuid,
    file_id: String,
) -> Option<String> {
    let result: Result<sqlx::postgres::PgRow, sqlx::Error> =
        sqlx::query("SELECT * FROM chat_file WHERE chat_id = $1 AND file_id = $2")
            .bind(&chat_id)
            .bind(&file_id)
            .fetch_one(&app_state.postgress_cli)
            .await;

    match result {
        Ok(_) => {
            let _ = sqlx::query("DELETE FROM chat_file WHERE chat_id = $1 AND file_id = $2")
                .bind(&chat_id)
                .bind(&file_id)
                .execute(&app_state.postgress_cli)
                .await;
            Some(String::from("Conexão removida"))
        }
        Err(sqlx::Error::RowNotFound) => {
            let _ = sqlx::query("INSERT INTO chat_file (chat_id, file_id) VALUES ($1, $2)")
                .bind(&chat_id)
                .bind(&file_id)
                .execute(&app_state.postgress_cli)
                .await;
            Some(String::from("Conexão estabelecida"))
        }
        Err(_) => None,
    }
}
